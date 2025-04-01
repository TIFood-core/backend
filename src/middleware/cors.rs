use tower_http::cors::CorsLayer;

use crate::config;

pub fn get_cors() -> CorsLayer {
    let cors_config = config::cors::get_cors_config();

    CorsLayer::new()
        .allow_origin(cors_config.origin.clone())
        .allow_methods(cors_config.methods.clone())
        .allow_headers(cors_config.headers.clone())
}
