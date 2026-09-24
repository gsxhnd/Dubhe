# mqtt_client

异步 MQTT 客户端，支持 **MQTT v3.1.1** 与 **MQTT v5.0**，基于 `mqtt_codec`。

## 能力

| 能力 | 说明 |
|------|------|
| TCP / TLS | `TlsOptions`；CLI `--tls` / `--tls-insecure` |
| 自动重连 | 指数退避；`ReconnectOptions`；CLI `--no-reconnect` |
| QoS 0/1/2 | 出站 inflight 窗口、`max_inflight`、超时重传（DUP） |
| 会话 | clean session / clean start；断线后按需重订阅与重传 |
| 认证 / Will | 用户名密码、Last Will |
| 事件 | `Connected` / `Reconnecting` / `Message` / ACK / `Disconnected` |

## 库用法

```rust
use std::time::Duration;
use mqtt_client::{ClientConfig, Event, MqttClient, ProtocolVersion, TlsOptions};

#[tokio::main]
async fn main() {
    let config = ClientConfig::new("broker.emqx.io:8883", "demo")
        .protocol_version(ProtocolVersion::V5)
        .tls(TlsOptions {
            enabled: true,
            insecure_skip_verify: false,
        })
        .max_inflight(16)
        .ack_timeout(Duration::from_secs(30));

    let (client, mut events) = MqttClient::new(config);

    while let Some(event) = events.recv().await {
        match event {
            Event::Connected { session_present } => {
                let _ = client.subscribe(vec![("test/#".into(), 1)]).await;
                let _ = session_present;
            }
            Event::Message { topic, payload, .. } => {
                println!("{topic}: {}", String::from_utf8_lossy(&payload));
            }
            Event::Reconnecting { attempt, delay_ms } => {
                eprintln!("reconnect #{attempt} in {delay_ms}ms");
            }
            _ => {}
        }
    }
}
```

## CLI

```bash
cargo run -p mqtt_client -- pub -t test/topic -m hello -q 1
cargo run -p mqtt_client -- --protocol v5 sub -H 127.0.0.1 -t test/#
cargo run -p mqtt_client -- --tls -H broker.emqx.io -p 8883 --protocol v5 sub -t test/#
```

## 测试

```bash
cargo test -p mqtt_client
cargo clippy -p mqtt_client --all-targets -- -D warnings
```
