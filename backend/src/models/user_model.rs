use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email(message = "O formato do e-mail é inválido."))]
    pub email: String,
    #[validate(length(min = 8, message = "A senha deve ter no mínimo 8 caracteres."))]
    pub password: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub email: String,
}

#[derive(Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "O formato de e-mail é inválido."))]
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
   pub token: String, 
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    User,
    Admin,
}

// estrutura interna para os dados que vão dentro do JWT (Claims)
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: usize,
    pub role: UserRole,
}
