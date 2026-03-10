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
use crate::database::models::{Product, ProductStatus, CreateProductRequest, ProductFilter};
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

    let product = Product {
        id: product_id,
        seller_id: authenticated_id,
        title: req.title,
        description: req.description,
        price: req.price,
        location: req.location,
        coordinates: req.coordinates,
        category: req.category,
        tags: req.tags,
        images: req.images,
        video: None,
        condition: req.condition,
        status: ProductStatus::Active,
        created_at: now,
        updated_at: now,
    };

    {
        let mut products = db.products.lock().unwrap();
        products.insert(product_id, product.clone());
    }
    db.save_all();

    (StatusCode::CREATED, Json(product)).into_response()
}

pub async fn get_products(
    State(db): State<Arc<Database>>,
    Query(filter): Query<ProductFilter>,
) -> impl IntoResponse {
    let products_map = db.products.lock().unwrap();
    let mut products: Vec<Product> = products_map.values()
        .filter(|p| {
            if let Some(cat) = &filter.category {
                if p.category != *cat { return false; }
            }
            if let Some(sid) = &filter.seller_id {
                if p.seller_id != *sid { return false; }
            }
            if let Some(min) = filter.min_price {
                if p.price < min { return false; }
            }
            if let Some(max) = filter.max_price {
                if p.price > max { return false; }
            }
            if let Some(q) = &filter.query {
                let q_lower = q.to_lowercase();
                if !p.title.to_lowercase().contains(&q_lower) && !p.description.to_lowercase().contains(&q_lower) {
                    return false;
                }
            }

            // Geo-search: Haversine formula
            if let (Some(user_lat), Some(user_lng), Some(radius_km)) = (filter.latitude, filter.longitude, filter.radius_km) {
                if let Some(ref coords) = p.coordinates {
                    let dist = haversine_distance(user_lat, user_lng, coords.lat, coords.lng);
                    if dist > radius_km { return false; }
                } else {
                    return false; // Product has no location, exclude
                }
            }

            true
        })
        .cloned()
        .collect();

    // Sort by created_at desc
    products.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    Json(products).into_response()
}

fn haversine_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let r = 6371.0; // Earth radius in km
    let lat1_rad = lat1 * PI / 180.0;
    let lat2_rad = lat2 * PI / 180.0;
    let d_lat = (lat2 - lat1) * PI / 180.0;
    let d_lon = (lon2 - lon1) * PI / 180.0;

    let a = (d_lat / 2.0).sin().powi(2) + lat1_rad.cos() * lat2_rad.cos() * (d_lon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().asin();

    r * c
}

pub async fn get_product(
    Path(id): Path<Uuid>,
    State(db): State<Arc<Database>>,
) -> impl IntoResponse {
    let products = db.products.lock().unwrap();
    match products.get(&id) {
        Some(p) => Json(Some(p.clone())).into_response(),
        None => (StatusCode::NOT_FOUND, "Product not found").into_response(),
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

    let mut products = db.products.lock().unwrap();
    if let Some(product) = products.get_mut(&id) {
        if product.seller_id != authenticated_id {
            return (StatusCode::FORBIDDEN, "Not your product").into_response();
        }
        
        product.title = req.title;
        product.description = req.description;
        product.price = req.price;
        product.location = req.location;
        product.coordinates = req.coordinates;
        product.category = req.category;
        product.tags = req.tags;
        product.images = req.images;
        product.condition = req.condition;
        product.updated_at = Utc::now();
        
        let p_clone = product.clone();
        drop(products);
        db.save_all();
        (StatusCode::OK, Json(p_clone)).into_response()
    } else {
        (StatusCode::NOT_FOUND, "Product not found").into_response()
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

    let mut products = db.products.lock().unwrap();
    if let Some(product) = products.get(&id) {
        if product.seller_id != authenticated_id {
            return (StatusCode::FORBIDDEN, "Not your product").into_response();
        }
        products.remove(&id);
        drop(products);
        db.save_all();
        StatusCode::NO_CONTENT.into_response()
    } else {
        (StatusCode::NOT_FOUND, "Product not found").into_response()
    }
}
