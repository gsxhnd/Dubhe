use crate::types::ProtocolVersion;

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct ConnectPacket {
    // Variable Header
    pub protocol_name: String,
    pub protocol_version: ProtocolVersion,
    pub clean_start: bool,
    pub keep_alive: u16,

    // Properties
    // pub session_expiry_interval: Option<SessionExpiryInterval>,
    // pub receive_maximum: Option<ReceiveMaximum>,
    // pub maximum_packet_size: Option<MaximumPacketSize>,
    // pub topic_alias_maximum: Option<TopicAliasMaximum>,
    // pub request_response_information: Option<RequestResponseInformation>,
    // pub request_problem_information: Option<RequestProblemInformation>,
    // pub user_properties: Vec<UserProperty>,
    // pub authentication_method: Option<AuthenticationMethod>,
    // pub authentication_data: Option<AuthenticationData>,

    // Payload
    pub client_id: String,
    // pub will: Option<FinalWill>,
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
