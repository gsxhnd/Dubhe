use futures_util::{SinkExt, StreamExt};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio_tungstenite;
use tokio_util::codec::Framed;
use tracing::info;

use crate::config::MqttConfig;
use crate::v5::codec::{self, Packet};
use crate::version::ProtocolVersion;

#[derive(Debug, Clone)]
pub enum BrokerMessage {
    _Test(String),
    // NewClient(Box<ConnectPacket>, Sender<ClientMessage>),
    // Publish(String, Box<PublishPacket>),
    // PublishAck(String, PublishAckPacket), // TODO - This can be handled by the client task
    // PublishRelease(String, PublishReleasePacket), // TODO - This can be handled by the client task
    // PublishReceived(String, PublishReceivedPacket),
    // PublishComplete(String, PublishCompletePacket),
    // PublishFinalWill(String, FinalWill),
    // Subscribe(String, SubscribePacket), // TODO - replace string client_id with int
    // Unsubscribe(String, UnsubscribePacket), // TODO - replace string client_id with int
    // Disconnect(String, WillDisconnectLogic),
}

#[derive(Debug, Clone)]
pub struct MqttServer<V3, V5> {
    v3: V3,
    v5: V5,
    config: MqttConfig,
    a: Arc<Mutex<i32>>,
    // receiver: Receiver<BrokerMessage>,
    // sender: Sender<BrokerMessage>,
}

impl MqttServer<DefaultProtocolServer, DefaultProtocolServer> {
    pub fn new(cfg: MqttConfig) -> Self {
        // let (sender, receiver) = mpsc::channel(100);
        MqttServer {
            v3: DefaultProtocolServer::new(ProtocolVersion::MQTT3),
            v5: DefaultProtocolServer::new(ProtocolVersion::MQTT5),
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
            info!("New connection: {}", addr);
            let (packet_sink, mut packet_stream) =
                Framed::new(stream, codec::MqttCodec::new()).split();

            let first_packet = packet_stream.next().await;
            match first_packet {
                Some(Ok(Packet::Connect(connect_packet))) => {
                    println!("get connect packet {:?}", connect_packet);
                    // packet_sink.send().await?;
                }
                Some(Ok(Packet::ConnAck(_, _))) => todo!(),
                Some(Err(_)) => return,
                None => return,
            }
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


#[derive(Debug, Clone)]
pub struct DefaultProtocolServer {
    ver: ProtocolVersion,
}

impl DefaultProtocolServer {
    fn new(ver: ProtocolVersion) -> Self {
        Self { ver }
    }
}
