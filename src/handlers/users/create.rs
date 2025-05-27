use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{state::AppState, utils::models::{schemas::users::CreateUserSchema, users::UserModel}};

pub async fn new_user(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateUserSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let new_user = UserModel::new(body.username.to_string(), body.pass_hash.to_string());
    match new_user.create(&State(data)).await {
        Ok(res) => {
            return Ok((StatusCode::CREATED, res));
        },
        Err(res) => {
            return Err((StatusCode::CONFLICT, res));
        }
    }
}