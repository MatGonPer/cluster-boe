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
    let _ = dotenvy::dotenv();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL deve ser definida");

   //conecta ao banco de dados
   let db_pool = PgPool::connect(&database_url)
       .await
       .expect("Falha ao conectar ao banco de dados");
    
    let app_state = Arc::new(AppState { db_pool });
    let app = router::create_router(app_state);

    //inicia o servidor
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Servidor escutando em http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
