use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use entity::user;
use sea_orm::{ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;
use validator::Validate;

use crate::{services, state::AppState};

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
    if payload.validate().is_err() {
        return (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({
                "error": "Invalid payload"
            })),
        );
    }

    let db = &*state.db_conn;

    let user_exists_result = user::Entity::find()
        .filter(user::Column::Email.eq(&payload.email))
        .one(db)
        .await;

    if user_exists_result.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Failed to find user"
            })),
        );
    }

    if user_exists_result.unwrap().is_some() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "User already exists"
            })),
        );
    }

    let encrypt_password = services::encryptor::encrypt_password(&payload.password);
    let email_activation = Uuid::new_v4();

    let user = user::ActiveModel {
        email: Set(payload.email.clone()),
        password: Set(encrypt_password),
        email_activation: Set(Some(email_activation)),
        ..Default::default()
    };

    let register_user_result = user::Entity::insert(user).exec(db).await;

    if register_user_result.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Failed to register user"
            })),
        );
    }

    // TODO
    // mandar email com o codigo de ativação

    (StatusCode::OK, Json(json!({})))
}
