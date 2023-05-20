use crate::{database::cassandra::query, helpers};
use warp::reply::{WithStatus, Json};
use anyhow::Result;
use uuid::Uuid;

/// Handle suspend route
pub async fn suspend(body: crate::model::Suspend, token: String) -> Result<WithStatus<Json>> {
    // Check token and set vanity
    let author_id = match helpers::get_jwt(token.clone(), dotenv::var("RSA_PUBLIC_KEY").expect("Missing env `RSA_PUBLIC_KEY`").as_bytes()) {
        Ok(res) => {
            res.claims.sub
        },
        Err(_) => {
            return Ok(super::err("Invalid token".to_string()));
        }
    };

    let user = reqwest::Client::new().get(dotenv::var("AUTHA_URL")?)
        .header("Authorization", token)
        .send()
        .await?
        .json::<crate::model::AuthaUser>()
        .await?;

    // Check bitfields
    if user.flags & 32 == 0 {
        return Ok(super::err("You haven't enough flags to perform this action".to_string()));
    }

    // Prevent selfban
    if body.vanity == author_id {
        return Ok(super::err("You can't suspend yourself".to_string()));
    }

    let platform_uri = match body.platform.to_lowercase().as_str() {
        "gravitalia" => {
            dotenv::var("GRAVITALIA_URL")?
        },
        "all" => {
            "all".to_string()
        }
        _ => {
            return Ok(super::err("Invalid platform".to_string()));
        }
    };

    query(format!("INSERT INTO signaly.punishment (id, affected_id, mod_id, platform, punishment, timestamp) VALUES (?, ?, ?, ?, 0, '{}')", chrono::Utc::now().format("%Y-%m-%d+0000")), vec![Uuid::new_v4().to_string(), body.vanity.clone(), user.vanity, body.platform.clone()])?;

    helpers::suspend(body.vanity.clone(), Some(platform_uri)).await?;
    helpers::alert(author_id.clone(), body.platform.to_lowercase(), body.vanity, "/".to_string(), format!("Suspended account by {}", author_id), false).await?;

    Ok(warp::reply::with_status(warp::reply::json(
        &crate::model::Error{
            error: false,
            message: "OK".to_string(),
        }
    ),
    warp::http::StatusCode::OK))
}