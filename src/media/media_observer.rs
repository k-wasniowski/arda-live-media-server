use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use webrtc::peer_connection::RTCPeerConnection;
use webrtc::rtcp::payload_feedbacks::picture_loss_indication::PictureLossIndication;
use webrtc::rtp::packet::Packet;
use webrtc::track::track_local::track_local_static_rtp::TrackLocalStaticRTP;
use async_trait::async_trait;
use webrtc::track::track_local::TrackLocalWriter;

#[async_trait]
pub trait IMediaObserver {
    async fn on_frame(&self, packet: &Packet);
}

pub struct MediaObserver {
    pub track: Arc<TrackLocalStaticRTP>,
}

impl MediaObserver {
    pub fn new(
        track: Arc<TrackLocalStaticRTP>,
    ) -> MediaObserver {
        Self {
            track,
        }
    }
}

#[async_trait]
impl IMediaObserver for MediaObserver {
    async fn on_frame(&self, packet: &Packet) {
        let write_result = self.track.write_rtp(&packet).await;
        match write_result {
            Ok(n) => {
                tracing::trace!("Frame of size {} has been sent", n);
            }
            Err(err) => {
                tracing::error!("Failed to send a frame with error - {:?}", err);
            }
        }
    }
}