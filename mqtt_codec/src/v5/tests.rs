#[cfg(test)]
mod tests {
    use crate::v5::codec::MqttCodec;
    use crate::v5::packet::*;
    use crate::{Decoder, Encoder};
    use bytes::BytesMut;

    #[test]
    fn test_connect_packet_encoding() {
        let mut codec = MqttCodec::new();
        let mut dst = BytesMut::new();

        let packet = Packet::Connect(ConnectPacket {
            protocol_name: "MQTT".to_string(),
            protocol_level: 5,
            clean_start: true,
            will_flag: false,
            will_qos: QoS::AtMostOnce,
            will_retain: false,
            password_flag: false,
            username_flag: false,
            keep_alive: 60,
            properties: Properties::default(),
            client_id: "test-client".to_string(),
            will_topic: None,
            will_message: None,
            will_properties: None,
            username: None,
            password: None,
        });

        let result = codec.encode(packet, &mut dst);
        assert!(result.is_ok());
        assert!(!dst.is_empty());
    }

    #[test]
    fn test_connack_packet_encoding() {
        let mut codec = MqttCodec::new();
        let mut dst = BytesMut::new();

        let packet = Packet::ConnAck(ConnAckPacket {
            session_present: false,
            reason_code: ReasonCode::Success,
            properties: Properties::default(),
        });

        let result = codec.encode(packet, &mut dst);
        assert!(result.is_ok());
        assert!(!dst.is_empty());
    }

    #[test]
    fn test_publish_packet_encoding() {
        let mut codec = MqttCodec::new();
        let mut dst = BytesMut::new();

        let packet = Packet::Publish(PublishPacket {
            topic_name: "test/topic".to_string(),
            packet_id: Some(1),
            payload: bytes::Bytes::from("Hello MQTT 5.0"),
            qos: QoS::AtLeastOnce,
            duplicate: false,
            retain: false,
            properties: Properties::default(),
        });

        let result = codec.encode(packet, &mut dst);
        assert!(result.is_ok());
        assert!(!dst.is_empty());
    }

    #[test]
    fn test_subscribe_packet_encoding() {
        let mut codec = MqttCodec::new();
        let mut dst = BytesMut::new();

        let packet = Packet::Subscribe(SubscribePacket {
            packet_id: 1,
            properties: Properties::default(),
            topics: vec![SubscriptionOption {
                topic_filter: "test/+".to_string(),
                qos: QoS::AtMostOnce,
                no_local: false,
                retain_as_published: false,
                retain_handling: 0,
            }],
        });

        let result = codec.encode(packet, &mut dst);
        assert!(result.is_ok());
        assert!(!dst.is_empty());
    }

    // =========================================================================
    // Encoding and Decoding Tests
    // =========================================================================

    #[test]
    fn test_connect_packet_roundtrip() {
        let mut codec = MqttCodec::new();
        let mut buffer = BytesMut::new();

        let packet = Packet::Connect(ConnectPacket {
            protocol_name: "MQTT".to_string(),
            protocol_level: 5,
            clean_start: true,
            will_flag: false,
            will_qos: QoS::AtMostOnce,
            will_retain: false,
            password_flag: false,
            username_flag: false,
            keep_alive: 60,
            properties: Properties {
                session_expiry_interval: Some(3600),
                receive_maximum: Some(100),
                maximum_packet_size: Some(65535),
                ..Default::default()
            },
            client_id: "test-client".to_string(),
            will_topic: None,
            will_message: None,
            will_properties: None,
            username: None,
            password: None,
        });

        codec.encode(packet.clone(), &mut buffer).unwrap();
        let decoded = codec.decode(&mut buffer).unwrap().unwrap();

        match (packet, decoded) {
            (Packet::Connect(orig), Packet::Connect(dec)) => {
                assert_eq!(orig.client_id, dec.client_id);
                assert_eq!(
                    orig.properties.session_expiry_interval,
                    dec.properties.session_expiry_interval
                );
                assert_eq!(
                    orig.properties.receive_maximum,
                    dec.properties.receive_maximum
                );
            }
            _ => panic!("Packet mismatch"),
        }
    }

    #[test]
    fn test_connect_packet_with_auth_roundtrip() {
        let mut codec = MqttCodec::new();
        let mut buffer = BytesMut::new();

        let packet = Packet::Connect(ConnectPacket {
            protocol_name: "MQTT".to_string(),
            protocol_level: 5,
            clean_start: true,
            will_flag: false,
            will_qos: QoS::AtMostOnce,
            will_retain: false,
            password_flag: true,
            username_flag: true,
            keep_alive: 60,
            properties: Properties {
                authentication_method: Some("SCRAM-SHA-256".to_string()),
                authentication_data: Some(bytes::Bytes::from("auth-data")),
                ..Default::default()
            },
            client_id: "auth-client".to_string(),
            will_topic: None,
            will_message: None,
            will_properties: None,
            username: Some("user".to_string()),
            password: Some(bytes::Bytes::from("pass")),
        });

        codec.encode(packet.clone(), &mut buffer).unwrap();
        let decoded = codec.decode(&mut buffer).unwrap().unwrap();

        match (packet, decoded) {
            (Packet::Connect(orig), Packet::Connect(dec)) => {
                assert_eq!(orig.client_id, dec.client_id);
                assert_eq!(orig.username, dec.username);
                assert_eq!(
                    orig.properties.authentication_method,
                    dec.properties.authentication_method
                );
            }
            _ => panic!("Packet mismatch"),
        }
    }

    #[test]
    fn test_connack_packet_roundtrip() {
        let mut codec = MqttCodec::new();
        let mut buffer = BytesMut::new();

        let packet = Packet::ConnAck(ConnAckPacket {
            session_present: true,
            reason_code: ReasonCode::Success,
            properties: Properties {
                assigned_client_identifier: Some("assigned-id".to_string()),
                server_keep_alive: Some(300),
                maximum_qos: Some(QoS::ExactlyOnce),
                retain_available: Some(true),
                ..Default::default()
            },
        });

        codec.encode(packet.clone(), &mut buffer).unwrap();
        let decoded = codec.decode(&mut buffer).unwrap().unwrap();

        match (packet, decoded) {
            (Packet::ConnAck(orig), Packet::ConnAck(dec)) => {
                assert_eq!(orig.session_present, dec.session_present);
                assert_eq!(orig.reason_code, dec.reason_code);
                assert_eq!(
                    orig.properties.assigned_client_identifier,
                    dec.properties.assigned_client_identifier
                );
                assert_eq!(orig.properties.maximum_qos, dec.properties.maximum_qos);
            }
            _ => panic!("Packet mismatch"),
        }
    }

    #[test]
    fn test_publish_packet_qos0_roundtrip() {
        let mut codec = MqttCodec::new();
        let mut buffer = BytesMut::new();

        let packet = Packet::Publish(PublishPacket {
            topic_name: "test/qos0".to_string(),
            packet_id: None,
            payload: bytes::Bytes::from("QoS 0 message"),
            qos: QoS::AtMostOnce,
            duplicate: false,
            retain: true,
            properties: Properties {
                content_type: Some("text/plain".to_string()),
                ..Default::default()
            },
        });

        codec.encode(packet.clone(), &mut buffer).unwrap();
        let decoded = codec.decode(&mut buffer).unwrap().unwrap();

        match (packet, decoded) {
            (Packet::Publish(orig), Packet::Publish(dec)) => {
                assert_eq!(orig.topic_name, dec.topic_name);
                assert_eq!(orig.payload, dec.payload);
                assert_eq!(orig.qos, dec.qos);
                assert_eq!(orig.retain, dec.retain);
                assert_eq!(orig.properties.content_type, dec.properties.content_type);
            }
            _ => panic!("Packet mismatch"),
        }
    }

    #[test]
    fn test_publish_packet_qos1_roundtrip() {
        let mut codec = MqttCodec::new();
        let mut buffer = BytesMut::new();

        let packet = Packet::Publish(PublishPacket {
            topic_name: "test/qos1".to_string(),
            packet_id: Some(100),
            payload: bytes::Bytes::from("QoS 1 message"),
            qos: QoS::AtLeastOnce,
            duplicate: false,
            retain: false,
            properties: Properties {
                message_expiry_interval: Some(300),
                response_topic: Some("response/topic".to_string()),
                correlation_data: Some(bytes::Bytes::from("corr-id")),
                ..Default::default()
            },
        });

        codec.encode(packet.clone(), &mut buffer).unwrap();
        let decoded = codec.decode(&mut buffer).unwrap().unwrap();

        match (packet, decoded) {
            (Packet::Publish(orig), Packet::Publish(dec)) => {
                assert_eq!(orig.topic_name, dec.topic_name);
                assert_eq!(orig.packet_id, dec.packet_id);
                assert_eq!(orig.payload, dec.payload);
                assert_eq!(orig.qos, dec.qos);
                assert_eq!(
                    orig.properties.message_expiry_interval,
                    dec.properties.message_expiry_interval
                );
                assert_eq!(
                    orig.properties.response_topic,
                    dec.properties.response_topic
                );
            }
            _ => panic!("Packet mismatch"),
        }
    }

    #[test]
    fn test_publish_packet_with_user_properties_roundtrip() {
        let mut codec = MqttCodec::new();
        let mut buffer = BytesMut::new();

        let user_props = vec![
            ("device-id".to_string(), "dev-001".to_string()),
            ("location".to_string(), "room-101".to_string()),
        ];

        let packet = Packet::Publish(PublishPacket {
            topic_name: "sensor/data".to_string(),
            packet_id: Some(1),
            payload: bytes::Bytes::from(r#"{"temp": 23.5}"#),
            qos: QoS::AtLeastOnce,
            duplicate: false,
            retain: false,
            properties: Properties {
                content_type: Some("application/json".to_string()),
                user_properties: user_props,
                ..Default::default()
            },
        });

        codec.encode(packet.clone(), &mut buffer).unwrap();
        let decoded = codec.decode(&mut buffer).unwrap().unwrap();

        match (packet, decoded) {
            (Packet::Publish(orig), Packet::Publish(dec)) => {
                assert_eq!(orig.properties.content_type, dec.properties.content_type);
                assert_eq!(
                    orig.properties.user_properties.len(),
                    dec.properties.user_properties.len()
                );
                // Verify all user properties are present
                for prop in &orig.properties.user_properties {
                    assert!(dec.properties.user_properties.contains(prop));
                }
            }
            _ => panic!("Packet mismatch"),
        }
    }

    #[test]
    fn test_puback_packet_roundtrip() {
        let mut codec = MqttCodec::new();
        let mut buffer = BytesMut::new();

        let packet = Packet::PubAck(PubAckPacket {
            packet_id: 50,
            reason_code: ReasonCode::Success,
            properties: Properties {
                reason_string: Some("Success".to_string()),
                ..Default::default()
            },
        });

        codec.encode(packet.clone(), &mut buffer).unwrap();
        let decoded = codec.decode(&mut buffer).unwrap().unwrap();

        match (packet, decoded) {
            (Packet::PubAck(orig), Packet::PubAck(dec)) => {
                assert_eq!(orig.packet_id, dec.packet_id);
                assert_eq!(orig.reason_code, dec.reason_code);
            }
            _ => panic!("Packet mismatch"),
        }
    }

    #[test]
    fn test_pubrec_packet_roundtrip() {
        let mut codec = MqttCodec::new();
        let mut buffer = BytesMut::new();

        let packet = Packet::PubRec(PubRecPacket {
            packet_id: 100,
            reason_code: ReasonCode::Success,
            properties: Properties::default(),
        });

        codec.encode(packet.clone(), &mut buffer).unwrap();
        let decoded = codec.decode(&mut buffer).unwrap().unwrap();

        match (packet, decoded) {
            (Packet::PubRec(orig), Packet::PubRec(dec)) => {
                assert_eq!(orig.packet_id, dec.packet_id);
                assert_eq!(orig.reason_code, dec.reason_code);
            }
            _ => panic!("Packet mismatch"),
        }
    }

    #[test]
    fn test_pubrel_packet_roundtrip() {
        let mut codec = MqttCodec::new();
        let mut buffer = BytesMut::new();

        let packet = Packet::PubRel(PubRelPacket {
            packet_id: 100,
            reason_code: ReasonCode::Success,
            properties: Properties::default(),
        });

        codec.encode(packet.clone(), &mut buffer).unwrap();
        let decoded = codec.decode(&mut buffer).unwrap().unwrap();

        match (packet, decoded) {
            (Packet::PubRel(orig), Packet::PubRel(dec)) => {
                assert_eq!(orig.packet_id, dec.packet_id);
                assert_eq!(orig.reason_code, dec.reason_code);
            }
            _ => panic!("Packet mismatch"),
        }
    }

    #[test]
    fn test_pubcomp_packet_roundtrip() {
        let mut codec = MqttCodec::new();
        let mut buffer = BytesMut::new();

        let packet = Packet::PubComp(PubCompPacket {
            packet_id: 100,
            reason_code: ReasonCode::Success,
            properties: Properties::default(),
        });

        codec.encode(packet.clone(), &mut buffer).unwrap();
        let decoded = codec.decode(&mut buffer).unwrap().unwrap();

        match (packet, decoded) {
            (Packet::PubComp(orig), Packet::PubComp(dec)) => {
                assert_eq!(orig.packet_id, dec.packet_id);
                assert_eq!(orig.reason_code, dec.reason_code);
            }
            _ => panic!("Packet mismatch"),
        }
    }

    #[test]
    fn test_subscribe_packet_roundtrip() {
        let mut codec = MqttCodec::new();
        let mut buffer = BytesMut::new();

        let packet = Packet::Subscribe(SubscribePacket {
            packet_id: 1,
            properties: Properties {
                subscription_identifiers: vec![42],
                ..Default::default()
            },
            topics: vec![
                SubscriptionOption {
                    topic_filter: "home/+/temperature".to_string(),
                    qos: QoS::AtMostOnce,
                    no_local: true,
                    retain_as_published: false,
                    retain_handling: 0,
                },
                SubscriptionOption {
                    topic_filter: "home/+/humidity".to_string(),
                    qos: QoS::AtLeastOnce,
                    no_local: false,
                    retain_as_published: true,
                    retain_handling: 1,
                },
            ],
        });

        codec.encode(packet.clone(), &mut buffer).unwrap();
        let decoded = codec.decode(&mut buffer).unwrap().unwrap();

        match (packet, decoded) {
            (Packet::Subscribe(orig), Packet::Subscribe(dec)) => {
                assert_eq!(orig.packet_id, dec.packet_id);
                assert_eq!(orig.topics.len(), dec.topics.len());
                assert_eq!(orig.topics[0].topic_filter, dec.topics[0].topic_filter);
                assert_eq!(orig.topics[0].no_local, dec.topics[0].no_local);
                assert_eq!(
                    orig.topics[1].retain_handling,
                    dec.topics[1].retain_handling
                );
            }
            _ => panic!("Packet mismatch"),
        }
    }

    #[test]
    fn test_suback_packet_roundtrip() {
        let mut codec = MqttCodec::new();
        let mut buffer = BytesMut::new();

        let packet = Packet::SubAck(SubAckPacket {
            packet_id: 1,
            properties: Properties::default(),
            reason_codes: vec![ReasonCode::GrantedQoS1, ReasonCode::NotAuthorized],
        });

        codec.encode(packet.clone(), &mut buffer).unwrap();
        let decoded = codec.decode(&mut buffer).unwrap().unwrap();

        match (packet, decoded) {
            (Packet::SubAck(orig), Packet::SubAck(dec)) => {
                assert_eq!(orig.packet_id, dec.packet_id);
                assert_eq!(orig.reason_codes.len(), dec.reason_codes.len());
                assert_eq!(orig.reason_codes[0], dec.reason_codes[0]);
                assert_eq!(orig.reason_codes[1], dec.reason_codes[1]);
            }
            _ => panic!("Packet mismatch"),
        }
    }

    #[test]
    fn test_unsubscribe_packet_roundtrip() {
        let mut codec = MqttCodec::new();
        let mut buffer = BytesMut::new();

        let packet = Packet::Unsubscribe(UnsubscribePacket {
            packet_id: 5,
            properties: Properties::default(),
            topics: vec![
                "home/+/temperature".to_string(),
                "home/+/humidity".to_string(),
            ],
        });

        codec.encode(packet.clone(), &mut buffer).unwrap();
        let decoded = codec.decode(&mut buffer).unwrap().unwrap();

        match (packet, decoded) {
            (Packet::Unsubscribe(orig), Packet::Unsubscribe(dec)) => {
                assert_eq!(orig.packet_id, dec.packet_id);
                assert_eq!(orig.topics, dec.topics);
            }
            _ => panic!("Packet mismatch"),
        }
    }

    #[test]
    fn test_unsuback_packet_roundtrip() {
        let mut codec = MqttCodec::new();
        let mut buffer = BytesMut::new();

        let packet = Packet::UnsubAck(UnsubAckPacket {
            packet_id: 5,
            properties: Properties::default(),
            reason_codes: vec![ReasonCode::Success, ReasonCode::NoMatchingSubscribers],
        });

        codec.encode(packet.clone(), &mut buffer).unwrap();
        let decoded = codec.decode(&mut buffer).unwrap().unwrap();

        match (packet, decoded) {
            (Packet::UnsubAck(orig), Packet::UnsubAck(dec)) => {
                assert_eq!(orig.packet_id, dec.packet_id);
                assert_eq!(orig.reason_codes, dec.reason_codes);
            }
            _ => panic!("Packet mismatch"),
        }
    }

    #[test]
    fn test_pingreq_packet_roundtrip() {
        let mut codec = MqttCodec::new();
        let mut buffer = BytesMut::new();

        let packet = Packet::PingReq(PingReqPacket);

        codec.encode(packet.clone(), &mut buffer).unwrap();
        assert_eq!(buffer.len(), 2);

        let decoded = codec.decode(&mut buffer).unwrap().unwrap();
        match (packet, decoded) {
            (Packet::PingReq(_), Packet::PingReq(_)) => {}
            _ => panic!("Packet mismatch"),
        }
    }

    #[test]
    fn test_pingresp_packet_roundtrip() {
        let mut codec = MqttCodec::new();
        let mut buffer = BytesMut::new();

        let packet = Packet::PingResp(PingRespPacket);

        codec.encode(packet.clone(), &mut buffer).unwrap();
        assert_eq!(buffer.len(), 2);

        let decoded = codec.decode(&mut buffer).unwrap().unwrap();
        match (packet, decoded) {
            (Packet::PingResp(_), Packet::PingResp(_)) => {}
            _ => panic!("Packet mismatch"),
        }
    }

    #[test]
    fn test_disconnect_packet_roundtrip() {
        let mut codec = MqttCodec::new();
        let mut buffer = BytesMut::new();

        let packet = Packet::Disconnect(DisconnectPacket {
            reason_code: ReasonCode::Success,
            properties: Properties {
                reason_string: Some("Client shutdown".to_string()),
                session_expiry_interval: Some(0),
                ..Default::default()
            },
        });

        codec.encode(packet.clone(), &mut buffer).unwrap();
        let decoded = codec.decode(&mut buffer).unwrap().unwrap();

        match (packet, decoded) {
            (Packet::Disconnect(orig), Packet::Disconnect(dec)) => {
                assert_eq!(orig.reason_code, dec.reason_code);
                assert_eq!(orig.properties.reason_string, dec.properties.reason_string);
            }
            _ => panic!("Packet mismatch"),
        }
    }

    #[test]
    fn test_auth_packet_roundtrip() {
        let mut codec = MqttCodec::new();
        let mut buffer = BytesMut::new();

        let packet = Packet::Auth(AuthPacket {
            reason_code: ReasonCode::ContinueAuthentication,
            properties: Properties {
                authentication_method: Some("SCRAM-SHA-256".to_string()),
                authentication_data: Some(bytes::Bytes::from("challenge")),
                ..Default::default()
            },
        });

        codec.encode(packet.clone(), &mut buffer).unwrap();
        let decoded = codec.decode(&mut buffer).unwrap().unwrap();

        match (packet, decoded) {
            (Packet::Auth(orig), Packet::Auth(dec)) => {
                assert_eq!(orig.reason_code, dec.reason_code);
                assert_eq!(
                    orig.properties.authentication_method,
                    dec.properties.authentication_method
                );
            }
            _ => panic!("Packet mismatch"),
        }
    }

    #[test]
    fn test_disconnect_with_server_reference() {
        let mut codec = MqttCodec::new();
        let mut buffer = BytesMut::new();

        let packet = Packet::Disconnect(DisconnectPacket {
            reason_code: ReasonCode::UseAnotherServer,
            properties: Properties {
                server_reference: Some("backup.mqtt.server".to_string()),
                reason_string: Some("Server overloaded".to_string()),
                ..Default::default()
            },
        });

        codec.encode(packet.clone(), &mut buffer).unwrap();
        let decoded = codec.decode(&mut buffer).unwrap().unwrap();

        match (packet, decoded) {
            (Packet::Disconnect(orig), Packet::Disconnect(dec)) => {
                assert_eq!(orig.reason_code, dec.reason_code);
                assert_eq!(
                    orig.properties.server_reference,
                    dec.properties.server_reference
                );
            }
            _ => panic!("Packet mismatch"),
        }
    }

    #[test]
    fn test_multiple_packets_sequence() {
        let mut codec = MqttCodec::new();
        let mut buffer = BytesMut::new();

        let packets = vec![
            Packet::Connect(ConnectPacket {
                protocol_name: "MQTT".to_string(),
                protocol_level: 5,
                clean_start: true,
                will_flag: false,
                will_qos: QoS::AtMostOnce,
                will_retain: false,
                password_flag: false,
                username_flag: false,
                keep_alive: 60,
                properties: Properties::default(),
                client_id: "multi-test".to_string(),
                will_topic: None,
                will_message: None,
                will_properties: None,
                username: None,
                password: None,
            }),
            Packet::ConnAck(ConnAckPacket {
                session_present: false,
                reason_code: ReasonCode::Success,
                properties: Properties::default(),
            }),
            Packet::Subscribe(SubscribePacket {
                packet_id: 1,
                properties: Properties::default(),
                topics: vec![SubscriptionOption {
                    topic_filter: "test/#".to_string(),
                    qos: QoS::AtMostOnce,
                    no_local: false,
                    retain_as_published: false,
                    retain_handling: 0,
                }],
            }),
            Packet::SubAck(SubAckPacket {
                packet_id: 1,
                properties: Properties::default(),
                reason_codes: vec![ReasonCode::Success],
            }),
        ];

        for packet in &packets {
            codec.encode(packet.clone(), &mut buffer).unwrap();
        }

        for expected in packets {
            let decoded = codec.decode(&mut buffer).unwrap().unwrap();
            assert_eq!(expected.packet_type(), decoded.packet_type());
        }

        assert!(buffer.is_empty());
    }

    #[test]
    fn test_publish_empty_topic_with_topic_alias() {
        let mut codec = MqttCodec::new();
        let mut buffer = BytesMut::new();

        let packet = Packet::Publish(PublishPacket {
            topic_name: String::new(),
            packet_id: None,
            payload: bytes::Bytes::from("payload"),
            qos: QoS::AtMostOnce,
            duplicate: false,
            retain: false,
            properties: Properties {
                topic_alias: Some(1),
                ..Default::default()
            },
        });

        codec.encode(packet.clone(), &mut buffer).unwrap();
        let decoded = codec.decode(&mut buffer).unwrap().unwrap();

        match (packet, decoded) {
            (Packet::Publish(orig), Packet::Publish(dec)) => {
                assert!(orig.topic_name.is_empty());
                assert_eq!(dec.properties.topic_alias, Some(1));
            }
            _ => panic!("Packet mismatch"),
        }
    }

    #[test]
    fn test_publish_qos0_rejects_packet_id_on_encode() {
        let mut codec = MqttCodec::new();
        let mut buffer = BytesMut::new();

        let packet = Packet::Publish(PublishPacket {
            topic_name: "t".to_string(),
            packet_id: Some(1),
            payload: bytes::Bytes::new(),
            qos: QoS::AtMostOnce,
            duplicate: false,
            retain: false,
            properties: Properties::default(),
        });

        assert!(codec.encode(packet, &mut buffer).is_err());
    }

    #[test]
    fn test_topic_alias_property() {
        let mut codec = MqttCodec::new();
        let mut buffer = BytesMut::new();

        let packet = Packet::Publish(PublishPacket {
            topic_name: "test/topic".to_string(),
            packet_id: Some(1),
            payload: bytes::Bytes::from("data"),
            qos: QoS::AtLeastOnce,
            duplicate: false,
            retain: false,
            properties: Properties {
                topic_alias: Some(5),
                ..Default::default()
            },
        });

        codec.encode(packet.clone(), &mut buffer).unwrap();
        let decoded = codec.decode(&mut buffer).unwrap().unwrap();

        match (packet, decoded) {
            (Packet::Publish(orig), Packet::Publish(dec)) => {
                assert_eq!(orig.properties.topic_alias, dec.properties.topic_alias);
            }
            _ => panic!("Packet mismatch"),
        }
    }
}
