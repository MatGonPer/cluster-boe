mod handlers;
mod models;
mod router;
mod errors;
mod middleware;

use handlers::auth_handler::AppState;
use sqlx::PgPool;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use axum::http::{HeaderValue, Method, header::{self, ACCESS_CONTROL_ALLOW_CREDENTIALS}};

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
        .allow_origin([
            //Necessário alterar posteriormente regras de CORS na fase de produção!
            "http://localhost:80".parse::<HeaderValue>().unwrap(),
            //Endereço para npm rum, apenas para ambiente de desenvolvimento!
            "http://localhost:5173".parse::<HeaderValue>().unwrap(),
        ])
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
        .allow_credentials(true);

    let app = router::create_router(app_state)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Servidor escutando em http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
