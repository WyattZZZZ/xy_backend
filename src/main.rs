mod verify;
mod user;
mod social;
mod database;

use axum::{Router, routing::get};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use std::sync::Arc;
use crate::database::Database;

#[tokio::main]
async fn main() {
    // Initialize central database (loads from files)
    let db = Arc::new(Database::new());

    let cors = tower_http::cors::CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any);

    // Build our application with shared state
    let app = Router::new()
        .nest("/auth", verify::routes(db.clone()))
        .nest("/user", user::routes(db.clone()))
        .nest("/social", social::social_routes(db.clone()))
        .nest("/conversation", social::conversation_routes(db.clone()))
        .nest("/group", social::group_routes(db.clone()))
        .route("/ws", get(social::ws_handler).with_state(db.clone()))
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8081));
    println!("Server running at http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}