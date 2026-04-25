pub mod search;

use axum::{
    routing::{get, post, delete},
    Json, Router,
    http::{StatusCode, HeaderMap},
    response::IntoResponse,
    extract::{Path, State, Query},
};
use std::sync::Arc;
use chrono::Utc;
use crate::database::Database;
use crate::database::models::{
    UserResponse, UpdateUserRequest, UserFilter, user_from_row, post_from_row, product_from_row,
};
use crate::verify::auth::get_user_id_from_token;

pub fn routes(db: Arc<Database>) -> Router {
    Router::new()
        .route("/", get(list_users_handler))
        .route("/:id", get(get_user_handler))
        .route("/:id", post(update_user_handler))
        .route("/:id", delete(delete_user_handler))
        .route("/:id/favorites", get(get_user_favorites))
        .with_state(db)
}

async fn get_user_handler(
    State(db): State<Arc<Database>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let row = sqlx::query("SELECT * FROM users WHERE id = $1")
        .bind(&id)
        .fetch_optional(&db.pool)
        .await;

    match row {
        Ok(Some(r)) => {
            let mut resp = UserResponse::from(user_from_row(&r));
            resp.is_online = db.conns.contains_key(&id);
            (StatusCode::OK, Json(resp)).into_response()
        }
        Ok(None) => (StatusCode::NOT_FOUND, "User not found").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }
}

async fn list_users_handler(
    State(db): State<Arc<Database>>,
    Query(filter): Query<UserFilter>,
) -> impl IntoResponse {
    let mut sql = "SELECT * FROM users WHERE 1=1".to_string();
    if let Some(role) = filter.role {
        let role_str = match role {
            crate::database::models::Role::Admin => "ADMIN",
            crate::database::models::Role::User => "USER",
        };
        sql.push_str(&format!(" AND role = '{}'", role_str));
    }
    if let Some(ref q) = filter.query {
        let q_escaped = q.replace('\'', "''");
        sql.push_str(&format!(
            " AND (LOWER(name) LIKE LOWER('%{0}%') OR LOWER(bio) LIKE LOWER('%{0}%') OR LOWER(location) LIKE LOWER('%{0}%') OR LOWER(email) LIKE LOWER('%{0}%'))",
            q_escaped
        ));
    }
    match filter.sort.as_deref() {
        Some("createdAt:asc") => sql.push_str(" ORDER BY created_at ASC"),
        Some("createdAt:desc") | _ => sql.push_str(" ORDER BY created_at DESC"),
    }

    let rows = sqlx::query(&sql).fetch_all(&db.pool).await;
    match rows {
        Ok(rows) => {
            let users: Vec<UserResponse> = rows.iter().map(|r| {
                let mut u = UserResponse::from(user_from_row(r));
                u.is_online = db.conns.contains_key(&u.id);
                u
            }).collect();
            Json(users).into_response()
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }
}

async fn update_user_handler(
    State(db): State<Arc<Database>>,
    Path(id): Path<String>,
    headers: HeaderMap,
    Json(payload): Json<UpdateUserRequest>,
) -> impl IntoResponse {
    let authenticated_id = match get_user_id_from_token(&headers) {
        Some(uid) => uid,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    if authenticated_id != id {
        let row = sqlx::query("SELECT role FROM users WHERE id = $1")
            .bind(&authenticated_id)
            .fetch_optional(&db.pool)
            .await;
        let is_admin = row.ok().flatten().map(|r| {
            let role: String = sqlx::Row::get(&r, "role");
            role == "ADMIN"
        }).unwrap_or(false);
        if !is_admin {
            return (StatusCode::FORBIDDEN, "Permission denied").into_response();
        }
    }

    let now = Utc::now();

    let name = payload.name;
    let avatar = payload.avatar;
    let gender = payload.gender;
    let bio = payload.bio;
    let location = payload.location;
    let posts = payload.posts;
    let following = payload.following;
    let fans = payload.fans;
    let rating = payload.rating;
    let reviews_count = payload.reviews_count;
    let coordinates = payload.coordinates;
    let is_verified = payload.is_verified;
    let role = payload.role;

    let result = sqlx::query(
        "UPDATE users SET
            name = COALESCE($1, name),
            avatar = COALESCE($2, avatar),
            gender = COALESCE($3, gender),
            bio = COALESCE($4, bio),
            location = COALESCE($5, location),
            posts = COALESCE($6, posts),
            following = COALESCE($7, following),
            fans = COALESCE($8, fans),
            rating = COALESCE($9, rating),
            reviews_count = COALESCE($10, reviews_count),
            lat = COALESCE($11, lat),
            lng = COALESCE($12, lng),
            is_verified = COALESCE($13, is_verified),
            role = COALESCE($14, role),
            updated_at = $15
         WHERE id = $16"
    )
    .bind(name)
    .bind(avatar)
    .bind(gender)
    .bind(bio)
    .bind(location)
    .bind(posts)
    .bind(following)
    .bind(fans)
    .bind(rating)
    .bind(reviews_count)
    .bind(coordinates.as_ref().map(|c| c.lat))
    .bind(coordinates.as_ref().map(|c| c.lng))
    .bind(is_verified)
    .bind(role.map(|r| match r {
        crate::database::models::Role::Admin => "ADMIN",
        crate::database::models::Role::User => "USER",
    }))
    .bind(now)
    .bind(&id)
    .execute(&db.pool)
    .await;

    match result {
        Ok(r) if r.rows_affected() == 0 => (StatusCode::NOT_FOUND, "User not found").into_response(),
        Ok(_) => {
            let row = sqlx::query("SELECT * FROM users WHERE id = $1")
                .bind(&id)
                .fetch_one(&db.pool)
                .await;
            match row {
                Ok(r) => (StatusCode::OK, Json(UserResponse::from(user_from_row(&r)))).into_response(),
                Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
            }
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }
}

async fn delete_user_handler(
    State(db): State<Arc<Database>>,
    Path(id): Path<String>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let authenticated_id = match get_user_id_from_token(&headers) {
        Some(uid) => uid,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    if authenticated_id != id {
        let row = sqlx::query("SELECT role FROM users WHERE id = $1")
            .bind(&authenticated_id)
            .fetch_optional(&db.pool)
            .await;
        let is_admin = row.ok().flatten().map(|r| {
            let role: String = sqlx::Row::get(&r, "role");
            role == "ADMIN"
        }).unwrap_or(false);
        if !is_admin {
            return (StatusCode::FORBIDDEN, "Permission denied").into_response();
        }
    }

    let result = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(&id)
        .execute(&db.pool)
        .await;

    match result {
        Ok(r) if r.rows_affected() == 0 => (StatusCode::NOT_FOUND, "User not found").into_response(),
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }
}

pub async fn get_user_favorites(
    State(db): State<Arc<Database>>,
    Path(user_id): Path<String>,
) -> impl IntoResponse {
    #[derive(serde::Serialize)]
    struct CombinedFavorites {
        products: Vec<crate::database::models::Product>,
        posts: Vec<crate::database::models::Post>,
    }

    let product_rows = sqlx::query(
        "SELECT p.* FROM products p
         JOIN product_favorites pf ON p.id = pf.product_id
         WHERE pf.user_id = $1"
    )
    .bind(&user_id)
    .fetch_all(&db.pool)
    .await
    .unwrap_or_default();

    let post_rows = sqlx::query(
        "SELECT p.* FROM posts p
         JOIN post_favorites pf ON p.id = pf.post_id
         WHERE pf.user_id = $1"
    )
    .bind(&user_id)
    .fetch_all(&db.pool)
    .await
    .unwrap_or_default();

    Json(CombinedFavorites {
        products: product_rows.iter().map(|r| product_from_row(r)).collect(),
        posts: post_rows.iter().map(|r| post_from_row(r)).collect(),
    })
}
