use std::sync::Arc;

use axum::Router;

use crate::{connections, middleware, state::AppState};

pub async fn create_app() -> Router {
    let db_conn = Arc::new(connections::database::get_database_connection().await);

    let state = AppState { db_conn };

    let app = Router::new()
        .layer(middleware::cors::get_cors())
        .with_state(state);

    app
}
