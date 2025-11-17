use std::fmt::Error;
use axum::{
    http::StatusCode,
    Router, routing::get,
};

use axum::body::Body;
use axum::{
    http::{Request},
    response::Response,
    middleware
};

use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};

use crate::repository::{RutaRepository};

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
#[derive(Debug)]
pub struct RoutesController {
    
}

impl RoutesController {
    pub fn new() -> Self {
        Self { }
    }

    pub fn routes(&self){
        
    }

    
    pub async fn create_route(&self
    ) ->  String {

        "create Route".to_string()
    }

    pub async fn update_route(&self
    ) -> String{


        "Update route".to_string()
    }

    pub async fn delete_route(&self
    ) -> String {
        "Delete route".to_string()
    }


    async fn find_route_by_id(&self,id: &str) -> Result<Route, Error> {
        // Aquí iría la lógica para buscar en la base de datos
        // Por ejemplo:
        // let route = database.find_route_by_id(id).await?;
        // Ok(route)
        
        // Ejemplo de retorno simulado
        todo!("Implementar búsqueda por ID")
    }


}

pub fn config_routes(pool :&Pool<Sqlite>) -> Router{
    // Crear repositorio
    let rutarepo = RutaRepository::new(pool.clone());

    let routes_controller = RoutesController::new();

    // Crear tabla si no existe
    let _ = rutarepo.crear_tabla();
    // add routes to axum router
    print!("repo creada");
    Router::new()
        //implement middleware
        .route("/routes", get(routes_controller.routes() ))
        // .route(
        //     "/routes/:id",
        //     get()
        //         .put(r)
        //         .delete(delete_route) .layer(middleware::from_fn(logging_middleware)),
        // )
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