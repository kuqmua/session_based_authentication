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

// use env_logger::Env;
use session_based_authentication::configuration::get_configuration;
use session_based_authentication::startup::run;
use sqlx::PgPool;
use std::net::TcpListener;

use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Redirect all `log`'s events to our subscriber
    LogTracer::init().expect("Failed to set logger");

    // We removed the `env_logger` line we had before!

    // We are falling back to printing all spans at info-level or above
    // if the RUST_LOG environment variable has not been set.
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new(
        "session_based_authentication".into(),
        // Output the formatted spans to stdout.
        std::io::stdout,
    );
    // The `with` method is provided by `SubscriberExt`, an extension
    // trait for `Subscriber` exposed by `tracing_subscriber`
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);
    // `set_global_default` can be used by applications to specify
    // what subscriber should be used to process spans.
    set_global_default(subscriber).expect("Failed to set subscriber");

    let configuration = get_configuration().expect("Failed to read configuration.");
    // Renamed!
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
