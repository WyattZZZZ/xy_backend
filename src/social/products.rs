use axum::{
    extract::{State, Path, Query},
    Json,
    response::IntoResponse,
    http::{StatusCode, HeaderMap},
};
use uuid::Uuid;
use chrono::Utc;
use std::sync::Arc;
use std::f64::consts::PI;
use crate::database::Database;
use crate::database::models::{CreateProductRequest, ProductFilter, product_from_row};
use crate::verify::auth::get_user_id_from_token;

pub async fn create_product(
    State(db): State<Arc<Database>>,
    headers: HeaderMap,
    Json(req): Json<CreateProductRequest>,
) -> impl IntoResponse {
    let authenticated_id = match get_user_id_from_token(&headers) {
        Some(uid) => uid,
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    let now = Utc::now();
    let product_id = Uuid::new_v4();
    let lat = req.coordinates.as_ref().map(|c| c.lat);
    let lng = req.coordinates.as_ref().map(|c| c.lng);

    let result = sqlx::query(
        "INSERT INTO products (id, seller_id, title, description, price, location, lat, lng, category, tags, images, condition, status, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, 'ACTIVE', $13, $13)"
    )
    .bind(product_id)
    .bind(&authenticated_id)
    .bind(&req.title)
    .bind(&req.description)
    .bind(req.price)
    .bind(&req.location)
    .bind(lat)
    .bind(lng)
    .bind(&req.category)
    .bind(&req.tags)
    .bind(&req.images)
    .bind(&req.condition)
    .bind(now)
    .execute(&db.pool)
    .await;

    if result.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
    }

    let row = sqlx::query("SELECT * FROM products WHERE id = $1").bind(product_id).fetch_one(&db.pool).await;
    match row {
        Ok(r) => (StatusCode::CREATED, Json(product_from_row(&r))).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }
}

pub async fn get_products(
    State(db): State<Arc<Database>>,
    Query(filter): Query<ProductFilter>,
) -> impl IntoResponse {
    let mut sql = "SELECT * FROM products WHERE 1=1".to_string();

    if let Some(ref cat) = filter.category {
        let cat_esc = cat.replace('\'', "''");
        sql.push_str(&format!(" AND category = '{}'", cat_esc));
    }
    if let Some(ref sid) = filter.seller_id {
        let sid_esc = sid.replace('\'', "''");
        sql.push_str(&format!(" AND seller_id = '{}'", sid_esc));
    }
    if let Some(min) = filter.min_price {
        sql.push_str(&format!(" AND price >= {}", min));
    }
    if let Some(max) = filter.max_price {
        sql.push_str(&format!(" AND price <= {}", max));
    }
    if let Some(ref q) = filter.query {
        let q_esc = q.replace('\'', "''");
        sql.push_str(&format!(
            " AND (LOWER(title) LIKE LOWER('%{0}%') OR LOWER(description) LIKE LOWER('%{0}%'))",
            q_esc
        ));
    }
    sql.push_str(" ORDER BY created_at DESC");

    let rows = sqlx::query(&sql).fetch_all(&db.pool).await;
    match rows {
        Ok(rows) => {
            let mut products: Vec<_> = rows.iter().map(|r| product_from_row(r)).collect();

            // Apply geo-filter in memory (Haversine)
            if let (Some(user_lat), Some(user_lng), Some(radius_km)) =
                (filter.latitude, filter.longitude, filter.radius_km)
            {
                products.retain(|p| {
                    if let Some(ref coords) = p.coordinates {
                        haversine_distance(user_lat, user_lng, coords.lat, coords.lng) <= radius_km
                    } else {
                        false
                    }
                });
            }

            Json(products).into_response()
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }
}

fn haversine_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let r = 6371.0;
    let lat1_rad = lat1 * PI / 180.0;
    let lat2_rad = lat2 * PI / 180.0;
    let d_lat = (lat2 - lat1) * PI / 180.0;
    let d_lon = (lon2 - lon1) * PI / 180.0;
    let a = (d_lat / 2.0).sin().powi(2) + lat1_rad.cos() * lat2_rad.cos() * (d_lon / 2.0).sin().powi(2);
    r * 2.0 * a.sqrt().asin()
}

pub async fn get_product(
    Path(id): Path<Uuid>,
    State(db): State<Arc<Database>>,
) -> impl IntoResponse {
    let row = sqlx::query("SELECT * FROM products WHERE id = $1")
        .bind(id)
        .fetch_optional(&db.pool)
        .await;

    match row {
        Ok(Some(r)) => Json(Some(product_from_row(&r))).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Product not found").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }
}

pub async fn update_product(
    Path(id): Path<Uuid>,
    State(db): State<Arc<Database>>,
    headers: HeaderMap,
    Json(req): Json<CreateProductRequest>,
) -> impl IntoResponse {
    let authenticated_id = match get_user_id_from_token(&headers) {
        Some(uid) => uid,
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    let seller: Option<String> = sqlx::query_scalar("SELECT seller_id FROM products WHERE id = $1")
        .bind(id).fetch_optional(&db.pool).await.unwrap_or(None);

    match seller {
        None => return (StatusCode::NOT_FOUND, "Product not found").into_response(),
        Some(sid) if sid != authenticated_id => return (StatusCode::FORBIDDEN, "Not your product").into_response(),
        _ => {}
    }

    let now = Utc::now();
    let lat = req.coordinates.as_ref().map(|c| c.lat);
    let lng = req.coordinates.as_ref().map(|c| c.lng);

    let _ = sqlx::query(
        "UPDATE products SET title=$1, description=$2, price=$3, location=$4, lat=$5, lng=$6,
         category=$7, tags=$8, images=$9, condition=$10, updated_at=$11 WHERE id=$12"
    )
    .bind(&req.title).bind(&req.description).bind(req.price).bind(&req.location)
    .bind(lat).bind(lng).bind(&req.category).bind(&req.tags).bind(&req.images)
    .bind(&req.condition).bind(now).bind(id)
    .execute(&db.pool).await;

    let row = sqlx::query("SELECT * FROM products WHERE id = $1").bind(id).fetch_one(&db.pool).await;
    match row {
        Ok(r) => (StatusCode::OK, Json(product_from_row(&r))).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }
}

pub async fn delete_product(
    Path(id): Path<Uuid>,
    State(db): State<Arc<Database>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let authenticated_id = match get_user_id_from_token(&headers) {
        Some(uid) => uid,
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    let seller: Option<String> = sqlx::query_scalar("SELECT seller_id FROM products WHERE id = $1")
        .bind(id).fetch_optional(&db.pool).await.unwrap_or(None);

    match seller {
        None => return (StatusCode::NOT_FOUND, "Product not found").into_response(),
        Some(sid) if sid != authenticated_id => return (StatusCode::FORBIDDEN, "Not your product").into_response(),
        _ => {}
    }

    let _ = sqlx::query("DELETE FROM products WHERE id = $1").bind(id).execute(&db.pool).await;
    StatusCode::NO_CONTENT.into_response()
}

#[derive(serde::Serialize)]
pub struct FavoriteStatusResponse {
    #[serde(rename = "isFavorited")]
    pub is_favorited: bool,
}

pub async fn favorite_product(
    State(db): State<Arc<Database>>,
    headers: HeaderMap,
    Path(product_id): Path<Uuid>,
) -> impl IntoResponse {
    let uid = match get_user_id_from_token(&headers) {
        Some(uid) => uid,
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    let _ = sqlx::query(
        "INSERT INTO product_favorites (product_id, user_id, created_at) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING"
    )
    .bind(product_id).bind(&uid).bind(Utc::now())
    .execute(&db.pool).await;

    (StatusCode::OK, "Favorited").into_response()
}

pub async fn unfavorite_product(
    State(db): State<Arc<Database>>,
    headers: HeaderMap,
    Path(product_id): Path<Uuid>,
) -> impl IntoResponse {
    let uid = match get_user_id_from_token(&headers) {
        Some(uid) => uid,
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    let _ = sqlx::query("DELETE FROM product_favorites WHERE product_id = $1 AND user_id = $2")
        .bind(product_id).bind(&uid).execute(&db.pool).await;

    (StatusCode::OK, "Unfavorited").into_response()
}

pub async fn check_product_favorite(
    State(db): State<Arc<Database>>,
    headers: HeaderMap,
    Path(product_id): Path<Uuid>,
) -> impl IntoResponse {
    let uid = match get_user_id_from_token(&headers) {
        Some(uid) => uid,
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    let is_favorited: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM product_favorites WHERE product_id = $1 AND user_id = $2)"
    )
    .bind(product_id).bind(&uid)
    .fetch_one(&db.pool).await.unwrap_or(false);

    (StatusCode::OK, Json(FavoriteStatusResponse { is_favorited })).into_response()
}
