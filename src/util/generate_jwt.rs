use std::env;

use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};

use std::collections::HashMap;

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

pub fn validate_token(token: &str) -> jsonwebtoken::TokenData<Claims>{
    let validacion = Validation::default();

    let datos: jsonwebtoken::TokenData<Claims> = decode::<Claims>(
        token,
        &DecodingKey::from_secret("clave_secreta".as_ref()),
        &validacion,
    ).unwrap();

    println!("Claims: {:?}", datos.claims);
    return datos
}

pub fn parse_cookies(cookie_string: &str) -> HashMap<String, String> {
    let mut cookies = HashMap::new();
    
    // Dividir por punto y coma para separar las cookies
    for cookie in cookie_string.split("; ") {
        if let Some((key, value)) = cookie.split_once('=') {
            cookies.insert(key.to_string(), value.to_string());
        }
    }
    
    cookies
}