use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

#[get("/profile")]
async fn get_user_profile() -> impl Responder {
    HttpResponse::Ok().body("User profile")
}

#[post("/")]
async fn create_user() -> impl Responder {
    HttpResponse::Ok().body("Create user")
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
