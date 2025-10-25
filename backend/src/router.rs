use crate::handlers::auth_handler::{self, AppState, login_user, register_user};
use axum::{
    routing::{get, post},
    Router,
    middleware,
};
use std::sync::Arc;

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/auth/register", post(register_user))
        .route("/api/auth/login", post(login_user))
        .with_state(app_state)
}
