use axum::response::Result;
use axum::{Form, extract::State, http::StatusCode, response::Html};
use serde::Deserialize;
use crate::routes::auth::AppState;

// Formulario de login
#[derive(Deserialize,Debug)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone)]

pub struct AuthController{

}
impl AuthController{

    pub fn new() -> Self{
        // TODO
        Self{}
    }
    // Método estático o función que pueda ser usada directamente
    pub async fn login_handle(
        State(_state): State<AppState>,
        Form(form): Form<LoginForm>,
    ) -> Result<Html<String>, StatusCode> {
        if form.username == "admin" && form.password == "password"{
            return Ok(Html(format!(r#""200""#)));
        }
        Err(StatusCode::UNAUTHORIZED)
    }


    pub async fn register_handle(){
        //TODO
    }

}

