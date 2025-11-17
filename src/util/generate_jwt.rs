use std::env;

use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};

use crate::model::Claims;

pub fn generate_jwt() -> String {

    dotenv::dotenv().ok(); // Carga el archivo .env
    let api_key = env::var("JWT_SECRET").expect("API_KEY no está definida");
    println!("La clave de la API es: {}", api_key);
    let my_claims = Claims {
        sub: api_key.to_owned(),
        exp: 2000000000, // ejemplo de timestamp futuro
    };

    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret("clave_secreta".as_ref()),
    ).unwrap();

    token
}

pub fn validate_token(token: &str) {
    let validacion = Validation::default();

    let datos = decode::<Claims>(
        token,
        &DecodingKey::from_secret("clave_secreta".as_ref()),
        &validacion,
    ).unwrap();

    println!("Claims: {:?}", datos.claims);
}