use std::sync::Arc;

use axum::{Router, body::Body, extract::State, http::{Request, Response, StatusCode}, middleware::{self, Next}, response::Html, routing::get};
use axum_cookie::cookie;
use sqlx::{ SqlitePool};

use crate::{controller::routes_controller::RoutesController, repository::RutaRepository, util::{parse_cookies, validate_token}};

pub struct RoutesRoute{
    ruta_controller:Arc<RoutesController>
}
impl RoutesRoute{
    pub fn new (pool:SqlitePool)->Self{
        // TODO
        let ruta_repo = RutaRepository::new(pool);
        let ruta_controller = Arc::new(RoutesController::new(ruta_repo));
        Self{ruta_controller}
    }
    pub async fn routes_handle(
        State(_state): State<Arc<RoutesController>>,

    )-> Html<String>{

        
        Html("200".to_owned())
        
    }

    pub async fn routes(&self) -> Router{
        let ruta_controller: Arc<RoutesController> = self.ruta_controller.clone();
        Router::new()
        .route("/routes",get(RoutesRoute::routes_handle))
        .with_state(ruta_controller)
        .layer(middleware::from_fn(
            |request: Request<Body>, next: Next,|
            my_middleware(request,next, )
        )) 
    }

}


async fn my_middleware(
    request: Request<Body>,
    next: Next,
) -> Response<Body> {
    // do something with `request`...
    // let state = request.extensions().get::<Arc<RoutesController>>();
    let cookies = parse_cookies(request.headers().get("cookie").unwrap().to_str().unwrap());
    let token = cookies.get("session_token").unwrap().as_str();

    println!("{:?}",cookies);

    let validations: jsonwebtoken::TokenData<crate::model::Claims> = validate_token(token);
    
    println!("validacion {:?}", validations);
    if validations.claims.exp == 2000000000{
        println!("validated")
    }else{
        println!("validation {:?}",validations.claims.exp)

    }
    println!("{:?}",request.headers());
    println!("{:?}",request.body());
    
    let response = next.run(request).await;


    // do something with `response`...
    response
}
