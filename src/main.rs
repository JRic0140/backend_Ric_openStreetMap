mod controller;
pub mod repository;
mod model;
mod routes;
mod sql_conf;
use tokio::net::TcpListener;


// run our app with hyper, listening globally on port 3000
#[tokio::main]
async fn main() {
    axum::serve(
        TcpListener::bind("0.0.0.0:3300").await.unwrap(),
         routes::config_routes().await).await.unwrap();
}
