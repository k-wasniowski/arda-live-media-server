use std::net::TcpListener;
use arda_live_media_server::configuration::get_configuration;
use arda_live_media_server::startup::run;
use arda_live_media_server::telemetry::{get_subscriber, init_subscriber};
use arda_live_media_server::webrtc_api::build_webrtc_api;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("arda-media-server".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let settings = get_configuration().expect("Failed to read configuration.");

    let webrtc_api = match build_webrtc_api() {
        Ok(api) => api,
        Err(e) => { return Err(std::io::Error::new(std::io::ErrorKind::Other, "foo")); }
    };

    let address = format!(
        "{}:{}",
        settings.application.host, settings.application.port
    );

    let listener = TcpListener::bind(address)?;

    run(listener, webrtc_api)?.await
}