use std::sync::Arc;

use axum::{
    routing::{post}, Router
};

use crate::{handlers::auth::new_token, state::AppState};

pub fn get_auth_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(new_token))
}