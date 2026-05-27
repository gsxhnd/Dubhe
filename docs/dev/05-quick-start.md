# 快速开始

> 当前仅 `mqtt_codec` 可构建和测试，Broker/API/Web GUI 尚未落地。

## 环境要求

- Rust 1.70+

## 获取代码

```bash
git clone https://github.com/gsxhnd/FlowBroker.git
cd FlowBroker
```

## 构建

```bash
cargo build
```

## 运行测试

```bash
cargo test -p mqtt_codec
```

## 了解更多

- `mqtt_codec/README.md` — Builder、Codec 和校验接口
- `mqtt_codec/examples/` — 使用示例
- [编解码实现](06-codec-implementation.md) — 模块架构与核心 Trait
