use crate::password;
use crate::{auth::middleware, database::DbPool};

use super::{dto::CreateUserDto, model::User, service};
use crate::error::AppError;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use common::{UpdateUserPasswordDto, UserResponse};

#[get("/profile")]
async fn get_user_profile(
    pool: web::Data<DbPool>,
    ctx: middleware::JwtMiddleware,
) -> actix_web::Result<impl Responder> {
    let user = web::block(move || {
        let mut conn = pool.get().map_err(|_| AppError::InternalError)?;

        let user_found =
            service::get_user_by_id(&mut conn, ctx.user_id).map_err(|_| AppError::InternalError)?;

        match user_found {
            Some(u) => Ok(u),
            None => Err(AppError::NotFound(String::from("User"))),
        }
    })
    .await??;

    Ok(HttpResponse::Ok().json(filter_user_output(&user)))
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

    Ok(HttpResponse::Created().json(filter_user_output(&new_user)))
}

#[put("/")]
async fn update_user_password(
    pool: web::Data<DbPool>,
    data: web::Json<UpdateUserPasswordDto>,
    ctx: middleware::JwtMiddleware,
) -> actix_web::Result<impl Responder> {
    web::block(move || {
        let mut conn = pool.get()?;
        let password_hash = password::hash_password(data.password.clone())?;
        service::update_user_password(&mut conn, ctx.user_id, password_hash)
    })
    .await?
    .map_err(|_| AppError::InternalError)?;

    Ok(HttpResponse::Ok().body("User password updated successfully"))
}

#[delete("/")]
async fn delete_user(
    pool: web::Data<DbPool>,
    ctx: middleware::JwtMiddleware,
) -> actix_web::Result<impl Responder> {
    web::block(move || {
        let mut conn = pool.get()?;
        service::delete_user(&mut conn, ctx.user_id)
    })
    .await?
    .map_err(|_| AppError::InternalError)?;

    Ok(HttpResponse::Ok().body("User deleted successfully"))
}

fn filter_user_output(user: &User) -> UserResponse {
    UserResponse {
        id: user.id,
        email: user.email.to_owned(),
        created_at: user.created_at,
        updated_at: user.updated_at,
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/users")
        .service(get_user_profile)
        .service(create_user)
        .service(update_user_password)
        .service(delete_user);

    conf.service(scope);
}
