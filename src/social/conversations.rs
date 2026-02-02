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
use crate::database::models::{Conversation, ChatListItem, ConversationFilter, CreateConversationRequest}; 
use crate::verify::auth::get_user_id_from_token;

pub async fn get_conversations(
    State(db): State<Arc<Database>>,
    headers: HeaderMap,
    Query(filter): Query<ConversationFilter>,
) -> impl IntoResponse {
    let user_id = filter.user_id.or_else(|| get_user_id_from_token(&headers)).unwrap_or_else(|| "test_user".to_string());
    let db_convs = db.conversations.lock().unwrap();
    let db_users = db.users.lock().unwrap();
    
    let my_convs: Vec<ChatListItem> = db_convs.values()
        .filter(|c| c.sender.as_deref() == Some(&user_id) || c.receiver.as_deref() == Some(&user_id))
        .map(|c| {
            let other_id = if c.sender.as_deref() == Some(&user_id) {
                c.receiver.as_ref()
            } else {
                c.sender.as_ref()
            };
            
            let (name, avatar) = if let Some(oid) = other_id {
                if let Some(user) = db_users.get(oid) {
                    (user.name.clone(), user.avatar.clone())
                } else {
                    ("Unknown User".to_string(), "https://picsum.photos/200".to_string())
                }
            } else {
                ("Deleted User".to_string(), "https://picsum.photos/200".to_string())
            };

            ChatListItem {
                id: c.id,
                name,
                avatar,
                last_msg: c.last_message.clone(),
                time: c.last_message_at.format("%H:%M").to_string(),
                unread: false, // Placeholder
                is_group: false,
                other_user_id: other_id.cloned(),
            }
        })
        .collect();
    Json(my_convs).into_response()
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
    let sender_id = &authenticated_id; 
    let receiver_id = req.receiver_id;

    // Check if an existing DM between these two already exists
    let mut db_convs = db.conversations.lock().unwrap();
    let existing = db_convs.values().find(|c| 
        (c.sender.as_deref() == Some(sender_id) && c.receiver.as_deref() == Some(&receiver_id)) ||
        (c.sender.as_deref() == Some(&receiver_id) && c.receiver.as_deref() == Some(sender_id))
    );

    if let Some(c) = existing {
        // If it was "deleted" by one side, restore it
        let mut c_clone = c.clone();
        let mut changed = false;
        if c_clone.sender.is_none() && c_clone.receiver.as_deref() == Some(&receiver_id) {
            c_clone.sender = Some(sender_id.to_string());
            changed = true;
        } else if c_clone.receiver.is_none() && c_clone.sender.as_deref() == Some(&receiver_id) {
            c_clone.receiver = Some(sender_id.to_string());
            changed = true;
        }
        
        if changed {
            db_convs.insert(c_clone.id, c_clone.clone());
            drop(db_convs);
            db.save_all();
        }
        return Json(c_clone).into_response();
    }

    let now = Utc::now();
    let conv = Conversation {
        id: Uuid::new_v4(),
        sender: Some(sender_id.to_string()),
        receiver: Some(receiver_id.to_string()),
        muteby: 0,
        last_message: "".to_string(),
        last_message_at: now,
        created_at: now,
    };

    db_convs.insert(conv.id, conv.clone());
    drop(db_convs);
    db.save_all();
    Json(conv).into_response()
}

pub async fn get_conversation(
    Path(id): Path<Uuid>,
    State(db): State<Arc<Database>>,
) -> impl IntoResponse {
    let db_convs = db.conversations.lock().unwrap();
    match db_convs.get(&id) {
        Some(c) => Json(Some(c.clone())).into_response(),
        None => Json(None::<Conversation>).into_response(),
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
    let user_id = authenticated_id;
    let mut db_convs = db.conversations.lock().unwrap();
    if let Some(c) = db_convs.get_mut(&id) {
        if c.sender.as_deref() == Some(&user_id) {
            c.sender = None;
        } else if c.receiver.as_deref() == Some(&user_id) {
            c.receiver = None;
        }
        
        drop(db_convs);
        db.save_all();
        return (StatusCode::OK, Json(serde_json::json!({"status": "ok"}))).into_response();
    }
    (StatusCode::NOT_FOUND, "Conversation not found").into_response()
}
