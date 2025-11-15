use serde::{Deserialize, Serialize};
use chrono::Utc;
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Option<i32>,
    pub user: String,
    pub password: String,
    pub token: String,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}