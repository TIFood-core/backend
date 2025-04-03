use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use serde_json::json;
use validator::Validate;

use crate::state::AppState;

#[derive(Deserialize, Validate)]
pub struct CreateUserPayload {
    #[validate(email)]
    email: String,
    #[validate(length(min = 8))]
    password: String,
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserPayload>,
) -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({
            "hello": "world"
        })),
    )
}
