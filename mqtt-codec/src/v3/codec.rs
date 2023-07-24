use bytes::BytesMut;
use tokio_util::codec::{Decoder, Encoder};
use tracing::info;

use crate::types::{DecodeError, EncodeError};
use crate::v3::packet::*;
use crate::v3::{decoder, encoder};

pub struct Codec {}
impl Codec {
    pub fn new() -> Self {
        Codec {}
    }
}

impl Decoder for Codec {
    type Item = Packet;
    type Error = DecodeError;
    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        // TODO - Ideally we should keep a state machine to store the data we've read so far.
        info!("v3 decode buf: {:?}", buf);
        decoder::decode_mqtt(buf)
    }
}

impl Encoder<Packet> for Codec {
    type Error = EncodeError;
    fn encode(&mut self, packet: Packet, bytes: &mut BytesMut) -> Result<(), Self::Error> {
        encoder::encode_mqtt(packet, bytes)
    }
}
