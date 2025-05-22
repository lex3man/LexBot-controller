pub mod bots;
mod system;
pub use system::{get_system_config, root, health_check};

pub async fn get_bots() {}
pub async fn start_bot() {}
pub async fn get_bot_config() {}