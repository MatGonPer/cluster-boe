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
