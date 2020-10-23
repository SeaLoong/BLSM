# Bilibili直播区协同监控

## 模型结构图

-----------------------------------

![BLSM模型结构图](https://cdn.jsdelivr.net/gh/SeaLoong/BLSM@master/blsm.png)

+ 服务端不直接连接直播间弹幕服务器，只负责转发客户端之间的数据

## 协议规范

-----------------------------------

### 流程(客户端-服务端)

+ **C** 表示客户端
+ **S** 表示服务端

-----------------------------------

1. **C** → **S** 建立WebSocket连接
2. **C** → **S** `表明身份`
3. **S** → **C** `要求间隔`
4. 进入 **任务进行中** 状态

-----------------------------------

#### 任务进行中

以下为可能会出现的数据包和流程

+ **S** → **C** `要求间隔`
+ **C** → **S** `申请任务`
+ **S** → **C** `通知`
+ **C** ←→ **S** `数据报告`
+ 任务改变
    1. **S** → **C** `任务改变`
    2. **C** → **S** `任务确认`

-----------------------------------

### 流程(服务端-服务端) ***未完成***

-----------------------------------

### 定义

-----------------------------------

#### 数据类型

| Name         | Bytes | Notes |
|--------------|:-----:|-------|
| Boolean      | 1     |       |
| Byte         | 1     | [Int8](https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/DataView/getInt8)  |
| Short        | 2     | [Int16](https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/DataView/getInt16) |
| Int          | 4     | [Int32](https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/DataView/getInt32) |
| Float        | 4     | [Float32](https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/DataView/getFloat32) |
| Long         | 8     | [BigInt64](https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/DataView/getBigInt64) |
| Double       | 8     | [Float64](https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/DataView/getFloat64) |
| String       | ≥ 4   | `Int` + n `Byte`s (UTF8) |
| X[]          | n * size of X | 表示 X 数组 |

-----------------------------------

#### 数据包格式

| Field Name | Field Type | Notes |
|------------|------------|-------|
| Packet ID  | Byte       | 数据包编号 |
| Protocol   | Byte       | 协议版本，当前为1 |
| Payload    | ByteArray  | 载荷（数据包体） |

-----------------------------------

#### 表明身份(Protocol = 1)

| From | To | Packet ID |
|:----:|:--:|:---------:|
| Client | Server | 0x01 |
| Admin  | Server | 0x01 |

| Field Name | Field Type | Notes |
| ---------- | ---------- | ----- |
| Identity   | Byte       | 0: 客户端; 1:服务端 2:管理员 |
| Token      | String     | 特定的令牌 |

-----------------------------------

#### 要求间隔(Protocol = 1)

| From | To | Packet ID |
|:----:|:--:|:---------:|
| Server | Client | 0x02 |

| Field Name | Field Type | Notes |
| ---------- | ---------- | ----- |
| Min Interval | Int   | 最小数据包发送间隔，单位为毫秒 |
| Max Interval | Int   | 最大数据包发送间隔，单位为毫秒 |

-----------------------------------

#### 申请任务(Protocol = 1)

| From | To | Packet ID |
|:----:|:--:|:---------:|
| Client | Server | 0x03 |

| Field Name | Field Type | Notes |
| ---------- | ---------- | ----- |
| Room Count | Short      | 可监听的房间数量 |

-----------------------------------

#### 任务改变(Protocol = 1)

| From | To | Packet ID |
|:----:|:--:|:---------:|
| Server | Client | 0x04 |

| Field Name | Field Type | Notes |
| ---------- | ---------- | ----- |
| Type       | Byte       | 0: 增加房间; 1:变更房间 |
| Room Count | Short      | 需要增加/变更的房间数量 |
| Room ID    | Int[]      | 需要增加/变更的房间ID |

-----------------------------------

#### 任务确认(Protocol = 1)

| From | To | Packet ID |
|:----:|:--:|:---------:|
| Client | Server | 0x05 |

| Field Name | Field Type | Notes |
| ---------- | ---------- | ----- |
| Room Count | Short      | 确认监听的房间数量 |
| Room ID    | Int[]      | 确认监听的房间ID |

-----------------------------------

#### 数据报告(Protocol = 1)

| From | To | Packet ID |
|:----:|:--:|:---------:|
| Client | Server | 0x11 |
| Server | Client | 0x12 |

| Field Name | Field Type | Notes |
| ---------- | ---------- | ----- |
| Type       | Byte       | 0:节奏风暴; 1:特殊礼物; 2:天选时刻 |
| Room ID    | Int        | 房间ID |
| ID         | String     | 抽奖ID |
| Time       | Int        | 持续时间 |
| Detail     | String     | 详细信息，JSON |

-----------------------------------

#### 通知

| From | To | Packet ID |
|:----:|:--:|:---------:|
| Client | Server | 0xFF |
| Server | Client | 0xFF |
| Admin  | Server | 0xFF |

| Field Name | Field Type | Notes |
| ---------- | ---------- | ----- |
| Type       | Byte       | 通知类型 |
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

导致封禁的原因

+ 过多的踢出
+ 无效的数据格式
+ 无效的数据包

-----------------------------------

### 踢出

服务端会断开导致踢出的连接，并记录踢出次数和IP

导致踢出的原因

+ 过多的连接
+ 长时间未收到数据包
+ 请求间隔过短
+ 非预期的数据包
+ 通信过程中出现错误
+ 在 `表明身份` 时提供的 `Token` 无效
