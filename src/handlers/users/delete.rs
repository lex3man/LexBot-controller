use std::sync::Arc;

use axum::{
    extract::{Path, State}, http::{StatusCode}, response::IntoResponse, Json
};

use crate::{state::AppState, utils::models::users::UserModel};

pub async fn delete_user(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user = UserModel::find(id, &State(data.clone())).await.unwrap();
    if user.delete(&State(data)).await.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("User with ID: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    Ok(StatusCode::NO_CONTENT)
}