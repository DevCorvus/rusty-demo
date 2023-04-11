use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/health")]
async fn healthcheck() -> impl Responder {
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(healthcheck))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
