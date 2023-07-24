use futures::{Sink, Stream};
use futures_util::{SinkExt, StreamExt};
use tracing::info;

use mqtt_codec::types::{DecodeError, EncodeError};
use mqtt_codec::v3::packet::*;
// use mqtt_codec::v5::codec::Packet as PacketV5;

type PacketV3Result = Result<Packet, DecodeError>;

pub async fn process_v3<ST, SI>(mut packet_stream: ST, mut packet_sink: SI)
where
    ST: Stream<Item = PacketV3Result> + Unpin + Send + Sync + 'static,
    SI: Sink<Packet, Error = EncodeError> + Unpin + Send + Sync + 'static,
{
    loop {
        match packet_stream.next().await {
            Some(Ok(Packet::Connect(packet))) => {
                if packet.client_id != "" {
                    info!("v3 codec get connection req packet: {:?}", packet);
                }

                let conn_ack_packet = Packet::ConnAck(ConnAckPacket {
                    session_present: true,
                    code: ConnectAckCode::Success,
                });
                info!("conn ack packet: {:?}", conn_ack_packet);
                let _ = packet_sink.send(conn_ack_packet).await;
            }
            Some(Ok(Packet::ConnAck(_packet))) => {}
            Some(Ok(Packet::PingReq(_packet))) => {}
            Some(Err(_err)) => {
                todo!()
            }
            None => {}
            _ => {}
        };
    }
}

// type PacketV5Result = Result<PacketV5, DecodeError>;
// pub fn process_v5<ST, SI>(_packet_stream: ST, _packet_sink: SI)
// where
//     ST: Stream<Item = PacketV5Result> + Unpin + Send + Sync + 'static,
//     SI: Sink<PacketV5, Error = EncodeError> + Unpin + Send + Sync + 'static,
// {
//     tokio::spawn(async move {});
// }
