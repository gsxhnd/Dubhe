use bytes::BytesMut;
use tokio_util::codec::Decoder;

use mqtt_codec::types::{DecodeError, ProtocolVersion};

pub struct VersionCodec;

impl Decoder for VersionCodec {
    type Item = ProtocolVersion;
    type Error = DecodeError;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        read_version(buf)
    }
}

// #[derive(Debug, Clone, PartialEq)]
// pub struct Header {
//     pub typ: PacketType,
//     pub dup: bool,
//     pub qos: QoS,
//     pub retain: bool,
// }

// impl Header {
//     pub fn new(hd: u8) -> Result<Header, DecodeError> {
//         println!("[header new] hd: {}", hd);
//         let (typ, flags_ok) = match hd >> 4 {
//             1 => (PacketType::CONNECT, hd & 0b1111 == 0),
//             _ => (PacketType::CONNECT, false),
//         };
//         if !flags_ok {
//             return Err(DecodeError::InvalidPacketType);
//         }
//         Ok(Header {
//             typ,
//             dup: hd & 0b1000 != 0,
//             qos: QoS::from_u8((hd & 0b110) >> 1)?,
//             retain: hd & 1 == 1,
//         })
//     }
// }

// pub fn read_header(
//     buf: &mut bytes::BytesMut,
//     offset: &mut usize,
// ) -> Result<Option<(Header, usize)>, DecodeError> {
//     let mut len: usize = 0;
//     for pos in 0..=3 {
//         println!("[read_header] offset: {}", *offset);
//         println!("[read_header] pos: {}", pos);
//         println!("[read_header] buf len: {}", buf.len());
//         println!("[read_header] full buf: {:?}", buf);

//         if buf.len() > *offset + pos + 1 {
//             let byte = buf[*offset + pos + 1];
//             println!("[read_header] byte: {}", byte);

//             len += (byte as usize & 0x7F) << (pos * 7);
//             println!("[read_header] len: {}", len);

//             if (byte & 0x80) == 0 {
//                 // Continuation bit == 0, length is parsed
//                 if buf.len() < *offset + 2 + pos + len {
//                     // Won't be able to read full packet
//                     return Ok(None);
//                 }
//                 println!("[read_header] header offset: {}", *offset);
//                 println!("[read_header] header buf: {:?}", buf);
//                 println!("[read_header] header buf offset 0: {:?}", buf[0]);
//                 println!("[read_header] header buf offset 1: {:?}", buf[1]);
//                 // Parse header byte, skip past the header, and return
//                 let header = Header::new(buf[*offset])?;
//                 *offset += pos + 2;
//                 return Ok(Some((header, len)));
//             }
//         } else {
//             // Couldn't read full length
//             return Ok(None);
//         }
//     }
//     // Continuation byte == 1 four times, that's illegal.
//     Err(DecodeError::InvalidPacketType)
// }

fn read_version(buf: &mut bytes::BytesMut) -> Result<Option<ProtocolVersion>, DecodeError> {
    if &mut buf[4..8] == b"MQTT" {
        let _ = match buf.get(8) {
            Some(4) => Ok(ProtocolVersion::MQTT4),
            Some(5) => Ok(ProtocolVersion::MQTT5),
            Some(_) => Err(DecodeError::InvalidPacketType),
            None => Err(DecodeError::InvalidPacketType),
        };
    } else if &mut buf[4..10] == b"MQIsdp" {
        let _ = match buf.get(10) {
            Some(3) => Ok(ProtocolVersion::MQTT3),
            Some(_) => Err(DecodeError::InvalidPacketType),
            None => Err(DecodeError::InvalidPacketType),
        };
    }
    Err(DecodeError::InvalidPacketType)
}

// pub fn read_packet(
//     header: Header,
//     _remaining_size: usize,
//     buf: &mut bytes::BytesMut,
//     offset: &mut usize,
// ) -> Result<ConnectPacket, DecodeError> {
//     if header.typ != PacketType::CONNECT {
//         return Err(DecodeError::InvalidPacketType);
//     }
//     Ok(ConnectPacket::from_buffer(buf, offset))
// }

#[cfg(test)]
pub fn connect_codec() -> BytesMut {
    use bytes::BufMut;
    // Fixed header
    let mut header: Vec<u8> = vec![
        0b00010000, // 报文类型为 CONNECT，保留位为 0,
        0b00010010, // remaining size is 18 bytes
    ];
    // Variable header
    let mut mut_header: Vec<u8> = vec![
        0b00000100, // 协议名长度为 4 字节 MSB
        0b00000100, // 协议名长度为 4 字节 LSB
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

// #[test]
// fn version_read_header_test() {
//     let mut b = connect_codec();
//     let mut offset = 0;

//     let (header, _reaming_size) = read_header(&mut b, &mut offset)
//         .unwrap()
//         .expect("read header error");

//     println!("header type: {:?}", header.typ)
// }

// #[test]
// fn version_read_packet() {
//     let mut buf = connect_codec();
//     let mut offset = 0;

//     let (header, reaming_size) = read_header(&mut buf, &mut offset)
//         .unwrap()
//         .expect("read header error");

//     let _ = read_packet(header, reaming_size, &mut buf, &mut offset);
// }

#[test]
fn version_read_version() {
    let mut buf = connect_codec();
    let source_buf = buf.clone();
    read_version(&mut buf);
    assert_eq!(source_buf, buf);
    // assert_ne!(source_buf, buf);
}
