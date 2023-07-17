use futures::{Sink, Stream};
use futures_util::{SinkExt, StreamExt};

use crate::types::{DecodeError, EncodeError};
use crate::v3::codec::Packet as PacketV3;
use crate::v5::codec::Packet as PacketV5;

type PacketV3Result = Result<PacketV3, DecodeError>;
type PacketV5Result = Result<PacketV5, DecodeError>;

pub fn process_v3<ST, SI>(mut packet_stream: ST, packet_sink: SI)
where
    ST: Stream<Item = PacketV3Result> + Unpin + Send + Sync + 'static,
    SI: Sink<PacketV3, Error = EncodeError> + Unpin + Send + Sync + 'static,
{
    tokio::spawn(async move {
        match packet_stream.next().await {
            Some(Ok(PacketV3::Connect(p))) => {
                println!("process_v3 {:?}", p)
            }
            Some(Ok(PacketV3::ConnAck(p))) => {}
            Some(Ok(PacketV3::PingReq(p))) => {}
            Some(Err(err)) => {}
            None => {
                todo!()
            }
        };
    });
}

pub fn process_v5<ST, SI>(packet_stream: ST, packet_sink: SI)
where
    ST: Stream<Item = PacketV5Result> + Unpin + Send + Sync + 'static,
    SI: Sink<PacketV5, Error = EncodeError> + Unpin + Send + Sync + 'static,
{
    tokio::spawn(async move {});
}
