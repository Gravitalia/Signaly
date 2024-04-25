use deadpool::managed;
use influxdb::{Client, Error};

#[derive(Debug)]
pub struct InfluxConnectionManager {
    urls: Vec<String>,
    database: String,
}

impl InfluxConnectionManager {
    /// Creates a new [`InfluxConnectionManager`].
    ///
    /// See [`influxdb::Client`] for a description of the parameter
    /// types.
    pub fn new(host: String, database: String) -> InfluxConnectionManager {
        InfluxConnectionManager {
            urls: vec![host],
            database,
        }
    }
}

impl managed::Manager for InfluxConnectionManager {
    type Type = Client;
    type Error = Error;

    async fn create(&self) -> Result<Self::Type, Self::Error> {
        Ok(Client::new(&self.urls[0], &self.database))
    }

    async fn recycle(
        &self,
        conn: &mut Self::Type,
        _: &managed::Metrics,
    ) -> managed::RecycleResult<Error> {
        match conn.ping().await {
            Ok(_) => Ok(()),
            Err(error) => {
                tracing::error!(target: "signaly-db.influxdb", "Connection could not be recycled: failed to ping");
                Err(managed::RecycleError::message(error.to_string()))
            },
        }
    }
}
