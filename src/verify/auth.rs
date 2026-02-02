use axum::{
    routing::post,
    Json, Router,
    http::StatusCode,
    response::IntoResponse,
    extract::State,
};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Utc, Duration};
use std::sync::Arc;
use axum::http::HeaderMap;
use crate::database::Database;
use crate::database::models::{
    User, Role, RegisterRequest, LoginRequest, AuthResponse, AuthUserResponse, Claims
};

pub const JWT_SECRET: &[u8] = b"secret_key_change_me_in_production";

pub fn get_user_id_from_token(headers: &HeaderMap) -> Option<String> {
    let auth_header = headers.get("Authorization")?.to_str().ok()?;
    if !auth_header.starts_with("Bearer ") {
        return None;
    }
    let token = &auth_header[7..];
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::default(),
    ).ok()?;
    Some(token_data.claims.sub)
}

pub fn routes(db: Arc<Database>) -> Router {
    Router::new()
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .with_state(db)
}

async fn register_handler(
    State(db): State<Arc<Database>>,
    Json(payload): Json<RegisterRequest>,
) -> impl IntoResponse {
    let mut users = db.users.lock().unwrap();

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
        name: payload.username.clone(), 
        avatar: "https://picsum.photos/id/64/200/200".to_string(), 
        gender: "other".to_string(), 
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
    drop(users);
    db.save_all();

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
    State(db): State<Arc<Database>>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    let users = db.users.lock().unwrap();
    
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
            username: user.name.clone(), 
            email: user.email.clone(),
        },
    };

    (StatusCode::OK, Json(response)).into_response()
}
