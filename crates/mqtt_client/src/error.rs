//! Client error types.

use thiserror::Error;

/// Errors that can occur during MQTT client operations.
#[derive(Debug, Error)]
pub enum ClientError {
    /// Network I/O error.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Codec encode/decode error.
    #[error("codec error: {0}")]
    Codec(#[from] mqtt_codec::MqttError),

    /// Connection was refused by the broker.
    #[error("connection refused: {reason}")]
    ConnectionRefused { reason: String },

    /// Operation timed out.
    #[error("operation timed out")]
    Timeout,

    /// Client has been disconnected.
    #[error("client disconnected")]
    Disconnected,

    /// The internal channel was closed unexpectedly.
    #[error("internal channel closed")]
    ChannelClosed,
}
