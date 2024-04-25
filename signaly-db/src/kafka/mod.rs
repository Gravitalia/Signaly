//! Manage Apache Kafka message broker pool of connections.

mod pool;

use pool::KafkaConnectionManager;
use signaly_error::{DatabaseError::PoolCreation, Error, ErrorType};

type Pool = deadpool::managed::Pool<KafkaConnectionManager>;

/// Manage InfluxDB pool connection.
#[allow(dead_code, missing_debug_implementations)]
pub struct Manager {
    /// Pool session.
    session: Pool,
}

impl Manager {
    /// Create a new pool of connections.
    ///
    /// # Examples
    /// ```rust
    /// use signaly_db::kafka::Manager as KafkaManger;
    ///
    /// let session = KafkaManger::new(
    ///     vec!["localhost:9092".to_string()]
    /// );
    /// // Do what ever you want with your cool new session...
    /// ```
    pub async fn new(urls: Vec<String>) -> Result<Self, Error> {
        Ok(Manager {
            session: Pool::builder(KafkaConnectionManager::new(urls))
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
}