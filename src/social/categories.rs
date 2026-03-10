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
    let categories_map = db.categories.lock().unwrap();
    let categories: Vec<Category> = categories_map.values().cloned().collect();
    Json(categories).into_response()
}
