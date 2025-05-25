mod handlers;
mod utils;
mod middleware;
mod config;
mod state;

use std::sync::Arc;

use axum::{
    routing::{get, post, patch},
    Router,
};
use config::Config;
use state::AppState;
use utils::database::get_connector;

#[tokio::main]
async fn main() {
    let config = Config::new();
    let database_url = config.get_database_url();
    let pool = get_connector(&database_url).await;
    let app_state = Arc::new(AppState { db: pool.clone() });

    let app = Router::new()
        .route("/", get(handlers::root))
        .route("/check", get(handlers::health_check))
        .route("/config", get(handlers::get_system_config))
        .route("/bots", get(handlers::bots::get_bots))
        .route("/stop/{id}", patch(handlers::bots::stop_bot))
        .route("/start/{id}", patch(handlers::bots::start_bot))
        .route("/bots/{id}", 
            get(handlers::bots::get_bot_config)
            .patch(handlers::bots::edit_bot)
            .delete(handlers::bots::delete_bot),
        )
        .route("/bots/new_bot", post(handlers::bots::new_bot))
        .with_state(app_state);
    
    let uri = format!("0.0.0.0:{}", config.server_port);
    let listener = tokio::net::TcpListener::bind(&uri).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}