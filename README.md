# FlowBroker

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

FlowBroker 是一个使用 Rust 编写的 MQTT 项目，目标是逐步演进为完整的 Broker、API、插件系统和管理界面。

## 当前状态

当前仓库已经落地的核心库包括：

| Crate | 说明 |
|-------|------|
| `mqtt_codec` | MQTT v3.1.1 / v5.0 控制报文编解码与协议校验 |
| `mqtt_client` | 异步 MQTT 客户端（开发中） |

- 已实现：`mqtt_codec` 编解码、校验、Builder、集成测试与示例
- 规划中：Broker 主程序、REST API、Web GUI、Docker 与集群能力

这意味着当前仓库不能直接启动 `flow_broker` Broker 服务。

## 快速开始

```bash
git clone https://github.com/gsxhnd/FlowBroker.git
cd FlowBroker
cargo build --workspace
cargo test -p mqtt_codec
```

更详细的说明见 [docs/dev/README.md](docs/dev/README.md) 与 [crates/mqtt_codec/README.md](crates/mqtt_codec/README.md)。

## 文档入口

- [开发文档索引](docs/dev/README.md)
- [项目概述](docs/dev/01-project-overview.md)
- [系统架构](docs/dev/03-architecture.md)
- [快速开始](docs/dev/05-quick-start.md)

## 当前项目结构

```text
FlowBroker/
├── crates/
│   ├── mqtt_codec/   # MQTT 编解码库
│   └── mqtt_client/  # MQTT 客户端库
├── docs/dev/         # 设计与开发文档
└── Cargo.toml        # workspace 配置
```

## 目标技术方向

- 后端: Rust、Tokio、Axum、Tonic、SeaORM、PostgreSQL
- 前端: Vue 3、Reka UI、Pinia、Vite

## 贡献

欢迎贡献，开始前建议先阅读开发文档。

## License

本项目采用 [MIT License](LICENSE) 授权。

## 相关链接

- [MQTT 3.1.1 规范](https://docs.oasis-open.org/mqtt/mqtt/v3.1.1/mqtt-v3.1.1.html)
- [MQTT 5.0 规范](https://docs.oasis-open.org/mqtt/mqtt/v5.0/mqtt-v5.0.html)
