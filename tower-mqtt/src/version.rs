use bytes::BytesMut;
use mqttrs::{decode_slice, Packet, Protocol};
use std::error::Error;
use tokio_util::codec::{Decoder, Encoder};

use crate::decoder;
use crate::types::QoS;
use crate::types::{DecodeError, EncodeError};

#[derive(Clone, Debug)]
pub enum ProtocolVersion {
    MQTT3,
    MQTT5,
    // MQISDP,
}

#[derive(Clone, Debug)]
pub enum VersionPacket {
    Connect(ConnectPacket),
}

#[derive(Debug, Clone)]
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

pub struct VersionCodec;

impl Decoder for VersionCodec {
    type Item = ProtocolVersion;
    type Error = DecodeError;

    fn decode(&mut self, buf: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let mut offset = 0;
        let mut offset = 0;
        if let Some((header, remaining_len)) = decoder::read_header(buf, &mut offset)? {
            let r = read_packet(header, remaining_len, buf, &mut offset)?;
            Ok(Some(r))
        } else {
            // Don't have a full packet
            Ok(None)
        }
        match decode_slice(buf) {
            Ok(Some(Packet::Connect(p))) => {
                let p = p.protocol;
                if p == Protocol::MQTT311 {
                    return Ok(Some(ProtocolVersion::MQTT3));
                } else {
                    return Err(DecodeError::InvalidProtocolVersion);
                }
            }
            // In real code you probably don't want to panic like that ;)
            Ok(None) => panic!("not enough data"),
            other => panic!("unexpected {:?}", other),
        }
    }
}
