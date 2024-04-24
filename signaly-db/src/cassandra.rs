//! Apache Cassandra and ScyllaDB pool connection handler.

use scylla::{
    frame::Compression, transport::session::PoolSize, Session, SessionBuilder,
};
use std::num::NonZeroUsize;

/// Manage Apache Cassandra or Scylla pool connection.
#[derive(Debug)]
#[allow(dead_code)]
pub struct Manager {
    /// Pool session.
    connection: Session,
}

impl Manager {
    /// Create a new pool of connections.
    ///
    /// # Examples
    /// ```rust
    /// use signaly_db::cassandra::Manager as ScyllaManager;
    ///
    /// let session = ScyllaManager::new(
    ///     vec!["127.0.0.1:9042".to_string()],
    ///     Some("cassandra".to_string()),
    ///     Some("cassandra".to_string()),
    ///     10,
    /// );
    ///
    /// // Do what ever you want with your cool new session...
    /// ```
    pub async fn new(
        hosts: Vec<String>,
        username: Option<String>,
        password: Option<String>,
        pool_size: usize,
    ) -> Self {
        let session = SessionBuilder::new()
            .known_nodes(hosts)
            .user(username.unwrap_or_default(), password.unwrap_or_default())
            .use_keyspace("accounts", true)
            .pool_size(PoolSize::PerHost(NonZeroUsize::new(pool_size).unwrap()))
            .compression(Some(Compression::Lz4))
            // Activate (true) if the application becomes bigger.
            // It should reduce latency if false, and increase write/read throughput if true.
            .write_coalescing(false)
            .build()
            .await
            .unwrap();

        Manager {
            connection: session,
        }
    }
}
