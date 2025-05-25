use axum::{
    response::{Json, IntoResponse},
};
use serde_json::{Value, json};

pub async fn get_system_config() -> Json<Value> {
    Json(json!({"data": "Ok"}))
}

pub async fn root() {}

pub async fn health_check() -> impl IntoResponse {
    const MESSAGE: &str = "Temp message";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}