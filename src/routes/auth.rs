use axum::routing::{ post};
use axum::{Router};
use tokio::sync::Mutex;
use crate::model::User;
use crate::repository::{ UserRepository};
use crate::controller::{AuthController, login_handle};
use std::{collections::HashMap, sync::Arc};
use tower_cookies::{ CookieManagerLayer};
#[derive(Debug, Clone)]
pub struct AppState {
    pub sessions: Arc<Mutex<HashMap<String, User>>>,
    pub auth_controller :AuthController
}
pub struct AuthRoute{
        pub user_repo: Arc<UserRepository> 

}


impl AuthRoute{

    pub async fn new(user_repo: Arc<UserRepository> )-> Self{
        _ = user_repo.crear_tabla().await;
        Self { user_repo }
    }

    pub async fn routes (&self)->Router{
        let app_state = AppState {
            sessions: Arc::new(Mutex::new(HashMap::new())),
            auth_controller: AuthController::new(self.user_repo.clone())
            
        };
        println!("LoginRoute working");

        // add routes to axum router
    Router::new()
    .route("/login", post(login_handle))
    .with_state(app_state)
    .layer(CookieManagerLayer::new())
    }

}
