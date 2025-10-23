mod handlers;
mod models;
mod router;
mod errors;

use handlers::auth_handler::AppState;
use sqlx::PgPool;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    //carrega as variáveis de ambiente definidas no arquivo .env
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL deve ser definida no .env");
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET deve ser definia no .env");
    let token_duration_str = std::env::var("JWT_EXPIRATION_SECONDS").expect("JWT_EXPIRATION_SECONDS deve ser definida no .env");
    let token_duration_seconds = token_duration_str.parse::<i64>().expect("JWT_EXPIRATION_SECONDS deve ser um número válido de segundos");

    //conecta ao banco de dados
    let db_pool = PgPool::connect(&database_url)
        .await
        .expect("Falha ao conectar ao banco de dados");
    let app_state = Arc::new(AppState {
        db_pool,
        jwt_secret,
        token_duration_seconds,
    });

    let app = router::create_router(app_state);

    //inicia o servidor
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Servidor escutando em http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
