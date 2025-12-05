use serde::{Deserialize, Serialize};




#[derive(Serialize, Deserialize,Debug)]
pub struct RouteRequestModel{
    pub name:String, 
    pub path:String
}