mod controller;
pub mod repository;
use controller::{
    routes_controller
};

use axum::{
    Router
};
use sqlx::SqlitePool;
use tokio::net::{
    TcpListener
};

use crate::repository::{
    RutaRepository
};

#[tokio::main]
async fn main() {
    // Conectar a la base de datos SQLite
    let database_url = "sqlite:./my_db.db"; //  usa una ruta absoluta
    // sqlite conn
    let pool = SqlitePool::connect(database_url).await.unwrap();
    // Crear repositorio
    let repo: RutaRepository = RutaRepository::new(pool);
    // Crear tabla si no existe
    let _ = repo.crear_tabla().await;
    // build our application with a single route
    let app = Router::new().merge(routes_controller::config_routes());

    // run our app with hyper, listening globally on port 3000
    axum::serve(
        TcpListener::bind("0.0.0.0:3300").await.unwrap(),
         app).await.unwrap();

}
