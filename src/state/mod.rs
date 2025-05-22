mod bots;
use bots::Bot;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AppState {
    pub status: String,
    pub bots: Vec<Bot>,
}