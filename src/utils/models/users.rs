use std::sync::Arc;

use axum::{extract::State, Json};
use serde_json::{json, Value};
use sqlx::FromRow;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

use crate::state::AppState;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct UserModel {
    pub id: Option<Uuid>,
    pub username: String,
    pub pass_hash: String,
    pub active: Option<bool>,
    pub user_group: String,
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl UserModel {
    pub fn new(username: String, password_hash: String) -> Self {
        UserModel {
            id: None,
            username: username,
            pass_hash: password_hash,
            active: Some(true),
            user_group: "user".to_string(),
            created_at: None,
            updated_at: None
        }
    }

    pub async fn create(&self, State(state): &State<Arc<AppState>>) -> Result<Json<Value>, Json<Value>> {
        let query_result = sqlx::query_as!(
            UserModel,
            "INSERT INTO users (username,pass_hash,user_group) VALUES ($1, $2, $3) RETURNING *",
            self.username.to_string(),
            self.pass_hash.to_string(),
            self.user_group.to_string()
        )
        .fetch_one(&state.db)
        .await;

        match query_result {
            Ok(user) => {
                let user_response = json!({"status": "success","data": json!({
                    "user": user
                })});

                return Ok(Json(user_response));
            }
            Err(e) => {
                if e.to_string()
                    .contains("duplicate key value violates unique constraint")
                {
                    let error_response = serde_json::json!({
                        "status": "fail",
                        "message": "User with that username already exists",
                    });
                    return Err(Json(error_response));
                }
                return Err(
                    Json(json!({"status": "error","message": format!("{:?}", e)}),
                ));
            }
        }
    }

    pub async fn find_all(State(state): &State<Arc<AppState>>) -> Result<Vec<UserModel>, Json<serde_json::Value>> {
        let query_result = sqlx::query_as!(UserModel, "SELECT * FROM users ORDER by id")
            .fetch_all(&state.db)
            .await;

        if query_result.is_err() {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": "Something bad happened while fetching all bots items",
            });
            return Err(Json(error_response));
        }
        let users = query_result.unwrap();
        Ok(users)
    }

    pub async fn find(
        id: Uuid,
        State(state): &State<Arc<AppState>>
    ) -> Result<UserModel, Json<serde_json::Value>> {
        let query_result = sqlx::query_as!(UserModel, "SELECT * FROM users WHERE id = $1", id)
            .fetch_one(&state.db)
            .await;

        if query_result.is_err() {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("User with ID: {} not found", id)
            });
            return Err(Json(error_response));
        }
        let user = query_result.unwrap();
        Ok(user)
    }

    pub async fn find_by_username(
        username: String,
        State(state): State<Arc<AppState>>
    ) -> Result<UserModel, Json<serde_json::Value>> {
        let query_result = sqlx::query_as!(UserModel, "SELECT * FROM users WHERE username = $1", username)
            .fetch_one(&state.db)
            .await;

        if query_result.is_err() {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("User with username: {} not found", username)
            });
            return Err(Json(error_response));
        }
        let user = query_result.unwrap();
        Ok(user)
    }

    pub async fn update(&self, State(state): &State<Arc<AppState>>) -> Result<UserModel, Json<serde_json::Value>> {
        let query_result = sqlx::query_as!(UserModel, "SELECT * FROM users WHERE id = $1", self.id.unwrap())
            .fetch_one(&state.db)
            .await;

        if query_result.is_err() {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("User with ID: {} not found", self.id.unwrap())
            });
            return Err(Json(error_response));
        }
        let now = chrono::Utc::now();

        let query_result = sqlx::query_as!(
            UserModel,
            "UPDATE users SET username = $1, pass_hash = $2, user_group = $3, active = $4, updated_at = $5 WHERE id = $6 RETURNING *",
            self.username,
            self.pass_hash,
            self.user_group,
            self.active,
            now,
            self.id.unwrap(),
        )
        .fetch_one(&state.db)
        .await;

        match query_result {
            Ok(user) => return Ok(user),
            Err(err) => return Err(Json(json!({"status": "error","message": format!("{:?}", err)})))
        }
    }

    

    pub async fn delete(
        &self, 
        State(state): &State<Arc<AppState>>
    ) -> Result<Json<serde_json::Value>, Json<serde_json::Value>> {
        let id = self.id.unwrap();
        let rows_affected = sqlx::query!("DELETE FROM users WHERE id = $1", id)
            .execute(&state.db)
            .await
            .unwrap()
            .rows_affected();

        if rows_affected == 0 {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("User with ID: {} not found", id)
            });
            return Err(Json(error_response));
        };
        Ok(Json(serde_json::json!({"status": "ok"})))
    }
}