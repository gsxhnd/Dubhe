use crate::{types::EncodeError, v5::packet::*};
use bytes::BytesMut;

pub fn encode_mqtt(packet: PacketType, bytes: &mut BytesMut) -> Result<(), EncodeError> {
    Ok(())
    // todo!()
    // let remaining_length = packet.calculate_size(protocol_version);
    // let packet_size =
    //     1 + VariableByteInt(remaining_length).calculate_size(protocol_version) + remaining_length;
    // bytes.reserve(packet_size as usize);

    // let first_byte = packet.to_byte();
    // let mut first_byte_val = (first_byte << 4) & 0b1111_0000;
    // first_byte_val |= packet.fixed_header_flags();

    // bytes.put_u8(first_byte_val);
    // encode_variable_int(remaining_length, bytes);

    // match packet {
    //     Packet::Connect(p) => encode_connect(p, bytes, protocol_version),
    //     // Packet::ConnAck(p) => encode_connect_ack(p, bytes, protocol_version),
    //     Packet::ConnAck(p, proper) => encode_connect_ack(p, bytes),
    //     Packet::Publish(p) => encode_publish(p, bytes, protocol_version),
    //     Packet::PublishAck(p) => encode_publish_ack(p, bytes, protocol_version),
    //     Packet::PublishReceived(p) => encode_publish_received(p, bytes, protocol_version),
    //     Packet::PublishRelease(p) => encode_publish_release(p, bytes, protocol_version),
    //     Packet::PublishComplete(p) => encode_publish_complete(p, bytes, protocol_version),
    //     Packet::Subscribe(p) => encode_subscribe(p, bytes, protocol_version),
    //     Packet::SubscribeAck(p) => encode_subscribe_ack(p, bytes, protocol_version),
    //     Packet::Unsubscribe(p) => encode_unsubscribe(p, bytes, protocol_version),
    //     Packet::UnsubscribeAck(p) => encode_unsubscribe_ack(p, bytes, protocol_version),
    //     Packet::PingRequest => {}
    //     Packet::PingResponse => {}
    //     Packet::Disconnect(p) => encode_disconnect(p, bytes, protocol_version),
    //     Packet::Authenticate(p) => encode_authenticate(p, bytes, protocol_version),
    // }
}
