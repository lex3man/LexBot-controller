use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{state::AppState, utils::models::{schemas::bots::CreateBotSchema, bots::BotModel}};

pub async fn new_bot(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateBotSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let new_bot = BotModel::new(&body.caption.to_string(), &body.token.to_string());
    match new_bot.create(&State(data)).await {
        Ok(res) => {
            return Ok((StatusCode::CREATED, res));
        },
        Err(res) => {
            return Err((StatusCode::CONFLICT, res));
        }
    }
}