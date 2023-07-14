use bytes::{BufMut, BytesMut};
use tokio_util::codec::Decoder;

use crate::types::{DecodeError, PacketType, ProtocolVersion, QoS};

pub struct VersionCodec;

impl Decoder for VersionCodec {
    type Item = Connecet;
    type Error = DecodeError;

    fn decode(&mut self, buf: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let mut offset = 0;
        if let Some((header, remaining_len)) = read_header(buf, &mut offset)? {
            let r: Connecet = read_packet(header, remaining_len, buf, &mut offset)?;
            // Ok(Some(r))
            todo!()
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug, Clone)]
pub struct Connecet {
    pub protocol_version: ProtocolVersion,
}
impl Connecet {
    fn from_buffer(buf: &mut bytes::BytesMut, offset: &mut usize) -> Self {
        todo!()
    }
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
) -> Result<Connecet, DecodeError> {
    Ok(match header.typ {
        PacketType::CONNECT => Connecet::from_buffer(buf, offset),
        _ => {
            todo!()
        }
    })
}

#[cfg(test)]
pub fn connect_codec() -> BytesMut {
    //  header
    let mut header: Vec<u8> = vec![
        0b00010000, // 报文类型为 CONNECT，保留位为 0,
        0b00010010, // remaining size is 18 bytes
    ];
    // header
    let mut mut_header: Vec<u8> = vec![
        0b00000100, // 协议名长度为 4 字节
        0x4D, 0x51, 0x54, 0x54,       // 协议名为 MQTT
        0b00000100, //  协议级别为 4
        0b11000010, //连接标志，表示清理会话、使用密码
        0x00, 0x0A, //# 保持连接时间为 10 秒
    ];
    // payload
    let mut payload: Vec<u8> = vec![
        0b00000100, // # client id length is 4 byte
        0x74, 0x65, 0x73, 0x74,       // # client id is test
        0b00000100, // # password length is 4 byte
        0x70, 0x61, 0x73, 0x73, // password is pass
    ];

    let mut bytes = BytesMut::with_capacity(1024);
    bytes.put_slice(&mut header);
    bytes.put_slice(&mut mut_header);
    bytes.put_slice(&mut payload);
    return bytes;
}

#[test]
fn version_read_header_test() {
    let mut b = connect_codec();
    let mut offset = 0;

    let (header, d) = read_header(&mut b, &mut offset)
        .unwrap()
        .expect("read header error");

    println!("header type: {:?}", header.typ)
}
