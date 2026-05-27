# 项目概述

> 状态: `mqtt_codec` 已实现，其余模块开发中或规划中。

## 项目简介

FlowBroker 是一个使用 Rust 编写的 MQTT Broker 与管理平台，覆盖协议处理、Broker 服务、API、插件系统、可视化界面和集群能力。

## 目标能力

- 完整的 MQTT v3.1.1 和 v5.0 协议支持
- 高性能、低延迟的消息代理服务
- API、插件和规则引擎扩展点
- 可视化管理界面
- 集群部署和高可用

## 能力状态矩阵

| 能力 | 说明 | 状态 |
|------|------|------|
| MQTT v3.1.1 编解码 | 协议包定义、编码、解码、校验 | 已实现 |
| MQTT v5.0 编解码 | 属性系统、编码、解码、校验 | 已实现 |
| Broker 核心 | TCP 接入、会话、主题、路由 | 开发中 |
| WebSocket | MQTT over WebSocket | 规划中 |
| 持久化 | PostgreSQL/ORM | 规划中 |
| REST API | 管理与监控接口 | 规划中 |
| 插件系统 | 认证、授权、钩子扩展 | 规划中 |
| Web GUI | Vue 3 + Reka UI 管理界面 | 规划中 |
| Raft 集群 | 节点协同与高可用 | 规划中 |

## 当前仓库结构

```text
FlowBroker/
├── crates/
│   ├── mqtt_codec/   # 已实现
│   └── mqtt_client/  # 开发中
├── docs/             # 项目文档
└── Cargo.toml
```

## 目标仓库结构

```text
FlowBroker/
├── crates/
│   ├── mqtt_codec/
│   ├── mqtt_client/
│   ├── flow_broker_core/
│   ├── flow_broker_auth/
│   ├── flow_broker_rules/
│   ├── flow_broker_api/
│   ├── flow_broker_cluster/
│   └── flow_broker_web/
└── docs/
```

## 当前可以做什么

- 运行 `cargo test -p mqtt_codec` 验证编解码实现
- 阅读 `mqtt_codec/README.md` 了解 Builder、Codec 和校验接口
- 参考 [编解码实现](06-codec-implementation.md) 理解代码结构

## 当前还不能做什么

- 启动 `./target/release/flow_broker` Broker
- 调用 REST API、健康检查或监控端点
- 使用 Docker、集群或 Web 管理界面
