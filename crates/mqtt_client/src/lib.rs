//! Async MQTT client library supporting v3.1.1 and v5.0 protocols.
//!
//! This crate provides a high-level async MQTT client built on top of `mqtt_codec`,
//! plus a CLI binary (`mqtt`) for publish/subscribe.
//!
//! Set [`ClientConfig::protocol_version`] or pass `--protocol v5` on the CLI.
//!
//! ```text
//! cargo run -p mqtt_client -- pub -t test/topic -m hello
//! cargo run -p mqtt_client -- --protocol v5 sub -H broker.emqx.io -t test/topic
//! ```
//!
//! # Architecture
//!
//! - [`MqttClient`] — The main client handle (cheaply cloneable, send across tasks).
//! - [`ClientConfig`] — Connection and behavior configuration.
//! - [`EventLoop`] — Internal event loop driving network I/O (spawned as a tokio task).
//! - [`Event`] — Events emitted by the client (incoming messages, acks, errors).

mod client;
mod config;
mod error;
mod event;
mod session;
mod transport;

pub use client::MqttClient;
pub use config::{ClientConfig, Credentials, LastWill, ProtocolVersion};
pub use error::ClientError;
pub use event::Event;
