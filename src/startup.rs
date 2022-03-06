use actix_web::web;
use actix_web::App;
use actix_web::HttpServer;

use crate::routes::home::home;

// fn run(/* */) -> Result</* */> {
//     let server = HttpServer::new(move || {
//         App::new()
//             .route("/", web::get().to(home))
//     })
// }

#[actix_web::main]
pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(home)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
