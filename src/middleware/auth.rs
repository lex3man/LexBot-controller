use std::sync::Arc;

use axum::{extract::State, Json};
use serde_json::Value;

use crate::{state::AppState, utils::models::auth_tokens::AuthTokenModel};

pub async fn auth_check(token: String, State(state): &State<Arc<AppState>>) -> Result<(), Json<Value>> {
    let token = AuthTokenModel::find_by_value(token, &State(state.clone())).await?;
    dbg!(token);
    Ok(())
}