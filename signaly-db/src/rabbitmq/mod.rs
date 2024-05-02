//! RabbitMQ message broker.

mod pool;

use lapin::{
    options::BasicPublishOptions, BasicProperties, ConnectionProperties,
};
use pool::LapinConnectionManager;
use signaly_error::{
    DatabaseError::{MessageNotSent, PoolCreation, PoolObtention},
    Error, ErrorType,
};

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

    /// Send a message via RabbitMQ.
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
                        "while trying to send a message via RabbitMQ"
                            .to_string(),
                    ),
                )
            })?
            .create_channel()
            .await
            .map_err(|error| {
                Error::new(
                    ErrorType::Unspecified,
                    Some(Box::new(error)),
                    Some("cannot create RabbitMQ channel".to_string()),
                )
            })?
            .basic_publish(
                "",
                topic.as_str(),
                BasicPublishOptions::default(),
                content.as_bytes(),
                BasicProperties::default(),
            )
            .await
            .map_err(|error| {
                Error::new(
                    ErrorType::Database(MessageNotSent),
                    Some(Box::new(error)),
                    None,
                )
            })?;

        Ok(())
    }
}
