//! Report and sanction aggregator to perform targeted research.
mod helpers;
mod models;
mod router;

use signaly_db::cassandra::Manager as ScyllaManager;
use tracing::{error, Level};
use tracing_subscriber::fmt;

#[tokio::main]
async fn main() -> signaly_error::Result<()> {
    #[cfg(not(debug_assertions))]
    fmt()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_max_level(Level::INFO)
        .init();

    #[cfg(debug_assertions)]
    fmt()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_max_level(Level::TRACE)
        .init();

    let scylla = match ScyllaManager::new(
        std::env::var("CASSANDRA_HOSTS")
            .unwrap_or_else(|_| "127.0.0.1:9042".to_string())
            .split(',')
            .map(ToString::to_string)
            .collect(),
        Some(
            std::env::var("CASSANDRA_USERNAME")
                .unwrap_or_else(|_| "cassandra".to_string()),
        ),
        Some(
            std::env::var("CASSANDRA_PASSWORD")
                .unwrap_or_else(|_| "cassandra".to_string()),
        ),
        std::env::var("CASSANDRA_POOL_SIZE")
            .unwrap_or_else(|_| "10".to_string())
            .parse()
            .unwrap_or(10),
    )
    .await
    {
        Ok(conn) => conn,
        Err(err) => {
            error!(
                target = "signaly",
                error = err.to_string(),
                "Failed to establish connection with Apache Cassandra. This results in an inability to process requests; messages from Kafka or RabbitMQ will not be cleared."
            );
            std::process::exit(0);
        },
    };

    scylla.create_tables().await?;

    match (std::env::var("KAFKA_BROKERS"), std::env::var("AMQP_BROKER")) {
        #[cfg(feature = "kafka")]
        (Ok(brokers), Err(_)) => {
            use signaly_db::kafka::{new_consumer, Manager as KafkaManager};

            let kafka_hosts: Vec<String> =
                brokers.split(',').map(ToString::to_string).collect();

            let kafka_producer = KafkaManager::new(
                kafka_hosts.clone(),
                std::env::var("KAFKA_POOL_SIZE")
                    .unwrap_or_else(|_| "5".to_string())
                    .parse()
                    .unwrap_or(5),
            )
            .await?;

            let kafka_consumer = new_consumer(
                std::env::var("TOPIC").unwrap_or_else(|_| "*".to_string()),
                kafka_hosts,
            )
            .await?;

            helpers::consume_messages(kafka_consumer);

            #[cfg(not(feature = "telemetry"))]
            loop {}
        },
        #[cfg(feature = "rabbitmq")]
        (Err(_), Ok(address)) => {
            use signaly_db::rabbitmq::Manager as LapinManger;

            let rabbitmq = LapinManger::new(address).await?;
            helpers::consume_messages(
                rabbitmq,
                std::env::var("TOPIC").unwrap_or_else(|_| "*".to_string()),
            );

            #[cfg(not(feature = "telemetry"))]
            loop {}
        },
        _ => {
            error!("No specified broker in environment (KAFKA_BROKERS or AMQP_BROKER) OR wrong feature built.");
            std::process::exit(0);
        },
    }

    #[cfg(feature = "telemetry")]
    {
        use signaly_telemetry::metrics::create_server;

        create_server().await;

        Ok(())
    }
}
