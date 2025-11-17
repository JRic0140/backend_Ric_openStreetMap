use sqlx::{SqlitePool, Row};

use crate::model::User;

#[derive(Debug)]
pub struct UserRepository{
    pool: SqlitePool,
}

impl UserRepository{
    pub fn new(pool:SqlitePool) -> Self {

        Self {  pool }
    }

    pub async fn crear_tabla(&self)-> Result<(), sqlx::Error>{

        println!("crear_tabla");

        let query = r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user TEXT NOT NULL,
            password TEXT NOT NULL,
            token TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#;
            
         sqlx::query(query)
            .execute(&self.pool)
            .await?;
        println!("UserRepository running");

        Ok(())


    }
    pub async fn guardar_ruta(&self, user: String,password:String) -> Result<User, sqlx::Error> {
            let query = r#"
            INSERT INTO users (user, password)
            VALUES ($1, $2)
            RETURNING id, user, password, created_at, updated_at
            "#;
            let row = sqlx::query(query)
            .bind(user)
            .bind(password)
            .fetch_one(&self.pool)
            .await?;
            
            Ok(

                User{
                        id: Some(row.get("id")),
                        user: row.get("user"),
                        password: row.get("password"),
                        token:"".to_owned(),
                        created_at: row.get("created_at"),
                        updated_at: row.get("updated_at")

                        }

            )
        }

}

