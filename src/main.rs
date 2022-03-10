use secrecy::ExposeSecret;
use session_based_authentication::configuration::get_configuration;
use session_based_authentication::startup::run;
use session_based_authentication::telemetry::{get_subscriber, init_subscriber};
// use sqlx::postgres::PgPool;
use std::net::TcpListener;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber(
        "session_based_authentication".into(),
        "info".into(),
        std::io::stdout,
    );
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPoolOptions::new()
    .connect_timeout(std::time::Duration::from_secs(2)).connect_lazy(&configuration.database.connection_string().expose_secret())
            .expect("Failed to connect to Postgres.");

    let address = format!(
                "{}:{}",
                configuration.application.host, configuration.application.port
            );
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await?;
    Ok(())
}
