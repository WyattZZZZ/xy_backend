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
    Role, User, UserResponse, UpdateUserRequest, UserFilter
};
use crate::verify::auth::get_user_id_from_token;

// --- Routes ---

pub fn routes(db: Arc<Database>) -> Router {
    Router::new()
        .route("/", get(list_users_handler))
        .route("/:id", get(get_user_handler))
        .route("/:id", post(update_user_handler))
        .route("/:id", delete(delete_user_handler))
        .with_state(db)
}

async fn get_user_handler(
    State(db): State<Arc<Database>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let users = db.users.lock().unwrap();
    match users.get(&id) {
        Some(user) => {
            let mut resp = UserResponse::from(user.clone());
            resp.is_online = db.conns.contains_key(&id);
            (StatusCode::OK, Json(resp)).into_response()
        },
        None => (StatusCode::NOT_FOUND, "User not found").into_response(),
    }
}

async fn list_users_handler(
    State(db): State<Arc<Database>>,
    Query(filter): Query<UserFilter>,
) -> impl IntoResponse {
    let users_map = db.users.lock().unwrap();
    
    // First filter by role
    let filtered: Vec<User> = users_map.values()
        .filter(|u| {
            if let Some(role) = filter.role {
                u.role == role
            } else {
                true
            }
        })
        .cloned()
        .collect();

    // Then apply search
    let mut users = if let Some(ref query) = filter.query {
        search::search_users(&filtered, query)
    } else {
        filtered.iter().map(|u| UserResponse::from(u.clone())).collect()
    };

    // Finally apply sorting
    if let Some(sort) = filter.sort {
        match sort.as_str() {
            "createdAt:asc" => users.sort_by(|a, b| a.created_at.cmp(&b.created_at)),
            "createdAt:desc" => users.sort_by(|a, b| b.created_at.cmp(&a.created_at)),
            _ => {}
        }
    }

    // Map with online status
    let users: Vec<UserResponse> = users.into_iter().map(|mut u| {
        u.is_online = db.conns.contains_key(&u.id);
        u
    }).collect();

    Json(users)
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
         let users_check = db.users.lock().unwrap();
         let auth_user = users_check.get(&authenticated_id);
         let is_admin = auth_user.map(|u| u.role == Role::Admin).unwrap_or(false);
         if !is_admin {
            return (StatusCode::FORBIDDEN, "Permission denied").into_response();
         }
    }

    let mut users = db.users.lock().unwrap();
    if let Some(user) = users.get_mut(&id) {
        if let Some(name) = payload.name { user.name = name; }
        if let Some(avatar) = payload.avatar { user.avatar = avatar; }
        if let Some(gender) = payload.gender { user.gender = gender; }
        if let Some(bio) = payload.bio { user.bio = bio; }
        if let Some(location) = payload.location { user.location = location; }
        if let Some(posts) = payload.posts { user.posts = posts; }
        if let Some(following) = payload.following { user.following = following; }
        if let Some(fans) = payload.fans { user.fans = fans; }
        if let Some(rating) = payload.rating { user.rating = rating; }
        if let Some(reviews) = payload.reviews_count { user.reviews_count = reviews; }
        if let Some(coords) = payload.coordinates { user.coordinates = Some(coords); }
        if let Some(verified) = payload.is_verified { user.is_verified = verified; }
        if let Some(role) = payload.role { user.role = role; }
        
        user.updated_at = Utc::now();
        let resp = UserResponse::from(user.clone());
        drop(users);
        db.save_all();
        (StatusCode::OK, Json(resp)).into_response()
    } else {
        (StatusCode::NOT_FOUND, "User not found").into_response()
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

    let mut users = db.users.lock().unwrap();
    let is_admin = users.get(&authenticated_id).map(|u| u.role == Role::Admin).unwrap_or(false);
    if authenticated_id != id && !is_admin {
        return (StatusCode::FORBIDDEN, "Permission denied").into_response();
    }

    if users.remove(&id).is_some() {
        drop(users);
        db.save_all();
        StatusCode::NO_CONTENT.into_response()
    } else {
        (StatusCode::NOT_FOUND, "User not found").into_response()
    }
}
