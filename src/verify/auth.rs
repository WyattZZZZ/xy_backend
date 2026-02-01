use axum::{
    routing::post,
    Json, Router,
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use jsonwebtoken::{encode, Header, EncodingKey};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Utc, Duration};
use crate::user::{User, UserDb, Role};

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String, // This will be used as the initial name
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(serde::Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: AuthUserResponse,
}

#[derive(serde::Serialize)]
pub struct AuthUserResponse {
    pub id: String,
    pub username: String,
    pub email: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

const JWT_SECRET: &[u8] = b"secret_key_change_me_in_production";

pub fn routes(db: UserDb) -> Router {
    Router::new()
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .with_state(db)
}

async fn register_handler(
    axum::extract::State(db): axum::extract::State<UserDb>,
    Json(payload): Json<RegisterRequest>,
) -> impl IntoResponse {
    let mut users = db.lock().unwrap();

    if users.values().any(|u| u.email == payload.email) {
        return (StatusCode::BAD_REQUEST, "Email already registered").into_response();
    }

    let hashed_password = match hash(payload.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Error hashing password").into_response(),
    };

    let user_id = uuid::Uuid::new_v4().to_string();
    let now = Utc::now();

    let new_user = User {
        id: user_id.clone(),
        email: payload.email.clone(),
        password_hash: hashed_password,
        name: payload.username.clone(), // Initialize name with username
        avatar: "https://picsum.photos/id/64/200/200".to_string(), // Default avatar
        gender: "other".to_string(), // Default gender
        bio: "".to_string(),
        location: "".to_string(),
        posts: 0,
        following: 0,
        fans: 0,
        rating: 0.0,
        reviews_count: 0,
        coordinates: None,
        is_verified: false,
        role: Role::User,
        created_at: now,
        updated_at: now,
    };

    users.insert(user_id.clone(), new_user);

    // Generate JWT for auto-login
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.clone(),
        exp: expiration,
    };

    let token = match encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_SECRET)) {
        Ok(t) => t,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Error generating token").into_response(),
    };

    let response = AuthResponse {
        token,
        user: AuthUserResponse {
            id: user_id,
            username: payload.username,
            email: payload.email,
        },
    };

    (StatusCode::CREATED, Json(response)).into_response()
}

async fn login_handler(
    axum::extract::State(db): axum::extract::State<UserDb>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    let users = db.lock().unwrap();
    
    let user = match users.values().find(|u| u.email == payload.email) {
        Some(u) => u,
        None => return (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response(),
    };

    let is_valid = match verify(payload.password, &user.password_hash) {
        Ok(v) => v,
        Err(_) => false,
    };

    if !is_valid {
        return (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response();
    }

    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user.id.clone(),
        exp: expiration,
    };

    let token = match encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_SECRET)) {
        Ok(t) => t,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Error generating token").into_response(),
    };

    let response = AuthResponse {
        token,
        user: AuthUserResponse {
            id: user.id.clone(),
            username: user.name.clone(), // using name as username for frontend compatibility
            email: user.email.clone(),
        },
    };

    (StatusCode::OK, Json(response)).into_response()
}
