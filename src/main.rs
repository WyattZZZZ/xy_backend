mod verify;
mod user;
mod social;
mod database;

use axum::{Router, routing::get, extract::DefaultBodyLimit};
use std::net::SocketAddr;
use tokio::net::TcpListener;

use crate::database::Database;

#[tokio::main]
async fn main() {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/xy".to_string());

    let db = Database::new(&database_url).await;

    let cors = tower_http::cors::CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any);

    let app = Router::new()
        .nest("/auth", verify::routes(db.clone()))
        .nest("/user", user::routes(db.clone()))
        .nest("/social", social::social_routes(db.clone()))
        .nest("/conversation", social::conversation_routes(db.clone()))
        .nest("/group", social::group_routes(db.clone()))
        .nest("/product", social::product_routes(db.clone()))
        .route("/category", get(social::categories::get_categories).with_state(db.clone()))
        .nest("/post", social::post_routes(db.clone()))
        .route("/ws", get(social::ws_handler).with_state(db.clone()))
        .layer(DefaultBodyLimit::max(20 * 1024 * 1024))
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8090));
    println!("Server running at http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
