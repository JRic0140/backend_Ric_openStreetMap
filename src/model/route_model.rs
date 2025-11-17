use serde::{Deserialize, Serialize};
use chrono::Utc;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ruta {
    pub id: Option<i32>,
    pub nombre: String,
    pub path: String,
    pub distancia: f64,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}