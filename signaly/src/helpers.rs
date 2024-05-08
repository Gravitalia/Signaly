//! utils functions to perform global actions.

use tokio::task;
use tracing::{error, info, trace};

/// Receive messages from Kafka.
#[cfg(feature = "kafka")]
pub fn consume_messages(mut conn: signaly_db::kafka::Consumer) {
    info!("Listening to incoming messages via Kafka.");

    task::spawn(async move {
        loop {
            if let Ok(mss) = conn.poll() {
                for ms in mss.iter() {
                    for message in ms.messages() {
                        trace!(
                            topic = ms.topic(),
                            partition = ms.partition(),
                            offset = message.offset,
                            "Received message."
                        );

                        match String::from_utf8(message.value.to_vec()) {
                            Ok(string) => {
                                if let Ok(_v) = serde_json::from_str::<
                                    crate::models::Event,
                                >(
                                    &string
                                ) {
                                    // todo: process v.
                                }
                            },
                            Err(_) => {
                                error!("Message was NOT encoded with UTF-8.")
                            },
                        }
                    }
                    let _ = conn.consume_messageset(ms);
                }

                let _ = conn.commit_consumed();
            }
        }
    });
}

/// Receive messages from RabbitMQ.
#[cfg(feature = "rabbitmq")]
pub fn consume_messages(conn: signaly_db::rabbitmq::Manager, topic: String) {
    use futures_lite::stream::StreamExt;
    use signaly_db::rabbitmq::{
        BasicAckOptions, BasicConsumeOptions, FieldTable,
    };

    info!("Listening to incoming messages via RabbitMQ.");

    task::spawn(async move {
        let channel = conn
            .session
            .get()
            .await
            .unwrap()
            .create_channel()
            .await
            .unwrap();
        let mut consumer = channel
            .basic_consume(
                &topic,
                "signaly",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .unwrap();

        loop {
            if let Some(delivery) = consumer.next().await {
                trace!(topic = topic, "Received message.");
    
                if let Ok(message) = delivery {
                    if let Ok(_v) = serde_json::from_slice::<crate::models::Event>(
                        &message.data,
                    ) {
                        // todo: process v.
                    }
                    message.ack(BasicAckOptions::default()).await.expect("ack");
                } else {
                    error!("RabbitMQ message cannot be decoded.");
                }
            }
        }
    });
}
