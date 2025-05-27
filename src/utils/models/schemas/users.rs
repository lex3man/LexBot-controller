use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserSchema {
    pub username: String,
    pub pass_hash: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    pub user_group: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateUserSchema {
    pub username: Option<String>,
    pub pass_hash: Option<String>,
    pub active: Option<bool>,
    pub user_group: Option<String>,
}