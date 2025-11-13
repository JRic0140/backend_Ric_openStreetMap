use serde::{Deserialize, Serialize};
use chrono::Utc;
use std::time::Duration;
use sqlx::{SqlitePool, Row};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ruta {
    pub id: Option<i32>,
    pub nombre: String,
    pub path: String,
    pub distancia: f64,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}
#[derive(Debug)]
pub struct RutaRepository {
    pool: SqlitePool,
}

impl RutaRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }


        // Crear tabla de rutas
    pub async fn crear_tabla(&self) -> Result<(), sqlx::Error> {
        let query = r#"
        CREATE TABLE IF NOT EXISTS rutas (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            nombre TEXT NOT NULL,
            path TEXT NOT NULL,
            distancia REAL NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#;
        
        sqlx::query(query)
            .execute(&self.pool)
            .await?;
            
        Ok(())
    }


        // Guardar una nueva ruta
    pub async fn guardar_ruta(&self, ruta: &Ruta) -> Result<Ruta, sqlx::Error> {
        let query = r#"
        INSERT INTO rutas (nombre, path, distancia, duracion)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, nombre, path , distancia, created_at, updated_at
        "#;
        
        let row = sqlx::query(query)
            .bind(&ruta.nombre)
            .bind(&ruta.path)
            .bind(ruta.distancia)
            .fetch_one(&self.pool)
            .await?;
        
        let ruta_guardada = Ruta {
            id: Some(row.get("id")),
            nombre: row.get("nombre"),
            path: row.get("path"),
            distancia: row.get("distancia"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        };
        
        Ok(ruta_guardada)
    }

}