use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;
use webrtc::api::interceptor_registry::register_default_interceptors;
use webrtc::api::media_engine::MediaEngine;
use webrtc::api::{API, APIBuilder};
use webrtc::interceptor::registry::Registry;

pub fn build_webrtc_api() -> Result<Arc<Mutex<API>>, &'static str> {
    let mut media_engine = MediaEngine::default();

    match media_engine.register_default_codecs() {
        Ok(_) => { println!("Registered default codecs"); }
        Err(_) => { return Err("Failed to register default codecs"); }
    };

    let mut interceptor_registry = Registry::new();
    interceptor_registry = match register_default_interceptors(interceptor_registry, &mut media_engine) {
        Ok(x) => x,
        Err(_) => { return Err("Failed to register default codecs"); }
    };

    let api = APIBuilder::new()
        .with_media_engine(media_engine)
        .with_interceptor_registry(interceptor_registry)
        .build();

    Ok(Arc::new(Mutex::new(api)))
}
