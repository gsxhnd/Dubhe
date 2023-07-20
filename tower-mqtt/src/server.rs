// use futures_util::sink::Send;
use futures::{Sink, Stream};
use futures_util::{SinkExt, StreamExt};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio_tungstenite;
use tokio_util::codec::Framed;
use tracing::info;

use mqtt_codec::types::{ConnectPacket, DecodeError, EncodeError, ProtocolVersion};
use mqtt_codec::v3::codec as MqttCodecV3;
use mqtt_codec::v3::codec::Packet as PacketV3;
use mqtt_codec::v5::codec as MqttCodecV5;
use mqtt_codec::v5::codec::Packet as PacketV5;

use crate::config::MqttConfig;
use crate::service;
use crate::version::{ConnectAckCode, ConnectAckPacket, VersionCodec};

#[derive(Debug, Clone)]
pub struct MqttServer {
    // v3: V3,
    // v5: V5,
    config: MqttConfig,
    a: Arc<Mutex<i32>>,
    // receiver: Receiver<BrokerMessage>,
    // sender: Sender<BrokerMessage>,
}

impl MqttServer {
    pub fn new(cfg: MqttConfig) -> Self {
        // let (sender, receiver) = mpsc::channel(100);
        MqttServer {
            // v3: DefaultProtocolServer::new(ProtocolVersion::MQTT3),
            // v5: DefaultProtocolServer::new(ProtocolVersion::MQTT5),
            config: cfg,
            a: Arc::new(Mutex::new(0)),
            // receiver,
            // sender,
        }
    }

    pub async fn run(self) {
        info!("mqtt server running");
        tokio::spawn(self.clone().listen_tcp());
        tokio::spawn(self.clone().listen_tls());
        tokio::spawn(self.clone().listen_ws());
        tokio::spawn(self.clone().listen_wss());
    }

    pub async fn listen_tcp(self) {
        let addr: SocketAddr = self
            .config
            .listener
            .tcp
            .addr
            .parse()
            .expect("address is not valid");

        let tcp_listener = TcpListener::bind(addr).await.expect("msg");
        info!(
            "mqtt tcp service started in: {}",
            self.config.listener.tcp.addr
        );
        loop {
            let (stream, addr) = tcp_listener.accept().await.unwrap();
            let mut framed = Framed::new(stream, VersionCodec);
            // let (mut packet_sink, mut packet_stream) = framed.split();
            // let connect_packet: crate::version::ConnectPacket = match packet_stream.next().await {
            let connect_packet: crate::version::ConnectPacket = match framed.next().await {
                Some(Ok(connect)) => connect,
                Some(Err(e)) => {
                    println!("{:?}", e);
                    // ProtocolVersion::MQTT5
                    todo!()
                }
                None => {
                    todo!()
                }
            };

            info!(
                "tcp connection established, mqtt version: {:?}",
                connect_packet
            );

            // let _ = packet_sink
            //     .send(ConnectAckPacket {
            //         session_present: true,
            //         code: ConnectAckCode::Success,
            //     })
            //     .await;
            // process(packet_stream, packet_sink)
            // service::process_v3(packet_stream, packet_sink);

            match connect_packet.protocol_version {
                ProtocolVersion::MQTT3 => {
                    let f = framed.map_codec(|_codec| MqttCodecV3::Codec::new());
                    let (packet_sink, packet_stream) = f.split();
                    service::process_v3(packet_stream, packet_sink);
                }
                ProtocolVersion::MQTT5 => {
                    println!("mqtt verssion: 5");
                    let mut v5_codec = MqttCodecV5::Codec::new();
                    let framed = framed.map_codec(|_codec| MqttCodecV5::Codec::new());
                    let (packet_sink, packet_stream) = framed.split();
                    service::process_v5(packet_stream, packet_sink);
                }
                _ => {
                    todo!()
                }
            };
        }
    }

    pub async fn listen_tls(self) {
        let addr: SocketAddr = self
            .config
            .listener
            .tls
            .addr
            .parse()
            .expect("address is not valid");

        let tls_listener = TcpListener::bind(addr).await.expect("msg");
        info!(
            "mqtt tls service started in: {}",
            self.config.listener.tls.addr
        );
        loop {
            let (_stream, addr) = tls_listener.accept().await.unwrap();
            info!("New connection: {} {:?}", addr, self.a);
            let mut a = self.a.lock().unwrap();
            *a += 1;
        }
    }

    pub async fn listen_ws(self) {
        let addr: SocketAddr = self
            .config
            .listener
            .ws
            .addr
            .parse()
            .expect("address is not valid");

        let ws_listener = TcpListener::bind(addr).await.expect("msg");
        info!(
            "mqtt ws service started in: {}",
            self.config.listener.ws.addr
        );

        while let Ok((stream, _)) = ws_listener.accept().await {
            tokio::spawn(async {
                let ws_stream = tokio_tungstenite::accept_async(stream).await.expect("msg");
                let client_addr = ws_stream.get_ref().peer_addr().unwrap();
                info!("New WebSocket connection: {}", client_addr);
                let (mut _write, mut read) = ws_stream.split();

                while let Some(message) = read.next().await {
                    let message = message.unwrap();

                    info!("Received a message from {}: {}", client_addr, message);
                    // write.send(message).await.unwrap();
                }
            });
        }
    }

    pub async fn listen_wss(self) {
        let addr: SocketAddr = self
            .config
            .listener
            .wss
            .addr
            .parse()
            .expect("address is not valid");

        let wss_listener = TcpListener::bind(addr).await.expect("msg");

        info!(
            "mqtt wss service started in: {}",
            self.config.listener.wss.addr
        );
        while let Ok((stream, _)) = wss_listener.accept().await {
            tokio::spawn(async {
                let ws_stream = tokio_tungstenite::accept_async(stream).await.expect("msg");
                let client_addr = ws_stream.get_ref().peer_addr().unwrap();
                info!("New WebSocket connection: {}", client_addr);
                let (mut _write, mut read) = ws_stream.split();

                while let Some(message) = read.next().await {
                    let message = message.unwrap();
                    info!("Received a message from {}: {}", client_addr, message,);
                    // write.send(message).await.unwrap();
                }
            });
        }
    }
}

type PacketResult = Result<crate::version::ConnectPacket, DecodeError>;
pub fn process<ST, SI>(mut packet_stream: ST, packet_sink: SI)
where
    ST: Stream<Item = PacketResult> + Unpin + Send + Sync + 'static,
    SI: Sink<ConnectAckPacket, Error = EncodeError> + Unpin + Send + Sync + 'static,
{
    tokio::spawn(async move {
        match packet_stream.next() {
            _ => {
                println!("process next packet");
            }
        }
    });
}
