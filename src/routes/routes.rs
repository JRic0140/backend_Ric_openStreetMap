use axum::Router;

pub struct RoutesRoute{

}
impl RoutesRoute{
    pub fn new ()->Self{
        // TODO
        Self{}
    }


    pub async fn routes(&self) -> Router{


        Router::new()
        // .route("/routes",get())
    }
}