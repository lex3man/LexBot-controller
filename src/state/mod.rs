mod bots;
use bots::Bot;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

#[derive(Serialize, Deserialize)]
pub struct SystemState {
    pub status: String,
    pub bots: Vec<Bot>,
}

pub struct AppState {
    pub db: Pool<Postgres>,
}