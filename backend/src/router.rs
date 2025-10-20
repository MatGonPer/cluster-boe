use crate::handlers::auth_handler::{self, AppState, login_user};
use axum::{routing::post, Router};
use std::sync::Arc;

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/auth/register", post(auth_handler::register_user))
        .route("/api/auth/login", post(login_user))
        .with_state(app_state)
}
