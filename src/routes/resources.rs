use std::sync::Arc;
use actix_web::{web, HttpResponse};
use tokio::sync::Mutex;
use webrtc::api::interceptor_registry::register_default_interceptors;
use webrtc::api::media_engine::MediaEngine;
use webrtc::api::{API, APIBuilder};
use webrtc::data_channel::data_channel_message::DataChannelMessage;
use webrtc::ice_transport::ice_server::RTCIceServer;
use webrtc::interceptor::registry::Registry;
use webrtc::peer_connection::configuration::RTCConfiguration;
use webrtc::peer_connection::math_rand_alpha;
use webrtc::peer_connection::offer_answer_options::RTCAnswerOptions;
use webrtc::peer_connection::peer_connection_state::RTCPeerConnectionState;
use webrtc::peer_connection::sdp::session_description::RTCSessionDescription;
use serde_json::{Result, Value};
use serde_json::json;

pub async fn resources(
    body: web::Bytes,
    webrtc_api: web::Data<Arc<Mutex<API>>>,
) -> HttpResponse {
    let sdp_str = String::from_utf8(Vec::from(body)).unwrap();

    println!("{}", sdp_str);

    let api = webrtc_api.lock().await;

    let config = RTCConfiguration {
        ice_servers: vec![RTCIceServer {
            urls: vec!["stun:stun.l.google.com:19302".to_owned()],
            ..Default::default()
        }],
        ..Default::default()
    };

    let peer_connection_result = api.new_peer_connection(config).await;
    let peer_connection = Arc::new(match peer_connection_result {
        Ok(x) => x,
        Err(_) => {
            return HttpResponse::InternalServerError().body("Failed to create peer connection");
        }
    });

    let (done_tx, mut done_rx) = tokio::sync::mpsc::channel::<()>(1);

    let sdp_json = json!({
        "type": "offer",
        "sdp": sdp_str,
    });

    let remote_sdp = match serde_json::from_str::<RTCSessionDescription>(&sdp_json.to_string()) {
        Ok(x) => x,
        Err(_) => { return HttpResponse::InternalServerError().body("Failed to parse sdp"); }
    };

    match peer_connection.set_remote_description(remote_sdp).await {
        Ok(_) => { println!("Successfully set remote description"); }
        Err(_) => { return HttpResponse::InternalServerError().finish(); }
    };

    let answer = match peer_connection.create_answer(None).await {
        Ok(answer) => answer,
        Err(_) => { return HttpResponse::InternalServerError().finish(); }
    };

    match peer_connection.set_local_description(answer).await {
        Ok(_) => { println!("Successfully set local description"); }
        Err(_) => { return HttpResponse::InternalServerError().finish(); }
    };

    let answer_string = match peer_connection.local_description().await {
        None => { return HttpResponse::InternalServerError().finish(); }
        Some(answer) => {
            String::from(answer.sdp.as_str())
        }
    };

    println!("Sending OK");
    HttpResponse::Ok().body(answer_string)
}