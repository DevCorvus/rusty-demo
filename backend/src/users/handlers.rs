use crate::database::DbPool;
use crate::password;

use super::{dto::CreateUserDto, service};
use actix_web::{
    delete, get,
    http::{header::ContentType, StatusCode},
    post, put, web, HttpResponse, Responder, ResponseError,
};
use thiserror::Error;

#[get("/profile")]
async fn get_user_profile() -> impl Responder {
    HttpResponse::Ok().body("User profile")
}

#[derive(Error, Debug)]
enum UserHandlerError {
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("Something went wrong")]
    InternalError,
}

impl ResponseError for UserHandlerError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }
    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self {
            Self::UserAlreadyExists => StatusCode::CONFLICT,
            Self::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[post("/")]
async fn create_user(
    pool: web::Data<DbPool>,
    data: web::Json<CreateUserDto>,
) -> actix_web::Result<impl Responder> {
    let new_user = web::block(move || {
        let mut conn = pool.get().map_err(|_| UserHandlerError::InternalError)?;

        let user_already_exists = service::user_exists_by_email(&mut conn, data.email.clone())
            .map_err(|_| UserHandlerError::InternalError)?;

        if user_already_exists {
            return Err(UserHandlerError::UserAlreadyExists);
        }

        let password_hash = password::hash_password(data.password.clone())
            .map_err(|_| UserHandlerError::InternalError)?;

        service::add_user(
            &mut conn,
            &CreateUserDto {
                email: data.email.clone(),
                password: password_hash,
            },
        )
        .map_err(|_| UserHandlerError::InternalError)
    })
    .await??;

    Ok(HttpResponse::Created().json(new_user))
}

#[put("/")]
async fn update_user() -> impl Responder {
    HttpResponse::Ok().body("Update user")
}

#[delete("/")]
async fn delete_user() -> impl Responder {
    HttpResponse::Ok().body("Delete user")
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/users")
        .service(get_user_profile)
        .service(create_user)
        .service(update_user)
        .service(delete_user);

    conf.service(scope);
}
