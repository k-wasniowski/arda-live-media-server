use std::sync::Arc;
use tokio::sync::Mutex;
use crate::media::MediaObserver;

pub struct LiveMediaStream {
    pub observers: Vec<MediaObserver>,
}

impl LiveMediaStream {
    pub fn new() -> Arc<Mutex<LiveMediaStream>> {
        Arc::new(
            Mutex::new(
                LiveMediaStream {
                    observers: Vec::new(),
                }
            )
        )
    }
}
