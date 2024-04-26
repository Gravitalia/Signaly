use deadpool::managed;
use kafka::{
    consumer::{Consumer, FetchOffset, GroupOffsetStorage},
    producer::{Compression, Producer, Record, RequiredAcks},
    Error,
};
use std::{fmt::Write, time::Duration};

pub enum Type {
    Producer(Producer),
    Consumer(Consumer),
}

#[derive(Debug)]
pub struct KafkaConnectionManager {
    urls: Vec<String>,
    mode: super::Type,
    topic: Option<String>,
}

impl KafkaConnectionManager {
    /// Creates a new [`KafkaConnectionManager`].
    ///
    /// See [`kafka::producer::Producer`] and [`kafka::consumer::Consumer`]
    /// for a description of the parameter types.
    pub fn new(
        urls: Vec<String>,
        topic: Option<String>,
        mode: super::Type,
    ) -> KafkaConnectionManager {
        KafkaConnectionManager { urls, mode, topic }
    }
}

impl managed::Manager for KafkaConnectionManager {
    type Type = Type;
    type Error = Error;

    async fn create(&self) -> Result<Self::Type, Self::Error> {
        match self.mode {
            super::Type::Producer => Ok(Type::Producer(
                Producer::from_hosts(self.urls.clone())
                    .with_ack_timeout(Duration::from_secs(1))
                    .with_required_acks(RequiredAcks::One)
                    .with_compression(Compression::GZIP)
                    .create()?,
            )),
            super::Type::Consumer => Ok(Type::Consumer(
                Consumer::from_hosts(self.urls.clone())
                    .with_topic(self.topic.clone().unwrap_or("*".to_string()))
                    .with_group(String::default())
                    .with_fallback_offset(FetchOffset::Earliest)
                    .with_offset_storage(Some(GroupOffsetStorage::Kafka))
                    .create()?,
            )),
        }
    }

    async fn recycle(
        &self,
        conn: &mut Self::Type,
        _: &managed::Metrics,
    ) -> managed::RecycleResult<Error> {
        match conn {
            Type::Producer(conn) => {
                let mut buf = String::with_capacity(1);
                let _ = write!(&mut buf, "{}", 0);

                match conn.send(&Record::from_value("test", buf)) {
                    Ok(_) => Ok(()),
                    Err(error) => {
                        tracing::error!(target: "signaly-db.kafka", "Connection could not be recycled: failed to send message to \"test\" topic.");
                        Err(managed::RecycleError::message(error.to_string()))
                    },
                }
            },
            Type::Consumer(_) => Ok(()),
        }
    }
}
