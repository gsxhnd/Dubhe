use serde::{Deserialize, Serialize};
use tower_mqtt::MqttConfig;

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
pub struct PeerConfig {
    pub server_addr: String,
    #[serde(default)]
    pub worker: u8,
    #[serde(default)]
    pub nodes: Vec<String>,
}
