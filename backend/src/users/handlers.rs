use crate::password;
use crate::{auth::middleware, database::DbPool};

use super::{dto::CreateUserDto, service};
use crate::error::AppError;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

#[get("/profile")]
async fn get_user_profile(_: middleware::JwtMiddleware) -> impl Responder {
    HttpResponse::Ok().body("User profile")
}

#[post("/")]
async fn create_user(
    pool: web::Data<DbPool>,
    data: web::Json<CreateUserDto>,
) -> actix_web::Result<impl Responder> {
    let new_user = web::block(move || {
        let mut conn = pool.get().map_err(|_| AppError::InternalError)?;

        let user_already_exists = service::user_exists_by_email(&mut conn, data.email.clone())
            .map_err(|_| AppError::InternalError)?;

        if user_already_exists {
            return Err(AppError::AlreadyExists(String::from("User")));
        }

        let password_hash =
            password::hash_password(data.password.clone()).map_err(|_| AppError::InternalError)?;

        service::add_user(
            &mut conn,
            &CreateUserDto {
                email: data.email.clone(),
                password: password_hash,
            },
        )
        .map_err(|_| AppError::InternalError)
    })
    .await??;

    Ok(HttpResponse::Created().json(new_user))
}

#[put("/")]
async fn update_user(_: middleware::JwtMiddleware) -> impl Responder {
    HttpResponse::Ok().body("Update user")
}

#[delete("/")]
async fn delete_user(_: middleware::JwtMiddleware) -> impl Responder {
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
