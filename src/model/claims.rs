use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,   // sujeto (ej: user_id)
    pub exp: usize,    // fecha de expiración en timestamp UNIX
}
