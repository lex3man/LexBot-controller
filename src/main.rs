mod handlers;
mod utils;
mod middleware;
mod config;
mod state;

use axum::{
    routing::{get, post},
    Router,
};
use config::Config;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handlers::root))
        .route("/check", get(handlers::health_check))
        .route("/config", get(handlers::get_system_config))
        .route("/bots", get(handlers::get_bots))
        .route("/start", post(handlers::start_bot))
        .route("/bot", get(handlers::get_bot_config));

    let config = Config::new();
    let uri = format!("0.0.0.0:{}", config.port);
    let listener = tokio::net::TcpListener::bind(&uri).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}