#![allow(clippy::toplevel_ref_arg)]
pub mod authentication;
pub mod configuration;
pub mod domain;
pub mod email_client;
pub mod idempotency;
pub mod issue_delivery_worker;
pub mod routes;
pub mod session_state;
pub mod startup;
pub mod telemetry;
pub mod utils;

use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer}; //HttpRequest, Responder
use std::net::TcpListener;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

// async fn greet(req: HttpRequest) -> impl Responder {
//     let name = req.match_info().get("name").unwrap_or("World");
//     format!("Hello {}!", &name)
// }

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    // No .await here!
    Ok(server)
}
