use bytes::BytesMut;

use crate::types::DecodeError;
use crate::v3::codec::Packet;


pub fn decode_mqtt(bytes: &mut BytesMut) -> Result<Option<Packet>, DecodeError> {
    todo!()
}
