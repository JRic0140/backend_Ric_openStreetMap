use std::sync::Arc;

use axum::response::Result;
use axum::{Form, extract::State, http::StatusCode, response::Html};
use serde::Deserialize;
use crate::model::User;
use crate::repository::UserRepository;
use crate::routes::auth::AppState;
use crate::util::GenerateJwt;
use argon2::{self, Config};
// Formulario de login
#[derive(Deserialize,Debug)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone)]

pub struct AuthController{
    user_repository:Arc<UserRepository>
}
impl AuthController{

    pub fn new(user_repository:Arc<UserRepository>) -> Self{
        // TODO
        Self{user_repository}
    }
    // Método estático o función que pueda ser usada directamente
    pub async fn login_handle(
        State(_state): State<AppState>,
        Form(form): Form<LoginForm>,
    ) -> Result<Html<String>, StatusCode> {
        let user = _state.auth_controller.user_repository.get_user_by_name(form.username.clone()).await;
        let user = user.unwrap();
        let login_verification = _state.auth_controller.verificar_login(&form.password, &user.password);

        // println!("login_handle  {:?}",&form.username);
        // println!("get_user_by_name  {:?}",&user);
        // println!("login_verification  {:?}",&login_verification);

        if login_verification {

            let session_token = GenerateJwt();

            // println!("session de token {:?} con usuario {:?} creada", &session_token, &user.user);
            let mut sessions = _state.sessions.lock().await;
            sessions.insert(session_token, user);
            println!("session {:?}",sessions.keys());
            return Ok(Html(format!(r#""200""#)));
        }
    
        Err(StatusCode::UNAUTHORIZED)
    }
    pub fn verificar_login(&self, password: &str, saved_hash: &str) -> bool {
    argon2::verify_encoded(saved_hash, password.as_bytes()).unwrap_or(false)
    }

}

