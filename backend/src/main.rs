mod handlers;
mod models;
mod router;
mod errors;
mod middleware;

use handlers::auth_handler::AppState;
use sqlx::PgPool;
use std::sync::Arc;
use tower_http::cors::{CorsLayer, Any};
use axum::http::Method;

#[tokio::main]
async fn main() {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL deve ser definida no .env");
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET deve ser definia no .env");
    let token_duration_str = std::env::var("JWT_EXPIRATION_SECONDS").expect("JWT_EXPIRATION_SECONDS deve ser definida no .env");
    let token_duration_seconds = token_duration_str.parse::<i64>().expect("JWT_EXPIRATION_SECONDS deve ser um número válido de segundos");

    let db_pool = PgPool::connect(&database_url)
        .await
        .expect("Falha ao conectar ao banco de dados");
    let app_state = Arc::new(AppState {
        db_pool,
        jwt_secret,
        token_duration_seconds,
    });
    
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::PUT, Method::DELETE])
        .allow_headers(Any);

    let app = router::create_router(app_state)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Servidor escutando em http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
