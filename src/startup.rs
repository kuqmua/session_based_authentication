use actix_web::web;
use actix_web::App;
use actix_web::HttpServer;

use crate::routes::home::home;
use crate::routes::login::login_form;

// fn run(/* */) -> Result</* */> {
//     let server = HttpServer::new(move || {
//         App::new()
//             .route("/", web::get().to(home))
//     })
// }

#[actix_web::main]
pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/login", web::get().to(login_form))
            .route("/", web::get().to(home))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
