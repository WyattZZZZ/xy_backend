use axum::{
    extract::State,
    Json,
    response::IntoResponse,
};
use std::sync::Arc;
use crate::database::Database;
use crate::database::models::Category;

pub async fn get_categories(
    State(db): State<Arc<Database>>,
) -> impl IntoResponse {
    let rows = sqlx::query("SELECT * FROM categories ORDER BY category_id")
        .fetch_all(&db.pool)
        .await
        .unwrap_or_default();

    let categories: Vec<Category> = rows.iter().map(|r| Category {
        category_id: sqlx::Row::get(r, "category_id"),
        english: sqlx::Row::get(r, "english"),
        chinese: sqlx::Row::get(r, "chinese"),
        icon: sqlx::Row::try_get(r, "icon").ok().flatten(),
    }).collect();

    Json(categories).into_response()
}
