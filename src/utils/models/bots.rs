use std::sync::Arc;

use axum::{extract::State, Json};
use serde_json::{json, Value};
use sqlx::FromRow;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

use crate::state::AppState;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct BotModel {
    pub id: Option<Uuid>,
    pub caption: String,
    pub token: String,
    pub active: Option<bool>,
    pub state: String,
    #[serde(rename = "lastStarted")]
    pub last_started: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "lastStop")]
    pub last_stop: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl BotModel {
    pub fn new(caption: &String, token: &String) -> Self {
        BotModel { 
            id: None, 
            caption: caption.to_string(), 
            token: token.to_string(), 
            active: Some(false),
            state: "init".to_string(), 
            last_started: None, 
            last_stop: None, 
            created_at: None, 
            updated_at: None 
        }
    }

    pub async fn create(&self, State(state): &State<Arc<AppState>>) -> Result<Json<Value>, Json<Value>> {
        let query_result = sqlx::query_as!(
            BotModel,
            "INSERT INTO bots (caption,token,state) VALUES ($1, $2, $3) RETURNING *",
            self.caption.to_string(),
            self.token.to_string(),
            self.state.to_string()
        )
        .fetch_one(&state.db)
        .await;

        match query_result {
            Ok(bot) => {
                let bot_response = json!({"status": "success","data": json!({
                    "bot": bot
                })});

                return Ok(Json(bot_response));
            }
            Err(e) => {
                if e.to_string()
                    .contains("duplicate key value violates unique constraint")
                {
                    let error_response = serde_json::json!({
                        "status": "fail",
                        "message": "Bot with that caption or token already exists",
                    });
                    return Err(Json(error_response));
                }
                return Err(
                    Json(json!({"status": "error","message": format!("{:?}", e)}),
                ));
            }
        }
    }

    pub async fn find_all(State(state): &State<Arc<AppState>>) -> Result<Vec<BotModel>, Json<serde_json::Value>> {
        let query_result = sqlx::query_as!(
            BotModel,
            "SELECT * FROM bots ORDER by id"
        )
            .fetch_all(&state.db)
            .await;

        if query_result.is_err() {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": "Something bad happened while fetching all bots items",
            });
            return Err(Json(error_response));
        }
        let bots = query_result.unwrap();
        Ok(bots)
    }

    pub async fn find(
        id: Uuid,
        State(state): &State<Arc<AppState>>
    ) -> Result<BotModel, Json<serde_json::Value>> {
        let query_result = sqlx::query_as!(BotModel, "SELECT * FROM bots WHERE id = $1", id)
            .fetch_one(&state.db)
            .await;

        if query_result.is_err() {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Bot with ID: {} not found", id)
            });
            return Err(Json(error_response));
        }
        let bot = query_result.unwrap();
        Ok(bot)
    }

    pub async fn update(&self, State(state): &State<Arc<AppState>>) -> Result<BotModel, Json<serde_json::Value>> {
        let query_result = sqlx::query_as!(BotModel, "SELECT * FROM bots WHERE id = $1", self.id.unwrap())
            .fetch_one(&state.db)
            .await;

        if query_result.is_err() {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Bot with ID: {} not found", self.id.unwrap())
            });
            return Err(Json(error_response));
        }
        let now = chrono::Utc::now();

        let query_result = sqlx::query_as!(
            BotModel,
            "UPDATE bots SET caption = $1, token = $2, active = $3, state = $4, updated_at = $5, last_started = $7, last_stop = $8 WHERE id = $6 RETURNING *",
            self.caption,
            self.token,
            self.active,
            self.state,
            now,
            self.id.unwrap(),
            self.last_started,
            self.last_stop
        )
        .fetch_one(&state.db)
        .await;

        match query_result {
            Ok(bot) => return Ok(bot),
            Err(err) => return Err(Json(json!({"status": "error","message": format!("{:?}", err)})))
        }
    }

    pub async fn delete(
        &self, 
        State(state): &State<Arc<AppState>>
    ) -> Result<Json<serde_json::Value>, Json<serde_json::Value>> {
        let id = self.id.unwrap();
        let rows_affected = sqlx::query!("DELETE FROM bots  WHERE id = $1", id)
            .execute(&state.db)
            .await
            .unwrap()
            .rows_affected();

        if rows_affected == 0 {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Bot with ID: {} not found", id)
            });
            return Err(Json(error_response));
        };
        Ok(Json(serde_json::json!({"status": "ok"})))
    }
}