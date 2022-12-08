use warp::{Filter, reject::Reject};
use tokio::sync::oneshot;
mod router;
mod model;

#[derive(Debug)]
struct UnknownError;
impl Reject for UnknownError {}

#[tokio::main]
async fn main() {
    let routes = warp::path("signal").and(warp::post()).and(warp::body::json()).and(warp::header("authorization")).map(router::signal::post);

    let (tx, rx) = oneshot::channel::<i32>();
    let (_addr, server) = warp::serve(routes)
    .bind_with_graceful_shutdown(([127, 0, 0, 1], 8889), async {
        rx.await.ok();
    });
    tokio::task::spawn(server);

    tokio::signal::ctrl_c().await.expect("failed to listen for event");
    let _ = tx.send(1);
}
