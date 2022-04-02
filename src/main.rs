use session_based_authentication::configuration::get_configuration;
use session_based_authentication::issue_delivery_worker::run_worker_until_stopped;
use session_based_authentication::startup::Application;
use session_based_authentication::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = get_subscriber(
        "session_based_authentication".into(),
        "info".into(),
        std::io::stdout,
    );
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration.clone())
        .await?
        .run_until_stopped();
    let worker = run_worker_until_stopped(configuration);
    tokio::select! {
        _ = application => {},
        _ = worker => {},
    };
    Ok(())
}
