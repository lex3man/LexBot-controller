use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{state::AppState, utils::models::bots::BotModel};

pub async fn get_bot_config(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match BotModel::find(id, &State(data)).await {
        Ok(bot) => {
            let bot_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "bot": bot
            })});

            return Ok(Json(bot_response));
        }
        Err(_) => {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Bot with ID: {} not found", id)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
    }
}

pub async fn get_bots(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, Json<serde_json::Value>> {

    let bots:Vec<BotModel> = BotModel::find_all(&State(data)).await?;

    let json_response = serde_json::json!({
        "status": "success",
        "results": bots.len(),
        "bots": bots
    });
    
    Ok(Json(json_response))
}