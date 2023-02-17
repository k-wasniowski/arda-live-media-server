use std::sync::Arc;
use std::time::Duration;
use actix_web::{web, HttpResponse};
use tokio::sync::{Mutex, MutexGuard};
use webrtc::api::interceptor_registry::register_default_interceptors;
use webrtc::api::media_engine::{MediaEngine, MIME_TYPE_VP8};
use webrtc::api::{API, APIBuilder};
use webrtc::data_channel::data_channel_message::DataChannelMessage;
use webrtc::ice_transport::ice_candidate::RTCIceCandidate;
use webrtc::ice_transport::ice_server::RTCIceServer;
use webrtc::interceptor::registry::Registry;
use webrtc::peer_connection::configuration::RTCConfiguration;
use webrtc::peer_connection::{math_rand_alpha, RTCPeerConnection};
use webrtc::peer_connection::offer_answer_options::RTCAnswerOptions;
use webrtc::peer_connection::peer_connection_state::RTCPeerConnectionState;
use webrtc::peer_connection::sdp::session_description::RTCSessionDescription;
use serde_json::{Result, Value};
use serde_json::json;
use webrtc::data_channel::RTCDataChannel;
use webrtc::track::track_remote::TrackRemote;
use webrtc::rtcp::payload_feedbacks::picture_loss_indication::PictureLossIndication;
use webrtc::rtp_transceiver::rtp_codec::RTCRtpCodecCapability;
use webrtc::track::track_local::track_local_static_rtp::TrackLocalStaticRTP;
use webrtc::track::track_local::{TrackLocal, TrackLocalWriter};
use webrtc::util::Unmarshal;
use crate::media::{IMediaObserver, MediaResources, MediaObserver, LiveMediaStream};
use webrtc::Error;

pub async fn post_resources(
    body: web::Bytes,
    path: web::Path<String>,
    webrtc_api: web::Data<Arc<Mutex<API>>>,
    media_endpoints_data: web::Data<Arc<Mutex<MediaResources>>>,
) -> HttpResponse {
    let sdp_str = String::from_utf8(Vec::from(body)).unwrap();

    let api = webrtc_api.lock().await;

    let mut media_endpoints = media_endpoints_data.lock().await;

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

    let sdp_json = json!({
        "type": "offer",
        "sdp": sdp_str,
    });

    let mut media_endpoints_data_1 = media_endpoints_data.clone();
    let resource_id = path.to_string();
    peer_connection.on_peer_connection_state_change(Box::new(move |state: RTCPeerConnectionState| {
        println!("Peer Connection State has changed: {state}");

        let mut media_endpoints_data_2 = media_endpoints_data_1.clone();
        let resource_id_1 = resource_id.clone();
        Box::pin(async move {
            let mut media_endpoints_data_3 = media_endpoints_data_2.clone();
            let resource_id_2 = resource_id_1.clone();

            match state {
                RTCPeerConnectionState::Failed | RTCPeerConnectionState::Closed => {
                    let media_endpoints = media_endpoints_data_3.lock().await;


                }
                _ => {}
            }
        })
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

    let live_media_stream = LiveMediaStream::new();

    //media_endpoints.media_endpoints.insert(path.to_string(), live_media_stream.clone());

    peer_connection.on_track(Box::new(move |track, _| {
        let video_track = track.unwrap();
        let live_media_stream_1 = live_media_stream.clone();

        tokio::spawn(async move {
            let mut result = Result::<usize>::Ok(0);

            let live_media_stream_2 = live_media_stream_1.clone();

            while result.is_ok() {
                let rtp_packet = video_track.read_rtp().await.unwrap();

                let locked_live_media_stream = live_media_stream_2.lock().await;

                for media_observer in &locked_live_media_stream.observers {
                    media_observer.on_frame(&rtp_packet.0).await;
                }
            }
        });

        Box::pin(async move {
            println!("On track received");
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

    HttpResponse::Ok().body(answer_string)
}

pub async fn get_resources(
    body: web::Bytes,
    path: web::Path<String>,
    webrtc_api: web::Data<Arc<Mutex<API>>>,
    media_endpoints_data: web::Data<Arc<Mutex<MediaResources>>>,
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

    let video_track = Arc::new(TrackLocalStaticRTP::new(
        RTCRtpCodecCapability {
            mime_type: MIME_TYPE_VP8.to_owned(),
            ..Default::default()
        },
        "video".to_owned(),
        "webrtc-rs".to_owned(),
    ));

    let rtp_sender = peer_connection
        .add_track(Arc::clone(&video_track) as Arc<dyn TrackLocal + Send + Sync>)
        .await;

    let media_endpoints_data_1 = media_endpoints_data.clone();
    let path_1 = Arc::new(path);
    peer_connection.on_peer_connection_state_change(Box::new(move |s: RTCPeerConnectionState| {
        println!("Peer Connection State has changed: {s}");

        if s == RTCPeerConnectionState::Failed {
            println!("Peer Connection has gone to failed exiting");
        }

        let path_2 = path_1.clone();
        let video_track_1 = video_track.clone();

        let media_endpoints_data_2 = media_endpoints_data_1.clone();

        Box::pin(async move {
            if s == RTCPeerConnectionState::Connected {
                let media_endpoints = media_endpoints_data_2.lock().await;

                let video_track_2 = video_track_1.clone();

                // let mut media_endpoint = media_endpoints.media_endpoints.get(path_2.as_str()).unwrap();
                //
                // let mut locked_media_endpoint = media_endpoint.lock().await;
                //
                // let media_observer = MediaObserver::new(
                //     video_track_2
                // );
                //
                // locked_media_endpoint.observers.push(media_observer);
            }
        })
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

    peer_connection.on_track(Box::new(move |track, _| {
        println!("HERE Track on received!!!!!!!!!!!!!");
        Box::pin(async move {
            println!("HERE Track on received!!!!!!!!!!!!!");
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
