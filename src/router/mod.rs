use warp::reply::{WithStatus, Json};
pub mod signal;
pub mod suspend;

fn err(message: String) -> WithStatus<Json> {
    warp::reply::with_status(warp::reply::json(
        &super::model::Error{
            error: true,
            message,
        }
    ),
    warp::http::StatusCode::BAD_REQUEST)
}