//! MQTT session drivers (v3.1.1 and v5.0).

mod v4;
mod v5;

use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::time;

use crate::client::Command;
use crate::config::{ClientConfig, ProtocolVersion};
use crate::error::ClientError;
use crate::event::Event;
use crate::transport;

/// Drives the MQTT connection until disconnect or fatal error.
pub(crate) async fn run_event_loop(
    config: ClientConfig,
    command_rx: mpsc::Receiver<Command>,
    event_tx: mpsc::Sender<Event>,
) -> Result<(), ClientError> {
    let stream = connect_tcp(&config, &event_tx).await?;

    match config.protocol_version {
        ProtocolVersion::V4 => v4::run(stream, &config, command_rx, event_tx).await,
        ProtocolVersion::V5 => v5::run(stream, &config, command_rx, event_tx).await,
    }
}

async fn connect_tcp(
    config: &ClientConfig,
    event_tx: &mpsc::Sender<Event>,
) -> Result<TcpStream, ClientError> {
    match time::timeout(config.connect_timeout, transport::connect_tcp(&config.broker_addr)).await {
        Ok(Ok(stream)) => Ok(stream),
        Ok(Err(e)) => {
            emit_disconnected(event_tx, Some(e.to_string())).await;
            Err(ClientError::Io(e))
        }
        Err(_) => {
            emit_disconnected(event_tx, Some("connection timed out".to_string())).await;
            Err(ClientError::Timeout)
        }
    }
}

pub(super) fn client_id(config: &ClientConfig) -> String {
    if config.client_id.is_empty() {
        format!("mqtt-{}", std::process::id())
    } else {
        config.client_id.clone()
    }
}

pub(super) fn keep_alive_secs(keep_alive: u16) -> std::time::Duration {
    if keep_alive == 0 {
        std::time::Duration::MAX
    } else {
        std::time::Duration::from_secs(u64::from(keep_alive))
    }
}

pub(super) fn reset_ping_deadline(
    deadline: &mut Option<time::Instant>,
    every: std::time::Duration,
) {
    if every != std::time::Duration::MAX {
        *deadline = Some(time::Instant::now() + every);
    }
}

pub(super) async fn emit_disconnected(event_tx: &mpsc::Sender<Event>, reason: Option<String>) {
    let _ = event_tx.send(Event::Disconnected { reason }).await;
}
