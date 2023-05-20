use warp::{Filter, reject::Reject, http::StatusCode, Reply};
use std::error::Error;

mod database;
mod helpers;
mod router;
mod model;

#[derive(Debug)]
struct UnknownError;
impl Reject for UnknownError {}

// This function receives a `Rejection` and tries to return a custom
// value, otherwise simply passes the rejection along.
async fn handle_rejection(err: warp::Rejection) -> Result<impl Reply, std::convert::Infallible> {
    let code;
    let message: String;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "Not found".to_string();
    } else if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
        message = match e.source() {
            Some(cause) => {
                cause.to_string()
            }
            None => "Invalid body".to_string(),
        };
        code = StatusCode::BAD_REQUEST;
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "Method not allowed".to_string();
    } else {
        eprintln!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Internal server error".to_string();
    }

    Ok(warp::reply::with_status(warp::reply::json(&model::Error {
        error: true,
        message,
    }), code))
}

#[tokio::main]
async fn main() {
    // Init database
    database::cassandra::init();
    let _ =database::mem::init();
    database::cassandra::create_tables();
    helpers::remove_deleted_account().await;

    // Create routes
    let routes = warp::path("report")
                .and(warp::post())
                .and(warp::body::json())
                .and(warp::header("Authorization"))
                .and_then(|body: model::Signal, authorization: String| async move {
                    match router::signal::post(body, authorization).await {
                        Ok(r) => {
                            Ok(r)
                        },
                        Err(_) => {
                            Err(warp::reject::custom(UnknownError))
                        }
                    }
                })
                .or(warp::path("suspend")
                .and(warp::post())
                .and(warp::body::json())
                .and(warp::header("Authorization"))
                .and_then(|body: model::Suspend, authorization: String| async move {
                    match router::suspend::suspend(body, authorization).await {
                        Ok(r) => {
                            Ok(r)
                        },
                        Err(_) => {
                            Err(warp::reject::custom(UnknownError))
                        }
                    }
                }))
                .recover(handle_rejection);

    let port = dotenv::var("PORT").expect("Missing env `PORT`").parse::<u16>().unwrap();
    println!("Server started on port {}", port);

    // Start server
    warp::serve(warp::any().and(warp::options()).map(|| "OK").or(warp::head().map(|| "OK")).or(routes))
    .run((
        [0, 0, 0, 0],
        port
    ))
    .await;
}
