use axum::{Router, response::Html, routing::get};
use crate::repository::UserRepository;

pub struct RegisterRoute<'s> {
    user_repo:&'s UserRepository<'s>
}


impl<'s> RegisterRoute<'s> {
    pub fn new(user_repo:&'s UserRepository<'s>) -> Self{
        Self {user_repo}
    }

    pub async fn routes(&self) -> Router{
        println!("RegisterRoute working");

        Router::new()
            .route("/register", get(
                async || Html("register route")
            )
            .post(Html(format!("success")) )
        )
            // .with_state(app_state)

    }


}
