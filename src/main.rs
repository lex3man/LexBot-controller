mod handlers;
mod utils;
mod middleware;
mod config;
mod state;
mod routers;

use std::sync::Arc;

use axum::{
    routing::get, Router
};
use config::Config;
use routers::{get_auth_routes, get_bots_routes, get_users_routes};
use state::AppState;
use utils::database::get_connector;

#[tokio::main]
async fn main() {
    let config = Config::new();
    let database_url = config.get_database_url();
    let pool = get_connector(&database_url).await;
    let app_state = Arc::new(AppState { db: pool.clone() });

    let bot_routes = get_bots_routes(&app_state);
    let user_routes = get_users_routes(&app_state);
    let auth_routes = get_auth_routes();

    let app = Router::new()
        .route("/", get(handlers::root))
        .route("/check", get(handlers::health_check))
        .route("/config", get(handlers::get_system_config))
        .nest("/bots", bot_routes)
        .nest("/users", user_routes)
        .nest("/auth", auth_routes)
        .with_state(app_state);
    
    let uri = format!("0.0.0.0:{}", config.server_port);
    let listener = tokio::net::TcpListener::bind(&uri).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}