// use crate::routes::home::home;
// use crate::routes::login::login;
// use crate::routes::login::login_form;
//
// fn run(/* */) -> Result</* */> {
//     let server = HttpServer::new(move || {
//         App::new()
//             .route("/", web::get().to(home))
//     })
// }

// #[actix_web::main]
// pub async fn run() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .route("/login_form", web::get().to(login_form))
//             .route("/login", web::post().to(login))
//             .route("/", web::get().to(home))
//     })
//     .bind("127.0.0.1:8000")?
//     .run()
//     .await
// }

// use crate::routes::{health_check, subscribe};
// use actix_web::dev::Server;
// use actix_web::{web, App, HttpServer};
// use sqlx::PgConnection;
// use std::net::TcpListener;

// pub fn run(listener: TcpListener, connection: PgConnection) -> Result<Server, std::io::Error> {
//     // Wrap the connection in a smart pointer
//     let connection = web::Data::new(connection);
//     // Capture `connection` from the surrounding environment
//     let server = HttpServer::new(move || {
//         App::new()
//             .route("/health_check", web::get().to(health_check))
//             .route("/subscriptions", web::post().to(subscribe))
//             // Get a pointer copy and attach it to the application state
//             .app_data(connection.clone())
//     })
//     .listen(listener)?
//     .run();
//     Ok(server)
// }

use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
// use actix_web::middleware::Logger;
use crate::email_client::EmailClient;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub fn run(
    listener: TcpListener,
    db_pool: PgPool,
    email_client: EmailClient,
) -> Result<Server, std::io::Error> {
    let db_pool = Data::new(db_pool);
    let email_client = Data::new(email_client);
    let server = HttpServer::new(move || {
        App::new()
            // Middlewares are added using the `wrap` method on `App`
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db_pool.clone())
            .app_data(email_client.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
