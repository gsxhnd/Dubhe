use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    pub enable: bool,
    pub listener_addr: String,
    pub web: WebConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebConfig {
    pub enable: bool,
}
