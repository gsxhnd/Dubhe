//! Async MQTT client library supporting v3.1.1 and v5.0 protocols.
//!
//! This crate provides a high-level async MQTT client built on top of `mqtt_codec`.
//! It handles connection management, automatic reconnection, subscription tracking,
//! and QoS message delivery.
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
mod transport;

pub use client::MqttClient;
pub use config::{ClientConfig, Credentials, LastWill, ProtocolVersion};
pub use error::ClientError;
pub use event::Event;
