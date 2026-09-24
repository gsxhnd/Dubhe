//! Async MQTT client library supporting v3.1.1 and v5.0 protocols.
//!
//! Built on `mqtt_codec`, with TCP/TLS transport, automatic reconnect,
//! QoS 1/2 inflight tracking, and a CLI binary (`mqtt`).
//!
//! ```text
//! cargo run -p mqtt_client -- pub -t test/topic -m hello
//! cargo run -p mqtt_client -- --protocol v5 --tls sub -H broker.emqx.io -p 8883 -t test/topic
//! ```
//!
//! # Architecture
//!
//! - [`MqttClient`] — cloneable handle; operations enqueue to a background event loop
//! - [`ClientConfig`] — broker, TLS, reconnect, inflight, and session options
//! - [`Event`] — connected / messages / acks / reconnect / disconnect

mod client;
mod config;
mod error;
mod event;
mod inflight;
mod session;
mod transport;

pub use client::MqttClient;
pub use config::{
    ClientConfig, Credentials, LastWill, ProtocolVersion, ReconnectOptions, TlsOptions,
};
pub use error::ClientError;
pub use event::Event;
