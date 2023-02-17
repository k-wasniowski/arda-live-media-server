use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::media::LiveMediaStream;

pub struct MediaResources {
    pub media_resources: HashMap<String, Arc<Mutex<LiveMediaStream>>>,
}

impl MediaResources {
    pub fn new() -> Arc<Mutex<MediaResources>> {
        Arc::new(
            Mutex::new(
                MediaResources {
                    media_resources: Default::default()
                }
            )
        )
    }

    pub fn insert(&mut self, resource_id: String, resource: Arc<Mutex<LiveMediaStream>>) {
        match self.media_resources.insert(resource_id, resource) {
            None => {}
            Some(_) => {}
        }
    }
}