use std::sync::Arc;
use tokio::sync::Mutex;

use crate::media_server::generic_rtsp_client::GenericRtspClient;
use crate::media_server::media_server::MediaServer;

pub fn run() -> Arc<Mutex<MediaServer>> {
    println!("Initializing Media Server");

    let generic_rtsp_client = GenericRtspClient::new();

    MediaServer::new(generic_rtsp_client)
}
