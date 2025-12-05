
use crate::{model::{RouteRequestModel, Ruta}, repository::RutaRepository};


pub struct RoutesController{
    routes_repo:RutaRepository
}
impl RoutesController{
    pub fn new(routes_repo:RutaRepository) -> Self{

        Self{
            routes_repo
        }}

    pub async fn add_route(&self,route: RouteRequestModel) -> Result<(), sqlx::Error> {

        self.routes_repo.guardar_ruta(route.name,route.path).await?;

        Ok(())
    }
    pub async fn get_routes(&self) -> Result<Vec<Ruta>, sqlx::Error> {

        let routes = self.routes_repo.obtener_rutas().await?;

        Ok(routes)
    }
    


}






