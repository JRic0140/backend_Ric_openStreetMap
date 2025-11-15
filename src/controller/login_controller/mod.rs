use std::{collections::HashMap, sync::Arc};

use axum::{Form, Router, extract::State, http::StatusCode, response::Html, routing::post};
use serde::Deserialize;
use sqlx::SqlitePool;
use tokio::sync::Mutex;

use crate::repository::{ UserRepository};
use crate::model::User;

// Formulario de login
#[derive(Deserialize)]
struct LoginForm {
    username: String,
    password: String,
}

#[derive(Debug, Clone)]
struct AppState {
    sessions: Arc<Mutex<HashMap<String, User>>>,
}


// Estado de la aplicación con sesiones
async fn login_handle(State(state): State<AppState>,
    Form(form): Form<LoginForm>,)  -> Result<Html<String>, StatusCode>{


        if form.username == "admin" && form.password == "password"{
            return Ok(Html(format!(
                        r#""200""#
                    )));
        }

       Err(StatusCode::UNAUTHORIZED)



}

pub fn config_routes(pool: SqlitePool) -> Router{
    let app_state = AppState {
            sessions: Arc::new(Mutex::new(HashMap::new())),
        };
    let user_repo = UserRepository::new(pool);
    let _ = user_repo.crear_tabla();
    // add routes to axum router
    Router::new()
        //implement middleware
        .route("/login", post(login_handle))
        
        // .route(
        //     "/routes/:id", 
        //     get(get_route)
        //         .put(update_route)
        //         .delete(delete_route) .layer(middleware::from_fn(logging_middleware)),
        // )
        .with_state(app_state)
}
