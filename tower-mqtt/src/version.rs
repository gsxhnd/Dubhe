use std::error::Error;
use tokio_util::codec::Decoder;

#[derive(Clone, Copy, Debug)]
pub enum ProtocolVersion {
    MQTT3,
    MQTT5,
}

pub struct VersionCodec;

impl Decoder for VersionCodec {
    type Item = ProtocolVersion;
    type Error = Box<dyn Error>;

    fn decode(&mut self, buf: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        // Check the first byte to determine the MQTT version
        if buf.len() >= 1 {
            let version_byte = buf[0];
            let version = match version_byte {
                3 => ProtocolVersion::MQTT3,
                5 => ProtocolVersion::MQTT5,
                _ => return Err("Unsupported MQTT version".into()),
            };
            Ok(Some(version))
        } else {
            Ok(None)
        }
    }
}
