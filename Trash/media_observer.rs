use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use webrtc::peer_connection::RTCPeerConnection;
use webrtc::rtcp::payload_feedbacks::picture_loss_indication::PictureLossIndication;
use webrtc::rtp::packet::Packet;
use webrtc::track::track_local::track_local_static_rtp::TrackLocalStaticRTP;

pub trait IMediaObserver {
    fn on_frame(&self, packet: &Packet);
}

pub struct MediaObserver {
    pub peer_connection: Arc<RTCPeerConnection>,
    pub video_track: Arc<TrackLocalStaticRTP>,
}

impl MediaObserver {
    pub fn new(
        peer_connection: Arc<RTCPeerConnection>,
        video_track: Arc<TrackLocalStaticRTP>,
    ) -> MediaObserver {
        Self {
            peer_connection,
            video_track,
        }
    }
}

impl IMediaObserver for MediaObserver {
    fn on_frame(&self, packet: &Packet) {
        println!("I got frame !!!");
    }
}

pub struct MediaPublisher {
    observers: Vec<MediaObserver>,
}

impl MediaPublisher {
    pub fn new() -> Arc<Mutex<MediaPublisher>> {
        Arc::new(
            Mutex::new(
                MediaPublisher {
                    observers: Vec::new(),
                }
            )
        )
    }

    pub fn on_frame(&self, packet: Packet) {
        println!("I got frame !!!");
    }
}