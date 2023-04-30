use warp::{Filter, reject::Reject, http::StatusCode, Reply};

mod database;
mod router;
mod model;

#[derive(Debug)]
struct UnknownError;
impl Reject for UnknownError {}

/// handle_rejection handle the errors and allows
/// to send a valid response as gravitalia wants
async fn handle_rejection(_: warp::Rejection) -> Result<impl Reply, std::convert::Infallible> {
    Ok(warp::reply::with_status(warp::reply::json(&model::Error {
        error: true,
        message: "Invalid headers or body".to_string(),
    }), StatusCode::BAD_REQUEST))
}

#[tokio::main]
async fn main() {
    // Init database
    database::cassandra::init();
    database::cassandra::create_tables();

    // Create routes
    let routes = warp::path("report")
                .and(warp::post())
                .and(warp::body::json())
                .and(warp::header("Authorization"))
                .map(router::signal::post).recover(handle_rejection);

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
