use bytes::BytesMut;
use tokio_util::codec::{Decoder, Encoder};

use crate::types::{DecodeError, EncodeError};
use crate::v5::packet::*;
// use crate::v5::decoder;

pub struct Codec {}
impl Codec {
    pub fn new() -> Self {
        Codec {}
    }
}

impl Decoder for Codec {
    type Error = DecodeError;
    type Item = Packet;
    fn decode(&mut self, _buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        // TODO - Ideally we should keep a state machine to store the data we've read so far.
        // let packet = decoder::decode_mqtt(buf);
        todo!()
    }
}

impl Encoder<Packet> for Codec {
    type Error = EncodeError;
    fn encode(&mut self, _packet: Packet, _bytes: &mut BytesMut) -> Result<(), Self::Error> {
        // self.encode(packet, bytes)
        todo!()
    }
}
