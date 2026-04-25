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
    RegisterRequest, LoginRequest, AuthResponse, AuthUserResponse, Claims, user_from_row,
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
    let existing = sqlx::query("SELECT id FROM users WHERE email = $1")
        .bind(&payload.email)
        .fetch_optional(&db.pool)
        .await;

    match existing {
        Ok(Some(_)) => return (StatusCode::BAD_REQUEST, "Email already registered").into_response(),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
        Ok(None) => {}
    }

    let hashed_password = match hash(&payload.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Error hashing password").into_response(),
    };

    let user_id = uuid::Uuid::new_v4().to_string();
    let now = Utc::now();
    let avatar = "https://picsum.photos/id/64/200/200".to_string();

    let result = sqlx::query(
        "INSERT INTO users (id, email, password_hash, name, avatar, gender, bio, location, posts, following, fans, rating, reviews_count, is_verified, role, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, 'other', '', '', 0, 0, 0, 0.0, 0, FALSE, 'USER', $6, $6)"
    )
    .bind(&user_id)
    .bind(&payload.email)
    .bind(&hashed_password)
    .bind(&payload.username)
    .bind(&avatar)
    .bind(now)
    .execute(&db.pool)
    .await;

    if result.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Error creating user").into_response();
    }

    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims { sub: user_id.clone(), exp: expiration };
    let token = match encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_SECRET)) {
        Ok(t) => t,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Error generating token").into_response(),
    };

    (StatusCode::CREATED, Json(AuthResponse {
        token,
        user: AuthUserResponse { id: user_id, username: payload.username, email: payload.email },
    })).into_response()
}

async fn login_handler(
    State(db): State<Arc<Database>>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    let row = sqlx::query("SELECT * FROM users WHERE email = $1")
        .bind(&payload.email)
        .fetch_optional(&db.pool)
        .await;

    let row = match row {
        Ok(Some(r)) => r,
        Ok(None) => return (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response(),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    };

    let user = user_from_row(&row);

    let is_valid = verify(&payload.password, &user.password_hash).unwrap_or(false);
    if !is_valid {
        return (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response();
    }

    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims { sub: user.id.clone(), exp: expiration };
    let token = match encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_SECRET)) {
        Ok(t) => t,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Error generating token").into_response(),
    };

    (StatusCode::OK, Json(AuthResponse {
        token,
        user: AuthUserResponse { id: user.id.clone(), username: user.name.clone(), email: user.email.clone() },
    })).into_response()
}
