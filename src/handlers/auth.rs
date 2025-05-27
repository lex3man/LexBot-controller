use std::sync::Arc;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use crate::{state::AppState, utils::models::{auth_tokens::AuthTokenModel, schemas::users::CreateUserSchema, users::UserModel}};

pub async fn new_token(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateUserSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user = UserModel::find_by_username(body.username, State(data.clone())).await.unwrap();
    if user.pass_hash == body.pass_hash {
        let token = AuthTokenModel::new(user);
        match token.create(&State(data)).await {
            Ok(res) => return Ok((StatusCode::CREATED, res)),
            Err(res) => return Err((StatusCode::CONFLICT, res))
        }
    } else {
        return Err((StatusCode::UNAUTHORIZED, Json(json!({"msg": "wrong password"}))));
    }
}