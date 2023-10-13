use std::net::TcpListener;
use arda_live_media_server::configuration::get_configuration;

use arda_live_media_server::telemetry::{get_subscriber, init_subscriber};

use arda_live_media_server::http_server::http_server_startup;
use arda_live_media_server::media_server::media_server_startup;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("arda-media-server".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let settings = get_configuration();
    let settings = match settings {
        Ok(settings) => settings,
        Err(e) => {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to read configuration."));
        }
    };

    let address = format!(
        "{}:{}",
        settings.application.host, settings.application.port
    );

    let listener = TcpListener::bind(address)?;

    let http_server_startup_result = http_server_startup::run(listener);
    let http_server = match http_server_startup_result {
        Ok(server) => server,
        Err(e) => {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to start HTTP server."));
        }
    };

    media_server_startup::run();

    http_server.await
}