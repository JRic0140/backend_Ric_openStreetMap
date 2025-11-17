use std::sync::Arc;

use axum::Router;
mod register;
use register::RegisterRoute;
pub mod auth;
use crate::repository::UserRepository;
use crate::routes::auth::AuthRoute;
use crate::sql_conf::{
    sql_con
};

pub async fn config_routes () -> Router{

    let pool: sqlx::Pool<sqlx::Sqlite>= sql_con().await;
    
    let user_repo = Arc::new(UserRepository::new(pool.clone()));

    let login_router = AuthRoute::new(user_repo.clone()).await;
    let register_router = RegisterRoute::new(user_repo.clone());

    Router::new().merge(login_router.routes().await)
    .merge(register_router.routes().await)
    
}