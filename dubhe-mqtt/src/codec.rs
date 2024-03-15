use bytes::BytesMut;
use mqtt_codec::types::{DecodeError, EncodeError};
use mqtt_codec::v3::{decoder as decoderV3, encoder as encoderV3, packet::Packet as PacketV3};
use mqtt_codec::v4::{decoder as decoderV4, encoder as encoderV4, packet::Packet as PacketV4};
use mqtt_codec::v5::{decoder as decoderV5, encoder as encoderV5, packet::Packet as PacketV5};
use tokio_util::codec::{Decoder, Encoder};
use tracing::info;

pub struct CodecV3 {}
impl CodecV3 {
    pub fn new() -> Self {
        CodecV3 {}
    }
}

impl Decoder for CodecV3 {
    type Item = PacketV3;
    type Error = DecodeError;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        // TODO - Ideally we should keep a state machine to store the data we've read so far.
        info!("v3 decode buf: {:?}", buf);
        println!("mqtt v5 codec decoder buffer length: {:?}", buf.len());
        decoderV3::decode_mqtt(buf)
    }
}

impl Encoder<PacketV3> for CodecV3 {
    type Error = EncodeError;
    fn encode(&mut self, packet: PacketV3, bytes: &mut BytesMut) -> Result<(), Self::Error> {
        encoderV3::encode_mqtt(packet, bytes)
    }
}

pub struct CodecV4 {}
impl CodecV4 {
    pub fn new() -> Self {
        CodecV4 {}
    }
}

impl Decoder for CodecV4 {
    type Item = PacketV4;
    type Error = DecodeError;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        // TODO - Ideally we should keep a state machine to store the data we've read so far.
        info!("v3 decode buf: {:?}", buf);
        decoderV4::decode_mqtt(buf)
    }
}

impl Encoder<PacketV3> for CodecV4 {
    type Error = EncodeError;
    fn encode(&mut self, packet: PacketV3, bytes: &mut BytesMut) -> Result<(), Self::Error> {
        encoderV4::encode_mqtt(packet, bytes)
    }
}

pub struct CodecV5 {}
impl CodecV5 {
    pub fn new() -> Self {
        CodecV5 {}
    }
}

impl Decoder for CodecV5 {
    type Item = PacketV5;
    type Error = DecodeError;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        // TODO - Ideally we should keep a state machine to store the data we've read so far.
        info!("v5 decode buf: {:?}", buf);
        println!("mqtt v5 codec decoder buffer length: {:?}", buf.len());
        decoderV5::decode_mqtt(buf)
    }
}

impl Encoder<PacketV5> for CodecV5 {
    type Error = EncodeError;
    fn encode(&mut self, packet: PacketV5, bytes: &mut BytesMut) -> Result<(), Self::Error> {
        encoderV5::encode_mqtt(packet, bytes)
    }
}
