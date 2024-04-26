//! Report and sanction aggregator to perform targeted research.
mod helpers;
mod router;

use signaly_db::cassandra::Manager as ScyllaManager;
use signaly_db::kafka::Manager as KafkaManager;
use signaly_db::kafka::Type;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> signaly_error::Result<()> {
    tracing_subscriber::registry()
        .with(
            fmt::layer()
                .with_file(true)
                .with_line_number(true)
                .with_thread_ids(true),
        )
        .init();

    let scylla = ScyllaManager::new(
        vec!["cassandra:9042".to_string()],
        Some("cassandra".to_string()),
        Some("cassandra".to_string()),
        10,
    )
    .await;

    let kafka_hosts: Vec<String> = std::env::var("KAFKA_BROKERS")
        .unwrap_or_else(|_| "localhost:9092".to_string())
        .split(",")
        .map(ToString::to_string)
        .collect();

    let kafka_producer =
        KafkaManager::new(Type::Producer, None, kafka_hosts.clone()).await?;
    let kafka_consumer = KafkaManager::new(
        Type::Consumer,
        Some(std::env::var("TOPIC").unwrap_or_else(|_| "*".to_string())),
        kafka_hosts,
    )
    .await?;

    Ok(())
}
