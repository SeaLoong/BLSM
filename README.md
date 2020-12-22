# Bilibili直播区协同监控

## 模型结构图

-----------------------------------

![BLSM模型结构图](https://cdn.jsdelivr.net/gh/SeaLoong/BLSM@master/blsm.png)

+ 服务端不直接连接直播间弹幕服务器，只负责转发客户端之间的数据

## 协议规范

-----------------------------------

### 流程(客户端-服务端) ***未完成***

+ **C** 表示客户端
+ **S** 表示服务端

客户端在收到一次来自服务端的数据包后，如果服务端要求回应，则应该在允许发送数据包的情况下立即回应，必须在服务端接下来两次的间隔计时结束前保证回应的数据包已被服务端收到。

客户端在短暂地断开后，重新连接时可以不再重新`申请任务`，直接`确认任务`(先前被分配的任务)即可。服务端决定客户端的此次`确认任务`是否有效，若无效将会进行`改变任务`流程

-----------------------------------

#### 握手(Handshaking)

1. **C** → **S** 建立WebSocket连接
2. **C** → **S** `表明身份`
3. **S** → **C** `速率限制`
4.  
   + 若先前未分配过任务
      1. **C** → **S** `任务申请`
      2. 进行一次 **任务改变** 流程
   + 若先前已分配过任务
      1. **C** → **S** `任务确认`
5. 进入 **工作中** 状态

-----------------------------------

#### 工作中(Working)

以下为可能会出现的数据包和流程

+ **S** → **C** `速率限制`
+ **C** → **S** `任务申请`
+ **C** → **S** `任务确认`
+ **S** → **C** `通知`
+ **C** ←→ **S** `数据报告`
+ 任务改变
    1. **S** → **C** `任务改变`
    2. **C** → **S** `任务确认`

-----------------------------------

### 流程(服务端-服务端) ***未完成***

-----------------------------------

### 定义(Definition)

-----------------------------------

#### 数据类型(Data Type)

| Name         | Bytes | Notes |
|--------------|:-----:|-------|
| Bool         | 1     | 0: false; 1: true |
| Uint8        | 1     | [Uint8](https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/DataView/getUint8)  |
| Int8        | 1     | [Int8](https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/DataView/getInt8)  |
| Uint16       | 2     | [Uint16](https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/DataView/getUint16) |
| Int16       | 2     | [Int16](https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/DataView/getInt16) |
| Uint32       | 4     | [Uint32](https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/DataView/getUint32) |
| Int32       | 4     | [Int32](https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/DataView/getInt32) |
| Uint64       | 8     | [BigUint64](https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/DataView/getBigUint64) |
| Int64       | 8     | [BigInt64](https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/DataView/getBigInt64) |
| Float32      | 4     | [Float32](https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/DataView/getFloat32) |
| Float64      | 8     | [Float64](https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/DataView/getFloat64) |
| VarInt       | 1 ~ 5 | 有效取值与 `Uint32` 一致 |
| String       | ≥ 1   | `VarInt` + n Bytes (UTF8) |
| X[]          | n * size of X | 表示 X 数组 |

-----------------------------------

#### 数据包格式(Packet Format)

| Field Name | Field Type | Notes |
|------------|------------|-------|
| Length     | VarInt     | 数据包体的长度（Length of Data） |
| ID         | VarInt     | 数据包编号 |
| Data       | Byte[]     | 数据包体 |

+ 允许以数据包为单位，合并多个数据包一次发送

-----------------------------------

#### 表明身份(Show Identity)

| From | To | Packet ID |
|:----:|:--:|:---------:|
| Client | Server | 0x01 |

| Field Name | Field Type | Notes |
| ---------- | ---------- | ----- |
| Category   | VarInt     | 1: 客户端; 2:服务端 3:管理员 |
| Token      | String     | 特定的字符串 |

-----------------------------------

#### 速率限制(Rate Limit)

| From | To | Packet ID |
|:----:|:--:|:---------:|
| Server | Client | 0x02 |

| Field Name | Field Type | Notes |
| ---------- | ---------- | ----- |
| Interval   | VarInt     | 补充间隔，每间隔这么多时间(毫秒)补充一个令牌 |
| Max Burst  | VarInt     | 最大令牌数 |

服务端使用 [**Generic Cell Rate Algorithm**](https://en.wikipedia.org/wiki/Generic_cell_rate_algorithm) 来控制速率，客户端在处理上可以当做 **Token Bucket** 算法来实现

`Interval` * 2 的时间间隔作为服务端要求回应时，响应的超时时长

`Interval` * `Max Burst` 的时间间隔作为心跳超时时长，任意数据包的到达都会重置心跳计时器

-----------------------------------

#### 任务申请(Task Application)

| From | To | Packet ID |
|:----:|:--:|:---------:|
| Client | Server | 0x03 |

| Field Name | Field Type | Notes |
| ---------- | ---------- | ----- |
| Room Count | VarInt     | 可监听的房间数量 |

在长时间无其他数据包可发送时，可发送 `Room Count` 为 0 的此数据包来重置心跳计时器

-----------------------------------

#### 任务改变(Task Change)

| From | To | Packet ID |
|:----:|:--:|:---------:|
| Server | Client | 0x04 |

| Field Name | Field Type | Notes |
| ---------- | ---------- | ----- |
| Room Count | VarInt     | 变更后的房间数量 |
| Room ID    | String[]   | 变更后的房间ID |

-----------------------------------

#### 任务确认(Task Confirm)

| From | To | Packet ID |
|:----:|:--:|:---------:|
| Client | Server | 0x05 |

| Field Name | Field Type | Notes |
| ---------- | ---------- | ----- |
| Room Count | VarInt     | 确认监听的房间数量 |
| Room ID    | String[]   | 确认监听的房间ID |

-----------------------------------

#### 数据报告(Data Report)

| From | To | Packet ID |
|:----:|:--:|:---------:|
| Server/Client | Client/Server | 0x06 |

| Field Name | Field Type | Notes |
| ---------- | ---------- | ----- |
| Category   | VarInt     | 1:节奏风暴; 2:特殊礼物; 3:天选时刻 |
| Room ID    | String     | 房间ID |
| ID         | String     | 抽奖ID |
| Time       | VarInt     | 持续时间 |
| Detail     | String     | 详细信息，JSON |

-----------------------------------

#### 通知(Notification)

| From | To | Packet ID |
|:----:|:--:|:---------:|
| Server/Client | Client/Server | 0xFF |

| Field Name | Field Type | Notes |
| ---------- | ---------- | ----- |
| Category   | VarInt     | 通知类型 |
| Message    | String     | 消息内容 |
| Token      | String     | 发送通知所需的令牌 |

-----------------------------------

## 安全机制

安全机制表现为服务端对客户端的不恰当请求时做出的反应，分为以下几个等级:

+ **封禁**
+ **踢出**

-----------------------------------

### 封禁

服务端会直接拒绝处于封禁状态的IP发起的连接，并断开该IP的所有现有连接

导致封禁的原因（Close事件错误码固定为1008）

+ 过多的踢出
+ 在 `表明身份` 时提供的 `Token` 无效

-----------------------------------

### 踢出

服务端会断开导致踢出的连接，并记录踢出次数和IP

导致踢出的原因（Close事件错误码）

+ 心跳超时（4000）
+ 响应超时（4001）
+ 速率限制（4002）
+ 过多的连接（4004）
+ 未被允许的数据包（4005）
+ 无效的数据包（4006）
+ 不正确的数据格式（4007）
