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
use crate::database::models::{Message, MessageType, GroupMessage};
use crate::social::WsPayload;

pub async fn get_messages(
    Path(id): Path<Uuid>,
    State(db): State<Arc<Database>>,
) -> impl IntoResponse {
    let db_msgs = db.messages.lock().unwrap();
    let messages: Vec<Message> = db_msgs.iter()
        .filter(|m| m.conversation_id == id)
        .cloned()
        .collect();
    Json(messages)
}

pub async fn handle_socket(
    socket: WebSocket,
    db: Arc<Database>,
) {
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
                if let Ok(id) = serde_json::from_str::<String>(&text) {
                     current_user_id = Some(id.clone());
                     db.conns.insert(id, tx.clone());
                }
                continue;
            }

            if let Ok(ws_payload) = serde_json::from_str::<WsPayload>(&text) {
                match ws_payload {
                    WsPayload::SendMessage { target_id, is_group, content, msg_type, reply_id } => {
                        handle_send_message(
                            current_user_id.as_ref().unwrap(),
                            target_id,
                            is_group,
                            content,
                            msg_type,
                            reply_id,
                            &db
                        ).await;
                    }
                    WsPayload::RecallMessage { message_id } => {
                        handle_recall_message(
                            current_user_id.as_ref().unwrap(),
                            message_id,
                            &db
                        ).await;
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
    let broadcast_msg: String;
    let mut broadcast_to = Vec::new();

    if is_group {
        let msg = GroupMessage {
            id: Uuid::new_v4(),
            group_id: target_id,
            sender_id: sender_user_id.to_string(),
            reply_id,
            content: content.clone(),
            msg_type,
            created_at: now,
        };

        // Check group and membership
        {
            let db_groups = db.groups.lock().unwrap();
            if db_groups.contains_key(&target_id) {
                 let db_members = db.group_members.lock().unwrap();
                 let members: Vec<String> = db_members.iter()
                    .filter(|m| m.group_id == target_id)
                    .map(|m| m.user_id.clone())
                    .collect();
                 
                 if !members.contains(&sender_user_id.to_string()) {
                     return;
                 }
                 broadcast_to = members;
            } else {
                return;
            }
        }

        {
            let mut db_msgs = db.group_messages.lock().unwrap();
            db_msgs.push(msg.clone());
        }
        db.save_all();
        broadcast_msg = serde_json::to_string(&msg).unwrap();

    } else {
        let msg = Message {
            id: Uuid::new_v4(),
            conversation_id: target_id,
            sender_id: sender_user_id.to_string(),
            reply_id,
            content: content.clone(),
            msg_type,
            created_at: now,
        };

        {
            let mut db_convs = db.conversations.lock().unwrap();
            if let Some(conv) = db_convs.get_mut(&target_id) {
                let s = conv.sender.clone();
                let r = conv.receiver.clone();
                
                let is_sender = s.as_deref() == Some(sender_user_id);
                let is_receiver = r.as_deref() == Some(sender_user_id);
                
                if !is_sender && !is_receiver {
                    return;
                }
                
                conv.last_message = content;
                conv.last_message_at = now;
                
                if let Some(sid) = s { broadcast_to.push(sid); }
                if let Some(rid) = r { broadcast_to.push(rid); }
            } else {
                return;
            }
        }

        {
            let mut db_msgs = db.messages.lock().unwrap();
            db_msgs.push(msg.clone());
        }
        db.save_all();
        broadcast_msg = serde_json::to_string(&msg).unwrap();
    }

    for p_id in broadcast_to {
        if let Some(p_tx) = db.conns.get(&p_id) {
            let _ = p_tx.send(WsMessage::Text(broadcast_msg.clone()));
        }
    }
}

async fn handle_recall_message(
    user_id: &str,
    msg_id: Uuid,
    db: &Arc<Database>,
) {
    let mut db_msgs = db.messages.lock().unwrap();
    let msg_idx = db_msgs.iter().position(|m| m.id == msg_id && m.sender_id == user_id);
    
    if let Some(idx) = msg_idx {
        let msg = db_msgs.remove(idx);
        let target_id = msg.conversation_id;
        drop(db_msgs);
        db.save_all();
        
        let mut broadcast_to = Vec::new();
        
        // Try to find if it's a group or DM
        {
            let db_groups = db.groups.lock().unwrap();
            if db_groups.contains_key(&target_id) {
                let db_members = db.group_members.lock().unwrap();
                broadcast_to = db_members.iter()
                    .filter(|m| m.group_id == target_id)
                    .map(|m| m.user_id.clone())
                    .collect();
            } else {
                let db_convs = db.conversations.lock().unwrap();
                if let Some(conv) = db_convs.get(&target_id) {
                    if let Some(s) = &conv.sender { broadcast_to.push(s.clone()); }
                    if let Some(r) = &conv.receiver { broadcast_to.push(r.clone()); }
                }
            }
        }

        let recall_notify = serde_json::json!({
            "type": "RECALL",
            "messageId": msg_id,
            "targetId": target_id
        }).to_string();

        for p_id in broadcast_to {
            if let Some(p_tx) = db.conns.get(&p_id) {
                let _ = p_tx.send(WsMessage::Text(recall_notify.clone()));
            }
        }
    }
}
