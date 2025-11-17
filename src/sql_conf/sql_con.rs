use sqlx::SqlitePool;

const DATABASE_URL:&str = "sqlite:./my_db.db";  // Notación con guion bajo para legibilidad

pub async  fn sql_con() -> sqlx::Pool<sqlx::Sqlite> {

    SqlitePool::connect(DATABASE_URL).await.unwrap()

}