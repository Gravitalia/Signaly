use deadpool::managed;
use lapin::{Connection, ConnectionProperties, ConnectionState, Error};

#[allow(missing_debug_implementations)]
pub struct LapinConnectionManager {
    host: String,
    properties: ConnectionProperties,
}

impl LapinConnectionManager {
    /// Creates a new [`LapinConnectionManager`].
    ///
    /// See [`lapin::Connection`] for a description of the parameter
    /// types.
    pub fn new(
        host: String,
        properties: ConnectionProperties,
    ) -> LapinConnectionManager {
        LapinConnectionManager { host, properties }
    }
}

impl managed::Manager for LapinConnectionManager {
    type Type = Connection;
    type Error = Error;

    async fn create(&self) -> Result<Self::Type, Self::Error> {
        Connection::connect(&self.host, self.properties.clone()).await
    }

    async fn recycle(
        &self,
        conn: &mut Self::Type,
        _: &managed::Metrics,
    ) -> managed::RecycleResult<Error> {
        let valid_states = [
            ConnectionState::Initial,
            ConnectionState::Connecting,
            ConnectionState::Connected,
        ];

        if valid_states.contains(&conn.status().state()) {
            Ok(())
        } else {
            Err(managed::RecycleError::message("Invalid connection"))
        }
    }
}
