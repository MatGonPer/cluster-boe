use crate::models::user_model::{RegisterRequest, UserResponse, LoginRequest, LoginResponse, Claims};
use crate::errors::AppError;
use axum::{extract::State, http::StatusCode, response::Json};
use sqlx::PgPool;
use std::sync::Arc;
use validator::Validate;
use jsonwebtoken::{encode, EncodingKey, Header};
use chrono::{Utc, Duration};

pub struct AppState {
    pub db_pool: PgPool,
    pub jwt_secret: String,
    pub token_duration_seconds: i64,
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

pub async fn login_user(State(state): State<Arc<AppState>>, Json(payload): Json<LoginRequest>) -> Result<Json<LoginResponse>, AppError> {
    // verifica formato do email
    if let Err(validation_errors) = payload.validate() {
        return Err(AppError::BadRequest(format!("Dados de login inválidos: {}", validation_errors)));
    }

    let email = payload.email.trim();
    let password_attempt = payload.password.trim();

    if email.is_empty() || password_attempt.is_empty() {
        return Err(AppError::BadRequest("Email e senha são obrigatórios.".to_string()));
    }

    let email_normalized = email.to_lowercase();

    let user_record = sqlx::query!("SELECT id, password_hash FROM users WHERE email = $1", email_normalized).fetch_optional(&state.db_pool).await
        .map_err(|e| {
            // log apenas para produção
            eprintln!("Erro de banco ao buscar usuário: {}", e);
            AppError::InternalServerError("Erro ao consultar o banco de dados.".to_string())
    })?;

    let user = match user_record {
        Some(record) => record,
        None => return Err(AppError::BadRequest("Credenciais inválidas.".to_string())),
    };

    let stored_hash = user.password_hash.clone();
    let password_attempt_clone = password_attempt.to_string();

    let is_valid_password = tokio::task::spawn_blocking(move || {
        bcrypt::verify(&password_attempt_clone, &stored_hash)
    })
    .await
    .map_err(|e| {
        // log apenas para produção
        eprintln!("Erro na task de verificação de senha: {}", e);
        AppError::InternalServerError("Falha no processamento interno.".to_string())
    })?
    .map_err(|e| {
        // log apenas para produção
        eprintln!("Erro no bcrypt::verify: {}", e);
        AppError::InternalServerError("Falha ao verificar a senha.".to_string())
    })?;

    if !is_valid_password {
        return Err(AppError::BadRequest("Credenciais inválidas".to_string()));
    }

    let now = Utc::now();
    let expires_at = now + Duration::seconds(state.token_duration_seconds);

    let claims = Claims {
        sub: user.id,
        exp: expires_at.timestamp() as usize,
    };

    let token = encode (
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.jwt_secret.as_ref()),
    )
    .map_err(|e| {
        // log apenas para produção
        eprintln!("Erro ao gerar JWT: {}", e);
        AppError::InternalServerError("Falha ao gerar o token de autenticação.".to_string())
    })?;

    let response = LoginResponse { token };
    Ok(Json(response))
}
