use bytes::BytesMut;
// use bytes::{Buf, BytesMut};
// use std::io::Cursor;

use crate::types::DecodeError;
use crate::v5::packet::*;

pub fn decode_mqtt(_bytes: &mut BytesMut) -> Result<Option<Packet>, DecodeError> {
    todo!()
    // let mut bytes = Cursor::new(bytes);
    // let first_byte = bytes.get_u8();

    // match packet_type_check(first_byte) {
    //     PacketType::CONNECT => {
    //         println!("connect packet");
    //         todo!()
    //     }
    //     PacketType::DISCONNECT => {
    //         println!("disconnect packet");
    //         todo!()
    //     }
    // PacketType::CONNECT => Ok(Packet::Connect(Box::new(Connect::decode(&mut src)?))),
    // PacketType::DISCONNECT => Ok(Packet::Disconnect(Disconnect::decode(&mut src)?)),
    // _ => Err(DecodeError::InvalidPacketType),
    // PacketType::PUBLISH_START..=PacketType::PUBLISH_END => Ok(Packet::Publish(
    //     Publish::decode(src, first_byte & 0b0000_1111)?,
    // )),
    // PacketType::PUBACK => Ok(Packet::PublishAck(PublishAck::decode(&mut src)?)),
    // PacketType::PINGREQ => Ok(Packet::PingRequest),
    // PacketType::PINGRESP => Ok(Packet::PingResponse),
    // PacketType::SUBSCRIBE => Ok(Packet::Subscribe(Subscribe::decode(&mut src)?)),
    // PacketType::SUBACK => Ok(Packet::SubscribeAck(SubscribeAck::decode(&mut src)?)),
    // PacketType::UNSUBSCRIBE => Ok(Packet::Unsubscribe(Unsubscribe::decode(&mut src)?)),
    // PacketType::UNSUBACK => Ok(Packet::UnsubscribeAck(UnsubscribeAck::decode(&mut src)?)),
    // PacketType::CONNACK => Ok(Packet::ConnectAck(Box::new(ConnectAck::decode(&mut src)?))),
    // PacketType::AUTH => Ok(Packet::Auth(Auth::decode(&mut src)?)),
    // PacketType::PUBREC => Ok(Packet::PublishReceived(PublishAck::decode(&mut src)?)),
    // PacketType::PUBREL => Ok(Packet::PublishRelease(PublishAck2::decode(&mut src)?)),
    // PacketType::PUBCOMP => Ok(Packet::PublishComplete(PublishAck2::decode(&mut src)?)),
    // }
}
