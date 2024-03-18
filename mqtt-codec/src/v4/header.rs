use bytes::{Buf, BufMut, BytesMut};

use crate::types::DecodeError;
use crate::v4::packet::PacketType;

#[derive(Debug, Clone, PartialEq)]
pub struct Header {
    pub packet_type: PacketType,
    // pub dup: bool,
    // pub qos: QoS,
    // pub retain: bool,
}

impl Header {
    pub fn new(hd: u8) -> Result<Header, DecodeError> {
        println!("[header new] hd: {}", hd);
        let packet_type: PacketType = PacketType::from(hd >> 4);

        // TODO: some packet type has flags
        // if !flags_ok {
        //     return Err(DecodeError::InvalidPacketType);
        // }

        // Ok(Header {
        //     typ,
        //     dup: hd & 0b1000 != 0,
        //     qos: QoS::from_u8((hd & 0b110) >> 1)?,
        //     retain: hd & 1 == 1,
        // })
        println!("first byte: {} {:?}", hd, packet_type);
        Ok(Header { packet_type })
    }

    pub fn read_header(buf: &mut BytesMut, offset: &mut usize) {}

    pub fn from(buffer: &mut BytesMut) -> Result<(Header, usize), DecodeError> {
        let first_byte = buffer.get_u8();
        let packet_type: PacketType = PacketType::from(first_byte >> 4);

        // TODO: some packet type has flags
        // if !flags_ok {
        //     return Err(DecodeError::InvalidPacketType);
        // }

        // Ok(Header {
        //     typ,
        //     dup: hd & 0b1000 != 0,
        //     qos: QoS::from_u8((hd & 0b110) >> 1)?,
        //     retain: hd & 1 == 1,
        // })

        println!(
            "first byte: {} packet type:{:?}, buffer len: {} ",
            first_byte,
            packet_type,
            buffer.len()
        );
        let remeaning_size = buffer.get_u8();
        println!("first byte: {:?},len: {}", remeaning_size, buffer.len());

        let mut remaining_length = 0;
        let mut multiplier = 1;

        for &byte in buffer.iter() {
            remaining_length += (byte & 0x7F) as usize * multiplier;
            multiplier *= 128;

            if byte & 0x80 == 0 {
                break;
            }
        }

        println!(
            "remaining size: {}, buffer len: {}",
            remaining_length,
            buffer.len()
        );

        Ok((Header { packet_type }, 1))
    }
}

#[test]
fn decode_fixed_header_general_test() {
    for expected_packet_type in [PacketType::PINGREQ, PacketType::PINGRESP].iter() {
        let mut buffer = BytesMut::from(&[(*expected_packet_type as u8) << 4, 0x02][..]);
        println!("before header from {:?}", buffer);
        Header::from(&mut buffer);
        println!("after header from {:?}", buffer);

        let mut test = BytesMut::from("aaaaaaaa");
        println!("{:?}", test.len());
        test.get_u8();
        println!("{:?}", test.len());

        // let (packet_type, publish_config, remaining_length) =
        // decode_fixed_header(&mut buffer);
        // assert_eq!(packet_type, *expected_packet_type);
        // assert_eq!(publish_config.is_none(), true);
        // assert_eq!(remaining_length, 2);
    }
}

#[test]
fn test_codec() {
    // msg len 6 msg: 313233
    let data: Vec<u8> = vec![0x30, 0x06, 0x00, 0x01, 0x31, 0x31, 0x32, 0x33];
    let mut buffer = BytesMut::with_capacity(8);
    for v in data.iter() {
        buffer.put_u8(*v);
    }

    println!("BytesMut 数据：{:?}", buffer);

    Header::from(&mut buffer);
}
