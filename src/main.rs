mod controller;
pub mod repository;
use controller::{
    routes_controller
};

use axum::{
    Router, routing::{delete as DELETE, get as GET, post as POST, put as PUT},

};
use sqlx::SqlitePool;
use tokio::net::{
    TcpListener
};

use crate::repository::routes_repo::RutaRepository;

#[tokio::main]
async fn main() {
    // Conectar a la base de datos SQLite
    let database_url = "sqlite:./rutas.db"; //  usa una ruta absoluta
    // sqlite conn
    let pool = SqlitePool::connect(database_url).await.unwrap();
    // Crear repositorio
    let repo = RutaRepository::new(pool);
    // Crear tabla si no existe
    let _ = repo.crear_tabla().await;
    // build our application with a single route
    let app = Router::new().merge(routes_controller::config_routes());

    // run our app with hyper, listening globally on port 3000
    axum::serve(
        TcpListener::bind("0.0.0.0:3000").await.unwrap(),
         app).await.unwrap();

}
