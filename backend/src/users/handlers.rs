use crate::database::DbPool;

use super::{dto::CreateUserDto, service};
use actix_web::{delete, error, get, post, put, web, HttpResponse, Responder};

#[get("/profile")]
async fn get_user_profile() -> impl Responder {
    HttpResponse::Ok().body("User profile")
}

#[post("/")]
async fn create_user(
    pool: web::Data<DbPool>,
    data: web::Json<CreateUserDto>,
) -> actix_web::Result<impl Responder> {
    let new_user = web::block(move || {
        let mut conn = pool.get()?;
        service::add_user(&mut conn, &data)
    })
    .await?
    .map_err(|_| error::ErrorConflict("User already exists"))?;

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
