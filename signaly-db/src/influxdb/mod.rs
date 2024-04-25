//! InfluxDB pool connection handler.

mod pool;

use pool::InfluxConnectionManager;
use signaly_error::{DatabaseError::PoolCreation, Error, ErrorType};

type Pool = deadpool::managed::Pool<InfluxConnectionManager>;

/// Manage InfluxDB pool connection.
#[derive(Debug)]
#[allow(dead_code)]
pub struct Manager {
    /// Pool session.
    session: Pool,
}

impl Manager {
    /// Create a new pool of connections.
    ///
    /// # Examples
    /// ```rust
    /// use signaly_db::influxdb::Manager as InfluxManager;
    ///
    /// let session = InfluxManager::new(
    ///     "http://127.0.0.1:8086".to_string(),
    ///     "signaly".to_string(),
    /// );
    /// // Do what ever you want with your cool new session...
    /// ```
    pub async fn new(host: String, database: String) -> Result<Self, Error> {
        Ok(Manager {
            session: Pool::builder(InfluxConnectionManager::new(
                host, database,
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
