use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{state::AppState, utils::models::{schemas::bots::UpdateBotSchema, bots::BotModel}};

pub async fn start_bot(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut bot = BotModel::find(id, &State(data.clone())).await.unwrap();
    bot.last_started = Some(chrono::Utc::now());
    bot.active = Some(true);

    match bot.update(&State(data)).await {
        Ok(bot) => {
            let bot_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "bot": bot
            })});

            return Ok((StatusCode::OK, Json(bot_response)));
        }
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", err)})),
            ));
        }
    }
}

pub async fn stop_bot(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut bot = BotModel::find(id, &State(data.clone())).await.unwrap();
    bot.last_stop = Some(chrono::Utc::now());
    bot.active = Some(false);

    match bot.update(&State(data)).await {
        Ok(bot) => {
            let bot_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "bot": bot
            })});

            return Ok((StatusCode::OK, Json(bot_response)));
        }
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", err)})),
            ));
        }
    }
}

pub async fn edit_bot(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateBotSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut bot = BotModel::find(id, &State(data.clone())).await.unwrap();
    bot.caption = body.caption.to_owned().unwrap_or(bot.caption);
    bot.token = body.token.to_owned().unwrap_or(bot.token);
    bot.active = Some(body.active.unwrap_or(bot.active.unwrap()));
    bot.state = body.state.to_owned().unwrap_or(bot.state);

    match bot.update(&State(data)).await {
        Ok(bot) => {
            let bot_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "status": "Done",
                "bot": bot
            })});

            return Ok(Json(bot_response));
        }
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", err)})),
            ));
        }
    }
}