use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_web::dev::Server;
use std::net::TcpListener;

async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

async fn subscribe() -> HttpResponse {
    HttpResponse::Ok().finish()
}
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    Ok(HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run())
}
