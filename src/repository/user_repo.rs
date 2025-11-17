



use sqlx::{SqlitePool};

#[derive(Debug)]
pub struct UserRepository<'a>{
    pool: &'a SqlitePool,
}

impl<'a> UserRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn crear_tabla(&self)-> Result<(), sqlx::Error>{

        println!("crear_tabla");

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
            .execute(self.pool)
            .await?;
        println!("UserRepository running");

        Ok(())


    }
}

