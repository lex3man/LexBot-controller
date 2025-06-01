use std::sync::Arc;

use axum::{
    routing::{get, patch, post}, Router
};

use crate::{handlers, middleware::AuthLayer, state::AppState};

pub fn get_users_routes(state: &Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(handlers::users::get_users))
        .route("/block/{id}", patch(handlers::users::block))
        .route("/activate/{id}", patch(handlers::users::activate))
        .route("/{id}", 
            get(handlers::users::get_user_info)
            .patch(handlers::users::edit_user)
            .delete(handlers::users::delete_user),
        )
        .route("/new", post(handlers::users::new_user))
        .layer(AuthLayer{ state: state.clone() })
}