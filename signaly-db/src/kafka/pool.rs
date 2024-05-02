use deadpool::managed;
use kafka::{
    producer::{Compression, Producer, Record, RequiredAcks},
    Error,
};
use std::{fmt::Write, time::Duration};

#[derive(Debug)]
pub struct KafkaConnectionManager {
    urls: Vec<String>,
}

impl KafkaConnectionManager {
    /// Creates a new [`KafkaConnectionManager`].
    ///
    /// See [`kafka::producer::Producer`] and [`kafka::consumer::Consumer`]
    /// for a description of the parameter types.
    pub fn new(urls: Vec<String>) -> KafkaConnectionManager {
        KafkaConnectionManager { urls }
    }
}

impl managed::Manager for KafkaConnectionManager {
    type Type = Producer;
    type Error = Error;

    async fn create(&self) -> Result<Self::Type, Self::Error> {
        Producer::from_hosts(self.urls.clone())
            .with_ack_timeout(Duration::from_secs(1))
            .with_required_acks(RequiredAcks::One)
            .with_compression(Compression::GZIP)
            .create()
    }

    async fn recycle(
        &self,
        conn: &mut Self::Type,
        _: &managed::Metrics,
    ) -> managed::RecycleResult<Error> {
        let mut buf = String::with_capacity(1);
        let _ = write!(&mut buf, "{}", 0);

        match conn.send(&Record::from_value("test", buf)) {
            Ok(_) => Ok(()),
            Err(error) => {
                tracing::error!(target: "signaly-db.kafka", "Connection could not be recycled: failed to send message to \"test\" topic.");
                Err(managed::RecycleError::message(error.to_string()))
            },
        }
    }
}
