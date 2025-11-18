use std::fmt::Error;
use std::sync::Arc;
use axum::{
    http::StatusCode,
    Router, routing::get,
};

use axum::body::Body;
use axum::{
    http::{Request},
    response::Response,
    middleware
};

use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};

use crate::repository::{RutaRepository};





pub struct RoutesController{
    routes_repo:RutaRepository
}
impl RoutesController{
    pub fn new(routes_repo:RutaRepository) -> Self{

        Self{
            routes_repo
        }

    }
}




