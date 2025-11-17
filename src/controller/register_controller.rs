use std::sync::Arc;

use axum::Form;
use axum::http::StatusCode;
use axum::response::Html;

use crate::controller::LoginForm;
use crate::repository::UserRepository;
use crate::util::{GenerateJwt,ValidateToken};
// use jsonwebtoken;
pub struct RegisterController{
    user_repo:Arc< UserRepository>,

}

impl RegisterController{
    pub fn new (user_repo:Arc< UserRepository>) -> Self{
        
        Self{user_repo}
    }

    pub async fn register_handler(&self,axum::extract::Form(form):Form<LoginForm>) -> Result<(),StatusCode>{
            
        println!("{:?}", form);

        Ok(())

    }
}