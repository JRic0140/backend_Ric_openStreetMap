use std::fmt::Error;

use axum::extract::Path;
use axum::response::IntoResponse;
// use actix_web::{web, HttpResponse, Responder};
use axum::{
    http::StatusCode,
    Router, routing::get,
    Json
};

use axum::body::Body;
use axum::{
    http::{Request},
    response::Response,
    middleware
};

use serde::{Deserialize, Serialize};

// use sqlx::{PgPool, Row};
// use uuid::Uuid;

/// Controller for /routes endpoints.
/// Expects a Postgres table `routes (id uuid primary key, name text, geometry jsonb)`.

#[derive(Serialize)]
pub struct Route {
    pub id: String,
    pub name: String,
    pub geometry: serde_json::Value,
}

#[derive(Deserialize)]
pub struct NewRoute {
    pub name: String,
    pub geometry: serde_json::Value,
}

#[derive(Deserialize)]
pub struct UpdateRoute {
    pub name: Option<String>,
    pub geometry: Option<serde_json::Value>,
}

pub async fn list_routes() ->  String {
    


    // quiero que me retorne un success o error dependiendo del resultado



    "Route List".to_string()
}



async fn get_route(Path(id): Path<String>) -> impl IntoResponse {
    // Aquí iría la lógica para obtener una ruta específica por ID
    // Por ejemplo, consultar una base de datos
    
    // Ejemplo básico de respuesta
    match find_route_by_id(&id).await {
        Ok(route) => {
            // Devolver la ruta encontrada
            Json(route).into_response()
        }
        Err(_) => {
            // Manejar el caso donde no se encuentra la ruta
            (StatusCode::NOT_FOUND, "Route not found").into_response()
        }
    }
}

async fn find_route_by_id(id: &str) -> Result<Route, Error> {
    // Aquí iría la lógica para buscar en la base de datos
    // Por ejemplo:
    // let route = database.find_route_by_id(id).await?;
    // Ok(route)
    
    // Ejemplo de retorno simulado
    todo!("Implementar búsqueda por ID")
}






pub async fn create_route(
) ->  String {

    "create Route".to_string()
}

pub async fn update_route(
) -> String{


    "Update route".to_string()
}

pub async fn delete_route(
) -> String {
    "Delete route".to_string()
}


pub fn config_routes() -> Router{

    // add routes to axum router
    Router::new()
        //implement middleware
        .route("/routes", get(|| async { "Hello, World!" } ))
        .route(
            "/routes/:id",
            get(get_route)
                .put(update_route)
                .delete(delete_route) .layer(middleware::from_fn(logging_middleware)),
        )
}


// middleware function
async fn logging_middleware(
    req: Request<Body>,
    next: middleware::Next,
) -> Result<Response, (StatusCode, String)> {
    println!("Método: {}", req.method());
    println!("Ruta: {}", req.uri());
   
    // get headers as string
    let headers = format!("{:?}", req.headers());
    // get token de autorizacion de los headers
    let auth_token = req.headers().get("authorization");
    let auth_str: &str = auth_token
        .and_then(|v| v.to_str().ok())
        .unwrap_or(&headers);

    println!("Headers: {}", auth_str);


    // Continuar con el handler original
    let res = next.run(req).await;
    Ok(res)
}