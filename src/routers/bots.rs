use std::sync::Arc;

use axum::{
    routing::{get, patch, post}, Router
};

use crate::{handlers, middleware::AuthLayer, state::AppState};

pub fn get_bots_routes(state: &Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(handlers::bots::get_bots))
        .route("/stop/{id}", patch(handlers::bots::stop_bot))
        .route("/start/{id}", patch(handlers::bots::start_bot))
        .route("/{id}", 
            get(handlers::bots::get_bot_config)
            .patch(handlers::bots::edit_bot)
            .delete(handlers::bots::delete_bot),
        )
        .route("/new", post(handlers::bots::new_bot))
        .layer(AuthLayer{ state: state.clone() })
}