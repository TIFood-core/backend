use axum::{Router, routing::post};

use crate::{handlers, state::AppState};

pub fn configure_routes() -> Router<AppState> {
    Router::new().route("/register", post(handlers::user::register))
}
