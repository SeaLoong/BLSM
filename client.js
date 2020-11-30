
class DataView extends DataView {
  getVarint (byteOffset) {
    let v = 0;
    let i = 0;
    let b;
    do {
      b = this.getUint8(byteOffset + i);
      v |= (b & 0x7F) << (i * 7);
      i++;
    } while (b & 0x80 && i < 5);
    return [v, i];
  }

  setVarint (byteOffset, value) {
    let i = 0;
    while (value & ~0x7F && i < 4) {
      this.setUint8(byteOffset + i, value & 0x7F | 0x80);
      value >>>= 7;
      i++;
    }
    this.setUint8(byteOffset + i, value & 0x7F);
    return i + 1;
  }

  getString (byteOffset) {
    const [len, pos] = this.getVarint(byteOffset);
    return [uintToString(new Uint8Array(this.buffer, this.byteOffset + byteOffset + pos, len)), pos + len];
  }

  setString (byteOffset, string) {
    const arr = stringToUint(string);
    const len = arr.byteLength;
    const pos = this.setVarint(byteOffset, len);
    for (let i = 0; i < len; i++) {
      this.setUint8(this.byteOffset + byteOffset + pos + i, arr[i]);
    }
    return pos + len;
  }
}

function stringToUint (s) {
  return new Uint8Array(s.split('').map(c => c.charCodeAt(0)));
}

function uintToString (arr) {
  return decodeURIComponent(escape(String.fromCharCode.apply(null, arr)));
}

class Packet {
  constructor (id, protocol, payload) {
    this.id = id;
    this.protocol = protocol || Packet.PROTOCOL;
    this.payload = payload;
  }

  toArrayBuffer () {
    const payloadBuf = (this.payload && this.payload.toArrayBuffer && this.payload.toArrayBuffer()) || this.payload;
    const buf = new ArrayBuffer(2 + (payloadBuf instanceof ArrayBuffer ? payloadBuf.byteLength : 0));
    const dv = new DataView(buf);
    dv.setUint8(0, this.id);
    dv.setUint8(1, this.protocol);
    if (payloadBuf instanceof ArrayBuffer) {
      const arr = new Uint8Array(payloadBuf);
      for (let i = 0; i < arr.length; i++) {
        dv.setUint8(2 + i, arr[i]);
      }
    }
    return buf;
  }

  static from (buffer) {
    const dv = new DataView(buffer);
    return new Packet(dv.getUint8(0), dv.getUint8(1), buffer.slice(2));
  }

  static wrap (payload, protocol) {
    let id = 0;
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
    const buf = new ArrayBuffer(6 + (this.token ? this.token.length : 0));
    const dv = new DataView(buf);
    dv.setUint8(0, this.identity);
    const len = dv.setString(1, this.token);
    return buf.slice(0, 1 + len);
  }
}

class RequireInterval {
  constructor (minInterval, maxInterval) {
    this.minInterval = minInterval;
    this.maxInterval = maxInterval;
  }

  static from (buffer) {
    const dv = new DataView(buffer);
    const [minInterval, l1] = dv.getVarint(0);
    const [maxInterval] = dv.getVarint(l1);
    return new RequireInterval(minInterval, maxInterval);
  }
}

class ApplyForTask {
  constructor (roomCount) {
    this.roomCount = roomCount;
  }

  toArrayBuffer () {
    const buf = new ArrayBuffer(5);
    const dv = new DataView(buf);
    const len = dv.setVarint(0, this.roomCount);
    return buf.slice(0, len);
  }
}

class ChangeTask {
  constructor (roomCount, roomIDs) {
    this.roomCount = roomCount;
    this.roomIDs = roomIDs;
  }

  static from (buffer) {
    const dv = new DataView(buffer);
    const [roomCount, l1] = dv.getVarint(0);
    const roomIDs = [];
    let p = l1;
    for (let i = 0; i < roomCount; i++) {
      const [s, l] = dv.getString(p);
      p += l;
      roomIDs.push(s);
    }
    return new ChangeTask(roomCount, roomIDs);
  }
}

class ConfirmTask {
  constructor (roomCount, roomIDs) {
    this.roomCount = roomCount;
    this.roomIDs = roomIDs;
  }

  toArrayBuffer () {
    const buf = new ArrayBuffer(5 + 5 * (this.roomIDs ? this.roomIDs.length : 0));
    const dv = new DataView(buf);
    let p = dv.setVarint(0, this.roomCount);
    for (let i = 0; i < this.roomIDs.length; i++) {
      p += dv.setVarint(p, this.roomIDs[i]);
    }
    return buf.slice(0, p);
  }
}

class ReportData {
  constructor (type, roomID, id, time, detail) {
    this.type = type;
    this.roomID = roomID;
    this.id = id;
    this.time = time;
    this.detail = detail;
  }

  toArrayBuffer () {
    const buf = new ArrayBuffer(21 + (this.roomID ? this.roomID.length : 0) + (this.id ? this.id.length : 0) + (this.detail ? this.detail.length : 0));
    const dv = new DataView(buf);
    let p = 0;
    dv.setUint8(p, this.type);
    p += 1;
    p += dv.setString(p, this.roomID);
    p += dv.setString(p, this.id);
    p += dv.setVarint(p, this.time);
    p += dv.setString(p, this.detail);
    return buf.slice(0, p);
  }

  static from (buffer) {
    const dv = new DataView(buffer);
    const type = dv.getUint8(0);
    let p = 1;
    const [roomID, l1] = dv.getString(p);
    p += l1;
    const [id, l2] = dv.getString(p);
    p += l2;
    const [time, l3] = dv.getVarint(p);
    p += l3;
    const [detail, l4] = dv.getString(p);
    p += l4;
    return new ReportData(type, roomID, id, time, detail);
  }
}

class Notify {
  constructor (type, message, token) {
    this.type = type;
    this.message = message;
    this.token = token;
  }

  toArrayBuffer () {
    const buf = new ArrayBuffer(11 + (this.message ? this.message.length : 0) + (this.token ? this.token.length : 0));
    const dv = new DataView(buf);
    let p = 0;
    dv.setUint8(p, this.type);
    p += 1;
    p += dv.setString(p, this.message);
    p += dv.setString(p, this.token);
    return buf.slice(0, p);
  }

  static from (buffer) {
    const dv = new DataView(buffer);
    const type = dv.getUint8(0);
    let p = 1;
    const [message, l1] = dv.getString(p);
    p += l1;
    const [token, l2] = dv.getString(p);
    p += l2;
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
      labour.sendPayload(new ConfirmTask(labour.roomCount, labour.roomIDs));
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
  labour.roomIDs = pkt.roomIDs;
  labour.sendPayload(new ConfirmTask(labour.roomCount, labour.roomIDs));
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