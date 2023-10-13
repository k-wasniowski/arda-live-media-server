use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing_actix_web::TracingLogger;

use crate::media_server::media_server::MediaServer;

pub fn run(
    listener: TcpListener,
    media_server: Arc<Mutex<MediaServer>>
) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/api/v1/health_check", web::get().to(crate::http_server::routes::health_check_controller::get_health_check))
            .route("/api/v1/resources/webrtc", web::post().to(crate::http_server::routes::webrtc_controller::post_webrtc_resource))
            .app_data(media_server)
    })
        .listen(listener)?
        .run();
    Ok(server)
}
