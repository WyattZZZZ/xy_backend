mod verify;
mod user;

use axum::Router;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    // Initialize shared state
    let db: user::UserDb = Arc::new(Mutex::new(HashMap::new()));

    let cors = tower_http::cors::CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any);

    // Build our application with shared state
    let app = Router::new()
        .nest("/auth", verify::routes(db.clone()))
        .nest("/user", user::routes(db.clone()))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8081));
    println!("Server running at http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}