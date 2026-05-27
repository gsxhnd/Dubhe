//! MQTT CLI — publish and subscribe against a broker.

use std::time::Duration;

use anyhow::{Context, Result};
use bytes::Bytes;
use clap::{Parser, Subcommand};
use mqtt_client::{ClientConfig, Event, MqttClient};
use tokio::sync::mpsc;

#[derive(Parser)]
#[command(name = "mqtt", about = "MQTT CLI client (v3.1.1 and v5.0)", version)]
struct Cli {
    /// Protocol version: v4 / 3.1.1 or v5 / 5.0.
    #[arg(
        long = "protocol",
        alias = "mqtt-version",
        global = true,
        default_value = "v4",
        value_parser = clap::value_parser!(mqtt_client::ProtocolVersion)
    )]
    protocol: mqtt_client::ProtocolVersion,

    /// Broker hostname.
    #[arg(short = 'H', long, global = true, default_value = "127.0.0.1")]
    host: String,

    /// Broker port.
    #[arg(short, long, global = true, default_value_t = 1883)]
    port: u16,

    /// Client identifier (auto-generated if empty).
    #[arg(short, long, global = true, default_value = "")]
    client_id: String,

    /// Username for authentication.
    #[arg(short, long, global = true)]
    username: Option<String>,

    /// Password for authentication.
    #[arg(short = 'P', long, global = true)]
    password: Option<String>,

    /// Keep-alive interval in seconds.
    #[arg(long, global = true, default_value_t = 60)]
    keep_alive: u16,

    /// Start a clean session.
    #[arg(long, global = true, default_value_t = true)]
    clean: bool,

    /// Connection timeout in seconds.
    #[arg(long, global = true, default_value_t = 5)]
    timeout: u64,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Publish a message to a topic.
    Pub {
        /// Topic name.
        #[arg(short, long)]
        topic: String,

        /// Message payload.
        #[arg(short, long)]
        message: String,

        /// QoS level (0, 1, or 2).
        #[arg(short, long, default_value_t = 0)]
        qos: u8,

        /// Retain flag.
        #[arg(short, long, default_value_t = false)]
        retain: bool,
    },
    /// Subscribe to a topic and print incoming messages.
    Sub {
        /// Topic filter.
        #[arg(short, long)]
        topic: String,

        /// Requested QoS (0, 1, or 2).
        #[arg(short, long, default_value_t = 0)]
        qos: u8,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = build_config(&cli);

    match cli.command {
        Commands::Pub {
            topic,
            message,
            qos,
            retain,
        } => run_publish(config, &topic, &message, qos, retain).await,
        Commands::Sub { topic, qos } => run_subscribe(config, &topic, qos).await,
    }
}

fn build_config(cli: &Cli) -> ClientConfig {
    let mut config = ClientConfig::new(
        format!("{}:{}", cli.host, cli.port),
        cli.client_id.clone(),
    )
    .keep_alive(cli.keep_alive)
    .clean_session(cli.clean)
    .connect_timeout(Duration::from_secs(cli.timeout))
    .protocol_version(cli.protocol);

    if let Some(username) = &cli.username {
        let password = cli.password.clone().map(Bytes::from);
        config = config.credentials(username.clone(), password);
    }

    config
}

async fn run_publish(
    config: ClientConfig,
    topic: &str,
    message: &str,
    qos: u8,
    retain: bool,
) -> Result<()> {
    let (client, mut events) = MqttClient::new(config);
    wait_connected(&mut events).await?;

    client
        .publish(topic, Bytes::from(message.to_string()), qos, retain)
        .await
        .context("failed to enqueue publish")?;

    if qos == 0 {
        tokio::time::sleep(Duration::from_millis(100)).await;
    } else if qos == 1 {
        wait_pub_ack(&mut events).await?;
    } else {
        wait_pub_comp(&mut events).await?;
    }

    client.disconnect().await.ok();
    Ok(())
}

async fn run_subscribe(config: ClientConfig, topic: &str, qos: u8) -> Result<()> {
    let (client, mut events) = MqttClient::new(config);
    wait_connected(&mut events).await?;

    client
        .subscribe(vec![(topic.to_string(), qos)])
        .await
        .context("failed to enqueue subscribe")?;

    wait_sub_ack(&mut events).await?;

    eprintln!("Subscribed to {topic} (Ctrl+C to exit)");

    let ctrl_c = tokio::signal::ctrl_c();
    tokio::pin!(ctrl_c);

    loop {
        tokio::select! {
            event = events.recv() => {
                match event {
                    Some(Event::Message { topic, payload, .. }) => {
                        let body = String::from_utf8_lossy(&payload);
                        println!("{topic} {body}");
                    }
                    Some(Event::Disconnected { reason }) => {
                        anyhow::bail!(
                            "disconnected: {}",
                            reason.unwrap_or_else(|| "unknown".into())
                        );
                    }
                    None => anyhow::bail!("event channel closed"),
                    _ => {}
                }
            }
            _ = &mut ctrl_c => {
                client.disconnect().await.ok();
                break;
            }
        }
    }

    Ok(())
}

async fn wait_connected(events: &mut mpsc::Receiver<Event>) -> Result<()> {
    loop {
        match events.recv().await {
            Some(Event::Connected) => return Ok(()),
            Some(Event::Disconnected { reason }) => {
                anyhow::bail!(
                    "connection failed: {}",
                    reason.unwrap_or_else(|| "unknown".into())
                );
            }
            None => anyhow::bail!("event channel closed before connect"),
            _ => {}
        }
    }
}

async fn wait_pub_ack(events: &mut mpsc::Receiver<Event>) -> Result<()> {
    wait_for_event(events, |e| matches!(e, Event::PubAck { .. }), "PUBACK").await
}

async fn wait_pub_comp(events: &mut mpsc::Receiver<Event>) -> Result<()> {
    wait_for_event(events, |e| matches!(e, Event::PubComp { .. }), "PUBCOMP").await
}

async fn wait_sub_ack(events: &mut mpsc::Receiver<Event>) -> Result<()> {
    wait_for_event(events, |e| matches!(e, Event::SubAck { .. }), "SUBACK").await
}

async fn wait_for_event<F>(events: &mut mpsc::Receiver<Event>, pred: F, label: &str) -> Result<()>
where
    F: Fn(&Event) -> bool,
{
    let deadline = tokio::time::Instant::now() + Duration::from_secs(10);
    loop {
        let remaining = deadline.saturating_duration_since(tokio::time::Instant::now());
        if remaining.is_zero() {
            anyhow::bail!("timed out waiting for {label}");
        }
        match tokio::time::timeout(remaining, events.recv()).await {
            Ok(Some(event)) if pred(&event) => return Ok(()),
            Ok(Some(Event::Disconnected { reason })) => {
                anyhow::bail!(
                    "disconnected while waiting for {label}: {}",
                    reason.unwrap_or_else(|| "unknown".into())
                );
            }
            Ok(Some(_)) => continue,
            Ok(None) => anyhow::bail!("event channel closed while waiting for {label}"),
            Err(_) => anyhow::bail!("timed out waiting for {label}"),
        }
    }
}
