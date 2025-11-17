use crate::{
    handlers::{
        auth_handler::{self, login_user, register_user},
        cluster_handler,
    },
    middleware::auth_middleware,
    AppState,
};
use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use std::sync::Arc;

pub fn create_router(app_state: Arc<AppState>) -> Router {
    let auth_routes = Router::new()
        .route("/register", post(register_user))
        .route("/login", post(login_user));

    let protected_routes = Router::new()
        .route("/admin/nodes", get(cluster_handler::list_nodes))
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth_middleware,
        ));

    Router::new()
        .nest("/api/auth", auth_routes)
        .nest("/api", protected_routes)
        .with_state(app_state)
}
