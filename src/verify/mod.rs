pub mod auth;

use std::sync::Arc;
use crate::database::Database;

pub fn routes(db: Arc<Database>) -> axum::Router {
    auth::routes(db)
}
