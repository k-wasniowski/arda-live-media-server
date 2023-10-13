use actix_web::HttpResponse;

pub async fn get_health_check() -> HttpResponse {
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    println!("MyProgram v{}", VERSION);

    HttpResponse::Ok().finish()
}