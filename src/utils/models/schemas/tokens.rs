use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthSchema {
    pub value: String,
    pub life_time_minutes: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    pub user_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateTokenSchema {
    pub value: Option<String>,
    pub life_time_minutes: Option<i32>,
    pub active: Option<bool>,
    pub user_id: Option<String>,
}