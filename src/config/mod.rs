use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DataBase {
    pub host: String,
    pub port: String,
    pub user: String,
    pub password: String,
    pub base_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub server_port: u32,
    pub database_config: DataBase,
}

impl Config {
    pub fn new() -> Self {
        let db_config = DataBase {
            host: "localhost".to_string(),
            port: "5432".to_string(),
            user: "admin".to_string(),
            password: "admin".to_string(),
            base_name: "lex_controller".to_string(),

        };
        Config { 
            server_port: 8080,
            database_config: db_config,
        }
    }

    pub fn get_database_url(&self) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}?schema=public",
            self.database_config.user,
            self.database_config.password,
            self.database_config.host,
            self.database_config.port,
            self.database_config.base_name
        )
    }
}