use futures::{Sink, Stream};
use futures_util::{SinkExt, StreamExt};
use tracing::info;

use mqtt_codec::types::{DecodeError, EncodeError};
use mqtt_codec::v3::codec as MqttCodecV3;
use mqtt_codec::v3::codec::Packet as PacketV3;
use mqtt_codec::v5::codec::Packet as PacketV5;

type PacketV3Result = Result<PacketV3, DecodeError>;
type PacketV5Result = Result<PacketV5, DecodeError>;

pub async fn process_v3<ST, SI>(mut packet_stream: ST, mut packet_sink: SI)
where
    ST: Stream<Item = PacketV3Result> + Unpin + Send + Sync + 'static,
    SI: Sink<PacketV3, Error = EncodeError> + Unpin + Send + Sync + 'static,
{
    loop {
        match packet_stream.next().await {
            Some(Ok(PacketV3::Connect(packet))) => {
                if packet.client_id != "" {
                    info!("v3 codec get connection req packet: {:?}", packet);
                }

                // let conn_ack_packet = MqttCodecV3::Packet::ConnAck(MqttCodecV3::ConnAck {
                //     session_present: true,
                //     code: MqttCodecV3::ConnectAckCode::Success,
                // });
                // info!("conn ack packet: {:?}", conn_ack_packet);
                // let _ = packet_sink.send(conn_ack_packet).await;
            }
            Some(Ok(PacketV3::ConnAck(_packet))) => {}
            Some(Ok(PacketV3::PingReq(_packet))) => {}
            Some(Err(_err)) => {
                todo!()
            }
            None => {
                todo!()
            }
        };
    }
}

pub fn process_v5<ST, SI>(packet_stream: ST, packet_sink: SI)
where
    ST: Stream<Item = PacketV5Result> + Unpin + Send + Sync + 'static,
    SI: Sink<PacketV5, Error = EncodeError> + Unpin + Send + Sync + 'static,
{
    tokio::spawn(async move {});
}
