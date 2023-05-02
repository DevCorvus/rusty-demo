use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};
use common::{ErrorResponse, ValidationError, ValidationErrorResponse};
use thiserror::Error;
use validator::Validate;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Could not parse request data")]
    ParseError,
    #[error("You are not authorized")]
    Unauthorized,
    #[error("{0} not found")]
    NotFound(String),
    #[error("{0} already exists")]
    AlreadyExists(String),
    #[error("Something went wrong")]
    InternalError,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(ErrorResponse {
                message: self.to_string(),
                status: self.status_code().as_u16() as usize,
            })
    }
    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self {
            Self::ParseError => StatusCode::BAD_REQUEST,
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::AlreadyExists(_) => StatusCode::CONFLICT,
            Self::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

pub fn get_validation_error_response(input: impl Validate) -> Option<HttpResponse> {
    match input.validate() {
        Ok(_) => None,
        Err(errors) => {
            let mut mapped_errors: Vec<String> = vec![];

            for (field, _) in errors.into_errors() {
                let msg;
                match field {
                    "email" => msg = ValidationError::Email.to_string(),
                    "password" => msg = ValidationError::Password.to_string(),
                    _ => msg = String::new(),
                }
                mapped_errors.push(msg);
            }

            Some(HttpResponse::BadRequest().json(ValidationErrorResponse {
                status: 400,
                message: "Validation error".to_string(),
                errors: mapped_errors,
            }))
        }
    }
}
