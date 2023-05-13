use std::vec;

use crate::database::cassandra::query;
use warp::reply::{WithStatus, Json};
use uuid::Uuid;

pub fn post(body: crate::model::Signal, token: String) -> WithStatus<Json> {
    let gv_public_key = dotenv::var("GRAVITALIA_PUBLIC_KEY").expect("Missing env `GRAVITALIA_PUBLIC_KEY`");

    // Select the good public key
    let public_key: &[u8] = match body.platform.to_lowercase().as_str() {
        "gravitalia" => {
            gv_public_key.as_bytes()
        },
        _ => {
            return super::err("Invalid platform".to_string());
        }
    };

    // Check token and set vanity
    let author_id = match crate::helpers::get_jwt(token, public_key) {
        Ok(res) => {
            res.claims.sub
        },
        Err(_) => {
            return super::err("Invalid token".to_string());
        }
    };

    // Prevent selfreport
    if body.vanity == author_id {
        return super::err("You can't report yourself".to_string());
    }

    let _ = query("INSERT INTO signaly.reports (id, affected_id, author_id, platform, reason, timestamp) VALUES (?, ?, ?, ?, ?, ?)", vec![Uuid::new_v4().to_string(), body.vanity.clone(), author_id, body.platform.to_lowercase()]);

    let query_res = match query("SELECT COUNT(id) FROM signaly.reports WHERE affected_id = ?", vec![body.vanity]) {
        Ok(x) => x.get_body().unwrap().as_cols().unwrap().rows_content.clone(),
        Err(_) => {
            return crate::router::err("Internal server error".to_string());
        }
    };

    let count = match query_res[0][0].clone().into_plain() {
        Some(d) => {
            u64::from_be_bytes(d.try_into().unwrap_or_default())
        },
        None => {
            return crate::router::err("Internal server error".to_string());
        }
    };

    println!("User have already {} reports in the month", count);

    super::err("Not implemented yet".to_string())
}