#![forbid(unsafe_code)]
#![deny(
    dead_code,
    unused_imports,
    unused_mut,
    missing_docs,
    missing_debug_implementations
)]
//! Quickly manage and exchange databases on Signaly.
//!
//! Supported databases:
//! - Apache Cassandra;
//! - ScyllaDB;
//! - InfluxDB.
//!
//! Supported brokers:
//! - Apache Kafka;
//! - Redpanda;
//! - RabbitMQ.

#[cfg(feature = "cassandra")]
pub mod cassandra;
#[cfg(feature = "timeseries")]
pub mod influxdb;
#[cfg(feature = "apache_kafka")]
pub mod kafka;
#[cfg(feature = "rabbitmq")]
pub mod rabbitmq;
