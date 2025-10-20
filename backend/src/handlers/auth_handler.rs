use crate::models::user_model::{RegisterRequest, UserResponse};
use crate::errors::AppError;
use axum::{extract::State, http::StatusCode, response::Json};
use sqlx::PgPool;
use std::sync::Arc;
use validator::Validate;

pub struct AppState {
    pub db_pool: PgPool,
}

pub async fn register_user(State(state): State<Arc<AppState>>, Json(payload): Json<RegisterRequest>,) -> Result<(StatusCode, Json<UserResponse>), AppError> {
    // validação de email e senha, verifica apenas o formato do email e tamanho da senha
    if let Err(validation_errors) = payload.validate() {
        return Err(AppError::BadRequest(format!("Dados de entrada inválidos: {}", validation_errors)))
    }

    let email = payload.email.trim();
    let password = payload.password.trim();
    let email_normalized = email.to_lowercase();

    let has_uppercase = password.chars().any(|c| c.is_ascii_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_ascii_lowercase());
    let has_digit =  password.chars().any(|c| c.is_ascii_digit());
    let has_symbol = password.chars().any(|c| !c.is_alphanumeric());

    if !(has_uppercase && has_lowercase && has_digit && has_symbol) {
        return Err(AppError::BadRequest(
                "A senha deve ter pelo menos uma letra maiúscula, uma minúscula, um número e um símbolo.".to_string(),
        ));
    }

    let password_for_hashing = password.to_string();
    let password_hash = tokio::task::spawn_blocking(move || {
        bcrypt::hash(password_for_hashing, bcrypt::DEFAULT_COST)
    })
    .await
    .map_err(|_| AppError::InternalServerError("Falha no processamento interno.".to_string()))?
    .map_err(|_| AppError::InternalServerError("Falha ao gerar o hash de senha.".to_string()))?;

    let result = sqlx::query_as!(UserResponse, "INSERT INTO users (email, password_hash) VALUES ($1, $2) RETURNING id, email", email_normalized, password_hash)
    .fetch_one(&state.db_pool)
    .await;

    match result {
        Ok(new_user) => Ok((StatusCode::CREATED, Json(new_user))),
        Err(sqlx::Error::Database(db_err)) => {
            if db_err.code() == Some("23505".into()) {
                Err(AppError::Conflict("Este endereço de e-mail já'está em uso.".to_string()))
            } else {
                Err(AppError::InternalServerError(format!("Erro de banco de dados: {}", db_err)))
            }
        },
        Err(_) => Err(AppError::InternalServerError("Falha ao criar o usuário.".to_string())),
    }
}
