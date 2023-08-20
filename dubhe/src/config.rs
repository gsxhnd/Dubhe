use dubhe_api::ApiConfig;
use dubhe_mqtt::MqttConfig;
use dubhe_raft::RaftConfig;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub log: LogConfig,
    pub mqtt: MqttConfig,
    pub peer: RaftConfig,
    pub api: ApiConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogConfig {
    level: String,
    dir: String,
    file: String,
}
