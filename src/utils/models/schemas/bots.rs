use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateBotSchema {
    pub caption: String,
    pub token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_started: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_stop: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateBotSchema {
    pub caption: Option<String>,
    pub token: Option<String>,
    pub active: Option<bool>,
    pub state: Option<String>,
    pub last_started: Option<chrono::DateTime<chrono::Utc>>,
    pub last_stop: Option<chrono::DateTime<chrono::Utc>>,
}