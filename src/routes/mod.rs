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

    let pool= sql_con().await;
    
    let user_repo = UserRepository::new(&pool);

    let login_router = AuthRoute::new(&user_repo).await;
    let register_router = RegisterRoute::new(&user_repo);

    Router::new().merge(login_router.routes().await)
    .merge(register_router.routes().await)
    
}