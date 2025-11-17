use std::fmt::Error;
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





struct RoutesController{

}
impl RoutesController{
    pub fn new() -> Self{

        Self{
            
        }

    }
}




