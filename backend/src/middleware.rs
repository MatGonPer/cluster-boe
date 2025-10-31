use axum::{
    extract::State,
    http::{header, Request},
    middleware::Next,
    response::Response,
};
use axum_extra::extract::CookieJar;
use axum::body::Body;
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::sync::Arc;
use crate::{
    errors::AppError,
    handlers::auth_handler::AppState,
    models::user_model::{Claims, UserRole},
};

#[derive(Clone, Copy, Debug)]
pub struct AuthenticatedUser {
    pub id: i32,
    pub role: UserRole,
}

pub async fn auth_middleware(
    State(state): State<Arc<AppState>>, 
    jar: CookieJar,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    let token = jar.get("authToken")
        .map(|cookie| cookie.value().to_string())
        .ok_or_else(|| {
            AppError::Unauthorized(
                "Token de autenticação ausente.".to_string(),
            )
        })?;

    let validation = Validation::default();
    let decoding_key = DecodingKey::from_secret(state.jwt_secret.as_ref());

    let token_data = decode::<Claims>(&token, &decoding_key, &validation)
        .map_err(|err: jsonwebtoken::errors::Error| {
           let error_message = match err.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => "Token expirado.".to_string(),
                jsonwebtoken::errors::ErrorKind::InvalidSignature
                | jsonwebtoken::errors::ErrorKind::InvalidToken => "Token inválido.".to_string(),
                _ => format!("Erro de validação do token: {}", err),
            };
            AppError::Unauthorized(error_message)
        })?;

    let user_id = token_data.claims.sub;
    let user_role = token_data.claims.role;

    request.extensions_mut().insert(AuthenticatedUser {
        id: user_id,
        role: user_role,
    });

    Ok(next.run(request).await)
}
