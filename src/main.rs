// pub mod routes {
//     pub mod home;
//     pub mod login;
//     // pub mod newsletters;
// }
// pub mod startup;

// use crate::startup::run;

// // #[tokio::main(flavor = "multi_thread", worker_threads = 4)]
// fn main() {
//     if let Err(e) = run() {
//         println!("run error {:#?}", e);
//     }
// }

///////////////////////////////

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            //order matters
            .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    // Import the code I want to test
    use super::*;
    // My tests
}
