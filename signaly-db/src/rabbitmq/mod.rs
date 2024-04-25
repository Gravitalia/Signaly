//! RabbitMQ message broker.

mod pool;

use lapin::ConnectionProperties;
use pool::LapinConnectionManager;
use signaly_error::{DatabaseError::PoolCreation, Error, ErrorType};

type Pool = deadpool::managed::Pool<LapinConnectionManager>;

/// Manage RabbitMQ pool connection.
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
    /// use signaly_db::rabbitmq::Manager as LapinManger;
    ///
    /// let session = LapinManger::new(
    ///     "amqp://127.0.0.1:5672/%2f".to_string()
    /// );
    /// // Do what ever you want with your cool new session...
    /// ```
    pub async fn new(host: String) -> Result<Self, Error> {
        Ok(Manager {
            session: Pool::builder(LapinConnectionManager::new(
                host,
                ConnectionProperties::default(),
            ))
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
