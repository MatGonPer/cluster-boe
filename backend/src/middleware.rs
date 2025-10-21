use axum::{
    extract::State,
    http::{header, Request},
    middleware::Next,
    response::Response,
};
use axum::body::Body;
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::sync::Arc;
use crate::{
    errors::AppError,
    handlers::auth_handler::AppState,
    models::user_model::Claims
};

#[derive(Clone, Copy)]
pub struct AuthenticatedUserId(pub i32);

pub async fn auth_middleware(State(state): State<Arc<AppState>>, mut request: Request<Body>, next: Next,) -> Result<Response, AppError> {
    let token = request.headers().get(header::AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_str| {
            auth_str.strip_prefix("Bearer ")
        })
        .ok_or_else(|| {
            AppError::Unauthorized(
                "Cabeçalho de autorização ausente ou mal formatado.".to_string(),
            )
        })?;

    let validation = Validation::default();
    let decoding_key = DecodingKey::from_secret(state.jwt_secret.as_ref());
    let token_data = decode::<Claims>(token, &decoding_key, &validation)
        .map_err(|err: jsonwebtoken::errors::Error| {
            let error_message = match err.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => "Token expirado.".to_string(),
                jsonwebtoken::errors::ErrorKind::InvalidSignature
                | jsonwebtoken::errors::ErrorKind::InvalidToken => "Token inválido.".to_string(),
                _ => format!("Erro de validação do token: {}", err),
            };
            AppError::Unauthorized(error_message)
        })?;
    
    let  user_id = token_data.claims.sub;
    request.extensions_mut().insert(AuthenticatedUserId(user_id));
    Ok(next.run(request).await)
}
