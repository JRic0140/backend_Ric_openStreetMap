use sqlx::{SqlitePool, Row};

#[derive(Debug)]
pub struct UserRepository {
    pool: SqlitePool,
}

impl UserRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn crear_tabla(&self) -> Result<(), sqlx::Error> {
        let query = r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user TEXT NOT NULL,
            password TEXT NOT NULL,
            token TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#;
        
        sqlx::query(query)
            .execute(&self.pool)
            .await?;
            
        Ok(())
    }

}

