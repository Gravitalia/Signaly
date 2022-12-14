use tracing_subscriber::fmt::format::FmtSpan;
use warp::{Filter, reject::Reject};
use tokio::sync::oneshot;

mod database;
mod helpers;
mod router;
mod model;

#[derive(Debug)]
struct UnknownError;
impl Reject for UnknownError {}

/// Check if a token is valid, have a real user behind (not suspended) and if the fingerprint
/// is valid
fn middleware(token: Option<String>, fallback: String, _finger: Option<String>) -> String {
    if token.is_some() && fallback == *"@me" {
        match helpers::get_jwt(token.unwrap()) {
            Ok(data) => {
                // Check if user isn't deleted or suspended
                data.claims.sub
            },
            Err(_) => "Invalid".to_string()
        }
    } else if fallback == *"@me" {
        "Invalid".to_string()
    } else {
        fallback
    }
}

#[tokio::main]
async fn main() {
    let port = dotenv::var("PORT").expect("Missing env `PORT`").parse::<u16>().unwrap();

    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(true)
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let _postgres = database::postgres::init().await;
    let routes = warp::path("signal")
                .and(warp::post())
                .and(warp::body::json())
                .and(warp::header("Authorization"))
                .map(|body: model::Signal, token: String| {
                    let mid = middleware(Some(token), "@me".to_string(), None);
                    if mid == "Invalid" {
                        warp::reply::with_status(warp::reply::json(
                            &model::Error{
                                error: true,
                                message: "Invalid Authorization token".to_string(),
                            }
                        ),
                        warp::http::StatusCode::UNAUTHORIZED)
                    } else {
                        router::signal::post(body, mid)
                    }
                });

    let (tx, rx) = oneshot::channel::<i32>();
    let (_addr, server) = warp::serve(routes.with(warp::trace::request()))
    .bind_with_graceful_shutdown(([127, 0, 0, 1], port), async {
        rx.await.ok();
    });
    tokio::task::spawn(server);

    tokio::signal::ctrl_c().await.expect("failed to listen for event");
    let _ = tx.send(1);
}
