use crate::types::DecodeError;
use crate::v3::packet::*;

use bytes::BytesMut;

pub fn decode_mqtt(bytes: &mut BytesMut) -> Result<Option<Packet>, DecodeError> {
    if bytes.is_empty() {
        return Ok(None);
    }
    // let packet_header = read_header(bytes,&mut 0);

    let p = Packet::Connect(ConnectPacket {
        protocol_name: "".to_string(),
        // protocol_version: ProtocolVersion::MQTT3,
        keep_alive: 0b1000_00000000,
        // user_name_flag: false,
        // password_flag: false,
        // will_retain: true,
        // will_qos: true,
        // will_flag: true,
        clean_start: true,
        client_id: "".to_string(),
        user_name: None,
        password: None,
    });
    bytes.clear();

    Ok(Some(p))
}

// #[derive(Debug, Clone, PartialEq)]
pub struct Header {
    // pub typ: PacketType,
    // pub dup: bool,
    // pub qos: QoS,
    // pub retain: bool,
}

impl Header {
    pub fn new(_hd: u8) -> Result<Header, DecodeError> {
        todo!()
        // println!("[header new] hd: {}", hd);
        // let (typ, flags_ok) = match hd >> 4 {
        //     1 => (PacketType::CONNECT, hd & 0b1111 == 0),
        //     _ => (PacketType::CONNECT, false),
        // };
        // if !flags_ok {
        //     return Err(DecodeError::InvalidPacketType);
        // }
        // Ok(Header {
        //     typ,
        //     dup: hd & 0b1000 != 0,
        //     qos: QoS::from_u8((hd & 0b110) >> 1)?,
        //     retain: hd & 1 == 1,
        // })
    }
}

pub fn read_header(
    buf: &mut BytesMut,
    offset: &mut usize,
) -> Result<Option<(Header, usize)>, DecodeError> {
    let mut len: usize = 0;
    for pos in 0..=3 {
        println!("[read_header] offset: {}", *offset);
        println!("[read_header] pos: {}", pos);
        println!("[read_header] buf len: {}", buf.len());
        println!("[read_header] full buf: {:?}", buf);

        if buf.len() > *offset + pos + 1 {
            let byte = buf[*offset + pos + 1];
            println!("[read_header] byte: {}", byte);

            len += (byte as usize & 0x7F) << (pos * 7);
            println!("[read_header] len: {}", len);

            if (byte & 0x80) == 0 {
                // Continuation bit == 0, length is parsed
                if buf.len() < *offset + 2 + pos + len {
                    // Won't be able to read full packet
                    return Ok(None);
                }
                println!("[read_header] header offset: {}", *offset);
                println!("[read_header] header buf: {:?}", buf);
                println!("[read_header] header buf offset 0: {:?}", buf[0]);
                println!("[read_header] header buf offset 1: {:?}", buf[1]);
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

// pub fn read_packet(
//     header: Header,
//     remaining_size: usize,
//     buf: &mut bytes::BytesMut,
//     offset: &mut usize,
// ) -> Result<ConnectPacket, DecodeError> {
//     if header.typ != PacketType::CONNECT {
//         return Err(DecodeError::InvalidPacketType);
//     }
//     Ok(ConnectPacket::from_buffer(buf, offset))
// }

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
