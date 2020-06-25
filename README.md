# Bilibili直播区协同监控

## 模型结构图

-----------------------------------



## 协议规范

-----------------------------------

+ `要求连接转移` 数据包可以在任意时刻出现，接收到的一段需要放弃当前连接。转而连接指定的服务端。

### 流程(客户端-服务端)

-----------------------------------

+ **C** 表示客户端
+ **S** 表示服务端

#### 握手

-----------------------------------

1. **C** → **S** 建立WebSocket连接
2. **C** → **S** `登录`
3. **S** → **C** `要求请求间隔`
4. **C** → **S** `心跳`


#### 任务进行中

-----------------------------------

以下为可能会出现的数据包


+ **S** → **C** `连接转移`
+ **S** → **C** `任务信息`(停止监听)
+ **C** → **S** `心跳`
+ **C** ←→ **S** `数据报告`
+ **C** ←→ **S** `通知`


+ 监听任务增加
    1. **S** → **C** `任务信息`(请求监听)
    2. **C** → **S** `任务信息`(正在监听)


+ 请求间隔改变
    1. **S** → **C** `要求请求间隔`
    2. **C** → **S** `心跳`


+ 要求数据报告
    1. **S** → **C** `要求数据报告`
    2. **C** → **S** `数据报告`



### 流程(服务端-服务端)

-----------------------------------

服务端与服务端之间连接的过程，需要建立双向 C-S 连接

+ **C1** 表示主动连接的服务端的客户端
+ **C2** 表示被连接服务端的客户端
+ **S1** 表示主动连接的服务端
+ **S2** 表示被连接服务端
+ **C** 表示通过客户端发送数据包
+ **S** 表示通过服务端发送数据包


#### 握手

-----------------------------------

握手过程与客户端的握手过程基本一致：


1. **C1** → **S2** 建立WebSocket连接
2. **C1** → **S2** `登录`
3. **C2** → **S1** 建立WebSocket连接
4. **C2** → **S1** `登录`
5. **S2** → **C1** `要求请求间隔`
6. **C1** → **S2** `心跳`
5. **S1** → **C2** `要求请求间隔`
6. **C2** → **S1** `心跳`


+ 反向 C-S 连接在第 **2** 步完成后由 ***被连接*** 的服务端进行


+ 这两个连接中任意一个如果出现断开，另一边的连接也应当断开

#### 任务进行中

-----------------------------------

以下为可能会出现的数据包


+ **S** → **C** `连接转移`
+ **S** → **C** `任务信息`(停止监听)
+ **C** → **S** `心跳`
+ **C** → **S** `数据报告`
+ **C** → **S** `通知`


+ 监听任务增加
    1. **S** → **C** `任务信息`(请求监听)
    2. **C** → **S** `任务信息`(正在监听)


+ 请求间隔改变
    1. **S** → **C** `要求请求间隔`
    2. **C** → **S** `心跳`


+ 要求数据报告
    1. **S** → **C** `要求数据报告`
    2. **C** → **S** `数据报告`


+ `任务信息`(正在监听) 的任务信息只需要 `任务信息`(请求监听) 包含的任务信息即可


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
| Double       | 8     | [Float64](https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/DataView/getFloat64) |
| String       | ≥ 1   | VarInt + n Bytes (UTF8) |
| VarInt       | ≥ 1 , ≤ 5 | [VarInts](http://developers.google.com/protocol-buffers/docs/encoding#varints) |
| X[]          | n * size of X | 表示X数组 |
| X Enum       | size of X | 表示X枚举 |

#### VarInt

+ 参考文档: [VarInts](http://developers.google.com/protocol-buffers/docs/encoding#varints)
+ 在读写实现上的代码(伪)
    ```
    public static int readVarInt() {
        int numRead = 0;
        int result = 0;
        byte read;
        do {
            read = readByte();
            int value = (read & 0b01111111);
            result |= (value << (7 * numRead));
    
            numRead++;
            if (numRead > 5) {
                throw new RuntimeException("VarInt is too big");
            }
        } while ((read & 0b10000000) != 0);
        return result;
    }
    ```
    ```
    public static void writeVarInt(int value) {
      do {
          byte temp = (byte)(value & 0b01111111);
          // Note: >>> means that the sign bit is shifted with the rest of the number rather than being left alone
          value >>>= 7;
          if (value != 0) {
              temp |= 0b10000000;
          }
          writeByte(temp);
      } while (value != 0);
    }
    ```

#### 数据包格式

| Field Name | Field Type | Notes |
|------------|------------|-------|
| Length     | Int        | 数据包ID+数据 的长度 |
| Packet ID  | Short Enum | 数据包ID |
| Sequence   | Int        | 顺序编号，为之前发送过的数据包的个数 |
| Payload    | ByteArray  | 载荷 |


#### 登录

| From | Packet ID |
|:----:|:---------:|
| Client / Server / Admin | 0x00 |

| Field Name | Field Type | Notes |
| ---------- | ---------- | ----- |
| Protocol   | Int        | 协议版本 |
| Identity   | VarInt Enum | 0: 客户端; 1:服务端 2:管理员 |
| Token      | String     | 客户端: B站uid; 服务端/管理员: 特定的令牌 |
| Url        | String    | 服务端: 服务器Url; 客户端/管理员: 留空 |


#### 连接转移

| From | Packet ID |
|:----:|:---------:|
| Server | 0x01 |

| Field Name | Field Type | Notes |
| ---------- | ---------- | ----- |
| Url        | String     | 服务器Url |

#### 要求请求间隔

| From | Packet ID |
|:----:|:---------:|
| Server | 0x02 |

| Field Name | Field Type | Notes |
| ---------- | ---------- | ----- |
| Min Interval | VarInt   | 最小数据包发送间隔，单位为毫秒 |
| Max Interval | VarInt   | 最大数据包发送间隔，单位为毫秒 |

+ 服务端检查间隔是以服务端收到数据包的时刻为准

#### 心跳

| From | Packet ID |
|:----:|:---------:|
| Client / Server | 0x03 |

| Field Name | Field Type | Notes |
| ---------- | ---------- | ----- |
| Room Count | VarInt     | 可多监听的房间数量，如果大于0，服务端可以分配新任务 |

#### 任务信息

| From | Packet ID |
|:----:|:---------:|
| Client / Server | 0x04 |

| Field Name | Field Type | Notes |
| ---------- | ---------- | ----- |
| Type       | VarInt Enum | 0: 请求监听; 1:正在监听; 2:停止监听 |
| Room Count | VarInt     | 房间数量 |
| Room ID    | VarInt[]   | 房间ID |


#### 要求数据报告

| From | Packet ID |
|:----:|:---------:|
| Server | 0x05 |

| Field Name | Field Type | Notes |
| ---------- | ---------- | ----- |
| Room Count | VarInt     | 要求报告数据的房间数 |
| Room ID    | VarInt[]   | 要求报告数据的房间ID |

#### 数据报告

| From | Packet ID |
|:----:|:---------:|
| Client / Server | 0x06 |

| Field Name | Field Type | Notes |
| ---------- | ---------- | ----- |
| Timestamp  | Long       | 时间戳，单位为秒，应当尽可能保证在这个时间之后就能获取奖励 |
| Type       | VarInt Enum | 0: 礼物抽奖; 1: 节奏风暴; 2: 大航海; 3:大乱斗; 4:天选时刻 |
| Room ID    | VarInt     | 房间ID |
| ID         | String     | 抽奖ID |
| Time       | VarInt     | 持续时间 |
| Detail     | ByteArray  | 详细信息，见下 |

+ 一般来说把等待时间 `time_wait` 这些算在 `Timestamp` 里

Detail(Type = 0)

| Field Name | Field Type | Notes |
| ---------- | ---------- | ----- |
| Title      | String     | 礼物抽奖名称 |
| Type       | String     | 参数-礼物类型 |

Detail(Type = 1)

无

Detail(Type = 2)

| Field Name | Field Type | Notes |
| ---------- | ---------- | ----- |
| Type       | String     | 参数-guard |

Detail(Type = 3)

| Field Name | Field Type | Notes |
| ---------- | ---------- | ----- |
| Type       | String     | 参数-pk |

Detail(Type = 4)

无


#### 通知

| From | Packet ID |
|:----:|:---------:|
| Client / Server /Admin | 0x10 |

| Field Name | Field Type | Notes |
| ---------- | ---------- | ----- |
| Type       | VarInt     | 通知类型 |
| Message    | String     | 消息内容 |
| Token      | String     | 发送通知所需的令牌 |



## 安全机制

-----------------------------------

安全机制表现为服务端对客户端的不恰当请求时做出的反应，分为以下几个等级:

+ **封禁**
+ **踢出**

### 封禁

服务端会直接拒绝处于封禁状态的IP发起的连接，并断开该IP的所有现有连接

导致封禁的原因
+ 过多的踢出
+ 无效的数据格式
+ 不正确的数据包
+ 在作为 `Server` 和 `Admin` 进行 `登录` 时提供的 `Token` 无效


### 踢出

服务端会断开导致踢出的连接，并记录踢出次数和IP

导致踢出的原因
+ 过多的连接
+ 过多的丢包
+ 请求间隔过短或过长
+ 通信过程中出现错误
