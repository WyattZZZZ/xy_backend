use axum::{
    extract::{ws::{WebSocket, Message as WsMessage}, State, Path},
    Json,
    response::IntoResponse,
};
use uuid::Uuid;
use chrono::Utc;
use futures_util::{StreamExt, SinkExt, stream::{SplitSink, SplitStream}};
use tokio::sync::mpsc;
use std::sync::Arc;
use crate::database::Database;
use crate::database::models::{MessageType, Claims, message_from_row, group_message_from_row, message_type_str};
use crate::social::WsPayload;
use crate::verify::auth::JWT_SECRET;
use jsonwebtoken::{decode, DecodingKey, Validation};

pub async fn get_messages(
    Path(id): Path<Uuid>,
    State(db): State<Arc<Database>>,
) -> impl IntoResponse {
    let rows = sqlx::query("SELECT * FROM messages WHERE conversation_id = $1 ORDER BY created_at ASC")
        .bind(id)
        .fetch_all(&db.pool)
        .await
        .unwrap_or_default();

    let messages: Vec<_> = rows.iter().map(|r| message_from_row(r)).collect();
    Json(messages)
}

pub async fn handle_socket(socket: WebSocket, db: Arc<Database>) {
    let (mut sender, mut receiver): (SplitSink<WebSocket, WsMessage>, SplitStream<WebSocket>) = socket.split();
    let (tx, mut rx) = mpsc::unbounded_channel::<WsMessage>();

    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sender.send(msg).await.is_err() {
                break;
            }
        }
    });

    let mut current_user_id: Option<String> = None;

    while let Some(Ok(msg)) = receiver.next().await {
        if let WsMessage::Text(text) = msg {
            if current_user_id.is_none() {
                if let Ok(token) = serde_json::from_str::<String>(&text) {
                    if let Ok(token_data) = decode::<Claims>(
                        &token,
                        &DecodingKey::from_secret(JWT_SECRET),
                        &Validation::default(),
                    ) {
                        let id = token_data.claims.sub;
                        current_user_id = Some(id.clone());
                        db.conns.insert(id, tx.clone());
                        continue;
                    }
                }
                break;
            }

            if let Ok(ws_payload) = serde_json::from_str::<WsPayload>(&text) {
                match ws_payload {
                    WsPayload::SendMessage { target_id, is_group, content, msg_type, reply_id } => {
                        handle_send_message(
                            current_user_id.as_ref().unwrap(),
                            target_id, is_group, content, msg_type, reply_id, &db,
                        ).await;
                    }
                    WsPayload::RecallMessage { message_id } => {
                        handle_recall_message(current_user_id.as_ref().unwrap(), message_id, &db).await;
                    }
                }
            }
        }
    }

    if let Some(id) = current_user_id {
        db.conns.remove(&id);
    }
}

async fn handle_send_message(
    sender_user_id: &str,
    target_id: Uuid,
    is_group: bool,
    content: String,
    msg_type: MessageType,
    reply_id: Option<Uuid>,
    db: &Arc<Database>,
) {
    let now = Utc::now();
    let msg_type_s = message_type_str(&msg_type);

    if is_group {
        let is_member: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM group_members WHERE group_id = $1 AND user_id = $2)"
        )
        .bind(target_id)
        .bind(sender_user_id)
        .fetch_one(&db.pool)
        .await
        .unwrap_or(false);

        if !is_member { return; }

        let msg_id = Uuid::new_v4();
        let _ = sqlx::query(
            "INSERT INTO group_messages (id, group_id, sender_id, reply_id, content, msg_type, created_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7)"
        )
        .bind(msg_id)
        .bind(target_id)
        .bind(sender_user_id)
        .bind(reply_id)
        .bind(&content)
        .bind(msg_type_s)
        .bind(now)
        .execute(&db.pool)
        .await;

        let broadcast_msg = serde_json::json!({
            "id": msg_id, "groupId": target_id, "senderId": sender_user_id,
            "replyId": reply_id, "content": content, "msgType": msg_type_s, "createdAt": now
        }).to_string();

        let member_ids: Vec<String> = sqlx::query_scalar(
            "SELECT user_id FROM group_members WHERE group_id = $1"
        )
        .bind(target_id)
        .fetch_all(&db.pool)
        .await
        .unwrap_or_default();

        for pid in member_ids {
            if let Some(p_tx) = db.conns.get(&pid) {
                let _ = p_tx.send(WsMessage::Text(broadcast_msg.clone()));
            }
        }
    } else {
        let row = sqlx::query("SELECT * FROM conversations WHERE id = $1")
            .bind(target_id)
            .fetch_optional(&db.pool)
            .await;

        let conv = match row.ok().flatten() {
            Some(r) => crate::database::models::conversation_from_row(&r),
            None => return,
        };

        let is_participant = conv.sender.as_deref() == Some(sender_user_id)
            || conv.receiver.as_deref() == Some(sender_user_id);
        if !is_participant { return; }

        let msg_id = Uuid::new_v4();
        let _ = sqlx::query(
            "INSERT INTO messages (id, conversation_id, sender_id, reply_id, content, msg_type, created_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7)"
        )
        .bind(msg_id)
        .bind(target_id)
        .bind(sender_user_id)
        .bind(reply_id)
        .bind(&content)
        .bind(msg_type_s)
        .bind(now)
        .execute(&db.pool)
        .await;

        let _ = sqlx::query(
            "UPDATE conversations SET last_message = $1, last_message_at = $2 WHERE id = $3"
        )
        .bind(&content)
        .bind(now)
        .bind(target_id)
        .execute(&db.pool)
        .await;

        let broadcast_msg = serde_json::json!({
            "id": msg_id, "conversationId": target_id, "senderId": sender_user_id,
            "replyId": reply_id, "content": content, "msgType": msg_type_s, "createdAt": now
        }).to_string();

        for pid in [conv.sender, conv.receiver].into_iter().flatten() {
            if let Some(p_tx) = db.conns.get(&pid) {
                let _ = p_tx.send(WsMessage::Text(broadcast_msg.clone()));
            }
        }
    }
}

async fn handle_recall_message(user_id: &str, msg_id: Uuid, db: &Arc<Database>) {
    // Try DM first
    let row = sqlx::query("SELECT * FROM messages WHERE id = $1 AND sender_id = $2")
        .bind(msg_id)
        .bind(user_id)
        .fetch_optional(&db.pool)
        .await;

    if let Ok(Some(r)) = row {
        let msg = message_from_row(&r);
        let _ = sqlx::query("DELETE FROM messages WHERE id = $1").bind(msg_id).execute(&db.pool).await;

        let conv_row = sqlx::query("SELECT * FROM conversations WHERE id = $1")
            .bind(msg.conversation_id)
            .fetch_optional(&db.pool)
            .await;

        let recall_notify = serde_json::json!({
            "type": "RECALL", "messageId": msg_id, "targetId": msg.conversation_id
        }).to_string();

        if let Ok(Some(cr)) = conv_row {
            let conv = crate::database::models::conversation_from_row(&cr);
            for pid in [conv.sender, conv.receiver].into_iter().flatten() {
                if let Some(p_tx) = db.conns.get(&pid) {
                    let _ = p_tx.send(WsMessage::Text(recall_notify.clone()));
                }
            }
        }
        return;
    }

    // Try group
    let row = sqlx::query("SELECT * FROM group_messages WHERE id = $1 AND sender_id = $2")
        .bind(msg_id)
        .bind(user_id)
        .fetch_optional(&db.pool)
        .await;

    if let Ok(Some(r)) = row {
        let msg = group_message_from_row(&r);
        let _ = sqlx::query("DELETE FROM group_messages WHERE id = $1").bind(msg_id).execute(&db.pool).await;

        let member_ids: Vec<String> = sqlx::query_scalar(
            "SELECT user_id FROM group_members WHERE group_id = $1"
        )
        .bind(msg.group_id)
        .fetch_all(&db.pool)
        .await
        .unwrap_or_default();

        let recall_notify = serde_json::json!({
            "type": "RECALL", "messageId": msg_id, "targetId": msg.group_id
        }).to_string();

        for pid in member_ids {
            if let Some(p_tx) = db.conns.get(&pid) {
                let _ = p_tx.send(WsMessage::Text(recall_notify.clone()));
            }
        }
    }
}
