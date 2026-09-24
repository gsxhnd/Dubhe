//! MQTT v3.1.1 session.

use std::time::Instant;

use bytes::{Bytes, BytesMut};
use mqtt_codec::v4::{
    ConnectPacketBuilder, MqttCodec, Packet, PingReqPacket, PublishPacketBuilder, QoS,
    SubscribePacketBuilder, UnsubscribePacketBuilder,
};
use mqtt_codec::{Decoder, Encoder};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;

use super::state::SessionState;
use super::{
    SessionOutcome, client_id, keep_alive_secs, reset_ping_deadline,
};
use crate::client::Command;
use crate::config::ClientConfig;
use crate::error::ClientError;
use crate::event::Event;
use crate::inflight::{InflightPublish, InflightStage, PendingPublish};
use crate::transport::BrokerStream;

pub(super) async fn run(
    stream: BrokerStream,
    config: &ClientConfig,
    command_rx: &mut mpsc::Receiver<Command>,
    event_tx: &mpsc::Sender<Event>,
    state: &mut SessionState,
) -> Result<SessionOutcome, ClientError> {
    let mut session = Session::new(stream, state);

    let session_present = match session.handshake(config).await {
        Ok(present) => present,
        Err(e) => {
            return Ok(SessionOutcome::ConnectionLost(Some(e.to_string())));
        }
    };

    let _ = event_tx
        .send(Event::Connected { session_present })
        .await;

    if config.clean_session || !session_present {
        session.resubscribe_all().await?;
    }
    session.retransmit_inflight().await?;

    while let Some(cmd) = session.state.pending_commands.pop_front() {
        if handle_command(&mut session, event_tx, cmd).await? {
            return Ok(SessionOutcome::GracefulDisconnect);
        }
    }

    let ping_every = keep_alive_secs(config.keep_alive);
    let mut ping_deadline = if config.keep_alive > 0 {
        Some(tokio::time::Instant::now() + ping_every)
    } else {
        None
    };
    let mut retry_deadline = tokio::time::Instant::now() + config.ack_timeout;

    loop {
        let ping_sleep = ping_deadline.map(tokio::time::sleep_until);
        let retry_sleep = tokio::time::sleep_until(retry_deadline);

        tokio::select! {
            cmd = command_rx.recv() => {
                match cmd {
                    Some(cmd) => {
                        if handle_command(&mut session, event_tx, cmd).await? {
                            return Ok(SessionOutcome::GracefulDisconnect);
                        }
                        reset_ping_deadline(&mut ping_deadline, ping_every);
                    }
                    None => return Ok(SessionOutcome::GracefulDisconnect),
                }
            }
            read_result = session.read_packet() => {
                match read_result {
                    Ok(None) => {
                        return Ok(SessionOutcome::ConnectionLost(Some(
                            "connection closed".into(),
                        )));
                    }
                    Ok(Some(packet)) => {
                        if let Err(e) = handle_packet(&mut session, event_tx, packet).await {
                            return Ok(SessionOutcome::ConnectionLost(Some(e.to_string())));
                        }
                        reset_ping_deadline(&mut ping_deadline, ping_every);
                    }
                    Err(e) => {
                        return Ok(SessionOutcome::ConnectionLost(Some(e.to_string())));
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
                if let Err(e) = session.ping().await {
                    return Ok(SessionOutcome::ConnectionLost(Some(e.to_string())));
                }
                reset_ping_deadline(&mut ping_deadline, ping_every);
            }
            () = retry_sleep => {
                if let Err(e) = session.retry_due(config.ack_timeout).await {
                    return Ok(SessionOutcome::ConnectionLost(Some(e.to_string())));
                }
                retry_deadline = tokio::time::Instant::now() + config.ack_timeout;
            }
        }
    }
}

async fn handle_command(
    session: &mut Session<'_>,
    _event_tx: &mpsc::Sender<Event>,
    cmd: Command,
) -> Result<bool, ClientError> {
    match cmd {
        Command::Publish {
            topic,
            payload,
            qos,
            retain,
        } => {
            session.publish(topic, payload, qos, retain).await?;
            Ok(false)
        }
        Command::Subscribe { topics } => {
            session.subscribe(topics).await?;
            Ok(false)
        }
        Command::Unsubscribe { topics } => {
            session.unsubscribe(topics).await?;
            Ok(false)
        }
        Command::Disconnect => {
            let _ = session.send_disconnect().await;
            Ok(true)
        }
    }
}

struct Session<'a> {
    stream: BrokerStream,
    codec: MqttCodec,
    read_buf: BytesMut,
    write_buf: BytesMut,
    state: &'a mut SessionState,
}

impl<'a> Session<'a> {
    fn new(stream: BrokerStream, state: &'a mut SessionState) -> Self {
        Self {
            stream,
            codec: MqttCodec::new(),
            read_buf: BytesMut::with_capacity(4096),
            write_buf: BytesMut::with_capacity(4096),
            state,
        }
    }

    async fn handshake(&mut self, config: &ClientConfig) -> Result<bool, ClientError> {
        let connect = build_connect(config)?;
        self.write_packet(Packet::Connect(connect)).await?;

        let packet = self.read_packet().await?.ok_or(ClientError::UnexpectedPacket(
            "connection closed before CONNACK".into(),
        ))?;

        match packet {
            Packet::ConnAck(ack) if ack.return_code.is_success() => Ok(ack.session_present),
            Packet::ConnAck(ack) => Err(ClientError::ConnectionRefused {
                reason: ack.return_code.description().to_string(),
            }),
            other => Err(ClientError::UnexpectedPacket(format!(
                "expected CONNACK, got {:?}",
                other.packet_type()
            ))),
        }
    }

    async fn publish(
        &mut self,
        topic: String,
        payload: Bytes,
        qos: u8,
        retain: bool,
    ) -> Result<(), ClientError> {
        let qos_enum = parse_qos(qos)?;
        if qos_enum == QoS::AtMostOnce {
            let packet = PublishPacketBuilder::new(&topic, payload)
                .qos(qos_enum)
                .retain(retain)
                .build();
            return self.write_packet(Packet::Publish(packet)).await;
        }

        if !self.state.inflight.has_capacity() {
            self.state.inflight.push_pending(PendingPublish {
                topic,
                payload,
                qos,
                retain,
            });
            return Ok(());
        }

        self.send_qos_publish(topic, payload, qos, retain, false)
            .await
    }

    async fn send_qos_publish(
        &mut self,
        topic: String,
        payload: Bytes,
        qos: u8,
        retain: bool,
        duplicate: bool,
    ) -> Result<(), ClientError> {
        let qos_enum = parse_qos(qos)?;
        let packet_id = self.state.alloc_packet_id();
        let stage = match qos_enum {
            QoS::AtLeastOnce => InflightStage::Ack,
            QoS::ExactlyOnce => InflightStage::Rec,
            QoS::AtMostOnce => unreachable!(),
        };

        let packet = PublishPacketBuilder::new(&topic, payload.clone())
            .qos(qos_enum)
            .retain(retain)
            .packet_id(packet_id)
            .duplicate(duplicate)
            .build();
        self.write_packet(Packet::Publish(packet)).await?;

        self.state.inflight.insert(
            packet_id,
            InflightPublish {
                topic,
                payload,
                qos,
                retain,
                stage,
                sent_at: Instant::now(),
            },
        );
        Ok(())
    }

    async fn flush_pending(&mut self) -> Result<(), ClientError> {
        while let Some(pending) = self.state.inflight.pop_pending() {
            self.send_qos_publish(
                pending.topic,
                pending.payload,
                pending.qos,
                pending.retain,
                false,
            )
            .await?;
        }
        Ok(())
    }

    async fn subscribe(&mut self, topics: Vec<(String, u8)>) -> Result<(), ClientError> {
        if topics.is_empty() {
            return Ok(());
        }
        for (filter, qos) in &topics {
            self.state
                .remember_subscription(filter.clone(), *qos);
        }
        self.send_subscribe(topics).await
    }

    async fn send_subscribe(&mut self, topics: Vec<(String, u8)>) -> Result<(), ClientError> {
        let packet_id = self.state.alloc_packet_id();
        let mut builder = SubscribePacketBuilder::new(packet_id);
        for (filter, qos) in topics {
            builder = builder.topic(filter, parse_qos(qos)?);
        }
        let subscribe = builder
            .build()
            .ok_or(ClientError::UnexpectedPacket("empty SUBSCRIBE".into()))?;
        self.write_packet(Packet::Subscribe(subscribe)).await
    }

    async fn resubscribe_all(&mut self) -> Result<(), ClientError> {
        if self.state.subscriptions.is_empty() {
            return Ok(());
        }
        let topics: Vec<(String, u8)> = self
            .state
            .subscriptions
            .iter()
            .map(|(f, q)| (f.clone(), *q))
            .collect();
        self.send_subscribe(topics).await
    }

    async fn unsubscribe(&mut self, topics: Vec<String>) -> Result<(), ClientError> {
        if topics.is_empty() {
            return Ok(());
        }
        self.state.forget_subscriptions(&topics);
        let packet_id = self.state.alloc_packet_id();
        let mut builder = UnsubscribePacketBuilder::new(packet_id);
        for topic in topics {
            builder = builder.topic(topic);
        }
        let unsubscribe = builder
            .build()
            .ok_or(ClientError::UnexpectedPacket("empty UNSUBSCRIBE".into()))?;
        self.write_packet(Packet::Unsubscribe(unsubscribe)).await
    }

    async fn retransmit_inflight(&mut self) -> Result<(), ClientError> {
        let snapshot = self.state.inflight.snapshot();
        for (packet_id, entry) in snapshot {
            match entry.stage {
                InflightStage::Ack | InflightStage::Rec => {
                    let qos_enum = parse_qos(entry.qos)?;
                    let packet = PublishPacketBuilder::new(&entry.topic, entry.payload.clone())
                        .qos(qos_enum)
                        .retain(entry.retain)
                        .packet_id(packet_id)
                        .duplicate(true)
                        .build();
                    self.write_packet(Packet::Publish(packet)).await?;
                    if let Some(slot) = self.state.inflight.get_mut(packet_id) {
                        slot.sent_at = Instant::now();
                    }
                }
                InflightStage::Comp => {
                    self.write_packet(Packet::PubRel(mqtt_codec::v4::PubRelPacket {
                        packet_id,
                    }))
                    .await?;
                    if let Some(slot) = self.state.inflight.get_mut(packet_id) {
                        slot.sent_at = Instant::now();
                    }
                }
            }
        }
        Ok(())
    }

    async fn retry_due(&mut self, timeout: std::time::Duration) -> Result<(), ClientError> {
        let due = self.state.inflight.due_for_retry(timeout);
        for packet_id in due {
            let Some(entry) = self.state.inflight.get_mut(packet_id) else {
                continue;
            };
            let stage = entry.stage;
            let topic = entry.topic.clone();
            let payload = entry.payload.clone();
            let qos = entry.qos;
            let retain = entry.retain;
            entry.sent_at = Instant::now();

            match stage {
                InflightStage::Ack | InflightStage::Rec => {
                    let qos_enum = parse_qos(qos)?;
                    let packet = PublishPacketBuilder::new(&topic, payload)
                        .qos(qos_enum)
                        .retain(retain)
                        .packet_id(packet_id)
                        .duplicate(true)
                        .build();
                    self.write_packet(Packet::Publish(packet)).await?;
                }
                InflightStage::Comp => {
                    self.write_packet(Packet::PubRel(mqtt_codec::v4::PubRelPacket {
                        packet_id,
                    }))
                    .await?;
                }
            }
        }
        Ok(())
    }

    async fn ping(&mut self) -> Result<(), ClientError> {
        self.write_packet(Packet::PingReq(PingReqPacket)).await
    }

    async fn send_disconnect(&mut self) -> Result<(), ClientError> {
        self.write_packet(Packet::Disconnect(Default::default()))
            .await
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
    session: &mut Session<'_>,
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
                            .write_packet(Packet::PubAck(mqtt_codec::v4::PubAckPacket {
                                packet_id,
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
                        .state
                        .qos2_incoming
                        .insert(packet_id, (topic, payload, retain));
                    session
                        .write_packet(Packet::PubRec(mqtt_codec::v4::PubRecPacket {
                            packet_id,
                        }))
                        .await?;
                }
            }
        }
        Packet::PubAck(ack) => {
            if session.state.inflight.remove(ack.packet_id).is_some() {
                let _ = event_tx
                    .send(Event::PubAck {
                        packet_id: ack.packet_id,
                    })
                    .await;
                session.flush_pending().await?;
            }
        }
        Packet::PubRec(rec) => {
            if let Some(entry) = session.state.inflight.get_mut(rec.packet_id) {
                entry.stage = InflightStage::Comp;
                entry.sent_at = Instant::now();
            }
            session
                .write_packet(Packet::PubRel(mqtt_codec::v4::PubRelPacket {
                    packet_id: rec.packet_id,
                }))
                .await?;
        }
        Packet::PubRel(rel) => {
            session
                .write_packet(Packet::PubComp(mqtt_codec::v4::PubCompPacket {
                    packet_id: rel.packet_id,
                }))
                .await?;
            if let Some((topic, payload, retain)) =
                session.state.qos2_incoming.remove(&rel.packet_id)
            {
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
            if session.state.inflight.remove(comp.packet_id).is_some() {
                let _ = event_tx
                    .send(Event::PubComp {
                        packet_id: comp.packet_id,
                    })
                    .await;
                session.flush_pending().await?;
            }
        }
        Packet::SubAck(ack) => {
            let return_codes: Vec<u8> = ack.return_codes.iter().map(|c| c.as_u8()).collect();
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
        Packet::PingResp(_) | Packet::Disconnect(_) => {}
        other => {
            return Err(ClientError::UnexpectedPacket(format!(
                "unexpected packet from broker: {:?}",
                other.packet_type()
            )));
        }
    }
    Ok(())
}

fn build_connect(config: &ClientConfig) -> Result<mqtt_codec::v4::ConnectPacket, ClientError> {
    let mut builder = ConnectPacketBuilder::new(client_id(config))
        .keep_alive(config.keep_alive)
        .clean_session(config.clean_session);

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
