use std::io::{Error, ErrorKind};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing_actix_web::TracingLogger;
use webrtc::api::API;

use crate::routes::{get_resources_endpoint, health_check};
use crate::routes::resources;
use crate::webrtc_api::build_webrtc_api;
use crate::media::MediaEndpoints;

pub fn run(
    listener: TcpListener,
    webrtc_api: Arc<Mutex<API>>,
    media_endpoints: Arc<Mutex<MediaEndpoints>>,
) -> Result<Server, std::io::Error> {
    let webrtc_api_data = web::Data::new(webrtc_api);
    let media_endpoints_data = web::Data::new(media_endpoints);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/api/v1/resources/{endpoint}", web::post().to(resources))
            .route("/api/v1/resources/{endpoint}", web::get().to(get_resources_endpoint))
            .app_data(webrtc_api_data.clone())
            .app_data(media_endpoints_data.clone())
    })
        .listen(listener)?
        .run();
    Ok(server)
}
