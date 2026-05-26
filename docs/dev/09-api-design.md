# API 设计

> 状态: 设计草案。当前仓库尚未提供 REST API 或 gRPC 实现。

## 设计目标

- 为 Broker 提供管理、监控和调试入口
- 保持资源命名、分页和错误响应风格一致
- 为 Web GUI 和自动化运维提供稳定接口

## REST API

### Base URL

```
/api/v1
```

### 路由结构

```text
/api/v1
├── /clients
├── /topics
├── /subscriptions
├── /rules
└── /system

/metrics    (Prometheus, 根路径)
```

### 认证

- 方案: Bearer Token
- 预留扩展: API Key 或管理用户会话

### 通用响应格式

成功:

```json
{
  "data": {},
  "error": null,
  "request_id": "optional"
}
```

错误:

```json
{
  "data": null,
  "error": {
    "code": "not_found",
    "message": "resource not found"
  }
}
```

### 错误码

| 错误码 | 描述 |
|-------|------|
| 400 | 请求参数错误 |
| 401 | 未授权 |
| 404 | 资源不存在 |
| 500 | 服务器内部错误 |

## 接口详情

### 客户端管理

```http
GET    /clients              # 列表 (支持 page, page_size, connected 参数)
GET    /clients/{client_id}  # 详情
DELETE /clients/{client_id}  # 断开连接
```

响应示例:

```json
{
  "total": 100,
  "page": 1,
  "page_size": 20,
  "data": [
    {
      "client_id": "client001",
      "protocol_version": 4,
      "ip_address": "192.168.1.100",
      "connected": true,
      "connected_at": "2026-03-23T10:00:00Z"
    }
  ]
}
```

### 主题管理

```http
GET  /topics                    # 主题列表
POST /topics/{topic}/publish    # 发布消息
```

发布请求体:

```json
{
  "payload": "Hello World",
  "qos": 1,
  "retain": false
}
```

### 系统监控

```http
GET /system/status    # 系统状态
GET /metrics          # Prometheus 指标 (根路径)
```

状态响应:

```json
{
  "uptime": 3600,
  "connections": 1000,
  "messages_received": 50000,
  "messages_sent": 48000
}
```

### 规则管理 (v0.5.0+)

```http
GET    /rules
POST   /rules
PUT    /rules/{rule_id}
DELETE /rules/{rule_id}
```

## gRPC (集群节点间通信)

```protobuf
service ClusterService {
  rpc SyncState(StateRequest) returns (StateResponse);
  rpc ForwardMessage(MessageRequest) returns (MessageResponse);
}
```

## 未决问题

- 管理 API 是否与数据平面完全隔离
- 认证与用户系统是否在同一版本交付
- 健康检查是独立端点还是并入 `/system`
