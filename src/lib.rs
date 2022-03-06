// pub mod authentication;

use actix_web::{web, App, HttpResponse, HttpServer, Responder}; //HttpRequest,

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

// async fn greet(req: HttpRequest) -> impl Responder {
//     let name = req.match_info().get("name").unwrap_or("World");
//     format!("Hello {}!", &name)
// }

pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // .route("/", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
        //order matters
        // .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}