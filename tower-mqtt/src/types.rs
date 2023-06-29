pub const MQTT: &[u8] = b"MQTT";
pub const MQISDP: &[u8] = b"MQIsdp";
pub const MQTT_LEVEL_31: u8 = 3;
pub const MQTT_LEVEL_311: u8 = 4;
pub const MQTT_LEVEL_5: u8 = 5;
pub const WILL_QOS_SHIFT: u8 = 3;

// Max possible packet size
pub const MAX_PACKET_SIZE: u32 = 0xF_FF_FF_FF;

/// Quality of Service
// #[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone)]
pub enum QoS {
    /// At most once delivery
    ///
    /// The message is delivered according to the capabilities of the underlying network.
    /// No response is sent by the receiver and no retry is performed by the sender.
    /// The message arrives at the receiver either once or not at all.
    AtMostOnce = 0,
    /// At least once delivery
    ///
    /// This quality of service ensures that the message arrives at the receiver at least once.
    /// A QoS 1 PUBLISH Packet has a Packet Identifier in its variable header
    /// and is acknowledged by a PUBACK Packet.
    AtLeastOnce = 1,
    /// Exactly once delivery
    ///
    /// This is the highest quality of service,
    /// for use when neither loss nor duplication of messages are acceptable.
    /// There is an increased overhead associated with this quality of service.
    ExactlyOnce = 2,
}
impl QoS {
    pub fn to_u8(&self) -> u8 {
        match *self {
            QoS::AtMostOnce => 0,
            QoS::AtLeastOnce => 1,
            QoS::ExactlyOnce => 2,
        }
    }

    pub fn from_u8(byte: u8) -> Result<QoS, DecodeError> {
        match byte {
            0 => Ok(QoS::AtMostOnce),
            1 => Ok(QoS::AtLeastOnce),
            2 => Ok(QoS::ExactlyOnce),
            _n => Err(DecodeError::InvalidQoS),
        }
    }
}

// pub struct ConnectFlags: u8 {
// }
// const USERNAME: u8 = 0b1000_0000;
// const PASSWORD: u8 = 0b0100_0000;
// const WILL_RETAIN: u8 = 0b0010_0000;
// const WILL_QOS: u8 = 0b0001_1000;
// const WILL: u8 = 0b0000_0100;
// const CLEAN_START: u8 = 0b0000_0010;

// pub struct ConnectAckFlags: u8 {
// }
// const SESSION_PRESENT: u8 = 0b0000_0001;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FixedHeader {
    /// Fixed Header byte
    pub first_byte: u8,
    /// the number of bytes remaining within the current packet,
    /// including data in the variable header and the payload.
    pub remaining_length: u32,
}

#[derive(Debug)]
pub enum DecodeError {
    InvalidPacketType,
    InvalidProtocolVersion,
    InvalidRemainingLength,
    PacketTooLarge,
    InvalidUtf8,
    InvalidQoS,
    InvalidRetainHandling,
    InvalidConnectReason,
    InvalidDisconnectReason,
    InvalidPublishAckReason,
    InvalidPublishReceivedReason,
    InvalidPublishReleaseReason,
    InvalidPublishCompleteReason,
    InvalidSubscribeAckReason,
    InvalidSubscriptionIdentifier,
    InvalidUnsubscribeAckReason,
    InvalidAuthenticateReason,
    InvalidPropertyId,
    InvalidPropertyForPacket,
    // InvalidTopic(TopicParseError),
    // InvalidTopicFilter(TopicParseError),
    Io(std::io::Error),
    BadTransport, // When errors occur on a lower level transport like WS
}
impl From<std::io::Error> for DecodeError {
    fn from(err: std::io::Error) -> Self {
        DecodeError::Io(err)
    }
}

#[derive(Debug)]
pub enum EncodeError {
    BadTransport,
    Io(std::io::Error),
}

impl From<std::io::Error> for EncodeError {
    fn from(err: std::io::Error) -> Self {
        EncodeError::Io(err)
    }
}

#[derive(Debug)]
pub enum ProtocolError {
    _MalformedPacket(DecodeError),
    _ConnectTimedOut,
    _FirstPacketNotConnect,
    _InvalidProtocolName,
    _KeepAliveTimeout,
}

#[derive(Debug, Clone)]
#[repr(u8)]
pub enum PacketType {
    // Connect = 1,
    // ConnectAck = 2,
    // Publish = 3,
    // PublishAck = 4,
    // PublishReceived = 5,
    // PublishRelease = 6,
    // PublishComplete = 7,
    // Subscribe = 8,
    // SubscribeAck = 9,
    // Unsubscribe = 10,
    // UnsubscribeAck = 11,
    // PingRequest = 12,
    // PingResponse = 13,
    // Disconnect = 14,
    // Authenticate = 15,
    CONNECT = 0b0001_0000,
    CONNACK = 0b0010_0000,
    PUBLISH = 0b0011_0000,
    // PUBLISH_END = 0b0011_1111,
    PUBACK = 0b0100_0000,
    PUBREC = 0b0101_0000,
    PUBREL = 0b0110_0010,
    PUBCOMP = 0b0111_0000,
    SUBSCRIBE = 0b1000_0010,
    SUBACK = 0b1001_0000,
    UNSUBSCRIBE = 0b1010_0010,
    UNSUBACK = 0b1011_0000,
    PINGREQ = 0b1100_0000,
    PINGRESP = 0b1101_0000,
    DISCONNECT = 0b1110_0000,
    AUTH = 0b1111_0000,
}
