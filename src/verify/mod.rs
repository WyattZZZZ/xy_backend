pub mod auth;

pub fn routes(db: crate::user::UserDb) -> axum::Router {
    auth::routes(db)
}
