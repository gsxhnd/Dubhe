use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MqttConfig {
    pub listener: MqttListenerConfig,
    pub workers: u8,
    pub max_connections: u32,
    pub max_clientid_len: u32,
    pub max_qos_allowed: u32,
    pub shared_subscription: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MqttListenerConfig {
    pub tcp: MqttTcpConfig,
    pub tls: MqttTlsConfig,
    pub ws: MqttWebsocketConfig,
    pub wss: MqttWebsocketTlsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MqttTcpConfig {
    pub enable: bool,
    pub addr: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MqttTlsConfig {
    pub enable: bool,
    pub addr: String,
    pub cert: String,
    pub key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MqttWebsocketConfig {
    pub enable: bool,
    pub addr: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MqttWebsocketTlsConfig {
    pub enable: bool,
    pub addr: String,
    pub cert: String,
    pub key: String,
}
