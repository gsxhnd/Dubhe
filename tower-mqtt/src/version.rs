use bytes::BytesMut;
use mqttrs::{decode_slice, Packet, Protocol};
use std::error::Error;
use tokio_tungstenite::tungstenite::http::Version;
use tokio_util::codec::{Decoder, Encoder};

use crate::types::{DecodeError, EncodeError};

#[derive(Clone, Copy, Debug)]
pub enum ProtocolVersion {
    MQTT3,
    MQTT5,
    // MQISDP,
}

// #[derive(Clone, Debug, PartialEq, Eq)]
// pub enum Packet {
//     Connect(ConnectPacket),
// }

// #[derive(Debug, PartialEq, Clone, Eq)]
// pub struct ConnectPacket {
//     // Variable Header
//     pub protocol_name: String,
//     // pub protocol_version: ProtocolVersion,
//     pub clean_start: bool,
//     pub keep_alive: u16,

//     // Properties
//     // pub session_expiry_interval: Option<SessionExpiryInterval>,
//     // pub receive_maximum: Option<ReceiveMaximum>,
//     // pub maximum_packet_size: Option<MaximumPacketSize>,
//     // pub topic_alias_maximum: Option<TopicAliasMaximum>,
//     // pub request_response_information: Option<RequestResponseInformation>,
//     // pub request_problem_information: Option<RequestProblemInformation>,
//     // pub user_properties: Vec<UserProperty>,
//     // pub authentication_method: Option<AuthenticationMethod>,
//     // pub authentication_data: Option<AuthenticationData>,

//     // Payload
//     pub client_id: String,
//     // pub will: Option<FinalWill>,
//     pub user_name: Option<String>,
//     pub password: Option<String>,
// }

pub struct VersionCodec;

impl Decoder for VersionCodec {
    type Item = ProtocolVersion;
    type Error = DecodeError;

    fn decode(&mut self, buf: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
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
        // Check the first byte to determine the MQTT version
        // if buf.len() >= 1 {
        //     let version_byte = buf[0];
        //     println!("version decode: {}", version_byte);
        //     let version = match version_byte {
        //         3 => ProtocolVersion::MQTT3,
        //         5 => ProtocolVersion::MQTT5,
        //         _ => return Err(DecodeError::InvalidProtocolVersion),
        //     };
        //     Ok(Some(version))
        // } else {
        //     Ok(None)
        // }
    }
}
