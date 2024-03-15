// use futures::{Sink, Stream};
// use futures_util::{SinkExt, StreamExt};
use futures_util::StreamExt;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio_tungstenite;
use tokio_util::codec::Framed;
use tracing::{error, info};

use mqtt_codec::types::ProtocolVersion;

use crate::codec;
use crate::config::MqttConfig;
use crate::service;
use crate::version::VersionCodec;

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

            let version = match framed.next().await {
                Some(Ok(v)) => v,
                Some(Err(e)) => {
                    error!("version codec error: {:?}", e);
                    continue;
                }
                None => {
                    todo!()
                }
            };

            info!(
                "tcp new connection established, mqtt version: {:?}, addr: {}",
                version,
                addr.to_string()
            );

            match version {
                ProtocolVersion::MQTT3 => {
                    let f = framed.map_codec(|_codec| codec::CodecV3::new());
                    let (packet_sink, packet_stream) = f.split();
                    tokio::spawn(async move {
                        service::process_v3(packet_stream, packet_sink).await;
                    });
                }
                ProtocolVersion::MQTT4 => {
                    let f = framed.map_codec(|_codec| codec::CodecV3::new());
                    let (packet_sink, packet_stream) = f.split();
                    tokio::spawn(async move {
                        service::process_v3(packet_stream, packet_sink).await;
                    });
                }
                ProtocolVersion::MQTT5 => {
                    let framed = framed.map_codec(|_codec| codec::CodecV5::new());
                    let (_packet_sink, _packet_stream) = framed.split();
                    // service::process_v5(packet_stream, packet_sink);
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
