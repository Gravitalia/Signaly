use crate::{database::{cassandra::query, mem}, helpers};
use warp::reply::{WithStatus, Json};
use anyhow::Result;
use uuid::Uuid;

const EXP_CONSTANT: f32 = -0.0000021;
const BASE_VALUE: f32 = 30000.0;

/// Get the requiered count to take a certain action
fn req_followers(followers: u32, multiplier: f32) -> u32 {
    let calculation = (1.0 - (EXP_CONSTANT * followers as f32).exp()) * BASE_VALUE;
    
    let max = match followers {
        1..=20 => if multiplier == 10.0 { 100.0 } else { 10.0 },
        _ => 1.0,
    };
        
    (calculation * multiplier).max(max) as u32
}

/// Handle report route and make action against users
pub async fn post(body: crate::model::Signal, token: String) -> Result<WithStatus<Json>> {
    // Check token and set vanity
    let author_id = match helpers::get_jwt(token, dotenv::var("RSA_PUBLIC_KEY")?.as_bytes()) {
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

    let rate_limit = match mem::get(format!("signaly_{}_{}", body.vanity, author_id))? {
        Some(r) => r.parse::<u16>().unwrap_or(0),
        None => 0,
    };
    if rate_limit >= 1 {
        return Ok(super::rate());
    }

    let mut post_author: Option<String> = None;
    // Get user and user's followers
    let followers = match body.platform.to_lowercase().as_str() {
        "gravitalia" => {
            if body.vanity.chars().all(|c| c.is_ascii_digit()) {
                let post_data = helpers::get_gravitalia_likes(body.vanity.clone()).await?;
                post_author = Some(post_data.author);
                
                post_data.like
            } else {
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
            }
        },
        _ => {
            return Ok(super::err("Invalid platform".to_string()));
        }
    };

    // Prevent reporting his own post
    if post_author.is_some() && post_author.unwrap_or_default() == author_id {
        return Ok(super::err("You can't report your own post".to_string()));
    }

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
        Err(e) => {
            eprintln!("Database error: {}", e);
            return Ok(crate::router::err("Internal server error".to_string()));
        }
    };

    let count = match query_res[0][0].clone().into_plain() {
        Some(d) => {
            u32::from_be_bytes(d.try_into().unwrap_or_default())
        },
        None => {
            0
        }
    };
    
    mem::set(format!("signaly_{}_{}", body.vanity, author_id), 1)?;

    if req_followers(followers, 10.0) < count {
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
    } else if req_followers(followers, 2.0) < count {
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
