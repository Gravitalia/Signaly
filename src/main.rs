#![deny(warnings)]
use tracing_subscriber::fmt::format::FmtSpan;
use warp::{Filter, reject::Reject};
use tokio::sync::oneshot;

mod database;
mod router;
mod model;

#[derive(Debug)]
struct UnknownError;
impl Reject for UnknownError {}

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
                .map(router::signal::post);

    let (tx, rx) = oneshot::channel::<i32>();
    let (_addr, server) = warp::serve(routes.with(warp::trace::request()))
    .bind_with_graceful_shutdown(([127, 0, 0, 1], port), async {
        rx.await.ok();
    });
    tokio::task::spawn(server);

    tokio::signal::ctrl_c().await.expect("failed to listen for event");
    let _ = tx.send(1);
}
