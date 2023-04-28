use actix_cors::Cors;
use actix_web::{get, http::header, web, App, HttpResponse, HttpServer, Responder};
use backend::{auth, database, users};
use dotenvy::dotenv;

#[get("/health")]
async fn healthcheck() -> impl Responder {
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pool = database::connect_sqlite();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![header::CONTENT_TYPE, header::AUTHORIZATION]);

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/api")
                    .service(healthcheck)
                    .configure(users::handlers::config)
                    .configure(auth::handlers::config),
            )
            .wrap(cors)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
