use bytes::Bytes;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PacketType {
    // Connect(ConnectPacket),
    // ConnAck(ConnAck, Option<ConnAckProperties>),
    // Publish(Publish, Option<PublishProperties>),
    // PubAck(PubAck, Option<PubAckProperties>),
    // PingReq(PingReq),
    // PingResp(PingResp),
    // Subscribe(Subscribe, Option<SubscribeProperties>),
    // SubAck(SubAck, Option<SubAckProperties>),
    // PubRec(PubRec, Option<PubRecProperties>),
    // PubRel(PubRel, Option<PubRelProperties>),
    // PubComp(PubComp, Option<PubCompProperties>),
    // Unsubscribe(Unsubscribe, Option<UnsubscribeProperties>),
    // UnsubAck(UnsubAck, Option<UnsubAckProperties>),
    // Disconnect(Disconnect, Option<DisconnectProperties>),
    RESERVED,
    CONNECT,
    CONNACK,
    PUBLISH,
    PUBACK,
    PUBREC,
    PUBREL,
    PUBCOMP,
    SUBSCRIBE,
    SUBACK,
    UNSUBSCRIBE,
    UNSUBACK,
    PINGREQ,
    PINGRESP,
    DISCONNECT,
    AUTH,
}

impl From<u8> for PacketType {
    fn from(num: u8) -> Self {
        match num {
            0 => PacketType::RESERVED,
            1 => PacketType::CONNECT,
            2 => PacketType::CONNACK,
            3 => PacketType::PUBLISH,
            4 => PacketType::PUBACK,
            5 => PacketType::PUBREC,
            6 => PacketType::PUBREL,
            7 => PacketType::PUBCOMP,
            8 => PacketType::SUBSCRIBE,
            9 => PacketType::SUBACK,
            10 => PacketType::UNSUBSCRIBE,
            11 => PacketType::UNSUBACK,
            12 => PacketType::PINGREQ,
            13 => PacketType::PINGRESP,
            14 => PacketType::DISCONNECT,
            15 => PacketType::AUTH,
            _ => panic!("{} is out of range", num),
        }
    }
}

/// Quality of service
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd)]
#[allow(clippy::enum_variant_names)]
pub enum QoS {
    #[default]
    AtMostOnce = 0,
    AtLeastOnce = 1,
    ExactlyOnce = 2,
}
