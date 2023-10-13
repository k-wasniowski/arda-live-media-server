use actix_web::HttpResponse;

pub async fn post_webrtc_resource() -> HttpResponse {
    HttpResponse::Ok().finish()
}