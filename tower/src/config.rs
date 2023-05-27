use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub log: LogConfig,
    pub mqtt: MqttConfig,
    pub peer: PeerConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogConfig {
    level: String,
    dir: String,
    file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MqttConfig {
    pub listener: MqttListener,
    pub workers: u8,
    pub max_connections: u32,
    pub max_clientid_len: u32,
    pub max_qos_allowed: u32,
    pub shared_subscription: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MqttListener {
    pub tcp: MqttListenerTCP,
    pub tls: MqttListenerTLS,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MqttListenerTCP {
    addr: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MqttListenerTLS {
    addr: String,
    cert: String,
    key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MqttListenerWS {
    addr: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MqttListenerWSS {
    addr: String,
    cert: String,
    key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerConfig {
    pub server_addr: String,
    #[serde(default)]
    pub worker: u8,
    #[serde(default)]
    pub nodes: Vec<String>,
}
