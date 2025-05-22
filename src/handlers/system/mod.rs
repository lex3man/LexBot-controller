use axum::{
    response::Json,
};
use serde_json::{Value, json};

pub async fn get_system_config() -> Json<Value> {
    Json(json!({"data": "Ok"}))
}
pub async fn root() {}
pub async fn health_check() {}