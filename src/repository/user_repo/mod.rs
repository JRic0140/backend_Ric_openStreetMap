use serde::{Deserialize, Serialize};
use chrono::Utc;
use std::time::Duration;
use sqlx::{SqlitePool, Row};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Option<i32>,
    pub user: String,
    pub password: String,
    pub token: String,

    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}
#[derive(Debug)]
pub struct UserRepository {
    pool: SqlitePool,
}

impl UserRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

}

