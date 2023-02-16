use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::media::MediaPublisher;

pub struct MediaEndpoints {
    pub media_endpoints: HashMap<String, Arc<Mutex<MediaPublisher>>>,
}

impl MediaEndpoints {
    pub fn new() -> Arc<Mutex<MediaEndpoints>> {
        Arc::new(
            Mutex::new(
                MediaEndpoints {
                    media_endpoints: Default::default()
                }
            )
        )
    }

    // pub fn find_by_name() -> Arc<Mutex<MediaPublisher>> {
    //
    // }
}