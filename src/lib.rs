use actix_web::dev::Server;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;
use diesel::prelude::*;
use diesel_async::AsyncPgConnection;

#[derive(serde::Deserialize)]
struct FormData{
    name: String,
    email: String,
}
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    Ok(server)
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

async fn subscribe(_form: web::Form<FormData>) -> impl Responder {
    HttpResponse::Ok()
}
