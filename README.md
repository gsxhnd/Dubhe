# Dubhe

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Dubhe 是一个使用 Rust 编写的 MQTT 项目，目标是逐步演进为完整的 Broker、API、插件系统和管理界面。

## 当前状态

当前仓库已经落地的核心内容是 `mqtt_codec`:

- 已实现: MQTT v3.1.1 编解码
- 已实现: MQTT v5.0 编解码
- 已实现: 基础校验、Builder、测试与示例
- 规划中: Broker、REST API、Web GUI、Docker 与集群能力

这意味着当前仓库不能直接启动 `dubhe` Broker 服务。

## 快速开始

```bash
git clone https://github.com/gsxhnd/Dubhe.git
cd Dubhe
cargo build
cargo test -p mqtt_codec
```

更详细的说明见 `docs/current-state.md` 和 `docs/06-user-guide/quick-start.md`。

## 文档入口

- [文档索引](docs/README.md)
- [当前状态](docs/current-state.md)
- [快速开始](docs/06-user-guide/quick-start.md)
- [路线图](docs/01-planning/roadmap.md)
- [协议编解码实现](docs/03-development/codec-implementation.md)

## 当前项目结构

```text
Dubhe/
├── mqtt_codec/   # 当前已实现模块
├── docs/         # 项目文档
└── Cargo.toml    # workspace 配置
```

## 目标技术方向

- 后端: Rust、Tokio、Axum、Tonic、SeaORM、PostgreSQL
- 前端: Vue 3、Reka UI、Pinia、Vite

## 贡献

欢迎贡献，开始前建议先阅读 `docs/03-development/contributing.md`。

## License

本项目采用 [MIT License](LICENSE) 授权。

## 相关链接

- [MQTT 3.1.1 规范](https://docs.oasis-open.org/mqtt/mqtt/v3.1.1/mqtt-v3.1.1.html)
- [MQTT 5.0 规范](https://docs.oasis-open.org/mqtt/mqtt/v5.0/mqtt-v5.0.html)
