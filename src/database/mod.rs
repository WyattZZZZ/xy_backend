pub mod models;

use sqlx::PgPool;
use std::sync::Arc;
use dashmap::DashMap;
use tokio::sync::mpsc;
use axum::extract::ws::Message as WsMessage;

pub type ActiveConnections = Arc<DashMap<String, mpsc::UnboundedSender<WsMessage>>>;

pub struct Database {
    pub pool: PgPool,
    pub conns: ActiveConnections,
}

impl Database {
    pub async fn new(database_url: &str) -> Arc<Self> {
        let pool = PgPool::connect(database_url)
            .await
            .expect("Failed to connect to PostgreSQL");

        sqlx::query(include_str!("../../migrations/001_init.sql"))
            .execute(&pool)
            .await
            .expect("Failed to run migrations");

        Arc::new(Self {
            pool,
            conns: Arc::new(DashMap::new()),
        })
    }
}
