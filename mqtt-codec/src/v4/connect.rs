use crate::types::ProtocolVersion;

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct ConnectPacket {
    // Variable Header
    pub protocol_name: String,
    pub protocol_version: ProtocolVersion,
    pub clean_start: bool,
    pub keep_alive: u16,

    // Payload
    pub client_id: String,
    pub user_name: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct ConnectPacketPayload {
    client_identifier: String,
    // will: Option<(TopicName, VarBytes)>,
    user_name: Option<String>,
    password: Option<String>,
}
