use crate::types::{DecodeError, EncodeError};
use bytes::{Bytes, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

use crate::v5::decoder;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Packet {
    Connect(ConnectPacket),
    ConnAck(ConnAck),
    // Publish(Publish, Option<PublishProperties>),
    // PubAck(PubAck, Option<PubAckProperties>),
    // PingReq(PingReq),
    // PingResp(PingResp),
    // Subscribe(Subscribe, Option<SubscribeProperties>),
    // SubAck(SubAck, Option<SubAckProperties>),
    // PubRec(PubRec, Option<PubRecProperties>),
    // PubRel(PubRel, Option<PubRelProperties>),
    // PubComp(PubComp, Option<PubCompProperties>),
    // Unsubscribe(Unsubscribe, Option<UnsubscribeProperties>),
    // UnsubAck(UnsubAck, Option<UnsubAckProperties>),
    // Disconnect(Disconnect, Option<DisconnectProperties>),
}

// Acknowledgement to QoS1 publish
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PubAck {
    pub pkid: u16,
    pub reason: PubAckReason,
}

/// Return code in puback
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PubAckReason {
    Success,
    NoMatchingSubscribers,
    UnspecifiedError,
    ImplementationSpecificError,
    NotAuthorized,
    TopicNameInvalid,
    PacketIdentifierInUse,
    QuotaExceeded,
    PayloadFormatInvalid,
}

/// Connection packet initiated by the client
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Connect {
    /// Mqtt keep alive time
    pub keep_alive: u16,
    /// Client Id
    pub client_id: String,
    /// Clean session. Asks the broker to clear previous state
    pub clean_session: bool,
}

/// Quality of service
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd)]
#[allow(clippy::enum_variant_names)]
pub enum QoS {
    #[default]
    AtMostOnce = 0,
    AtLeastOnce = 1,
    ExactlyOnce = 2,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LastWill {
    pub topic: Bytes,
    pub message: Bytes,
    pub qos: QoS,
    pub retain: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LastWillProperties {
    pub delay_interval: Option<u32>,
    pub payload_format_indicator: Option<u8>,
    pub message_expiry_interval: Option<u32>,
    pub content_type: Option<String>,
    pub response_topic: Option<String>,
    pub correlation_data: Option<Bytes>,
    pub user_properties: Vec<(String, String)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectProperties {
    /// Expiry interval property after loosing connection
    pub session_expiry_interval: Option<u32>,
    /// Maximum simultaneous packets
    pub receive_maximum: Option<u16>,
    /// Maximum packet size
    pub max_packet_size: Option<u32>,
    /// Maximum mapping integer for a topic
    pub topic_alias_max: Option<u16>,
    pub request_response_info: Option<u8>,
    pub request_problem_info: Option<u8>,
    /// List of user properties
    pub user_properties: Vec<(String, String)>,
    /// Method of authentication
    pub authentication_method: Option<String>,
    /// Authentication data
    pub authentication_data: Option<Bytes>,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnAck {
    pub session_present: bool,
    pub code: ConnectAckCode,
}
impl ConnAck {
    fn to_buffer(&self, buf: &mut [u8], offset: &mut usize) -> Result<(), EncodeError> {
        println!("start conn ack to buffer: {}", buf[*offset..].len());
        // check_remaining(buf, offset, 4)?;
        // if buf[*offset..].len() < 4 {
        //     return Err(EncodeError::BadTransport);
        // }

        let header: u8 = 0b00100000;
        let length: u8 = 2;
        let mut flags: u8 = 0b00000000;
        if self.session_present {
            flags |= 0b1;
        };
        let rc = self.code.to_u8();
        write_u8(buf, offset, header)?;
        write_u8(buf, offset, length)?;
        write_u8(buf, offset, flags)?;
        write_u8(buf, offset, rc)?;
        println!("{:?}", buf);
        Ok(())
    }
}
fn write_u8(buf: &mut [u8], offset: &mut usize, val: u8) -> Result<(), EncodeError> {
    buf[*offset] = val;
    *offset += 1;
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnAckProperties {
    pub session_expiry_interval: Option<u32>,
    pub receive_max: Option<u16>,
    pub max_qos: Option<u8>,
    pub retain_available: Option<u8>,
    pub max_packet_size: Option<u32>,
    pub assigned_client_identifier: Option<String>,
    pub topic_alias_max: Option<u16>,
    pub reason_string: Option<String>,
    pub user_properties: Vec<(String, String)>,
    pub wildcard_subscription_available: Option<u8>,
    pub subscription_identifiers_available: Option<u8>,
    pub shared_subscription_available: Option<u8>,
    pub server_keep_alive: Option<u16>,
    pub response_information: Option<String>,
    pub server_reference: Option<String>,
    pub authentication_method: Option<String>,
    pub authentication_data: Option<Bytes>,
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

pub struct Codec {}

impl Codec {
    pub fn new() -> Self {
        Codec {}
    }
}

impl Decoder for Codec {
    type Error = DecodeError;
    type Item = Packet;
    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        // TODO - Ideally we should keep a state machine to store the data we've read so far.
        // let packet = decoder::decode_mqtt(buf);
        todo!()
    }
}

impl Encoder<Packet> for Codec {
    type Error = EncodeError;
    fn encode(&mut self, packet: Packet, bytes: &mut BytesMut) -> Result<(), Self::Error> {
        // self.encode(packet, bytes)
        // todo!()
        println!("{:?}", bytes);
        // bytes.resize(2048, b'0');
        match packet {
            Packet::ConnAck(conn) => conn.to_buffer(bytes, &mut 0),
            Packet::Connect(conn) => {
                todo!()
            }
        }
    }
}
