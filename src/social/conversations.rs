use axum::{
    extract::{State, Path, Query},
    Json,
    response::IntoResponse,
    http::{StatusCode, HeaderMap},
};
use uuid::Uuid;
use chrono::Utc;
use std::sync::Arc;
use crate::database::Database;
use crate::database::models::{ChatListItem, ConversationFilter, CreateConversationRequest, conversation_from_row};
use crate::verify::auth::get_user_id_from_token;

pub async fn get_conversations(
    State(db): State<Arc<Database>>,
    headers: HeaderMap,
    Query(filter): Query<ConversationFilter>,
) -> impl IntoResponse {
    let user_id = filter.user_id
        .or_else(|| get_user_id_from_token(&headers))
        .unwrap_or_else(|| "test_user".to_string());

    let rows = sqlx::query(
        "SELECT c.*, u.name, u.avatar, u.id as other_id
         FROM conversations c
         LEFT JOIN users u ON u.id = CASE
             WHEN c.sender = $1 THEN c.receiver
             ELSE c.sender
         END
         WHERE c.sender = $1 OR c.receiver = $1
         ORDER BY c.last_message_at DESC"
    )
    .bind(&user_id)
    .fetch_all(&db.pool)
    .await;

    match rows {
        Ok(rows) => {
            let items: Vec<ChatListItem> = rows.iter().map(|r| {
                let conv = conversation_from_row(r);
                let other_user_id: Option<String> = sqlx::Row::try_get(r, "other_id").ok().flatten();
                let name: Option<String> = sqlx::Row::try_get(r, "name").ok().flatten();
                let avatar: Option<String> = sqlx::Row::try_get(r, "avatar").ok().flatten();
                ChatListItem {
                    id: conv.id,
                    name: name.unwrap_or_else(|| "Deleted User".to_string()),
                    avatar: avatar.unwrap_or_else(|| "https://picsum.photos/200".to_string()),
                    last_msg: conv.last_message,
                    time: conv.last_message_at.format("%H:%M").to_string(),
                    unread: false,
                    is_group: false,
                    other_user_id,
                }
            }).collect();
            Json(items).into_response()
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }
}

pub async fn create_conversation(
    State(db): State<Arc<Database>>,
    headers: HeaderMap,
    Json(req): Json<CreateConversationRequest>,
) -> impl IntoResponse {
    let authenticated_id = match get_user_id_from_token(&headers) {
        Some(uid) => uid,
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    let receiver_id = req.receiver_id;

    // Check if conversation already exists between the two users
    let existing = sqlx::query(
        "SELECT * FROM conversations WHERE
         (sender = $1 AND receiver = $2) OR (sender = $2 AND receiver = $1)"
    )
    .bind(&authenticated_id)
    .bind(&receiver_id)
    .fetch_optional(&db.pool)
    .await;

    if let Ok(Some(row)) = existing {
        let mut conv = conversation_from_row(&row);
        let mut changed = false;
        if conv.sender.is_none() {
            let _ = sqlx::query("UPDATE conversations SET sender = $1 WHERE id = $2")
                .bind(&authenticated_id)
                .bind(conv.id)
                .execute(&db.pool)
                .await;
            conv.sender = Some(authenticated_id.clone());
            changed = true;
        } else if conv.receiver.is_none() {
            let _ = sqlx::query("UPDATE conversations SET receiver = $1 WHERE id = $2")
                .bind(&authenticated_id)
                .bind(conv.id)
                .execute(&db.pool)
                .await;
            conv.receiver = Some(authenticated_id.clone());
            changed = true;
        }
        let _ = changed;
        return Json(conv).into_response();
    }

    let now = Utc::now();
    let conv_id = Uuid::new_v4();

    let result = sqlx::query(
        "INSERT INTO conversations (id, sender, receiver, muteby, last_message, last_message_at, created_at)
         VALUES ($1, $2, $3, 0, '', $4, $4)"
    )
    .bind(conv_id)
    .bind(&authenticated_id)
    .bind(&receiver_id)
    .bind(now)
    .execute(&db.pool)
    .await;

    if result.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
    }

    let row = sqlx::query("SELECT * FROM conversations WHERE id = $1")
        .bind(conv_id)
        .fetch_one(&db.pool)
        .await;

    match row {
        Ok(r) => Json(conversation_from_row(&r)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }
}

pub async fn get_conversation(
    Path(id): Path<Uuid>,
    State(db): State<Arc<Database>>,
) -> impl IntoResponse {
    let row = sqlx::query("SELECT * FROM conversations WHERE id = $1")
        .bind(id)
        .fetch_optional(&db.pool)
        .await;

    match row {
        Ok(Some(r)) => Json(Some(conversation_from_row(&r))).into_response(),
        Ok(None) => Json(None::<()>).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }
}

pub async fn delete_conversation(
    Path(id): Path<Uuid>,
    State(db): State<Arc<Database>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let authenticated_id = match get_user_id_from_token(&headers) {
        Some(uid) => uid,
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    let row = sqlx::query("SELECT * FROM conversations WHERE id = $1")
        .bind(id)
        .fetch_optional(&db.pool)
        .await;

    match row {
        Ok(Some(r)) => {
            let conv = conversation_from_row(&r);
            if conv.sender.as_deref() == Some(&authenticated_id) {
                let _ = sqlx::query("UPDATE conversations SET sender = NULL WHERE id = $1")
                    .bind(id).execute(&db.pool).await;
            } else if conv.receiver.as_deref() == Some(&authenticated_id) {
                let _ = sqlx::query("UPDATE conversations SET receiver = NULL WHERE id = $1")
                    .bind(id).execute(&db.pool).await;
            }
            (StatusCode::OK, Json(serde_json::json!({"status":"ok"}))).into_response()
        }
        Ok(None) => (StatusCode::NOT_FOUND, "Conversation not found").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }
}
