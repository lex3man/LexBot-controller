use axum::{body::Body, http::{header, Request}, Json, middleware::{Next}};
// use axum::{body::Body, extract::State, http::{header, Request}, Json, middleware::{Next}};
use serde_json::Value;

// use crate::{utils::models::auth_tokens::AuthTokenModel};

pub async fn auth_check(req: Request<Body>, _next: Next) -> Result<(), Json<Value>> {
    let _token = req.headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| {
                    if auth_value.starts_with("Bearer ") {
                        Some(auth_value[7..].to_owned())
                    } else {
                        None
                    }
                }).unwrap_or("none".to_string());
    // let _ = AuthTokenModel::find_by_value(token, &State(state.clone())).await?;
    Ok(())
}