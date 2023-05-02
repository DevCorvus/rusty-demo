use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use validator::Validate;

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub message: String,
    pub status: usize,
}

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Invalid email")]
    Email,
    #[error("Password must be at least 6 characters long")]
    Password,
}

#[derive(Serialize, Deserialize)]
pub struct ValidationErrorResponse {
    pub message: String,
    pub status: usize,
    pub errors: Vec<String>,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct CreateUserDto {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String,
}

#[derive(Deserialize, Serialize, Validate, Clone)]
pub struct UpdateUserPasswordDto {
    #[validate(length(min = 6))]
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserResponse {
    pub id: i32,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct LoginDto {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub access_token: String,
}
