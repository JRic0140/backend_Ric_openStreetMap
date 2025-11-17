use std::sync::Arc;

use axum::{Form, Router, response::Html, routing::get};
use crate::{controller::{LoginForm, RegisterController}, repository::UserRepository};

pub struct RegisterRoute {
    register_controller:RegisterController
}

impl RegisterRoute {
    pub fn new(user_repo:Arc<UserRepository>) -> Self{
        let register_controller = RegisterController::new(user_repo);
        
        Self {register_controller}
    }

    pub async fn routes(&self) -> Router{
        
        println!("RegisterRoute working");

        Router::new()
            .route("/register", get(
                async || Html("register route")
            )
            // .post(|f: Form<LoginForm>| self.register_controller.register_handler(f))
            // .with_state()
        )

    }


}
