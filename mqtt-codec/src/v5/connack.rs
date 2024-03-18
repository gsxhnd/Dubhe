use bytes::{Bytes, BytesMut};

use crate::types::DecodeError;

// Return code in connack
// This contains return codes for both MQTT v311 and v5
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectReturnCode {
    Success,
    RefusedProtocolVersion,
    BadClientId,
    ServiceUnavailable,
    UnspecifiedError,
    MalformedPacket,
    ProtocolError,
    ImplementationSpecificError,
    UnsupportedProtocolVersion,
    ClientIdentifierNotValid,
    BadUserNamePassword,
    NotAuthorized,
    ServerUnavailable,
    ServerBusy,
    Banned,
    BadAuthenticationMethod,
    TopicNameInvalid,
    PacketTooLarge,
    QuotaExceeded,
    PayloadFormatInvalid,
    RetainNotSupported,
    QoSNotSupported,
    UseAnotherServer,
    ServerMoved,
    ConnectionRateExceeded,
}

impl ConnectReturnCode {
    fn to_u8(&self) -> u8 {
        match self {
            ConnectReturnCode::Success => 0,
            ConnectReturnCode::UnspecifiedError => 128,
            ConnectReturnCode::MalformedPacket => 129,
            ConnectReturnCode::ProtocolError => 130,
            ConnectReturnCode::ImplementationSpecificError => 131,
            ConnectReturnCode::UnsupportedProtocolVersion => 132,
            ConnectReturnCode::ClientIdentifierNotValid => 133,
            ConnectReturnCode::BadUserNamePassword => 134,
            ConnectReturnCode::NotAuthorized => 135,
            ConnectReturnCode::ServerUnavailable => 136,
            ConnectReturnCode::ServerBusy => 137,
            ConnectReturnCode::Banned => 138,
            ConnectReturnCode::BadAuthenticationMethod => 140,
            ConnectReturnCode::TopicNameInvalid => 144,
            ConnectReturnCode::PacketTooLarge => 149,
            ConnectReturnCode::QuotaExceeded => 151,
            ConnectReturnCode::PayloadFormatInvalid => 153,
            ConnectReturnCode::RetainNotSupported => 154,
            ConnectReturnCode::QoSNotSupported => 155,
            ConnectReturnCode::UseAnotherServer => 156,
            ConnectReturnCode::ServerMoved => 157,
            ConnectReturnCode::ConnectionRateExceeded => 159,
            _ => todo!(),
        }
    }

    fn from_u8(num: u8) -> Result<Self, DecodeError> {
        let a = match num {
            0 => ConnectReturnCode::Success,
            128 => ConnectReturnCode::UnspecifiedError,
            129 => ConnectReturnCode::MalformedPacket,
            130 => ConnectReturnCode::ProtocolError,
            131 => ConnectReturnCode::ImplementationSpecificError,
            132 => ConnectReturnCode::UnsupportedProtocolVersion,
            133 => ConnectReturnCode::ClientIdentifierNotValid,
            134 => ConnectReturnCode::BadUserNamePassword,
            135 => ConnectReturnCode::NotAuthorized,
            136 => ConnectReturnCode::ServerUnavailable,
            137 => ConnectReturnCode::ServerBusy,
            138 => ConnectReturnCode::Banned,
            140 => ConnectReturnCode::BadAuthenticationMethod,
            144 => ConnectReturnCode::TopicNameInvalid,
            149 => ConnectReturnCode::PacketTooLarge,
            151 => ConnectReturnCode::QuotaExceeded,
            153 => ConnectReturnCode::PayloadFormatInvalid,
            154 => ConnectReturnCode::RetainNotSupported,
            155 => ConnectReturnCode::QoSNotSupported,
            156 => ConnectReturnCode::UseAnotherServer,
            157 => ConnectReturnCode::ServerMoved,
            159 => ConnectReturnCode::ConnectionRateExceeded,
            _ => todo!(),
        };
        Ok(a)
    }
}

/// Acknowledgement to connect packet
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnAck {
    pub session_present: bool,
    pub code: ConnectReturnCode,
    pub properties: Option<ConnAckProperties>,
}

impl ConnAck {
    fn write(&self, buffer: &mut BytesMut) {
        let a = self.code.to_u8();
        println!("{}", a);
        match ConnectReturnCode::from_u8(a) {
            Ok(v) => {
                match v {
                    ConnectReturnCode::BadAuthenticationMethod => {
                        println!("Bad authentication method")
                    }
                    _ => {}
                }
                println!("{:?}", v)
            }
            Err(e) => todo!(),
        }
    }
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

#[test]
fn test_write() {
    let a = ConnAck {
        session_present: false,
        code: ConnectReturnCode::BadAuthenticationMethod,
        properties: None,
    };

    a.write(&mut BytesMut::new());
}
