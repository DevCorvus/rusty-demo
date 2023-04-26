use crate::{password::compare_password, users};
use actix_web::{post, web, HttpResponse, Responder};

use serde_json::json;

use super::{dto::LoginDto, jwt::encode_jwt};
use crate::{database::DbPool, error::AppError};

#[post("/login")]
async fn login(
    pool: web::Data<DbPool>,
    data: web::Json<LoginDto>,
) -> actix_web::Result<impl Responder> {
    let user_email = data.email.clone();

    let user = web::block(move || {
        let mut conn = pool.get().map_err(|_| AppError::InternalError)?;

        let user_found = users::service::get_user_by_email(&mut conn, user_email)
            .map_err(|_| AppError::InternalError)?;

        match user_found {
            Some(u) => Ok(u),
            None => Err(AppError::NotFound(String::from("User"))),
        }
    })
    .await??;

    let do_passwords_match = compare_password(data.password.clone(), user.password)
        .map_err(|_| AppError::InternalError)?;

    if do_passwords_match {
        let token = encode_jwt(user.id).map_err(|_| AppError::InternalError)?;
        Ok(HttpResponse::Ok().json(json!({ "token": token })))
    } else {
        Err(AppError::Unauthorized.into())
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/auth").service(login);

    conf.service(scope);
}
