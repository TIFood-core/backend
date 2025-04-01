use std::{env, sync::OnceLock};

use axum::http::{HeaderValue, Method};
use tower_http::cors::Any;

#[derive(Clone)]
pub struct CorsConfig {
    pub origin: HeaderValue,
    pub methods: Vec<Method>,
    pub headers: Any,
}

static CORS_CONFIG: OnceLock<CorsConfig> = OnceLock::new();

pub fn get_cors_config() -> &'static CorsConfig {
    CORS_CONFIG.get_or_init(|| {
        let origin = env::var("CORS_ORIGIN")
            .expect("CORS_ORIGIN not found at .env file")
            .parse::<HeaderValue>()
            .unwrap();

        let methods = vec![Method::GET, Method::POST, Method::PUT, Method::DELETE];

        let headers = Any;

        CorsConfig {
            origin,
            methods,
            headers,
        }
    })
}
