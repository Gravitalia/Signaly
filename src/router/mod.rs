use warp::reply::{WithStatus, Json};
pub mod signal;
pub mod suspend;

/// Create message error easier
fn err(message: String) -> WithStatus<Json> {
    warp::reply::with_status(warp::reply::json(
        &super::model::Error{
            error: true,
            message,
        }
    ),
    warp::http::StatusCode::BAD_REQUEST)
}

/// Return an error with rate limit informations
fn rate() -> WithStatus<Json> {
    warp::reply::with_status(warp::reply::json(
        &crate::model::Error{
            error: true,
            message: "Too many requests".to_string(),
        }
    ),
    warp::http::StatusCode::TOO_MANY_REQUESTS)
}