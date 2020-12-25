
class Bytes {
  constructor (size = 0) {
    if (size instanceof ArrayBuffer) {
      this.buffer = size;
    } else {
      this.buffer = new ArrayBuffer(Math.max(8, size));
    }
    this.dv = new DataView(this.buffer);
    this.rpos = 0;
    this.wpos = 0;
  }

  remaining () {
    return this.buffer.byteLength - this.rpos;
  }

  advanceRpos (n) {
    this.rpos += n;
  }

  advanceWpos (n) {
    this.wpos += n;
  }

  tryExtend (n) {
    const wpos = this.wpos + n;
    if (wpos >= this.buffer.byteLength) {
      const newBuffer = new ArrayBuffer(Math.ceil(1.75 * wpos));
      copy(this.buffer, newBuffer);
      this.buffer = newBuffer;
      this.dv = new DataView(this.buffer);
    }
  }

  toArrayBuffer () {
    return this.buffer.slice(0, this.wpos);
  }

  getBool () {
    const r = this.dv.getUint8(this.rpos);
    this.advanceRpos(1);
    return r > 0;
  }

  putBool (v) {
    this.tryExtend(1);
    this.dv.setUint8(this.wpos, v ? 1 : 0);
    this.advanceWpos(1);
  }

  getUint8 () {
    const r = this.dv.getUint8(this.rpos);
    this.advanceRpos(1);
    return r;
  }

  putUint8 (v) {
    this.tryExtend(1);
    this.dv.setUint8(this.wpos, v);
    this.advanceWpos(1);
  }

  getInt8 () {
    const r = this.dv.getInt8(this.rpos);
    this.advanceRpos(1);
    return r;
  }

  putInt8 (v) {
    this.tryExtend(1);
    this.dv.setInt8(this.wpos, v);
    this.advanceWpos(1);
  }

  getUint16 () {
    const r = this.dv.getUint16(this.rpos);
    this.advanceRpos(2);
    return r;
  }

  putUint16 (v) {
    this.tryExtend(2);
    this.dv.setUint16(this.wpos, v);
    this.advanceWpos(2);
  }

  getInt16 () {
    const r = this.dv.getInt16(this.rpos);
    this.advanceRpos(2);
    return r;
  }

  putInt16 (v) {
    this.tryExtend(2);
    this.dv.setInt16(this.wpos, v);
    this.advanceWpos(2);
  }

  getUint32 () {
    const r = this.dv.getUint32(this.rpos);
    this.advanceRpos(4);
    return r;
  }

  putUint32 (v) {
    this.tryExtend(4);
    this.dv.setUint32(this.wpos, v);
    this.advanceWpos(4);
  }

  getInt32 () {
    const r = this.dv.getInt32(this.rpos);
    this.advanceRpos(4);
    return r;
  }

  putInt32 (v) {
    this.tryExtend(4);
    this.dv.setInt32(this.wpos, v);
    this.advanceWpos(4);
  }

  getUint64 () {
    const r = this.dv.getBigUint64(this.rpos);
    this.advanceRpos(8);
    return r;
  }

  putUint64 (v) {
    this.tryExtend(8);
    this.dv.setBigUint64(this.wpos, v);
    this.advanceWpos(8);
  }

  getInt64 () {
    const r = this.dv.getBigInt64(this.rpos);
    this.advanceRpos(8);
    return r;
  }

  putInt64 (v) {
    this.tryExtend(8);
    this.dv.setBigInt64(this.wpos, v);
    this.advanceWpos(8);
  }

  getFloat32 () {
    const r = this.dv.getFloat32(this.rpos);
    this.advanceRpos(4);
    return r;
  }

  putFloat32 (v) {
    this.tryExtend(4);
    this.dv.setFloat32(this.wpos, v);
    this.advanceWpos(4);
  }

  getFloat64 () {
    const r = this.dv.getFloat64(this.rpos);
    this.advanceRpos(8);
    return r;
  }

  putFloat64 (v) {
    this.tryExtend(8);
    this.dv.setFloat64(this.wpos, v);
    this.advanceWpos(8);
  }

  getArrayBuffer (length) {
    let r;
    if (length > 0) {
      r = this.buffer.slice(this.rpos, this.rpos + length);
    } else {
      r = this.buffer.slice(this.rpos);
    }
    this.advanceRpos(r.byteLength);
    return r;
  }

  putArrayBuffer (buffer) {
    this.tryExtend(buffer.byteLength);
    copy(buffer, new DataView(this.buffer, this.wpos));
    this.advanceWpos(buffer.byteLength);
  }

  getVarInt () {
    let v = 0;
    let i = 0;
    let b = 0x80;
    while (i < 5 && (b & 0x80)) {
      b = this.getUint8();
      v |= (b & 0x7F) << (i * 7);
      i += 1;
    }
    return v;
  }

  putVarInt (v) {
    let i = 0;
    while ((v >>> 7) && i < 4) {
      this.putUint8(v & 0x7F | 0x80);
      v >>>= 7;
      i++;
    }
    this.putUint8(v & 0x7F);
  }

  getString () {
    try {
      const len = this.getVarInt();
      const s = uint8ArrayToString(new Uint8Array(this.getArrayBuffer(len)));
      this.advanceRpos(len);
      return s;
    } catch {
      return null;
    }
  }

  putString (s) {
    const arr = stringToUint8Array(s);
    this.tryExtend(arr.byteLength + 4);
    this.putVarInt(arr.byteLength);
    this.putArrayBuffer(arr.buffer);
  }
}

function stringToUint8Array (s) {
  return new Uint8Array(s.split('').map(c => c.charCodeAt(0)));
}

function uint8ArrayToString (arr) {
  return decodeURIComponent(escape(String.fromCharCode.apply(null, arr)));
}

function copy (source, dest) {
  if (source instanceof ArrayBuffer) source = new DataView(source);
  if (dest instanceof ArrayBuffer) dest = new DataView(dest);
  let len = Math.min(source.byteLength, dest.byteLength);
  let pos = 0;
  const t = Math.floor(len / 8);
  for (let i = 0; i < t; i++) dest.setFloat64(i * 8, source.getFloat64(i * 8));
  pos += t * 8;
  len -= t * 8;
  if (len >= 4) {
    dest.setUint32(pos, source.getUint32(pos));
    pos += 4;
    len -= 4;
  }
  if (len >= 2) {
    dest.setUint16(pos, source.getUint16(pos));
    pos += 2;
    len -= 2;
  }
  if (len >= 1) {
    dest.setUint8(pos, source.getUint8(pos));
    pos += 1;
    len -= 1;
  }
}

const constants = {
  packetId: {
    SHOW_IDENTITY: 0x01,
    RATE_LIMIT: 0x02,
    TASK_APPLICATION: 0x03,
    TASK_CHANGE: 0x04,
    TASK_CONFIRM: 0x05,
    DATA_REPORT: 0x06,
    NOTIFICATION: 0xFF
  },
  show_identity: {
    category: {
      CLIENT: 1,
      SERVER: 2
    }
  },
  data_report: {
    category: {
      STORM: 1,
      SPECIAL_GIFT: 2,
      LOTTERY: 3
    }
  }
};

class Packet {
  constructor (length, id, data) {
    this.length = length;
    this.id = id;
    this.data = data;
  }

  toArrayBuffer () {
    const buf = (this.data && this.data.toArrayBuffer && this.data.toArrayBuffer()) || this.data;
    const bytes = new Bytes();
    bytes.putVarInt(this.length);
    bytes.putVarInt(this.id);
    bytes.putArrayBuffer(buf);
    return bytes.toArrayBuffer();
  }

  static parse (buffer) {
    const bytes = new Bytes(buffer);
    const length = bytes.getVarInt();
    const id = bytes.getVarInt();
    const buf = bytes.getArrayBuffer(length);
    let Clazz;
    const { SHOW_IDENTITY, RATE_LIMIT, TASK_APPLICATION, TASK_CHANGE, TASK_CONFIRM, DATA_REPORT, NOTIFICATION } = constants.packetId;
    switch (id) {
      case SHOW_IDENTITY:
        Clazz = ShowIdentity;
        break;
      case RATE_LIMIT:
        Clazz = RateLimit;
        break;
      case TASK_APPLICATION:
        Clazz = TaskApplication;
        break;
      case TASK_CHANGE:
        Clazz = TaskChange;
        break;
      case TASK_CONFIRM:
        Clazz = TaskConfirm;
        break;
      case DATA_REPORT:
        Clazz = DataReport;
        break;
      case NOTIFICATION:
        Clazz = Notification;
        break;
      default:
        throw new Error('Unexpected data!');
    }
    return [new Packet(length, id, Clazz.fromArrayBuffer(buf)), bytes.getArrayBuffer()];
  }

  static wrap (data) {
    const { SHOW_IDENTITY, RATE_LIMIT, TASK_APPLICATION, TASK_CHANGE, TASK_CONFIRM, DATA_REPORT, NOTIFICATION } = constants.packetId;
    let id;
    if (data instanceof ShowIdentity) id = SHOW_IDENTITY;
    else if (data instanceof RateLimit) id = RATE_LIMIT;
    else if (data instanceof TaskApplication) id = TASK_APPLICATION;
    else if (data instanceof TaskChange) id = TASK_CHANGE;
    else if (data instanceof TaskConfirm) id = TASK_CONFIRM;
    else if (data instanceof DataReport) id = DATA_REPORT;
    else if (data instanceof Notification) id = NOTIFICATION;
    else throw new Error('Unexpected data!');
    const buf = (data && data.toArrayBuffer && data.toArrayBuffer()) || data;
    return new Packet(buf.byteLength, id, buf);
  }
}

class ShowIdentity {
  constructor (category, token) {
    this.category = category;
    this.token = token;
  }

  toArrayBuffer () {
    const bytes = new Bytes();
    bytes.putVarInt(this.category);
    bytes.putString(this.token);
    return bytes.toArrayBuffer();
  }
}

class RateLimit {
  constructor (interval, maxBurst) {
    this.interval = interval;
    this.maxBurst = maxBurst;
  }

  static fromArrayBuffer (buffer) {
    const bytes = new Bytes(buffer);
    const interval = bytes.getVarInt();
    const maxBurst = bytes.getVarInt();
    return new RateLimit(interval, maxBurst);
  }
}

class TaskApplication {
  constructor (roomCount) {
    this.roomCount = roomCount;
  }

  toArrayBuffer () {
    const bytes = new Bytes();
    bytes.putVarInt(this.roomCount);
    return bytes.toArrayBuffer();
  }
}

class TaskChange {
  constructor (roomCount, roomIds) {
    this.roomCount = roomCount;
    this.roomIds = roomIds;
  }

  static fromArrayBuffer (buffer) {
    const bytes = new Bytes(buffer);
    const roomCount = bytes.getVarInt();
    const roomIds = [];
    for (let i = 0; i < roomCount; i++) {
      const s = bytes.getString();
      roomIds.push(s);
    }
    return new TaskChange(roomCount, roomIds);
  }
}

class TaskConfirm {
  constructor (roomCount, roomIds) {
    this.roomCount = roomCount;
    this.roomIds = roomIds;
  }

  toArrayBuffer () {
    const bytes = new Bytes();
    bytes.putVarInt(this.roomCount);
    for (let i = 0; i < this.roomIds.length; i++) {
      bytes.putString(this.roomIds[i]);
    }
    return bytes.toArrayBuffer();
  }
}

class DataReport {
  constructor (category, roomId, id, time, detail) {
    this.category = category;
    this.roomId = roomId;
    this.id = id;
    this.time = time;
    this.detail = detail;
  }

  toArrayBuffer () {
    const bytes = new Bytes();
    bytes.putVarInt(this.category);
    bytes.putString(this.roomId);
    bytes.putString(this.id);
    bytes.putVarInt(this.time);
    bytes.putString(JSON.stringify(this.detail));
    return bytes.toArrayBuffer();
  }

  static fromArrayBuffer (buffer) {
    const bytes = new Bytes(buffer);
    const category = bytes.getVarInt();
    const roomId = bytes.getString();
    const id = bytes.getString();
    const time = bytes.getVarInt();
    const detail = JSON.parse(bytes.getString());
    return new DataReport(category, roomId, id, time, detail);
  }
}

class Notification {
  constructor (category, message, token) {
    this.category = category;
    this.message = message;
    this.token = token;
  }

  toArrayBuffer () {
    const bytes = new Bytes();
    bytes.putVarInt(this.category);
    bytes.putString(this.message);
    bytes.putString(this.token);
    return bytes.toArrayBuffer();
  }

  static fromArrayBuffer (buffer) {
    const bytes = new Bytes(buffer);
    const category = bytes.getVarInt();
    const message = bytes.getString();
    const token = bytes.getString();
    return new Notification(category, message, token);
  }
}

class TokenBucket {
  constructor (interval = 1000, maxBurst = 5) {
    this.interval = interval;
    this.maxBurst = maxBurst;
    this.lastFillTime = Date.now();
    this.nextFillTime = this.lastFillTime + interval;
    this.count = maxBurst;
  }

  tryCosume (cnt = 1) {
    this.produce();
    if (this.count >= cnt) {
      this.count -= cnt;
      return true;
    }
    return false;
  }

  produce () {
    const fillCnt = Math.floor((Date.now() - this.lastFillTime) / this.interval);
    if (fillCnt > 0) {
      this.lastFillTime += fillCnt * this.interval;
      this.nextFillTime = this.lastFillTime + this.interval;
      this.count = Math.min(this.maxBurst, this.count + fillCnt);
    }
    return fillCnt;
  }

  doAtNextFillTime (f) {
    return setTimeout(f, this.nextFillTime - Date.now());
  }
}

const state = {
  HANDSHAKING: 1,
  WORKING: 2
};

const HANDLE_MAP = (() => {
  const { HANDSHAKING, WORKING } = state;
  const { RATE_LIMIT, TASK_CHANGE, DATA_REPORT, NOTIFICATION } = constants.packetId;
  return new Map([
    [RATE_LIMIT, (labour, data) => {
      console.log('State: ', labour.state, ', RateLimit: ', data);
      labour.tokenBucket.interval = data.interval;
      labour.tokenBucket.maxBurst = data.maxBurst;
      if (labour.state === HANDSHAKING) {
        const previousTask = labour.config.previousTask;
        if (previousTask && previousTask.roomCount !== undefined && previousTask.roomIds instanceof Array) {
          labour.sendData(new TaskConfirm(previousTask.roomCount, previousTask.roomIds));
          labour.state = WORKING;
        } else {
          labour.sendData(new TaskApplication(labour.config.roomCount));
        }
      }
    }],
    [TASK_CHANGE, (labour, data) => {
      console.log('State: ', labour.state, ', TaskChange', data);
      labour.roomCount = data.roomCount;
      labour.roomIds = data.roomIds;
      labour.sendData(new TaskConfirm(labour.roomCount, labour.roomIds));
      if (labour.state === HANDSHAKING) {
        labour.state = WORKING;
      }
    }],
    [DATA_REPORT, (labour, data) => {
      if (labour.state === WORKING) {
        console.log('State: ', labour.state, ', DataReport: ', data);
      }
    }],
    [NOTIFICATION, (labour, data) => {
      if (labour.state === WORKING) {
        console.log('State: ', labour.state, ', Notification: ', data);
      }
    }]
  ]);
})();

class Labour {
  constructor (config = {}) {
    this.config = config;
  }

  start () {
    if (this.ws) return;
    const ws = new WebSocket(this.config.url); // eslint-disable-line no-undef
    this.ws = ws;
    this.state = state.HANDSHAKING;
    this.tokenBucket = new TokenBucket();
    this.sendBuffer = null;
    ws.binaryType = 'arraybuffer';
    ws.onopen = ev => {
      this.sendData(new ShowIdentity(constants.show_identity.category.CLIENT, this.config.token));
    };
    ws.onmessage = ev => {
      let buffer = ev.data;
      while (buffer && buffer.byteLength > 0) {
        const r = Packet.parse(buffer);
        const pkt = r[0];
        buffer = r[1];
        console.log('pkt:', pkt);
        const handle = HANDLE_MAP.get(pkt.id);
        if (handle instanceof Function) {
          handle.call(this, this, pkt.data);
          continue;
        }
        console.error('unexpected packet!');
        ws.close();
      }
    };
    ws.onclose = ev => {
      this.ws = null;
      console.warn('disconnected', ev);
    };
  }

  stop () {
    if (!this.ws) return;
    this.ws.close();
  }

  sendData (data) {
    if (!this.ws) return;
    data = Packet.wrap(data).toArrayBuffer();
    console.log(data);
    if (this.sendBuffer) {
      this.sendBuffer.putArrayBuffer(data);
      console.log(this.sendBuffer);
      return;
    }
    if (this.tokenBucket.tryCosume()) {
      this.ws.send(data);
    } else {
      if (!this.sendBuffer) this.sendBuffer = new Bytes();
      this.sendBuffer.putArrayBuffer(data);
      const f = () => {
        if (!this.sendBuffer) return;
        if (this.tokenBucket.tryCosume()) {
          this.ws.send(this.sendBuffer.toArrayBuffer());
          console.log(this.sendBuffer);
          this.sendBuffer = null;
          return;
        }
        this.tokenBucket.doAtNextFillTime(f);
      };
      this.tokenBucket.doAtNextFillTime(f);
    }
  }
}

function createLabour (token) {
  return new Labour({
    url: 'ws://localhost:8181',
    token: token || Math.random().toString(16),
    previousTask: {}
  });
}
