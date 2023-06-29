use crate::types::DecodeError;
use crate::types::{PacketType, QoS};

#[derive(Debug, Clone)]
pub struct Connecet {}
impl Connecet {
    fn from_buffer() {}
}

#[derive(Debug, Clone)]
pub struct Header {
    pub typ: PacketType,
    pub dup: bool,
    pub qos: QoS,
    pub retain: bool,
}

impl Header {
    pub fn new(hd: u8) -> Result<Header, DecodeError> {
        let (typ, flags_ok) = match hd >> 4 {
            1 => (PacketType::CONNECT, hd & 0b1111 == 0),
            // 2 => (PacketType::Connack, hd & 0b1111 == 0),
            // 3 => (PacketType::Publish, true),
            // 4 => (PacketType::Puback, hd & 0b1111 == 0),
            // 5 => (PacketType::Pubrec, hd & 0b1111 == 0),
            // 6 => (PacketType::Pubrel, hd & 0b1111 == 0b0010),
            // 7 => (PacketType::Pubcomp, hd & 0b1111 == 0),
            // 8 => (PacketType::Subscribe, hd & 0b1111 == 0b0010),
            // 9 => (PacketType::Suback, hd & 0b1111 == 0),
            // 10 => (PacketType::Unsubscribe, hd & 0b1111 == 0b0010),
            // 11 => (PacketType::Unsuback, hd & 0b1111 == 0),
            // 12 => (PacketType::Pingreq, hd & 0b1111 == 0),
            // 13 => (PacketType::Pingresp, hd & 0b1111 == 0),
            // 14 => (PacketType::Disconnect, hd & 0b1111 == 0),
            _ => (PacketType::CONNECT, false),
        };
        if !flags_ok {
            return Err(DecodeError::InvalidPacketType);
        }
        Ok(Header {
            typ,
            dup: hd & 0b1000 != 0,
            qos: QoS::from_u8((hd & 0b110) >> 1)?,
            retain: hd & 1 == 1,
        })
    }
}

pub fn read_header(
    buf: &mut bytes::BytesMut,
    offset: &mut usize,
) -> Result<Option<(Header, usize)>, DecodeError> {
    let mut len: usize = 0;
    for pos in 0..=3 {
        if buf.len() > *offset + pos + 1 {
            let byte = buf[*offset + pos + 1];
            len += (byte as usize & 0x7F) << (pos * 7);
            if (byte & 0x80) == 0 {
                // Continuation bit == 0, length is parsed
                if buf.len() < *offset + 2 + pos + len {
                    // Won't be able to read full packet
                    return Ok(None);
                }
                // Parse header byte, skip past the header, and return
                let header = Header::new(buf[*offset])?;
                *offset += pos + 2;
                return Ok(Some((header, len)));
            }
        } else {
            // Couldn't read full length
            return Ok(None);
        }
    }
    // Continuation byte == 1 four times, that's illegal.
    Err(DecodeError::InvalidPacketType)
}

pub fn read_packet(
    header: Header,
    remaining_len: usize,
    buf: &mut bytes::BytesMut,
    offset: &mut usize,
) -> Result<PacketType, DecodeError> {
    Ok(match header.typ {
        PacketType::CONNECT => Connect::from_buffer(buf, offset)?.into(),
        // PacketType::Disconnect => Packet::Disconnect,
        // PacketType::Pingreq => Packet::Pingreq,
        // PacketType::Pingresp => Packet::Pingresp,
        // PacketType::Connack => Connack::from_buffer(buf, offset)?.into(),
        // PacketType::Publish => Publish::from_buffer(&header, remaining_len, buf, offset)?.into(),
        // PacketType::Puback => Packet::Puback(Pid::from_buffer(buf, offset)?),
        // PacketType::Pubrec => Packet::Pubrec(Pid::from_buffer(buf, offset)?),
        // PacketType::Pubrel => Packet::Pubrel(Pid::from_buffer(buf, offset)?),
        // PacketType::Pubcomp => Packet::Pubcomp(Pid::from_buffer(buf, offset)?),
        // PacketType::Subscribe => Subscribe::from_buffer(remaining_len, buf, offset)?.into(),
        // PacketType::Suback => Suback::from_buffer(remaining_len, buf, offset)?.into(),
        // PacketType::Unsubscribe => Unsubscribe::from_buffer(remaining_len, buf, offset)?.into(),
        // PacketType::Unsuback => Packet::Unsuback(Pid::from_buffer(buf, offset)?),
        _ => {
            todo!()
        }
    })
}
