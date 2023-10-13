use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub fn run(
    listener: TcpListener
) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/api/v1/health_check", web::get().to(crate::http_server::routes::health_check_controller::get_health_check))
    })
        .listen(listener)?
        .run();
    Ok(server)
}
