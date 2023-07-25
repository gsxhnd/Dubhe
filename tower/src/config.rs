use serde::{Deserialize, Serialize};
use tower_mqtt::MqttConfig;
use tower_raft::RaftConfig;

use crate::api::ApiConfig;

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
