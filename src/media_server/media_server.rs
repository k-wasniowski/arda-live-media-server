use std::sync::Arc;
use tokio::sync::Mutex;

use crate::media_server::generic_rtsp_client::GenericRtspClient;

pub struct MediaServer {
    pub generic_rtsp_client: GenericRtspClient,
}

impl MediaServer {
    pub fn new(generic_rtsp_client: GenericRtspClient) -> Arc<Mutex<MediaServer>> {
        Arc::new(Mutex::new(MediaServer {
            generic_rtsp_client
        }))
    }
}