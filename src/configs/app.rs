use std::{env, sync::OnceLock};

pub struct AppConfig {
    pub port: u16,
}

static APP_CONFIG: OnceLock<AppConfig> = OnceLock::new();

pub fn get_app_config() -> &'static AppConfig {
    APP_CONFIG.get_or_init(|| {
        let port = env::var("APP_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(3000);

        AppConfig { port }
    })
}
