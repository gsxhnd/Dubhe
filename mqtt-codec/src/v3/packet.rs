#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Packet {
    Connect(ConnectPacket),
    ConnAck(ConnAck),
    // Publish(Publish, Option<PublishProperties>),
    // PubAck(PubAck, Option<PubAckProperties>),
    PingReq(PingReq),
    PingResp(PingResp),
    // Subscribe(Subscribe, Option<SubscribeProperties>),
    // SubAck(SubAck, Option<SubAckProperties>),
    // PubRec(PubRec, Option<PubRecProperties>),
    // PubRel(PubRel, Option<PubRelProperties>),
    // PubComp(PubComp, Option<PubCompProperties>),
    // Unsubscribe(Unsubscribe, Option<UnsubscribeProperties>),
    // UnsubAck(UnsubAck, Option<UnsubAckProperties>),
    // Disconnect(Disconnect, Option<DisconnectProperties>),
}

// Control Packets
#[derive(Debug, PartialEq, Clone, Eq)]
pub struct ConnectPacket {
    // Variable Header
    pub protocol_name: String,
    // pub protocol_version: ProtocolVersion,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnAck {
    pub session_present: bool,
    pub code: ConnectAckCode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectAckCode {
    Success,
    RefusedProtocolVersion,
    // BadClientId,
    // ServiceUnavailable,
    // UnspecifiedError,
    // MalformedPacket,
    // ProtocolError,
    // ImplementationSpecificError,
    // UnsupportedProtocolVersion,
    ClientIdentifierNotValid,
    BadUserNamePassword,
    NotAuthorized,
    ServerUnavailable,
    // ServerBusy,
    // Banned,
    // BadAuthenticationMethod,
    // TopicNameInvalid,
    // PacketTooLarge,
    // QuotaExceeded,
    // PayloadFormatInvalid,
    // RetainNotSupported,
    // QoSNotSupported,
    // UseAnotherServer,
    // ServerMoved,
    // ConnectionRateExceeded,
}
impl ConnectAckCode {
    pub fn to_u8(&self) -> u8 {
        match *self {
            ConnectAckCode::Success => 0,
            ConnectAckCode::RefusedProtocolVersion => 1,
            ConnectAckCode::ClientIdentifierNotValid => 2,
            ConnectAckCode::ServerUnavailable => 3,
            ConnectAckCode::BadUserNamePassword => 4,
            ConnectAckCode::NotAuthorized => 5,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct PingReq {}

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct PingResp {}
