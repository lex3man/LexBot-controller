use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Bot {
    pub caption: String,
    pub token: String,
    pub active: bool,
    pub up_time: i64,
}