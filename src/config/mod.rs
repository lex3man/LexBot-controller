use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub port: u32,
}

impl Config {
    pub fn new() -> Self {
        Config { port: 8080 }
    }
}