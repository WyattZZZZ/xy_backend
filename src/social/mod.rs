pub mod conversations;
pub mod messages;
pub mod follows;
pub mod groups;
pub mod products;
pub mod categories;
pub mod posts;

use axum::{
    Router,
    routing::{get, post},
    extract::ws::WebSocketUpgrade,
    response::IntoResponse,
    extract::State,
    Json,
    http::{StatusCode, HeaderMap},
};
use uuid::Uuid;
use std::sync::Arc;
use crate::database::Database;
use crate::database::models::WsPayload;


// 对应 /social 路由
pub fn social_routes(db: Arc<Database>) -> Router {
    Router::new()
        .route("/follow", get(follows::get_follows))
        .route("/follow/:id", post(follows::follow_user).delete(follows::unfollow_user))
        .route("/report", post(report_content))
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

// 对应 /product 路由
pub fn product_routes(db: Arc<Database>) -> Router {
    Router::new()
        .route("/", post(products::create_product).get(products::get_products))
        .route("/:id", get(products::get_product).with_state(db.clone()).post(products::update_product).put(products::update_product).delete(products::delete_product))
        .route("/:id/favorite", post(products::favorite_product).delete(products::unfavorite_product).get(products::check_product_favorite))
        .with_state(db)
}


// 对应 /post 路由
pub fn post_routes(db: Arc<Database>) -> Router {
    Router::new()
        .route("/", post(posts::create_post).get(posts::get_posts))
        .route("/:id", get(posts::get_post).put(posts::update_post).delete(posts::delete_post))
        .route("/:id/favorite", post(posts::favorite_post).delete(posts::unfavorite_post).get(posts::check_post_favorite))
        .route("/:id/comments", get(posts::get_post_comments).post(posts::add_comment))
        .route("/:id/comments/:comment_id", post(posts::delete_comment).delete(posts::delete_comment))
        .route("/:id/reply", post(posts::add_reply))
        .route("/comment/:comment_id/replies", get(posts::get_replies))
        .with_state(db)
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(db): State<Arc<Database>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| messages::handle_socket(socket, db))
}

pub async fn report_content(
    State(db): State<Arc<Database>>,
    headers: HeaderMap,
    Json(req): Json<crate::database::models::CreateReportRequest>,
) -> impl IntoResponse {
    let uid = match crate::verify::auth::get_user_id_from_token(&headers) {
        Some(uid) => uid,
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    let report_id = Uuid::new_v4();
    let now = chrono::Utc::now();

    let result = sqlx::query(
        "INSERT INTO reports (id, target_id, target_type, reason, details, user_id, created_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7)"
    )
    .bind(report_id)
    .bind(&req.target_id)
    .bind(&req.target_type)
    .bind(&req.reason)
    .bind(&req.details)
    .bind(&uid)
    .bind(now)
    .execute(&db.pool)
    .await;

    if result.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
    }

    (StatusCode::CREATED, "Report submitted successfully").into_response()
}
