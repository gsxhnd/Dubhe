use crate::v5::packet::PacketType;

#[derive(Debug, Clone, PartialEq)]
pub struct FixedHeader {
    // pub packet_type: PacketType,
    // pub dup: bool,
    // pub qos: QoS,
    // pub retain: bool,
}

impl FixedHeader {
    pub fn new() -> FixedHeader {
        FixedHeader {}
    }

    // pub from() {}
}
