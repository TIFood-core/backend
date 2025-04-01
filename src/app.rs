use axum::Router;

use crate::middleware::cors::get_cors;

pub async fn create_app() -> Router {
    let app = Router::new().layer(get_cors());

    app
}
