use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{state::AppState, utils::models::{schemas::users::UpdateUserSchema, users::UserModel}};

pub async fn activate(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut user = UserModel::find(id, &State(data.clone())).await.unwrap();
    user.active = Some(true);

    match user.update(&State(data)).await {
        Ok(user) => {
            let user_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "user": user
            })});

            return Ok((StatusCode::OK, Json(user_response)));
        }
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", err)})),
            ));
        }
    }
}

pub async fn block(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut user = UserModel::find(id, &State(data.clone())).await.unwrap();
    user.active = Some(false);

    match user.update(&State(data)).await {
        Ok(user) => {
            let user_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "user": user
            })});

            return Ok((StatusCode::OK, Json(user_response)));
        }
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", err)})),
            ));
        }
    }
}

pub async fn edit_user(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateUserSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut user = UserModel::find(id, &State(data.clone())).await.unwrap();
    user.username = body.username.to_owned().unwrap_or(user.username);
    user.pass_hash = body.pass_hash.to_owned().unwrap_or(user.pass_hash);
    user.active = Some(body.active.unwrap_or(user.active.unwrap()));
    user.user_group = body.user_group.to_owned().unwrap_or(user.user_group);

    match user.update(&State(data)).await {
        Ok(user) => {
            let user_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "status": "Done",
                "user": user
            })});

            return Ok(Json(user_response));
        }
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", err)})),
            ));
        }
    }
}