use std::sync::Arc;

use argon2::Config;
use axum::response::Html;
use rand::{Rng, thread_rng};
use crate::controller::LoginForm;
use crate::repository::UserRepository;
// use jsonwebtoken;
pub struct RegisterController{
    user_repo:Arc< UserRepository>,

}

impl RegisterController{
    pub fn new (user_repo:Arc< UserRepository>) -> Self{
        
        Self{user_repo}
    }


    pub async fn register_handler(&self,f:LoginForm) -> Html<String>{

        // println!("{:?}",&f);

        let password_hash = self.generate_argon(&f.password);

        let result = self.user_repo.guardar_ruta(f.username.clone(), password_hash.clone()).await;

        println!("generate_argon {:?}",&password_hash);

        println!("guardar_ruta {:?}",&result);
        
        Html(format!("success"))

    }
    

    fn generate_argon(&self, password:&str)->String{
            let salt: [u8; 16] = thread_rng().random(); // salt de 16 bytes
            let config = Config::default();


            // hash_encoded devuelve un String, lo retornamos directamente
            argon2::hash_encoded(password.as_bytes(), &salt, &config).unwrap()
    }

}