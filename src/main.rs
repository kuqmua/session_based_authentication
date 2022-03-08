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

#[cfg(test)]
mod tests {
    // Import the code I want to test
    use super::*;
    // My tests
}

// mod configuration;

// use session_based_authentication::configuration::get_configuration;
// use session_based_authentication::run;
// use std::net::TcpListener;

// #[tokio::main]
// async fn main() -> std::io::Result<()> {
//     let configuration = get_configuration().expect("Failed to read configuration.");
//     let address = format!("127.0.0.1:{}", configuration.application_port);
//     let listener = TcpListener::bind(address).expect("Failed to bind random port");
//     // We retrieve the port assigned to us by the OS
//     run(listener)?.await
// }

// use session_based_authentication::configuration::get_configuration;
// use session_based_authentication::startup::run;
// use sqlx::{Connection, PgConnection};
// use std::net::TcpListener;

// #[tokio::main]
// async fn main() -> std::io::Result<()> {
//     let configuration = get_configuration().expect("Failed to read configuration.");
//     let connection = PgConnection::connect(&configuration.database.connection_string())
//         .await
//         .expect("Failed to connect to Postgres.");
//     let address = format!("127.0.0.1:{}", configuration.application_port);
//     let listener = TcpListener::bind(address)?;
//     run(listener, connection)?.await
// }

use session_based_authentication::configuration::get_configuration;
use session_based_authentication::startup::run;
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    // Renamed!
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
