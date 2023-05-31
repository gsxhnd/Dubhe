use serde::{Deserialize, Serialize};
use tower_mqtt::MqttConfig;
use tower_raft::RaftConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub log: LogConfig,
    pub mqtt: MqttConfig,
    pub peer: RaftConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogConfig {
    level: String,
    dir: String,
    file: String,
}
