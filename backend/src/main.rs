use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
mod users;

#[get("/health")]
async fn healthcheck() -> impl Responder {
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/api")
                .service(healthcheck)
                .configure(users::handlers::config),
        )
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
