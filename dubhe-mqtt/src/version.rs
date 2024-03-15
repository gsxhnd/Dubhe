use bytes::{Buf, BytesMut};
use tokio_util::codec::Decoder;

use mqtt_codec::types::{DecodeError, ProtocolVersion, MQISDP, MQTT};

pub struct VersionCodec;

impl Decoder for VersionCodec {
    type Item = ProtocolVersion;
    type Error = DecodeError;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        read_version(buf)
    }
}

fn read_version(buf: &mut bytes::BytesMut) -> Result<Option<ProtocolVersion>, DecodeError> {
    if &mut buf[4..10] == MQISDP {
        return match buf.get(10) {
            Some(3) => Ok(Some(ProtocolVersion::MQTT3)),
            Some(_) => Err(DecodeError::InvalidPacketType),
            None => Err(DecodeError::InvalidPacketType),
        };
    } else if &mut buf[4..8] == MQTT {
        return match buf.get(8) {
            Some(4) => Ok(Some(ProtocolVersion::MQTT4)),
            Some(5) => Ok(Some(ProtocolVersion::MQTT5)),
            Some(_) => Err(DecodeError::InvalidPacketType),
            None => Err(DecodeError::InvalidPacketType),
        };
    }
    Err(DecodeError::BadTransport)
}

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
        0b00000100, // 协议级别为 4
        0b11000010, // 连接标志，表示清理会话、使用密码
        0x00, 0x0A, // 保持连接时间为 10 秒
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
fn version_read_version() {
    let mut buf = connect_codec();
    let source_buf = buf.clone();
    let _ = match read_version(&mut buf) {
        Ok(Some(v)) => {
            println!("protocol version: {:?}", v);
        }
        Ok(None) => {}
        Err(e) => {
            println!("protocol version error: {:?}", e);
        }
    };
    assert_eq!(source_buf, buf);
}
