//! Manage Apache Kafka message broker pool of connections.

mod pool;

pub use kafka::consumer::Consumer;
use kafka::{
    consumer::{FetchOffset, GroupOffsetStorage},
    producer::Record,
    Error as KafkaError,
};
use pool::KafkaConnectionManager;
use signaly_error::{
    DatabaseError::{PoolCreation, PoolObtention},
    Error, ErrorType,
};

type Pool = deadpool::managed::Pool<KafkaConnectionManager>;

/// Manage Apache Kafka pool connection.
#[allow(dead_code, missing_debug_implementations)]
pub struct Manager {
    /// Pool session.
    pub session: Pool,
}

impl Manager {
    /// Create a new pool of producer connections.
    ///
    /// # Examples
    /// ```rust
    /// use signaly_db::kafka::Manager as KafkaManger;
    ///
    /// let session = KafkaManger::new(
    ///     vec!["localhost:9092".to_string()],
    ///     10,
    /// );
    /// // Do what ever you want with your cool new session...
    /// ```
    pub async fn new(
        urls: Vec<String>,
        pool_size: usize,
    ) -> Result<Self, Error> {
        Ok(Manager {
            session: Pool::builder(KafkaConnectionManager::new(urls))
                .max_size(pool_size)
                .build()
                .map_err(|error| {
                    Error::new(
                        ErrorType::Database(PoolCreation),
                        Some(Box::new(error)),
                        None,
                    )
                })?,
        })
    }

    /// Send a message via Kafka broker.
    pub async fn send(
        &self,
        topic: String,
        content: String,
    ) -> Result<(), Error> {
        self.session
            .get()
            .await
            .map_err(|error| {
                Error::new(
                    ErrorType::Database(PoolObtention),
                    Some(Box::new(error)),
                    Some(
                        "while trying to send a message via Kafka".to_string(),
                    ),
                )
            })?
            .send(&Record::from_value(topic.as_str(), content))
            .map_err(|error| {
                Error::new(
                    ErrorType::Unspecified,
                    Some(Box::new(error)),
                    Some(
                        "while trying to send a message via Kafka".to_string(),
                    ),
                )
            })
    }
}

/// Create a consumer connection.
pub async fn new_consumer(
    topic: String,
    urls: Vec<String>,
) -> Result<Consumer, KafkaError> {
    Consumer::from_hosts(urls)
        .with_topic(topic)
        .with_group("my-group".to_string())
        .with_fallback_offset(FetchOffset::Earliest)
        .with_offset_storage(Some(GroupOffsetStorage::Kafka))
        .create()
}
