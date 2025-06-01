use std::sync::Arc;

use axum::{extract::State, Json};
use serde_json::{json, Value};
use sqlx::FromRow;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

use crate::{state::AppState, utils::generate_token};

use super::users::UserModel;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct AuthTokenModel {
    pub id: Option<Uuid>,
    pub value: String,
    pub user_id: String,
    pub life_time_minutes: i32,
    pub active: Option<bool>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl AuthTokenModel {
    pub fn new(user: UserModel) -> Self {
        AuthTokenModel {
            id: None,
            value: generate_token(),
            user_id: user.id.unwrap().to_string(),
            life_time_minutes: 60,
            active: Some(true),
            created_at: None,
            updated_at: None
        }
    }

    pub async fn create(&self, State(state): &State<Arc<AppState>>) -> Result<Json<Value>, Json<Value>> {
        let query_result = sqlx::query_as!(
            AuthTokenModel,
            "INSERT INTO tokens (value,user_id,life_time_minutes) VALUES ($1, $2, $3) RETURNING *",
            self.value.to_string(),
            self.user_id.to_string(),
            self.life_time_minutes
        )
        .fetch_one(&state.db)
        .await;

        match query_result {
            Ok(token) => {
                let token_response = json!({"status": "success","data": json!({
                    "token": token
                })});

                return Ok(Json(token_response));
            }
            Err(e) => {
                if e.to_string()
                    .contains("duplicate value violates unique constraint")
                {
                    let error_response = serde_json::json!({
                        "status": "fail",
                        "message": "Token with that value already exists",
                    });
                    return Err(Json(error_response));
                }
                return Err(
                    Json(json!({"status": "error","message": format!("{:?}", e)}),
                ));
            }
        }
    }

    // pub async fn find_all(State(state): &State<Arc<AppState>>) -> Result<Vec<AuthTokenModel>, Json<serde_json::Value>> {
    //     let query_result = sqlx::query_as!(AuthTokenModel, "SELECT * FROM tokens ORDER by id")
    //         .fetch_all(&state.db)
    //         .await;

    //     if query_result.is_err() {
    //         let error_response = serde_json::json!({
    //             "status": "fail",
    //             "message": "Something bad happened while fetching all bots items",
    //         });
    //         return Err(Json(error_response));
    //     }
    //     let users = query_result.unwrap();
    //     Ok(users)
    // }

    pub async fn find_by_value(value: String, State(state): &State<Arc<AppState>>) -> Result<AuthTokenModel, Json<serde_json::Value>> {
        let query_result = sqlx::query_as!(AuthTokenModel, "SELECT * FROM tokens WHERE value = $1", value)
            .fetch_one(&state.db)
            .await;

        if query_result.is_err() {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Token with value: {} not found", value)
            });
            return Err(Json(error_response));
        }
        let token = query_result.unwrap();
        Ok(token)
    }
}