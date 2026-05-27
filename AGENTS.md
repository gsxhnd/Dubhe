# AGENTS.md

Focused guidance for AI coding agents working in this repository.

**Goal:** understand crate boundaries, run/build/test, and follow project-specific patterns.

## Project

FlowBroker — Rust MQTT broker (early stage). The workspace currently contains **`mqtt_codec`** (MQTT v3.1.1 + v5.0 control-packet encode/decode with spec validation) and **`mqtt_client`** (async client, in progress). The runnable broker (`flow_broker`), MQTT server (`flow_broker_mqtt`), Raft clustering, and API layers are planned but not yet in this repo (see [docs/dev/](docs/dev/) and **Planned architecture** below).

## Current workspace layout

```
Cargo.toml          # workspace root (resolver = "2"); members = ["crates/*"]
crates/
  mqtt_codec/       # canonical MQTT codec library
    src/lib.rs      # Encoder/Decoder traits, MqttError, v4/v5 re-exports
    src/v4/         # MQTT 3.1.1 (protocol level 4)
    src/v5/         # MQTT 5.0 (protocol level 5)
    src/error.rs    # MqttError (thiserror)
    tests/          # integration tests (v4/, v5/ subdirectories)
    examples/       # v4_encode, v4_builder, v5_encode, v5_builder
  mqtt_client/      # async MQTT client library
    src/lib.rs      # public API re-exports
    src/client.rs   # MqttClient handle + event loop
    src/config.rs   # ClientConfig, Credentials, LastWill
    src/error.rs    # ClientError
    src/event.rs    # Event enum
    src/transport.rs # TCP transport layer
docs/dev/           # design docs (broker, cluster, API — aspirational)
```

## Commands

| Task | Command |
|------|---------|
| Build | `cargo build --workspace` |
| Test all | `cargo test --workspace` |
| Test codec | `cargo test -p mqtt_codec` |
| Lint | `cargo clippy --all-targets --all-features --locked -- -D warnings` |
| Format | `cargo fmt` |
| Run example | `cargo run -p mqtt_codec --example v4_encode` |

Use `-p mqtt_codec` or `-p mqtt_client` to scope work to a specific crate.

## Conventions

- Rust edition 2024; dependencies: `bytes` (with serde), `thiserror`, `tokio`
- Integration tests live under each crate's `tests/` directory (not inline `#[cfg(test)]`)
- **`mqtt_codec` is the canonical codec crate** — do not add logic to any legacy `mqtt-codec` name
- Never use `unwrap()`/`expect()` outside tests
- Use `#[expect(clippy::lint)]` over `#[allow(...)]` with justification
- Every `TODO` needs a linked issue: `// TODO(#42): ...`
- Prefer `&T` over `.clone()` unless ownership transfer is required
- Name tests descriptively: `process_should_return_error_when_input_empty()`
- Documentation and README are written in Chinese (Simplified)

## Key files (`mqtt_codec`)

| Area | Path |
|------|------|
| Traits | [crates/mqtt_codec/src/lib.rs](crates/mqtt_codec/src/lib.rs) |
| v3.1.1 codec | [crates/mqtt_codec/src/v4/codec.rs](crates/mqtt_codec/src/v4/codec.rs), `encoder.rs`, `decoder.rs`, `validation.rs` |
| v5.0 codec | [crates/mqtt_codec/src/v5/codec.rs](crates/mqtt_codec/src/v5/codec.rs), `properties_codec.rs`, `validation.rs` |
| Packet types | [crates/mqtt_codec/src/v4/packet.rs](crates/mqtt_codec/src/v4/packet.rs), [crates/mqtt_codec/src/v5/packet.rs](crates/mqtt_codec/src/v5/packet.rs) |
| Builders | [crates/mqtt_codec/src/v4/builder.rs](crates/mqtt_codec/src/v4/builder.rs), [crates/mqtt_codec/src/v5/builder.rs](crates/mqtt_codec/src/v5/builder.rs) |

## Codec design

- `Encoder<T>` and `Decoder` in `mqtt_codec/src/lib.rs` follow `tokio_util::codec` patterns (`Framed<S, C>`).
- `v4` and `v5` are independent module trees: packet types, builders, codecs, validation.
- **Encode** and **decode** both call `validate_packet()` — protocol MUST rules are enforced on the wire path.
- v5 adds Properties, Reason Codes, and AUTH; property parsing lives in `v5/properties_codec.rs`.
- Builder pattern for constructing packets (see `examples/`).

## When editing code

**Protocol codec changes**

- Implement v3.1.1 vs v5.0 in `mqtt_codec/src/v4/` and `mqtt_codec/src/v5/` separately; keep APIs parallel where sensible.
- After changing validation or wire format, run `cargo test -p mqtt_codec`.
- Errors use `MqttError` at the crate root (not per-module codec error aliases).

**Testing**

- Integration tests live under `crates/mqtt_codec/tests/` (and per-crate `tests/` for other members).
- Add future broker tests under `crates/flow_broker/tests/` when that crate lands.

**Common pitfalls**

- Don't assume other workspace crates exist beyond `mqtt_codec` and `mqtt_client` today.
- Codec validates packets; it does **not** implement sessions, QoS state machines, or broker routing.
- `Maximum packet size` is enforced only on the v5 `MqttCodec` wrapper when configured.

## Planned architecture (not in repo yet)

Target multi-crate layout from project design docs:

| Component | Role |
|-----------|------|
| `flow_broker` | Binary entry: CLI (`--config`), YAML config, `tokio` runtime, spawns services |
| `flow_broker_mqtt` | TCP/TLS/WebSocket listeners; `VersionCodec` on CONNECT; swap v4/v5 codec per connection |
| `mqtt_codec` | **Implemented** — packet encode/decode |
| `flow_broker_raft` | Raft node/server; protobufs via `tonic-build` |

**Planned workflow (when crates land)**

- Install `protoc` for `tonic-build` (`brew install protobuf` on macOS).
- Run app: `cargo run -p flow_broker -- --config conf/test.yaml`
- Example configs: `conf/test.yaml`, `conf/cluster/` (not present until broker crate exists).
- `VersionCodec` reads protocol name + level only; full CONNECT validation happens after codec selection.

**Planned networking patterns**

- `tokio` for all I/O; one multi-thread runtime in app, services as spawned tasks.
- Per-connection handler after protocol version is detected from CONNECT.

## CI

- Workflow: [.github/workflows/test.yml](.github/workflows/test.yml) — checkout on push to `master`.
- Extend with `cargo test --workspace` and `cargo clippy` when CI hardening is desired.
- Future `tonic-build` / `protoc` steps apply only after gRPC/Raft crates are added.

## See also

- [.agents/skills/rust-best-practices/SKILL.md](.agents/skills/rust-best-practices/SKILL.md) — Rust coding conventions
- [docs/dev/README.md](docs/dev/README.md) — full product/design documentation
