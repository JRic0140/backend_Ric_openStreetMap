
use axum::routing::get;
use axum::{Form, Router, extract::State, response::Html};
use tokio::sync::Mutex;
use crate::model::User;
use crate::repository::{ UserRepository};
use crate::controller::{AuthController,LoginForm};
use std::{collections::HashMap, sync::Arc};

#[derive(Debug, Clone)]
pub struct AppState {
    sessions: Arc<Mutex<HashMap<String, User>>>,
}
pub struct AuthRoute{
        user_repo: Arc<UserRepository> ,
}
impl AuthRoute{

    pub async fn new(user_repo: Arc<UserRepository> )-> Self{
        _ = user_repo.crear_tabla().await;
        Self { user_repo }
    }


    pub async fn routes (&self)->Router{

        let app_state = AppState {
            sessions: Arc::new(Mutex::new(HashMap::new())),
        };
        println!("LoginRoute working");

        // add routes to axum router
        Router::new()
            
            .route("/login", get(
                async || Html("login route")
            )
            .post(|s: State<AppState>,
                f: Form<LoginForm>| 
                AuthController::login_handle(s,f)))
            
            // .route("/register", post(LoginController::register_handle))
            .with_state(app_state)
    }

}

