//! Client configuration.

use std::time::Duration;

use bytes::Bytes;

/// MQTT protocol version to use for the connection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProtocolVersion {
    /// MQTT v3.1.1 (protocol level 4).
    #[default]
    V4,
    /// MQTT v5.0 (protocol level 5).
    V5,
}

/// Authentication credentials.
#[derive(Debug, Clone)]
pub struct Credentials {
    pub username: String,
    pub password: Option<Bytes>,
}

/// Last Will and Testament configuration.
#[derive(Debug, Clone)]
pub struct LastWill {
    pub topic: String,
    pub message: Bytes,
    pub qos: u8,
    pub retain: bool,
}

/// Configuration for the MQTT client.
#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// Broker address in the form `host:port`.
    pub broker_addr: String,
    /// Client identifier. If empty, the broker may assign one (v5) or reject (v4).
    pub client_id: String,
    /// Protocol version to use.
    pub protocol_version: ProtocolVersion,
    /// Keep-alive interval in seconds.
    pub keep_alive: u16,
    /// Whether to start a clean session/start.
    pub clean_session: bool,
    /// Optional credentials.
    pub credentials: Option<Credentials>,
    /// Optional Last Will and Testament.
    pub last_will: Option<LastWill>,
    /// Connection timeout.
    pub connect_timeout: Duration,
    /// Maximum number of in-flight QoS 1/2 messages.
    pub max_inflight: u16,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            broker_addr: "127.0.0.1:1883".to_string(),
            client_id: String::new(),
            protocol_version: ProtocolVersion::default(),
            keep_alive: 60,
            clean_session: true,
            credentials: None,
            last_will: None,
            connect_timeout: Duration::from_secs(5),
            max_inflight: 65535,
        }
    }
}

impl ClientConfig {
    /// Create a new configuration with the given broker address and client ID.
    pub fn new(broker_addr: impl Into<String>, client_id: impl Into<String>) -> Self {
        Self {
            broker_addr: broker_addr.into(),
            client_id: client_id.into(),
            ..Default::default()
        }
    }

    /// Set the protocol version.
    pub fn protocol_version(mut self, version: ProtocolVersion) -> Self {
        self.protocol_version = version;
        self
    }

    /// Set the keep-alive interval in seconds.
    pub fn keep_alive(mut self, seconds: u16) -> Self {
        self.keep_alive = seconds;
        self
    }

    /// Set clean session flag.
    pub fn clean_session(mut self, clean: bool) -> Self {
        self.clean_session = clean;
        self
    }

    /// Set authentication credentials.
    pub fn credentials(mut self, username: impl Into<String>, password: Option<Bytes>) -> Self {
        self.credentials = Some(Credentials {
            username: username.into(),
            password,
        });
        self
    }

    /// Set the Last Will and Testament.
    pub fn last_will(mut self, will: LastWill) -> Self {
        self.last_will = Some(will);
        self
    }

    /// Set the connection timeout.
    pub fn connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = timeout;
        self
    }
}
