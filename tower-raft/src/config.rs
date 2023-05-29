use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaftConfig {
    pub listener_addr: String,
    #[serde(default)]
    pub worker: u8,
    #[serde(default)]
    pub nodes: Vec<String>,
}
