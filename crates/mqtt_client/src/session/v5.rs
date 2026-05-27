//! MQTT v5.0 session.

use std::collections::HashMap;

use bytes::{Bytes, BytesMut};
use mqtt_codec::v5::{
    ConnectPacketBuilder, MqttCodec, Packet, PingReqPacket, Properties, PublishPacketBuilder,
    PubAckPacket, PubCompPacket, PubRecPacket, PubRelPacket, QoS, ReasonCode,
    SubscribePacketBuilder, UnsubscribePacketBuilder,
};
use mqtt_codec::{Decoder, Encoder};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc;

use super::{client_id, emit_disconnected, keep_alive_secs, reset_ping_deadline};
use crate::client::Command;
use crate::config::ClientConfig;
use crate::error::ClientError;
use crate::event::Event;

pub(super) async fn run(
    stream: TcpStream,
    config: &ClientConfig,
    mut command_rx: mpsc::Receiver<Command>,
    event_tx: mpsc::Sender<Event>,
) -> Result<(), ClientError> {
    let mut session = Session::new(stream);
    if let Err(e) = session.handshake(config).await {
        emit_disconnected(&event_tx, Some(e.to_string())).await;
        return Err(e);
    }

    let _ = event_tx.send(Event::Connected).await;

    let ping_every = keep_alive_secs(config.keep_alive);
    let mut ping_deadline = if config.keep_alive > 0 {
        Some(tokio::time::Instant::now() + ping_every)
    } else {
        None
    };

    let mut running = true;
    while running {
        let ping_sleep = ping_deadline.map(tokio::time::sleep_until);

        tokio::select! {
            cmd = command_rx.recv() => {
                match cmd {
                    Some(Command::Publish { topic, payload, qos, retain }) => {
                        session.publish(&topic, payload, qos, retain).await?;
                        reset_ping_deadline(&mut ping_deadline, ping_every);
                    }
                    Some(Command::Subscribe { topics }) => {
                        session.subscribe(topics).await?;
                        reset_ping_deadline(&mut ping_deadline, ping_every);
                    }
                    Some(Command::Unsubscribe { topics }) => {
                        session.unsubscribe(topics).await?;
                        reset_ping_deadline(&mut ping_deadline, ping_every);
                    }
                    Some(Command::Disconnect) => {
                        let _ = session.send_disconnect().await;
                        running = false;
                    }
                    None => running = false,
                }
            }
            read_result = session.read_packet() => {
                match read_result? {
                    None => running = false,
                    Some(packet) => {
                        handle_packet(&mut session, &event_tx, packet).await?;
                        reset_ping_deadline(&mut ping_deadline, ping_every);
                    }
                }
            }
            _ = async {
                match ping_sleep {
                    Some(sleep) => sleep.await,
                    None => std::future::pending::<()>().await,
                }
            },
            if ping_deadline.is_some() => {
                session.ping().await?;
                reset_ping_deadline(&mut ping_deadline, ping_every);
            }
        }
    }

    emit_disconnected(&event_tx, None).await;
    Ok(())
}

struct Session {
    stream: TcpStream,
    codec: MqttCodec,
    read_buf: BytesMut,
    write_buf: BytesMut,
    next_packet_id: u16,
    qos2_incoming: HashMap<u16, (String, Bytes, bool)>,
}

impl Session {
    fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            codec: MqttCodec::new(),
            read_buf: BytesMut::with_capacity(4096),
            write_buf: BytesMut::with_capacity(4096),
            next_packet_id: 1,
            qos2_incoming: HashMap::new(),
        }
    }

    async fn handshake(&mut self, config: &ClientConfig) -> Result<(), ClientError> {
        let connect = build_connect(config)?;
        self.write_packet(Packet::Connect(connect)).await?;

        let packet = self.read_packet().await?.ok_or(ClientError::UnexpectedPacket(
            "connection closed before CONNACK".into(),
        ))?;

        match packet {
            Packet::ConnAck(ack) if ack.reason_code == ReasonCode::Success => Ok(()),
            Packet::ConnAck(ack) => Err(ClientError::ConnectionRefused {
                reason: format!("CONNACK reason 0x{:02X}", ack.reason_code.as_u8()),
            }),
            other => Err(ClientError::UnexpectedPacket(format!(
                "expected CONNACK, got {:?}",
                other.packet_type()
            ))),
        }
    }

    fn alloc_packet_id(&mut self) -> u16 {
        let id = self.next_packet_id;
        self.next_packet_id = if id == u16::MAX { 1 } else { id + 1 };
        id
    }

    async fn publish(
        &mut self,
        topic: &str,
        payload: Bytes,
        qos: u8,
        retain: bool,
    ) -> Result<(), ClientError> {
        let qos = parse_qos(qos)?;
        let mut builder = PublishPacketBuilder::new(topic, payload).qos(qos).retain(retain);
        if qos != QoS::AtMostOnce {
            builder = builder.packet_id(self.alloc_packet_id());
        }
        self.write_packet(Packet::Publish(builder.build())).await
    }

    async fn subscribe(&mut self, topics: Vec<(String, u8)>) -> Result<(), ClientError> {
        if topics.is_empty() {
            return Ok(());
        }
        let packet_id = self.alloc_packet_id();
        let mut builder = SubscribePacketBuilder::new(packet_id);
        for (filter, qos) in topics {
            builder = builder.topic(filter, parse_qos(qos)?);
        }
        let subscribe = builder
            .build()
            .ok_or(ClientError::UnexpectedPacket("empty SUBSCRIBE".into()))?;
        self.write_packet(Packet::Subscribe(subscribe)).await
    }

    async fn unsubscribe(&mut self, topics: Vec<String>) -> Result<(), ClientError> {
        if topics.is_empty() {
            return Ok(());
        }
        let packet_id = self.alloc_packet_id();
        let mut builder = UnsubscribePacketBuilder::new(packet_id);
        for topic in topics {
            builder = builder.topic(topic);
        }
        let unsubscribe = builder
            .build()
            .ok_or(ClientError::UnexpectedPacket("empty UNSUBSCRIBE".into()))?;
        self.write_packet(Packet::Unsubscribe(unsubscribe)).await
    }

    async fn ping(&mut self) -> Result<(), ClientError> {
        self.write_packet(Packet::PingReq(PingReqPacket)).await
    }

    async fn send_disconnect(&mut self) -> Result<(), ClientError> {
        self.write_packet(Packet::Disconnect(Default::default())).await
    }

    async fn write_packet(&mut self, packet: Packet) -> Result<(), ClientError> {
        self.write_buf.clear();
        self.codec.encode(packet, &mut self.write_buf)?;
        self.stream
            .write_all(&self.write_buf)
            .await
            .map_err(ClientError::Io)?;
        self.stream.flush().await.map_err(ClientError::Io)
    }

    async fn read_packet(&mut self) -> Result<Option<Packet>, ClientError> {
        loop {
            if let Some(packet) = self.codec.decode(&mut self.read_buf)? {
                return Ok(Some(packet));
            }
            let n = self
                .stream
                .read_buf(&mut self.read_buf)
                .await
                .map_err(ClientError::Io)?;
            if n == 0 {
                return Ok(None);
            }
        }
    }
}

async fn handle_packet(
    session: &mut Session,
    event_tx: &mpsc::Sender<Event>,
    packet: Packet,
) -> Result<(), ClientError> {
    match packet {
        Packet::Publish(publish) => {
            let qos = u8::from(publish.qos);
            let retain = publish.retain;
            let topic = publish.topic_name;
            let payload = publish.payload;

            match publish.qos {
                QoS::AtMostOnce => {
                    let _ = event_tx
                        .send(Event::Message {
                            topic,
                            payload,
                            qos,
                            retain,
                        })
                        .await;
                }
                QoS::AtLeastOnce => {
                    if let Some(packet_id) = publish.packet_id {
                        session
                            .write_packet(Packet::PubAck(PubAckPacket {
                                packet_id,
                                reason_code: ReasonCode::Success,
                                properties: Properties::new(),
                            }))
                            .await?;
                    }
                    let _ = event_tx
                        .send(Event::Message {
                            topic,
                            payload,
                            qos,
                            retain,
                        })
                        .await;
                }
                QoS::ExactlyOnce => {
                    let Some(packet_id) = publish.packet_id else {
                        return Err(ClientError::UnexpectedPacket(
                            "QoS 2 PUBLISH missing packet id".into(),
                        ));
                    };
                    session
                        .qos2_incoming
                        .insert(packet_id, (topic, payload, retain));
                    session
                        .write_packet(Packet::PubRec(PubRecPacket {
                            packet_id,
                            reason_code: ReasonCode::Success,
                            properties: Properties::new(),
                        }))
                        .await?;
                }
            }
        }
        Packet::PubAck(ack) => {
            let _ = event_tx
                .send(Event::PubAck {
                    packet_id: ack.packet_id,
                })
                .await;
        }
        Packet::PubRec(rec) => {
            session
                .write_packet(Packet::PubRel(PubRelPacket {
                    packet_id: rec.packet_id,
                    reason_code: ReasonCode::Success,
                    properties: Properties::new(),
                }))
                .await?;
        }
        Packet::PubRel(rel) => {
            session
                .write_packet(Packet::PubComp(PubCompPacket {
                    packet_id: rel.packet_id,
                    reason_code: ReasonCode::Success,
                    properties: Properties::new(),
                }))
                .await?;
            if let Some((topic, payload, retain)) = session.qos2_incoming.remove(&rel.packet_id) {
                let _ = event_tx
                    .send(Event::Message {
                        topic,
                        payload,
                        qos: 2,
                        retain,
                    })
                    .await;
            }
        }
        Packet::PubComp(comp) => {
            let _ = event_tx
                .send(Event::PubComp {
                    packet_id: comp.packet_id,
                })
                .await;
        }
        Packet::SubAck(ack) => {
            let return_codes: Vec<u8> = ack.reason_codes.iter().map(|c| c.as_u8()).collect();
            let _ = event_tx
                .send(Event::SubAck {
                    packet_id: ack.packet_id,
                    return_codes,
                })
                .await;
        }
        Packet::UnsubAck(ack) => {
            let _ = event_tx
                .send(Event::UnsubAck {
                    packet_id: ack.packet_id,
                })
                .await;
        }
        Packet::PingResp(_) | Packet::Disconnect(_) | Packet::Auth(_) => {}
        other => {
            return Err(ClientError::UnexpectedPacket(format!(
                "unexpected packet from broker: {:?}",
                other.packet_type()
            )));
        }
    }
    Ok(())
}

fn build_connect(config: &ClientConfig) -> Result<mqtt_codec::v5::ConnectPacket, ClientError> {
    let mut builder = ConnectPacketBuilder::new(client_id(config))
        .keep_alive(config.keep_alive)
        .clean_start(config.clean_session);

    if let Some(creds) = &config.credentials {
        builder = builder.username(&creds.username);
        if let Some(password) = &creds.password {
            builder = builder.password(password.clone());
        }
    }

    if let Some(will) = &config.last_will {
        builder = builder.will(
            &will.topic,
            will.message.clone(),
            parse_qos(will.qos)?,
            will.retain,
        );
    }

    Ok(builder.build())
}

fn parse_qos(qos: u8) -> Result<QoS, ClientError> {
    QoS::try_from(qos).map_err(|_| ClientError::InvalidQoS(qos))
}
