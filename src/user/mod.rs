use axum::{
    routing::{get, post, delete},
    Json, Router,
    http::{StatusCode, HeaderMap},
    response::IntoResponse,
    extract::{Path, State, Query},
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

// --- Models ---

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Role {
    User,
    Admin,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Coordinates {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub email: String,
    pub password_hash: String,
    pub name: String,
    pub avatar: String,
    pub gender: String,
    pub bio: String,
    pub location: String,
    pub posts: i32,
    pub following: i32,
    pub fans: i32,
    pub rating: f32,
    pub reviews_count: i32,
    pub coordinates: Option<Coordinates>,
    pub is_verified: bool,
    pub role: Role,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub name: String,
    pub avatar: String,
    pub gender: String,
    pub bio: String,
    pub location: String,
    pub posts: i32,
    pub following: i32,
    pub fans: i32,
    pub rating: f32,
    pub reviews_count: i32,
    pub coordinates: Option<Coordinates>,
    pub is_verified: bool,
    pub role: Role,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id,
            email: user.email,
            name: user.name,
            avatar: user.avatar,
            gender: user.gender,
            bio: user.bio,
            location: user.location,
            posts: user.posts,
            following: user.following,
            fans: user.fans,
            rating: user.rating,
            reviews_count: user.reviews_count,
            coordinates: user.coordinates,
            is_verified: user.is_verified,
            role: user.role,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub avatar: Option<String>,
    pub gender: Option<String>, // Added gender field
    pub bio: Option<String>,
    pub location: Option<String>,
    pub posts: Option<i32>,
    pub following: Option<i32>,
    pub fans: Option<i32>,
    pub rating: Option<f32>,
    pub reviews_count: Option<i32>,
    pub coordinates: Option<Coordinates>,
    pub is_verified: Option<bool>,
    pub role: Option<Role>,
}

#[derive(Deserialize)]
pub struct UserFilter {
    pub role: Option<Role>,
    pub sort: Option<String>, // e.g., "created_at:desc"
}

pub type UserDb = Arc<Mutex<HashMap<String, User>>>;

// --- Routes ---

pub fn routes(state: UserDb) -> Router {
    Router::new()
        .route("/", get(list_users_handler))
        .route("/me", get(get_me_handler))
        .route("/:id", get(get_user_handler))
        .route("/:id", post(update_user_handler))
        .route("/:id", delete(delete_user_handler))
        .with_state(state)
}

// --- Auth Utilities (duplicated from auth.rs for now, or move to a common module) ---
// In a real app, these would be in a middleware or common library
const JWT_SECRET: &[u8] = b"secret_key_change_me_in_production";

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

fn get_user_id_from_token(headers: &HeaderMap) -> Option<String> {
    let auth_header = headers.get("Authorization")?.to_str().ok()?;
    if !auth_header.starts_with("Bearer ") {
        return None;
    }
    let token = &auth_header[7..];
    let token_data = jsonwebtoken::decode::<Claims>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(JWT_SECRET),
        &jsonwebtoken::Validation::default(),
    ).ok()?;
    Some(token_data.claims.sub)
}

// --- Handlers ---

async fn get_user_handler(
    State(db): State<UserDb>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let users = db.lock().unwrap();
    match users.get(&id) {
        Some(user) => (StatusCode::OK, Json(UserResponse::from(user.clone()))).into_response(),
        None => (StatusCode::NOT_FOUND, "User not found").into_response(),
    }
}

async fn list_users_handler(
    State(db): State<UserDb>,
    Query(filter): Query<UserFilter>,
) -> impl IntoResponse {
    let users_map = db.lock().unwrap();
    let mut users: Vec<UserResponse> = users_map.values()
        .filter(|u| {
            if let Some(role) = filter.role {
                u.role == role
            } else {
                true
            }
        })
        .map(|u| UserResponse::from(u.clone()))
        .collect();

    // Sort if needed
    if let Some(sort) = filter.sort {
        match sort.as_str() {
            "createdAt:asc" => users.sort_by(|a, b| a.created_at.cmp(&b.created_at)),
            "createdAt:desc" => users.sort_by(|a, b| b.created_at.cmp(&a.created_at)),
            _ => {}
        }
    }

    Json(users)
}

async fn get_me_handler(
    State(db): State<UserDb>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let user_id = match get_user_id_from_token(&headers) {
        Some(id) => id,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let users = db.lock().unwrap();
    match users.get(&user_id) {
        Some(user) => (StatusCode::OK, Json(UserResponse::from(user.clone()))).into_response(),
        None => (StatusCode::UNAUTHORIZED, "Invalid user").into_response(),
    }
}

async fn update_user_handler(
    State(db): State<UserDb>,
    Path(id): Path<String>,
    headers: HeaderMap,
    Json(payload): Json<UpdateUserRequest>,
) -> impl IntoResponse {
    let authenticated_id = match get_user_id_from_token(&headers) {
        Some(uid) => uid,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    // Permission check: only self or admin can update (simple check)
    // In a full app, we'd check if auth_user.role == Admin
    if authenticated_id != id {
         // Check if authenticated user is admin
         let users_check = db.lock().unwrap();
         let auth_user = users_check.get(&authenticated_id);
         let is_admin = auth_user.map(|u| u.role == Role::Admin).unwrap_or(false);
         if !is_admin {
            return (StatusCode::FORBIDDEN, "Permission denied").into_response();
         }
    }

    let mut users = db.lock().unwrap();
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
        (StatusCode::OK, Json(UserResponse::from(user.clone()))).into_response()
    } else {
        (StatusCode::NOT_FOUND, "User not found").into_response()
    }
}

async fn delete_user_handler(
    State(db): State<UserDb>,
    Path(id): Path<String>,
    headers: HeaderMap,
) -> impl IntoResponse {
     let authenticated_id = match get_user_id_from_token(&headers) {
        Some(uid) => uid,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    // Only Admin can delete or self-delete
    let mut users = db.lock().unwrap();
    
    let is_admin = users.get(&authenticated_id).map(|u| u.role == Role::Admin).unwrap_or(false);
    if authenticated_id != id && !is_admin {
        return (StatusCode::FORBIDDEN, "Permission denied").into_response();
    }

    if users.remove(&id).is_some() {
        StatusCode::NO_CONTENT.into_response()
    } else {
        (StatusCode::NOT_FOUND, "User not found").into_response()
    }
}
