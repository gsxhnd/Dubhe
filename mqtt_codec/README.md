# MQTT 编解码库 (mqtt_codec)

[![Crates.io](https://img.shields.io/crates/v/mqtt_codec.svg)](https://crates.io/crates/mqtt_codec)
[![Documentation](https://docs.rs/mqtt_codec/badge.svg)](https://docs.rs/mqtt_codec)

一个高性能、异步友好的 MQTT 协议编解码库，支持 MQTT v3.1.1 和 v5.0。

## 核心特性

- **双版本支持**：MQTT v3.1.1 (Level 4) 与 MQTT v5.0 (Level 5) 全部 14/15 种控制报文的编解码。
- **异步友好**：基于 `bytes` 库构建，完美集成 `tokio-util` 的 `Framed` 机制。
- **类型安全**：利用 Rust 的强类型系统和 `enum` 特性，确保报文结构的合法性。
- **严格验证**：编解码路径自动执行协议校验（主题/过滤器、QoS 与 Packet ID、Fixed Header 保留位、v5 属性作用域与 Reason Code 等）。
- **构建器模式**：为复杂的报文（如 v5.0 CONNECT）提供流畅的构建器接口。
- **详细错误处理**：基于 `thiserror` 提供结构化的错误类型，方便定位协议违规或解析异常。

## 模块架构

该项目采用模块化设计，清晰划分了协议版本：

- `v4/`：MQTT v3.1.1 协议实现。
  - `packet.rs`：报文结构定义。
  - `codec.rs`：统一的编解码器接口。
  - `decoder.rs` / `encoder.rs`：底层解析与序列化逻辑。
  - `validation.rs`：协议一致性校验。
- `v5/`：MQTT v5.0 协议实现。
  - 包含与 v4 类似的结构，并额外支持 **属性 (Properties)** 和 **原因码 (Reason Codes)**。
- `error.rs`：全局错误类型定义。
- `lib.rs`：导出核心 Trait 和常用类型。

## 快速开始

### 依赖配置

在 `Cargo.toml` 中添加：

```toml
[dependencies]
mqtt_codec = "0.2"
bytes = "1.0"
```

### 使用示例 (MQTT v3.1.1)

```rust
use mqtt_codec::v4::{MqttCodec, Packet, ConnectPacket};
use mqtt_codec::{Encoder, Decoder};
use bytes::BytesMut;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut codec = MqttCodec::new();
    let mut buffer = BytesMut::new();

    // 1. 编码 CONNECT 报文
    let connect = ConnectPacket {
        client_id: "example-client".to_string(),
        keep_alive: 60,
        clean_session: true,
        ..Default::default()
    };
    codec.encode(Packet::Connect(connect), &mut buffer)?;

    // 2. 解码报文
    if let Some(packet) = codec.decode(&mut buffer)? {
        match packet {
            Packet::Connect(p) => println!("收到连接请求: {:?}", p.client_id),
            _ => println!("收到其他报文"),
        }
    }

    Ok(())
}
```

## 性能注意事项

1. **缓冲区重用**：建议重用 `BytesMut` 缓冲区以减少内存分配开销。
2. **零拷贝解析**：在可能的情况下，库会尝试减少不必要的数据复制，但在解析 UTF-8 字符串时会进行必要的转换。
3. **验证开销**：编码时会自动执行严格验证。在高性能场景下，请确保报文数据在构建阶段即符合规范，以避免运行时错误。

## 单元测试

本项目包含详尽的单元测试，覆盖了：

- 各种控制报文的正向与逆向解析。
- 剩余长度 (Remaining Length) 的变长编码处理。
- 边界条件（如空 Payload、最大长度限制）的验证。
- 协议违规场景的错误触发。

可以通过以下命令运行测试：

```bash
cargo test
```

## 许可证

本项目采用 MIT 许可证。
