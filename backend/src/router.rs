use crate::handlers::auth_handler::{self, AppState};
use axum::{routing::post, Router};
use std::sync::Arc;

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/auth/register", post(auth_handler::register_user))
        .with_state(app_state)
}
