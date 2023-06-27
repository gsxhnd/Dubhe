use bytes::{BufMut, Bytes, BytesMut};

use crate::types::EncodeError;
use crate::v3::codec::ConnectAckCode;
use crate::v3::codec::Packet;

pub fn encode_mqtt(packet: Packet, bytes: &mut BytesMut) -> Result<(), EncodeError> {
    println!("{:?}", bytes);
    // bytes.resize(2048, b'0');
    match packet {
        Packet::ConnAck(conn) => {
            // bytes.put_u8
            let len = 2;
            bytes.put_u8(0x20);
            // let count = write_remaining_length(buffer, len)?;
            let mut done = false;
            let mut x = len;
            let mut count = 0;

            while !done {
                let mut byte = (x % 128) as u8;
                x /= 128;
                if x > 0 {
                    byte |= 128;
                }

                bytes.put_u8(byte);
                count += 1;
                done = x == 0;
            }

            bytes.put_u8(conn.session_present as u8);
            bytes.put_u8(connect_code(conn.code));
            Ok(())
        }
        Packet::Connect(conn) => {
            todo!()
        }
    }
}

fn connect_code(return_code: ConnectAckCode) -> u8 {
    match return_code {
        ConnectAckCode::Success => 0,
        ConnectAckCode::RefusedProtocolVersion => 1,
        ConnectAckCode::BadUserNamePassword => 4,
        ConnectAckCode::NotAuthorized => 5,
        _ => unreachable!(),
    }
}
