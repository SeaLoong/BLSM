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

#### 握手

1. **C** → **S** 建立WebSocket连接
2. **C** → **S** `表明身份`
3. **S** → **C** `要求验证`
4. **C** 进行验证操作
5. **C** → **S** `心跳`
6. **S** 进行验证检查
   + 验证成功
     1. **S** → **C** `要求间隔`
     2. **C** → **S** `心跳`
   + 验证失败
     1. **S** 断开连接
7. 握手完成

-----------------------------------

#### 任务进行中

以下为可能会出现的数据包

+ **S** → **C** `任务信息`(停止监听)
+ **C** → **S** `心跳`
+ **C** ←→ **S** `数据报告`
+ **S** → **C** `通知`

+ 监听任务增加
    1. **S** → **C** `任务信息`(请求监听)
    2. **C** → **S** `任务信息`(正在监听)

+ 请求间隔改变
    1. **S** → **C** `要求请求间隔`
    2. **C** → **S** `心跳`

+ 要求数据报告
    1. **S** → **C** `要求数据报告`
    2. **C** → **S** `数据报告`

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
| Long         | 8     | [BigInt64](https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/DataView/getBigInt64) |
| Float        | 4     | [Float32](https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/DataView/getFloat32) |
| Double       | 8     | [Float64](https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/DataView/getFloat64) |
| String       | ≥ 4   | `Int` + n `Byte`s (UTF8) |
| X[]          | n * size of X | 表示 X 数组 |

-----------------------------------

#### 数据包格式

| Field Name | Field Type | Notes |
|------------|------------|-------|
| Packet ID  | Byte       | 数据包编号 |
| Protocol   | Byte       | 协议版本，当前为1 |
| Sequence   | Int        | 顺序编号，为之前发送过的数据包的个数 |
| Payload    | ByteArray  | 载荷（数据包体） |

-----------------------------------

#### 表明身份(Protocol = 1)

| From | To | Packet ID |
|:----:|:--:|:---------:|
| Client | Server | 0x00 |
| Admin  | Server | 0x00 |

| Field Name | Field Type | Notes |
| ---------- | ---------- | ----- |
| Identity   | Byte       | 0: 客户端; 1:服务端 2:管理员 |
| Token      | String     | 特定的令牌 |

-----------------------------------

#### 心跳(Protocol = 1)

| From | To | Packet ID |
|:----:|:--:|:---------:|
| Client | Server | 0x01 |

| Field Name | Field Type | Notes |
| ---------- | ---------- | ----- |
| Room Count | Short      | 可多监听的房间数量，如果大于0，服务端可以分配新任务 |

-----------------------------------

#### 任务信息C(Protocol = 1)

| From | To | Packet ID |
|:----:|:--:|:---------:|
| Client | Server | 0x02 |

| Field Name | Field Type | Notes |
| ---------- | ---------- | ----- |
| Room Count | Short      | 房间数量 |
| Room ID    | Int[]      | 房间ID |

-----------------------------------

#### 要求验证(Protocol = 1)

| From | To | Packet ID |
|:----:|:--:|:---------:|
| Server | Client | 0x80 |

| Field Name | Field Type | Notes |
| ---------- | ---------- | ----- |
| Data       | String     | 相关数据，JSON |

-----------------------------------

#### 要求间隔(Protocol = 1)

| From | To | Packet ID |
|:----:|:--:|:---------:|
| Server | Client | 0x81 |

| Field Name | Field Type | Notes |
| ---------- | ---------- | ----- |
| Min Interval | Int   | 最小数据包发送间隔，单位为毫秒 |
| Max Interval | Int   | 最大数据包发送间隔，单位为毫秒 |

+ 服务端检查间隔是以服务端收到数据包的时刻为准

-----------------------------------

#### 任务信息S(Protocol = 1)

| From | To | Packet ID |
|:----:|:--:|:---------:|
| Server | Client | 0x82 |

| Field Name | Field Type | Notes |
| ---------- | ---------- | ----- |
| Type       | Byte       | 0: 请求监听; 1:停止监听 |
| Room Count | Short      | 房间数量 |
| Room ID    | Int[]      | 房间ID |

-----------------------------------

#### 数据报告(Protocol = 1)

| From | To | Packet ID |
|:----:|:--:|:---------:|
| Client | Server | 0x03 |
| Server | Client | 0x83 |

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
| Client | Server | 0x04 |
| Server | Client | 0x84 |
| Admin  | Server | 0x04 |

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
+ 过多的丢包
+ 请求间隔过短
+ 非预期的数据包
+ 通信过程中出现错误
+ 在 `表明身份` 时提供的 `Token` 无效
