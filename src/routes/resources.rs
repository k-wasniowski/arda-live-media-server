use std::sync::Arc;
use actix_web::{web, HttpResponse};
use tokio::sync::Mutex;
use webrtc::api::interceptor_registry::register_default_interceptors;
use webrtc::api::media_engine::MediaEngine;
use webrtc::api::{API, APIBuilder};
use webrtc::data_channel::data_channel_message::DataChannelMessage;
use webrtc::ice_transport::ice_candidate::RTCIceCandidate;
use webrtc::ice_transport::ice_server::RTCIceServer;
use webrtc::interceptor::registry::Registry;
use webrtc::peer_connection::configuration::RTCConfiguration;
use webrtc::peer_connection::math_rand_alpha;
use webrtc::peer_connection::offer_answer_options::RTCAnswerOptions;
use webrtc::peer_connection::peer_connection_state::RTCPeerConnectionState;
use webrtc::peer_connection::sdp::session_description::RTCSessionDescription;
use serde_json::{Result, Value};
use serde_json::json;
use webrtc::data_channel::RTCDataChannel;

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

    let pending_candidates: Arc<Mutex<Vec<RTCIceCandidate>>> = Arc::new(Mutex::new(vec![]));

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

    let pending_candidates2 = Arc::clone(&pending_candidates);
    peer_connection.on_ice_candidate(Box::new(move |c: Option<RTCIceCandidate>| {
        println!("on_ice_candidate {:?}", c);

        let pending_candidates3 = Arc::clone(&pending_candidates2);
        Box::pin(async move {
            println!("Push new ice candidate - step 1");

            if let Some(c) = c {
                println!("Push new ice candidate - step 2");

                let mut cs = pending_candidates3.lock().await;
                cs.push(c);
            }
        })
    }));

    peer_connection.on_peer_connection_state_change(Box::new(move |s: RTCPeerConnectionState| {
        println!("Peer Connection State has changed: {s}");

        if s == RTCPeerConnectionState::Failed {
            // Wait until PeerConnection has had no network activity for 30 seconds or another failure. It may be reconnected using an ICE Restart.
            // Use webrtc.PeerConnectionStateDisconnected if you are interested in detecting faster timeout.
            // Note that the PeerConnection may come back from PeerConnectionStateDisconnected.
            println!("Peer Connection has gone to failed exiting");
            let _ = done_tx.try_send(());
        }

        Box::pin(async {})
    }));

    peer_connection.on_data_channel(Box::new(move |d: Arc<RTCDataChannel>| {
        println!("on_data_channel");

        Box::pin(async move {
            d.on_open(Box::new(move || {
                Box::pin(async move {
                    println!("On data channel open");
                })
            }));
            d.on_message(Box::new(move |message: DataChannelMessage| {
                Box::pin(async move {
                    println!("On data channel message: {:?}", message.data);
                })
            }));
        })
    }));


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

    let mut gather_complete = peer_connection.gathering_complete_promise().await;

    let _ = gather_complete.recv().await;

    let mut answer_string = match peer_connection.local_description().await {
        None => { return HttpResponse::InternalServerError().finish(); }
        Some(answer) => {
            String::from(answer.sdp.as_str())
        }
    };

    println!("Sending OK");
    HttpResponse::Ok().body(answer_string)
}