// pub mod authentication;

use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer, Responder}; //HttpRequest,

async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

// async fn greet(req: HttpRequest) -> impl Responder {
//     let name = req.match_info().get("name").unwrap_or("World");
//     format!("Hello {}!", &name)
// }

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .bind("127.0.0.1:8000")?
        .run();
    // No .await here!
    Ok(server)
}
