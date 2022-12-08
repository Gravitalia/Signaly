use warp::reply::{WithStatus, Json};

pub fn post(_body: crate::model::Signal, _token: String) -> WithStatus<Json> {
    super::err("Not implemented yet".to_string())
}