use jsonwebtoken::{decode, Algorithm, Validation, DecodingKey, TokenData};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    exp: u64,
    iss: String,
    iat: u64
}

/// Decode a JWT token and check if it is valid
pub fn get_jwt(token: String, public_key: &[u8]) -> Result<TokenData<Claims>, String> {
    let public_key = DecodingKey::from_rsa_pem(public_key).expect("Failed to load public key");

    decode::<Claims>(&token, &public_key, &Validation::new(Algorithm::RS256)).map_err(|err| err.to_string())
}