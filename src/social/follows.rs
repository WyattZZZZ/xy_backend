use axum::{
    extract::{State, Path, Query},
    Json,
    response::IntoResponse,
    http::{StatusCode, HeaderMap},
};
use chrono::Utc;
use std::sync::Arc;
use crate::database::Database;
use crate::database::models::FollowFilter;
use crate::verify::auth::get_user_id_from_token;

pub async fn get_follows(
    Query(filter): Query<FollowFilter>,
    State(db): State<Arc<Database>>,
) -> impl IntoResponse {
    let rows = if let Some(ref follower_id) = filter.follower_id {
        sqlx::query(
            "SELECT u.id, u.name, u.avatar FROM follows f
             JOIN users u ON u.id = f.following_id
             WHERE f.follower_id = $1"
        )
        .bind(follower_id)
        .fetch_all(&db.pool)
        .await
    } else if let Some(ref following_id) = filter.following_id {
        sqlx::query(
            "SELECT u.id, u.name, u.avatar FROM follows f
             JOIN users u ON u.id = f.follower_id
             WHERE f.following_id = $1"
        )
        .bind(following_id)
        .fetch_all(&db.pool)
        .await
    } else {
        sqlx::query("SELECT u.id, u.name, u.avatar FROM follows f JOIN users u ON u.id = f.following_id")
            .fetch_all(&db.pool)
            .await
    };

    match rows {
        Ok(rows) => {
            let results: Vec<crate::database::models::FollowUserItem> = rows.iter().map(|r| {
                crate::database::models::FollowUserItem {
                    id: sqlx::Row::get(r, "id"),
                    name: sqlx::Row::get(r, "name"),
                    avatar: sqlx::Row::get(r, "avatar"),
                }
            }).collect();
            Json(results).into_response()
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }
}

pub async fn follow_user(
    Path(id): Path<String>,
    headers: HeaderMap,
    State(db): State<Arc<Database>>,
) -> impl IntoResponse {
    let authenticated_id = match get_user_id_from_token(&headers) {
        Some(uid) => uid,
        None => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"status":"error","message":"Unauthorized"}))).into_response(),
    };

    if authenticated_id == id {
        return (StatusCode::BAD_REQUEST, Json(serde_json::json!({"status":"error","message":"Cannot follow yourself"}))).into_response();
    }

    let now = Utc::now();
    let result = sqlx::query(
        "INSERT INTO follows (follower_id, following_id, created_at) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING"
    )
    .bind(&authenticated_id)
    .bind(&id)
    .bind(now)
    .execute(&db.pool)
    .await;

    if let Ok(r) = result {
        if r.rows_affected() > 0 {
            let _ = sqlx::query("UPDATE users SET following = following + 1 WHERE id = $1")
                .bind(&authenticated_id)
                .execute(&db.pool)
                .await;
            let _ = sqlx::query("UPDATE users SET fans = fans + 1 WHERE id = $1")
                .bind(&id)
                .execute(&db.pool)
                .await;
        }
    }

    Json(serde_json::json!({"status":"ok"})).into_response()
}

pub async fn unfollow_user(
    Path(id): Path<String>,
    headers: HeaderMap,
    State(db): State<Arc<Database>>,
) -> impl IntoResponse {
    let authenticated_id = match get_user_id_from_token(&headers) {
        Some(uid) => uid,
        None => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"status":"error","message":"Unauthorized"}))).into_response(),
    };

    let result = sqlx::query("DELETE FROM follows WHERE follower_id = $1 AND following_id = $2")
        .bind(&authenticated_id)
        .bind(&id)
        .execute(&db.pool)
        .await;

    if let Ok(r) = result {
        if r.rows_affected() > 0 {
            let _ = sqlx::query("UPDATE users SET following = GREATEST(following - 1, 0) WHERE id = $1")
                .bind(&authenticated_id)
                .execute(&db.pool)
                .await;
            let _ = sqlx::query("UPDATE users SET fans = GREATEST(fans - 1, 0) WHERE id = $1")
                .bind(&id)
                .execute(&db.pool)
                .await;
        }
    }

    Json(serde_json::json!({"status":"ok"})).into_response()
}
