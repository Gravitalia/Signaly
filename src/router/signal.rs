use crate::{database::cassandra::query, helpers};
use warp::reply::{WithStatus, Json};
use anyhow::Result;
use uuid::Uuid;

/// Get the requiered followers to take an action
fn req_followers(followers: u32, percent: f32) -> u32 {
    let calcul = (followers as f32 * percent).round() as u32;
    
    if percent < 0.15 {
        calcul.max(10)
    } else if (0.15..0.25).contains(&percent) {
        calcul.max(30)
    } else {
        calcul.max(100)
    }
}

/// Handle report route and make action against users
pub async fn post(body: crate::model::Signal, token: String) -> Result<WithStatus<Json>> {
    let gv_public_key = dotenv::var("GRAVITALIA_PUBLIC_KEY").expect("Missing env `GRAVITALIA_PUBLIC_KEY`");

    // Select the good public key
    let public_key: &[u8] = match body.platform.to_lowercase().as_str() {
        "gravitalia" => {
            gv_public_key.as_bytes()
        },
        _ => {
            return Ok(super::err("Invalid platform".to_string()));
        }
    };

    // Check token and set vanity
    let author_id = match helpers::get_jwt(token, public_key) {
        Ok(res) => {
            res.claims.sub
        },
        Err(_) => {
            return Ok(super::err("Invalid token".to_string()));
        }
    };

    // Prevent selfreport
    if body.vanity == author_id {
        return Ok(super::err("You can't report yourself".to_string()));
    }

    // Get user and user's followers
    let followers = match body.platform.to_lowercase().as_str() {
        "gravitalia" => {
            match helpers::get_gravitalia_sub(body.vanity.clone()).await {
                Ok(d) => {
                    if d.suspended {
                        return Ok(super::err("Invalid user".to_string()));
                    }

                    d.followers
                },
                Err(_) => {
                    return Ok(super::err("Invalid user".to_string()));
                }
            }
        },
        _ => {
            return Ok(super::err("Invalid platform".to_string()));
        }
    };

    // Check if reason is valid
    let reason  = match body.reason {
        0 => "Other",
        1 => "Violence, abuse or criminal content",
        2 => "Hate and harassement",
        3 => "Suicide or self-harm",
        4 => "NSFW content",
        5 => "Misinformation",
        6 => "Dangerous content",
        7 => "Personal data leak",
        8 => "Copyright/intellectual property violation",
        _ => {
            return Ok(super::err("Invalid reason".to_string()));
        }
    };

    query(format!("INSERT INTO signaly.reports (id, affected_id, author_id, platform, reason, timestamp) VALUES (?, ?, ?, ?, {}, '{}')", body.reason, chrono::Utc::now().format("%Y-%m-%d %H:%M:%S%:z")), vec![Uuid::new_v4().to_string(), body.vanity.clone(), author_id.clone(), body.platform.to_lowercase()])?;

    // Get total reports
    let query_res = match query("SELECT COUNT(id) FROM signaly.reports WHERE affected_id = ?", vec![body.vanity.clone()]) {
        Ok(x) => x.get_body().unwrap().as_cols().unwrap().rows_content.clone(),
        Err(_) => {
            return Ok(crate::router::err("Internal server error".to_string()));
        }
    };

    let count = match query_res[0][0].clone().into_plain() {
        Some(d) => {
            u32::from_be_bytes(d.try_into().unwrap_or_default())
        },
        None => {
            return Ok(crate::router::err("Internal server error".to_string()));
        }
    };

    if req_followers(followers, 0.26) > count {
        let platform_uri = match body.platform.to_lowercase().as_str() {
            "gravitalia" => {
                dotenv::var("GRAVITALIA_URL")?
            },
            _ => {
                return Ok(super::err("Invalid platform".to_string()));
            }
        };

        query(format!("INSERT INTO signaly.suspend (id, user_id, platform, expire_at) VALUES (?, ?, ?, '{}')", (chrono::Utc::now()+chrono::Duration::days(30)).format("%Y-%m-%d+0000")), vec![Uuid::new_v4().to_string(), body.vanity.clone(), platform_uri.clone()])?;
        helpers::suspend(body.vanity.clone(), Some(platform_uri)).await?;
        helpers::alert(author_id, body.platform.to_lowercase(), body.vanity, reason.to_string(), "Suspended account, check if it is a false-positive".to_string(), true).await?;
    } else if req_followers(followers, 0.19) > count {
        helpers::alert(author_id, body.platform.to_lowercase(), body.vanity, reason.to_string(), "Alerting support: too many reports".to_string(), true).await?;
    } else {
        helpers::alert(author_id, body.platform.to_lowercase(), body.vanity, reason.to_string(), "/".to_string(), false).await?;
    }

    Ok(warp::reply::with_status(warp::reply::json(
        &crate::model::Error{
            error: false,
            message: "OK".to_string(),
        }
    ),
    warp::http::StatusCode::OK))
}