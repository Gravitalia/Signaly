//! metrics provide real-time measurements for CPU time-usage, memory usage,
//! number of sanctions taken, and much more.
//!
//! # Example
//! ```rust
//!
//! ```

use prometheus::{Encoder, IntCounterVec, Opts, Registry};
use signaly_error::{Error, ErrorType, IoError::WriteError};
use std::{
    convert::Infallible,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};
use tracing::{error, info, trace};
use warp::{http::Response, reply::Response as ReplyResponse, Filter, Reply};

lazy_static! {
    static ref REGISTRY: Registry = Registry::new();
    // metrics about reports processed.
    pub static ref REPORTS_COLLECTOR: IntCounterVec = IntCounterVec::new(
        Opts::new("reports", "Information about reports"),
        &["platform", "reason"]
    )
    .expect("reports metric could not be created");
    // metrics about sanctions taken.
    pub static ref SANCTIONS_COLLECTOR: IntCounterVec = IntCounterVec::new(
        Opts::new("sanctions", "Information about sanctions"),
        &["reason", "moderator", "sanction"]
    )
    .expect("sanctions metric could not be created");
}

#[inline]
fn register_custom_metrics() {
    REGISTRY
        .register(Box::new(REPORTS_COLLECTOR.clone()))
        .expect("collector can be registered");

    REGISTRY
        .register(Box::new(SANCTIONS_COLLECTOR.clone()))
        .expect("collector can be registered");
}

#[inline]
async fn serve_req() -> Result<impl Reply, Error> {
    trace!("Metrics requested.");

    let encoder = prometheus::TextEncoder::new();

    let mut buffer = Vec::new();
    encoder
        .encode(&REGISTRY.gather(), &mut buffer)
        .map_err(|error| {
            Error::new(
                ErrorType::InuputOutput(WriteError),
                Some(Box::new(error)),
                Some("while encoding prometheus metrics".to_string()),
            )
        })?;
    let mut res = String::from_utf8(buffer.clone()).map_err(|error| {
        Error::new(
            ErrorType::InuputOutput(WriteError),
            Some(Box::new(error)),
            Some("while encoding prometheus metrics to string".to_string()),
        )
    })?;
    buffer.clear();

    let mut buffer = Vec::new();
    encoder
        .encode(&prometheus::gather(), &mut buffer)
        .map_err(|error| {
            Error::new(
                ErrorType::InuputOutput(WriteError),
                Some(Box::new(error)),
                Some("while encoding prometheus metrics".to_string()),
            )
        })?;
    let res_custom = String::from_utf8(buffer.clone()).map_err(|error| {
        Error::new(
            ErrorType::InuputOutput(WriteError),
            Some(Box::new(error)),
            Some("while encoding prometheus metrics to string".to_string()),
        )
    })?;
    buffer.clear();

    res.push_str(&res_custom);

    Ok(res)
}

/// Inits metrics and create Hyper server to handle `/metrics` route.
#[inline]
pub async fn create_server() {
    register_custom_metrics();

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 1112);
    info!(
        "Server is listening to {} awaiting requests for metrics.",
        addr
    );

    warp::serve(warp::any().and(warp::path("metrics").and_then(|| async {
        println!("o");
        match serve_req().await {
            Ok(metrics) => {
                Ok::<ReplyResponse, Infallible>(metrics.into_response())
            },
            Err(err) => {
                error!("Cannot retrieve metrics: {}", err);
                Ok(Response::builder()
                    .body("cannot retrieve metrics")
                    .into_response())
            },
        }
    })))
    .run(addr)
    .await;
}
