use jsonwebtoken::{decode, Algorithm, Validation, DecodingKey, TokenData};
use chrono::{Duration as ChronoDuration, Utc};
use serde::{Serialize, Deserialize};
use std::time::Duration;
use anyhow::Result;
use crate::model;

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

/// Allows to get the Gravitalia post data and likes
pub async fn get_gravitalia_likes(id: String) -> Result<crate::model::GravitaliaUser> {
    Ok(reqwest::get(format!("{}posts/{}", dotenv::var("GRAVITALIA_URL")?, id))
        .await?
        .json::<crate::model::GravitaliaPost>()
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

/// Read the config content and return it
pub fn read() -> model::Config {
    let config: model::Config = serde_yaml::from_reader(std::fs::File::open("config.yaml").expect("Could not find config.yaml file")).expect("Could not read values of config.yaml file");
    config
}

/// Suspend a user with his vanity
pub async fn suspend(vanity: String, platform: Option<String>) -> Result<()> {
    if platform.is_some() {
        reqwest::Client::new().post(format!("{}account/suspend?vanity={}", platform.unwrap(), vanity))
        .header("Authorization", dotenv::var("GLOBAL_AUTH")?)
        .body("")
        .send()
        .await?;
    } else {
        // Send request to Autha
        reqwest::Client::new().post(format!("{}/account/suspend?vanity={}", dotenv::var("AUTHA_URL")?, vanity))
        .header("Authorization", dotenv::var("GLOBAL_AUTH")?)
        .body("")
        .send()
        .await?;

    // Send request to all other services
    for service in read().services {
        reqwest::Client::new().post(format!("{}/account/suspend?vanity={}", service, vanity))
        .header("Authorization", dotenv::var("GLOBAL_AUTH")?)
        .body("")
        .send()
        .await?;
    }
    }

    Ok(())
}

/// Check every day at 00h00 if users need to be deleted
pub async fn remove_deleted_account() {
    tokio::task::spawn(async {
        loop {
            let now = Utc::now();
            let time = (now.naive_utc().date().and_hms_opt(0, 0, 0).unwrap() + ChronoDuration::days(1)).timestamp()-now.timestamp();
            std::thread::sleep(Duration::from_secs(time as u64));

            if let Ok(x) = crate::database::cassandra::query(format!("SELECT user_id, platform FROM signaly.suspend WHERE expire_at = '{}'", now.format("%Y-%m-%d+0000")), vec![]) {
                let res = x.get_body().unwrap().as_cols().unwrap().rows_content.clone();

                for acc in res.iter() {
                    match std::str::from_utf8(&acc[1].clone().into_plain().unwrap()[..]).unwrap() {
                        "all" => {
                            reqwest::Client::new().delete(format!("{}/users/{}", dotenv::var("AUTHA_URL").expect("Missing env `AUTHA_URL`"), std::str::from_utf8(&acc[0].clone().into_plain().unwrap()[..]).unwrap()))
                            .header("Authorization", dotenv::var("GLOBAL_AUTH").expect("Missing env `GLOBAL_AUTH`"))
                            .body("")
                            .send()
                            .await
                            .unwrap();
    
                            for service in read().services {
                                reqwest::Client::new().delete(format!("{}/account/deletion?vanity={}", service, std::str::from_utf8(&acc[0].clone().into_plain().unwrap()[..]).unwrap()))
                                    .header("Authorization", dotenv::var("GLOBAL_AUTH").expect("Missing env `GLOBAL_AUTH`"))
                                    .body("")
                                    .send()
                                    .await
                                    .unwrap();
                            }
                        },
                        _ => {
                            reqwest::Client::new().delete(format!("{}account/deletion?vanity={}", std::str::from_utf8(&acc[1].clone().into_plain().unwrap()[..]).unwrap(), std::str::from_utf8(&acc[0].clone().into_plain().unwrap()[..]).unwrap()))
                                .header("Authorization", dotenv::var("GLOBAL_AUTH").expect("Missing env `GLOBAL_AUTH`"))
                                .body("")
                                .send()
                                .await
                                .unwrap();
                        }
                    }
                }
            };
        }
    });
}
