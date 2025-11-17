use std::sync::Arc;
use axum::{Form, Router, extract::State, response::Html, routing::get};
use crate::{controller::{LoginForm, RegisterController}, repository::UserRepository};

pub struct RegisterRoute {
    register_controller:Arc<RegisterController>
}

impl RegisterRoute {
    pub fn new(user_repo:Arc<UserRepository>) -> Self{
        let register_controller: Arc<RegisterController> = Arc::new(RegisterController::new(user_repo));
        
        Self {register_controller}
    }


    pub async fn routes(&self) -> Router{
        
        println!("RegisterRoute working");
        let controller = self.register_controller.clone();
        Router::new()
            .route("/register", get(
                async || Html("register route")
            )
            .post(
                register_route
             )
            .with_state(controller)
        )

    }


}

async fn register_route(
    State(controller): State<Arc<RegisterController>>,
    axum::extract::Form(form): Form<LoginForm>,
) -> Html<String> {
    controller.register_handler(form).await
}