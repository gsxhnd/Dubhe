//! MQTT client handle and event loop.

use bytes::Bytes;
use tokio::sync::mpsc;

use crate::config::ClientConfig;
use crate::error::ClientError;
use crate::event::Event;

/// Commands sent from the client handle to the event loop.
#[derive(Debug)]
enum Command {
    Publish {
        topic: String,
        payload: Bytes,
        qos: u8,
        retain: bool,
    },
    Subscribe {
        topics: Vec<(String, u8)>,
    },
    Unsubscribe {
        topics: Vec<String>,
    },
    Disconnect,
}

/// The MQTT client handle.
///
/// This is cheaply cloneable and can be shared across tasks.
/// All operations are non-blocking and communicate with the internal
/// event loop via channels.
#[derive(Debug, Clone)]
pub struct MqttClient {
    command_tx: mpsc::Sender<Command>,
}

impl MqttClient {
    /// Create a new MQTT client and start the event loop.
    ///
    /// Returns the client handle and a receiver for incoming events.
    /// The event loop runs as a background tokio task.
    pub fn new(config: ClientConfig) -> (Self, mpsc::Receiver<Event>) {
        let (command_tx, command_rx) = mpsc::channel(256);
        let (event_tx, event_rx) = mpsc::channel(256);

        tokio::spawn(async move {
            let _ = event_loop(config, command_rx, event_tx).await;
        });

        let client = Self { command_tx };
        (client, event_rx)
    }

    /// Publish a message to the given topic.
    pub async fn publish(
        &self,
        topic: impl Into<String>,
        payload: impl Into<Bytes>,
        qos: u8,
        retain: bool,
    ) -> Result<(), ClientError> {
        self.command_tx
            .send(Command::Publish {
                topic: topic.into(),
                payload: payload.into(),
                qos,
                retain,
            })
            .await
            .map_err(|_| ClientError::ChannelClosed)
    }

    /// Subscribe to one or more topic filters.
    pub async fn subscribe(
        &self,
        topics: Vec<(String, u8)>,
    ) -> Result<(), ClientError> {
        self.command_tx
            .send(Command::Subscribe { topics })
            .await
            .map_err(|_| ClientError::ChannelClosed)
    }

    /// Unsubscribe from one or more topic filters.
    pub async fn unsubscribe(&self, topics: Vec<String>) -> Result<(), ClientError> {
        self.command_tx
            .send(Command::Unsubscribe { topics })
            .await
            .map_err(|_| ClientError::ChannelClosed)
    }

    /// Gracefully disconnect from the broker.
    pub async fn disconnect(&self) -> Result<(), ClientError> {
        self.command_tx
            .send(Command::Disconnect)
            .await
            .map_err(|_| ClientError::ChannelClosed)
    }
}

/// Internal event loop that drives the MQTT connection.
///
/// Handles:
/// - TCP connection establishment
/// - CONNECT/CONNACK handshake
/// - Sending outgoing packets (PUBLISH, SUBSCRIBE, UNSUBSCRIBE, PINGREQ)
/// - Receiving incoming packets and emitting events
/// - Keep-alive ping management
/// - QoS 1/2 acknowledgment state machines
async fn event_loop(
    _config: ClientConfig,
    mut _command_rx: mpsc::Receiver<Command>,
    _event_tx: mpsc::Sender<Event>,
) -> Result<(), ClientError> {
    // TODO(#1): Implement connection establishment
    // TODO(#2): Implement CONNECT/CONNACK handshake
    // TODO(#3): Implement packet read/write loop
    // TODO(#4): Implement keep-alive ping timer
    // TODO(#5): Implement QoS state machines
    Ok(())
}
