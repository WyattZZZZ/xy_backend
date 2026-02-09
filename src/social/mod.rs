pub mod conversations;
pub mod messages;
pub mod follows;
pub mod groups;

use axum::{
    Router,
    routing::{get, post},
    extract::ws::WebSocketUpgrade,
    response::IntoResponse,
    extract::State,
};
use std::sync::Arc;
use crate::database::Database;
use crate::database::models::WsPayload;

// 对应 /social 路由
pub fn social_routes(db: Arc<Database>) -> Router {
    Router::new()
        .route("/follow", get(follows::get_follows))
        .route("/follow/:id", post(follows::follow_user).delete(follows::unfollow_user))
        .with_state(db)
}

// 对应 /conversation 路由 (1v1)
pub fn conversation_routes(db: Arc<Database>) -> Router {
    Router::new()
        .route("/", post(conversations::create_conversation).get(conversations::get_conversations))
        .route("/:id", get(conversations::get_conversation).delete(conversations::delete_conversation).post(conversations::delete_conversation))
        .route("/:id/messages", get(messages::get_messages))
        .with_state(db)
}

// 对应 /group 路由
pub fn group_routes(db: Arc<Database>) -> Router {
    Router::new()
        .route("/", post(groups::create_group).get(groups::get_groups))
        .route("/:id", get(groups::get_group).post(groups::handle_group_action).delete(groups::delete_group).put(groups::update_group_info))
        .route("/:id/members", get(groups::get_group_members))
        .route("/:id/message", get(groups::get_group_messages).post(groups::post_group_message))
        .with_state(db)
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(db): State<Arc<Database>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| messages::handle_socket(socket, db))
}
