
class Bytes {
  static from (buffer) {
    return new Bytes(buffer);
  }

  constructor (size) {
    if (size instanceof ArrayBuffer) {
      this.buffer = size;
    } else {
      this.buffer = new ArrayBuffer(10);
    }
    this.dv = new DataView(this.buffer);
    this.rpos = 0;
    this.wpos = 0;
  }

  remaining () {
    return this.buffer.byteLength - this.rpos;
  }

  hasRemaining () {
    return this.remaining() > 0;
  }

  advanceRpos (n) {
    this.rpos += n;
  }

  advanceWpos (n) {
    this.wpos += n;
    if (this.wpos > this.buffer.byteLength) {
      const newBuffer = new ArrayBuffer(Math.ceil(1.5 * this.wpos));
      copyTo(this.buffer, newBuffer);
      this.buffer = newBuffer;
      this.dv = new DataView(this.buffer);
    }
  }

  toArrayBuffer () {
    return this.buffer.slice(0, this.wpos);
  }

  getArrayBuffer (length) {
    let r;
    if (length > 0) {
      r = this.buffer.slice(this.wpos, this.wpos + length);
    } else {
      r = this.buffer.slice(this.wpos);
    }
    this.advanceRpos(r.byteLength);
    return r;
  }

  putArrayBuffer (buffer) {
    this.advanceWpos(buffer.byteLength);
    copyTo(buffer, this.buffer, this.wpos - buffer.byteLength);
  }

  getBool () {
    const r = this.dv.getUint8(this.rpos);
    this.advanceRpos(1);
    return r > 0;
  }

  putBool (v) {
    this.advanceWpos(1);
    this.dv.setUint8(this.wpos, v ? 1 : 0);
  }

  getUint8 () {
    const r = this.dv.getUint8(this.rpos);
    this.advanceRpos(1);
    return r;
  }

  putUint8 (v) {
    this.advanceWpos(1);
    this.dv.setUint8(this.wpos, v);
  }

  getInt8 () {
    const r = this.dv.getInt8(this.rpos);
    this.advanceRpos(1);
    return r;
  }

  putInt8 (v) {
    this.advanceWpos(1);
    this.dv.setInt8(this.wpos, v);
  }

  getUint16 () {
    const r = this.dv.getUint16(this.rpos);
    this.advanceRpos(2);
    return r;
  }

  putUint16 (v) {
    this.advanceWpos(2);
    this.dv.setUint16(this.wpos, v);
  }

  getInt16 () {
    const r = this.dv.getInt16(this.rpos);
    this.advanceRpos(2);
    return r;
  }

  putInt16 (v) {
    this.advanceWpos(2);
    this.dv.setInt16(this.wpos, v);
  }

  getUint32 () {
    const r = this.dv.getUint32(this.rpos);
    this.advanceRpos(4);
    return r;
  }

  putUint32 (v) {
    this.advanceWpos(4);
    this.dv.setUint32(this.wpos, v);
  }

  getInt32 () {
    const r = this.dv.getInt32(this.rpos);
    this.advanceRpos(4);
    return r;
  }

  putInt32 (v) {
    this.advanceWpos(4);
    this.dv.setInt32(this.wpos, v);
  }

  getUint64 () {
    const r = this.dv.getBigUint64(this.rpos);
    this.advanceRpos(8);
    return r;
  }

  putUint64 (v) {
    this.advanceWpos(8);
    this.dv.setBigUint64(this.wpos, v);
  }

  getInt64 () {
    const r = this.dv.getBigInt64(this.rpos);
    this.advanceRpos(8);
    return r;
  }

  putInt64 (v) {
    this.advanceWpos(8);
    this.dv.setBigInt64(this.wpos, v);
  }

  getFloat32 () {
    const r = this.dv.getFloat32(this.rpos);
    this.advanceRpos(4);
    return r;
  }

  putFloat32 (v) {
    this.advanceWpos(4);
    this.dv.setFloat32(this.wpos, v);
  }

  getFloat64 () {
    const r = this.dv.getFloat64(this.rpos);
    this.advanceRpos(8);
    return r;
  }

  putFloat64 (v) {
    this.advanceWpos(8);
    this.dv.setFloat64(this.wpos, v);
  }

  getVarint () {
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

  putVarint (v) {
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
      const len = this.getVarint();
      const s = uintToString(new Uint8Array(this.getArrayBuffer(len)));
      this.advanceRpos(len);
      return s;
    } catch {
      return null;
    }
  }

  putString (s) {
    const arr = stringToUint(s);
    const len = arr.byteLength;
    this.putVarint(len);
    for (let i = 0; i < len; i++) {
      this.putUint8(arr[i]);
    }
  }
}

function stringToUint (s) {
  return new Uint8Array(s.split('').map(c => c.charCodeAt(0)));
}

function uintToString (arr) {
  return decodeURIComponent(escape(String.fromCharCode.apply(null, arr)));
}

function copyTo (source, dest, destOffset = 0) {
  let nextOffset = 0;
  let leftBytes = source.byteLength;
  [8, 4, 2, 1].forEach(wordSize => {
    const result = copyToWith(wordSize, source, dest, destOffset, nextOffset, leftBytes);
    nextOffset = result.nextOffset;
    leftBytes = result.leftBytes;
  });
}

function copyToWith (wordSize, source, dest, destOffset, nextOffset, leftBytes) {
  let ViewClass = Uint8Array;
  switch (wordSize) {
    case 8:
      ViewClass = Float64Array;
      break;
    case 4:
      ViewClass = Float32Array;
      break;
    case 2:
      ViewClass = Uint16Array;
      break;
    default:
      ViewClass = Uint8Array;
      break;
  }
  const dvSource = new ViewClass(source, nextOffset, Math.trunc(leftBytes / wordSize));
  const dvDest = new ViewClass(dest, destOffset + nextOffset, Math.trunc(leftBytes / wordSize));
  for (let i = 0; i < dvDest.length; i++) {
    dvDest[i] = dvSource[i];
  }
  return {
    nextOffset: dvSource.byteOffset + dvSource.byteLength,
    leftBytes: source.byteLength - (dvSource.byteOffset + dvSource.byteLength)
  };
}

class Packet {
  constructor (id, protocol, payload) {
    this.id = id;
    this.protocol = protocol || Packet.PROTOCOL;
    this.payload = payload;
  }

  toArrayBuffer () {
    const payloadBuf = (this.payload && this.payload.toArrayBuffer && this.payload.toArrayBuffer()) || this.payload;
    const bytes = new Bytes();
    bytes.putUint8(this.id);
    bytes.putUint8(this.protocol);
    bytes.putArrayBuffer(payloadBuf);
    return bytes.toArrayBuffer();
  }

  static from (buffer) {
    const bytes = new Bytes(buffer);
    const id = bytes.getUint8();
    const protocol = bytes.getUint8();
    const payload = bytes.getArrayBuffer();
    let Clazz;
    switch (id) {
      case 2:
        Clazz = RequireInterval;
        break;
      case 4:
        Clazz = ChangeTask;
        break;
      case 6:
        Clazz = ReportData;
        break;
      case 0xFF:
        Clazz = Notify;
        break;
      default:
        throw new Error('Unexpected payload!');
    }
    return new Packet(id, protocol, Clazz.from(payload));
  }

  static wrap (payload, protocol) {
    let id;
    if (payload instanceof Identity) id = 1;
    else if (payload instanceof ApplyForTask) id = 3;
    else if (payload instanceof ConfirmTask) id = 5;
    else if (payload instanceof ReportData) id = 7;
    else if (payload instanceof Notify) id = 0xFF;
    else throw new Error('Unexpected payload!');
    return new Packet(id, protocol, payload);
  }
}
Packet.PROTOCOL = 1;

class Identity {
  constructor (identity, token) {
    this.identity = identity;
    this.token = token;
  }

  toArrayBuffer () {
    const bytes = new Bytes();
    bytes.putUint8(this.identity);
    bytes.putString(this.token);
    return bytes.toArrayBuffer();
  }
}

class RequireInterval {
  constructor (minInterval, maxInterval) {
    this.minInterval = minInterval;
    this.maxInterval = maxInterval;
  }

  static from (buffer) {
    const bytes = new Bytes(buffer);
    const minInterval = bytes.getVarint();
    const maxInterval = bytes.getVarint();
    return new RequireInterval(minInterval, maxInterval);
  }
}

class ApplyForTask {
  constructor (roomCount) {
    this.roomCount = roomCount;
  }

  toArrayBuffer () {
    const bytes = new Bytes();
    bytes.putVarint(this.roomCount);
    return bytes.toArrayBuffer();
  }
}

class ChangeTask {
  constructor (roomCount, roomIds) {
    this.roomCount = roomCount;
    this.roomIds = roomIds;
  }

  static from (buffer) {
    const bytes = new Bytes(buffer);
    const roomCount = bytes.getVarint();
    const roomIds = [];
    for (let i = 0; i < roomCount; i++) {
      const s = bytes.getString();
      roomIds.push(s);
    }
    return new ChangeTask(roomCount, roomIds);
  }
}

class ConfirmTask {
  constructor (roomCount, roomIds) {
    this.roomCount = roomCount;
    this.roomIds = roomIds;
  }

  toArrayBuffer () {
    const bytes = new Bytes();
    bytes.putVarint(this.roomCount);
    for (let i = 0; i < this.roomIds.length; i++) {
      bytes.putString(this.roomIds[i]);
    }
    return bytes.toArrayBuffer();
  }
}

class ReportData {
  constructor (type, roomId, id, time, detail) {
    this.type = type;
    this.roomId = roomId;
    this.id = id;
    this.time = time;
    this.detail = detail;
  }

  toArrayBuffer () {
    const bytes = new Bytes();
    bytes.putUint8(this.type);
    bytes.putString(this.roomId);
    bytes.putString(this.id);
    bytes.putVarint(this.time);
    bytes.putString(JSON.stringify(this.detail));
    return bytes.toArrayBuffer();
  }

  static from (buffer) {
    const bytes = new Bytes(buffer);
    const type = bytes.getUint8();
    const roomId = bytes.getString();
    const id = bytes.getString();
    const time = bytes.getVarint();
    const detail = JSON.parse(bytes.getString());
    return new ReportData(type, roomId, id, time, detail);
  }
}

class Notify {
  constructor (type, message, token) {
    this.type = type;
    this.message = message;
    this.token = token;
  }

  toArrayBuffer () {
    const bytes = new Bytes();
    bytes.putUint8(this.type);
    bytes.putString(this.message);
    bytes.putString(this.token);
    return bytes.toArrayBuffer();
  }

  static from (buffer) {
    const bytes = new Bytes(buffer);
    const type = bytes.getUint8();
    const message = bytes.getString();
    const token = bytes.getString();
    return new Notify(type, message, token);
  }
}

class StateSet {
  constructor (other) {
    this.inner = other ? other.inner : 0;
  }

  has (state) {
    if (state === undefined || state === null) return false;
    return this.inner & state;
  }

  add (state) {
    this.inner |= state;
  }

  remove (state) {
    this.inner &= ~state;
  }

  onlyhas (state) {
    return this.inner === state;
  }

  onlyadd (state) {
    this.inner = state;
  }

  clear () {
    this.inner = 0;
  }
}

class State extends Number {
  constructor (value, handler) {
    super(value);
    this.handler = handler;
  }
}

State.REQUIRE_INTERVAL = new State(2, (packet, labour) => {
  const pkt = RequireInterval.from(packet.payload);
  console.log('RequireInterval: ', pkt);
  labour.minInterval = pkt.minInterval;
  labour.maxInterval = pkt.maxInterval;
  if (this.allowedStates.onlyhas(State.REQUIRE_INTERVAL)) {
    labour.allowedStates.add(State.CHANGE_TASK);
    labour.allowedStates.add(State.REPORT_DATA);
    labour.allowedStates.add(State.NOTIFY);
    if (labour.roomCount) {
      labour.sendPayload(new ConfirmTask(labour.roomCount, labour.roomIds));
    } else {
      labour.sendPayload(new ApplyForTask(labour.config.roomCount));
    }
  }
});

State.REPORT_DATA = new State(8, (packet, labour) => {
  const pkt = ReportData.from(packet.payload);
  console.log('ReportData: ', pkt);
});

State.CHANGE_TASK = new State(16, (packet, labour) => {
  const pkt = ChangeTask.from(packet.payload);
  console.log(pkt);
  labour.roomCount = pkt.roomCount;
  labour.roomIds = pkt.roomIds;
  labour.sendPayload(new ConfirmTask(labour.roomCount, labour.roomIds));
});

State.NOTIFY = new State(64, (packet, labour) => {
  const pkt = Notify.from(packet.payload);
  console.log('Notify: ', pkt);
});

const StateMap = new Map([
  [2, State.REQUIRE_INTERVAL],
  [4, State.CHANGE_TASK],
  [6, State.REPORT_DATA],
  [0xFF, State.NOTIFY]]);

class Labour {
  constructor (config) {
    this.config = config;
    this.allowedStates = new StateSet();
  }

  start () {
    if (this.ws) return;
    const ws = new WebSocket(this.config.url); // eslint-disable-line no-undef
    this.ws = ws;
    ws.binaryType = 'arraybuffer';
    ws.onopen = ev => {
      this.sendPayload(new Identity(0, this.config.token));
      this.allowedStates.onlyadd(State.REQUIRE_INTERVAL);
    };
    ws.onmessage = ev => {
      const pkt = Packet.from(ev.data);
      const state = StateMap.get(pkt.id);
      if (state !== undefined) {
        if (this.allowedStates.has(state)) {
          state.handler.call(this, pkt, this);
          return;
        }
        console.error('Disallowed Packet');
        return;
      }
      console.error('Invalid Packet');
    };
    ws.onclose = ev => {
      this.ws = null;
      console.warn('Disconnected');
    };
  }

  stop () {
    this.ws.close();
  }

  sendPayload (payload, protocol) {
    this.ws.send(Packet.wrap(payload, protocol).toArrayBuffer());
  }
}

const labour = new Labour({ url: 'http://localhost:8181' });
