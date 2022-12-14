use jsonwebtoken::{decode, Algorithm, Validation, DecodingKey, TokenData};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub nonce: Option<String>,
    pub aud: Option<String>,
    exp: u128,
    iss: String,
    iat: u128
}

// Decode a JWT token and check if it is valid
pub fn get_jwt(token: String) -> Result<TokenData<Claims>, String> {
    match DecodingKey::from_rsa_pem("-----BEGIN PUBLIC KEY-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEArREXIedDPRu/ai/cJ/dr\nLJKZlb5An6Hg0DcZgERe9uI8cZMoEuxsWg7t9oWPp/xgJQdefK9a1PzfTKf9fE8y\nUEBPsV139ReHjnd7fzLM5wiGVhn0IAqr/vENi/2/3tD981CA+XQD7nkujTxUy66p\neNU2YkQQYryS4cTPibT3r+1ImyIogF4CcxFEIDeUBs17L4makd8bdAQ1GYZx2DTp\nbUFVbnupc8bJf9vsqiLp+LXSvRXmMhrWc7CgEyL/yOafAzTRCRCDqB1dpG4UwB/K\nzMZnrpecLuDaDxh0DxGVEEVAN30pksbKQNVXdZofBf81+UpaIKyLrA82MPFY/xzF\nQwIDAQAB\n-----END PUBLIC KEY-----".as_bytes()) {
        Ok(d) => {
            match decode::<Claims>(&token, &d, &Validation::new(Algorithm::RS256)) {
                Ok(token_data) => {
                    Ok(token_data)
                },
                Err(err) => Err(err.to_string()),
            }
        },
        Err(_) => Err("Error".to_string()),
    }
}