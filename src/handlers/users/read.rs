use std::sync::Arc;

use axum::{
    extract::{Path, State}, http::{StatusCode}, response::IntoResponse, Json
};

use crate::{state::AppState, utils::models::users::UserModel};

pub async fn get_user_info(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match UserModel::find(id, &State(data)).await {
        Ok(user) => {
            let user_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "user": user
            })});

            return Ok(Json(user_response));
        }
        Err(_) => {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("User with ID: {} not found", id)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
    }
}

pub async fn get_users(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, Json<serde_json::Value>> {

    let users:Vec<UserModel> = UserModel::find_all(&State(data)).await?;
    let json_response = serde_json::json!({
        "status": "success",
        "results": users.len(),
        "users": users
    });
    
    Ok(Json(json_response))
}