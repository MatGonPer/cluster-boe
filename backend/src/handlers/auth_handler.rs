use crate::models::user_model::{RegisterRequest, UserResponse};
use crate::errors::AppError;
use axum::{extract::State, http::StatusCode, response::Json};
use sqlx::PgPool;
use std::sync::Arc;

pub struct AppState {
    pub db_pool: PgPool,
}

pub async fn register_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<UserResponse>), AppError> {
    // lógica para sanitizar e validar os campos, por enquanto ainda está apenas validando username
    // e password, mas deve ser melhorado para ter mais regras de validação e sanitização
    if payload.username.trim().is_empty() {
        return Err(AppError::BadRequest("O nome de usuário não pode estar vazio.".to_string()))
    }

    if payload.password.len() < 8 {
        return Err(AppError::BadRequest("A senha deve ter no mínimo 8 caracteres.".to_string()))
    }

    let password_clone = payload.password.clone();
    let password_hash = tokio::task::spawn_blocking(move || {
        bcrypt::hash(password_clone, bcrypt::DEFAULT_COST)
    })
    .await
    .map_err(|_| AppError::InternalServerError("Falha no processamento interno.".to_string()))?
    .map_err(|_| AppError::InternalServerError("Falha ao gerar o hash da senha.".to_string()))?;

    let result = sqlx::query_as!(
        UserResponse,
        "INSERT INTO users (username, password_hash) VALUES ($1, $2) RETURNING id, username", payload.username, password_hash
    )
    .fetch_one(&state.db_pool)
    .await;

    match result {
        Ok(new_user) => {
            Ok((StatusCode::CREATED, Json(new_user)))
        },
        Err(_) => {
            Err(AppError::InternalServerError("Falha ao criar o usuário.".to_string()))
        }

    }
}
