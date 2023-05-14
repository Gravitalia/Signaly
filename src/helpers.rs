use jsonwebtoken::{decode, Algorithm, Validation, DecodingKey, TokenData};
use serde::{Serialize, Deserialize};
use anyhow::Result;

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

/// Allows to get the Gravitalia profile and subscriers
pub async fn get_gravitalia_sub(vanity: String) -> Result<crate::model::GravitaliaUser> {
    Ok(reqwest::get(format!("{}users/{}", dotenv::var("GRAVITALIA_URL")?, vanity))
        .await?
        .json::<crate::model::GravitaliaUser>()
        .await?)
}

/// Allows to get the Gravitalia profile and subscriers
pub async fn alert(vanity: String, platform: String, affected_user: String, reason: String, action_taken: String, with_mention: bool) -> Result<()> {
    let msg_begin = if with_mention {
        "<@&843784021494333471> ".to_string()
    } else {
        "".to_string()
    };

    let json_body = format!(r#"
        {{
            "content": "{}New report request from [{}](https://www.gravitalia.com/{}) for the `{}` platform, against [{}](https://www.gravitalia.com/{}) for **{}**.",
            "embeds": [
                {{
                    "color": 3353411,
                    "fields": [
                        {{
                            "name": "Author",
                            "value": "{}",
                            "inline": true
                        }},
                        {{
                            "name": "Affected user",
                            "value": "{}",
                            "inline": true
                        }},
                        {{
                            "name": "Reason",
                            "value": "{}",
                            "inline": true
                        }},
                        {{
                            "name": "Action taken",
                            "value": "{}"
                        }}
                    ],
                    "timestamp": "{}"
                }}
            ],
            "attachments": []
        }}
        "#,
        msg_begin,
        vanity,
        vanity,
        platform,
        affected_user,
        affected_user,
        reason,
        vanity,
        affected_user,
        reason,
        action_taken,
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S%:z")
    );

    reqwest::Client::new().post(dotenv::var("DISCORD_WEBHOOK")?)
        .header("Content-Type", "application/json")
        .body(json_body)
        .send()
        .await?;

    Ok(())
}