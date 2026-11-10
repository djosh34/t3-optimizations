// sync-core's phone, from crates/phone-wasm. That crate's lib.rs documents
// the ABI and the byte encodings this file reads.

/** Bytes on a plain ArrayBuffer, as WebRTC and IndexedDB take them. */
export type Bytes = Uint8Array<ArrayBuffer>;

export type EntryState = "queued" | "sent" | "server-has";

export interface ObjectKey {
  /** 0 object, 1 reply, 2 log entry. */
  readonly kind: number;
  readonly id: bigint;
}

export interface Received {
  readonly batch: bigint;
  readonly changed: ReadonlyArray<{ readonly key: ObjectKey; readonly saved: Bytes }>;
  readonly removed: ReadonlyArray<ObjectKey>;
  readonly replies: ReadonlyArray<{ readonly seq: bigint; readonly payload: Bytes }>;
  readonly newBytes: number;
}

export interface ObjectView {
  readonly meta: Uint8Array;
  readonly rev: bigint;
  readonly len: number;
  /** Byte ranges the phone has, [start, end). */
  readonly ranges: ReadonlyArray<readonly [number, number]>;
  readonly data: Bytes;
}

export interface PhoneStats {
  readonly sendRate: number;
  readonly deliveredRate: number;
  readonly loss: number;
  readonly srttMs: number | null;
  readonly minRttMs: number | null;
  readonly sentPackets: number;
  readonly sentWireBytes: number;
  readonly receivedPackets: number;
  readonly receivedWireBytes: number;
  readonly lastHeardMs: number | null;
  readonly unsyncedBytes: number;
  readonly inflightPackets: number;
  /** Bytes sync-core handed to the app that it had handed over before (should stay 0). */
  readonly duplicateBytes: number;
}

interface Exports {
  readonly memory: WebAssembly.Memory;
  phone_alloc(len: number): number;
  phone_dealloc(ptr: number, len: number): void;
  phone_out_ptr(): number;
  phone_out_len(): number;
  phone_create(token: number, epoch: bigint, now: number): number;
  phone_load(ptr: number, len: number, now: number): number;
  phone_connected(): number;
  phone_receive(ptr: number, len: number, now: number): number;
  phone_committed(batch: bigint): number;
  phone_poll(now: number): number;
  phone_next_wakeup(now: number): number;
  phone_command(ptr: number, len: number): bigint;
  phone_call(ptr: number, len: number, background: number): bigint;
  phone_entry_state(seq: bigint): number;
  phone_save_state(): number;
  phone_object(id: bigint): number;
  phone_object_ids(): number;
  phone_stats(): number;
  phone_hash_id(ptr: number, len: number): number;
}

class Reader {
  private at = 0;
  private readonly view: DataView;
  constructor(private readonly buf: Bytes) {
    this.view = new DataView(buf.buffer, buf.byteOffset, buf.byteLength);
  }
  u8() {
    return this.view.getUint8(this.at++);
  }
  u32() {
    const v = this.view.getUint32(this.at, true);
    this.at += 4;
    return v;
  }
  u64() {
    const v = this.view.getBigUint64(this.at, true);
    this.at += 8;
    return v;
  }
  f64() {
    const v = this.view.getFloat64(this.at, true);
    this.at += 8;
    return v;
  }
  bytes(n: number) {
    const v = this.buf.slice(this.at, this.at + n);
    this.at += n;
    return v;
  }
  key(): ObjectKey {
    return { kind: this.u8(), id: this.u64() };
  }
}

export class PhoneWasmError extends Error {}

/** The phone in its WebAssembly module. One per page. */
export class PhoneWasm {
  private constructor(private readonly x: Exports) {}

  static async load(url: string): Promise<PhoneWasm> {
    const response = await fetch(url);
    if (!response.ok) throw new PhoneWasmError(`${url}: HTTP ${response.status}`);
    const { instance } = await WebAssembly.instantiate(await response.arrayBuffer());
    return new PhoneWasm(instance.exports as unknown as Exports);
  }

  private out(): Bytes {
    return new Uint8Array(
      this.x.memory.buffer,
      this.x.phone_out_ptr(),
      this.x.phone_out_len(),
    ).slice();
  }

  private check(status: number): Bytes {
    if (status < 0) throw new PhoneWasmError(new TextDecoder().decode(this.out()));
    return this.out();
  }

  /** Copies `bytes` into the module for one call. */
  private withInput<T>(bytes: Uint8Array, call: (ptr: number, len: number) => T): T {
    const ptr = this.x.phone_alloc(bytes.length);
    new Uint8Array(this.x.memory.buffer, ptr, bytes.length).set(bytes);
    try {
      return call(ptr, bytes.length);
    } finally {
      this.x.phone_dealloc(ptr, bytes.length);
    }
  }

  create(token: Uint8Array, epoch: bigint, now: number) {
    this.check(this.withInput(token, (ptr) => this.x.phone_create(ptr, epoch, now)));
  }

  /** From the stored state and objects. */
  load(state: Uint8Array, objects: ReadonlyArray<Uint8Array>, now: number) {
    const parts = [state, ...objects];
    const buf = new Uint8Array(parts.reduce((n, p) => n + 4 + p.length, 0));
    const view = new DataView(buf.buffer);
    let at = 0;
    for (const p of parts) {
      view.setUint32(at, p.length, true);
      buf.set(p, at + 4);
      at += 4 + p.length;
    }
    this.check(this.withInput(buf, (ptr, len) => this.x.phone_load(ptr, len, now)));
  }

  connected() {
    this.check(this.x.phone_connected());
  }

  receive(packet: Uint8Array, now: number): Received {
    const r = new Reader(
      this.check(this.withInput(packet, (ptr, len) => this.x.phone_receive(ptr, len, now))),
    );
    const batch = r.u64();
    const changed = Array.from({ length: r.u32() }, () => {
      const key = r.key();
      return { key, saved: r.bytes(r.u32()) };
    });
    const removed = Array.from({ length: r.u32() }, () => r.key());
    const replies = Array.from({ length: r.u32() }, () => {
      const seq = r.u64();
      return { seq, payload: r.bytes(r.u32()) };
    });
    return { batch, changed, removed, replies, newBytes: Number(r.u64()) };
  }

  committed(batch: bigint) {
    this.check(this.x.phone_committed(batch));
  }

  /** The next packet to send now, if any. */
  poll(now: number): Bytes | null {
    const status = this.x.phone_poll(now);
    const out = this.check(status);
    return status === 1 ? out : null;
  }

  nextWakeup(now: number): number {
    return this.x.phone_next_wakeup(now);
  }

  command(payload: Uint8Array): bigint {
    const seq = this.withInput(payload, (ptr, len) => this.x.phone_command(ptr, len));
    if (seq === 0n) this.check(-1);
    return seq;
  }

  call(payload: Uint8Array, background: boolean): bigint {
    const seq = this.withInput(payload, (ptr, len) =>
      this.x.phone_call(ptr, len, background ? 1 : 0),
    );
    if (seq === 0n) this.check(-1);
    return seq;
  }

  /** Null once the entry has left the log. */
  entryState(seq: bigint): EntryState | null {
    switch (this.x.phone_entry_state(seq)) {
      case 0:
        return "queued";
      case 1:
        return "sent";
      case 2:
        return "server-has";
      default:
        return null;
    }
  }

  saveState(): Bytes {
    return this.check(this.x.phone_save_state());
  }

  object(id: bigint): ObjectView | null {
    const status = this.x.phone_object(id);
    const out = this.check(status);
    if (status !== 1) return null;
    const r = new Reader(out);
    const meta = r.bytes(r.u32());
    const rev = r.u64();
    const len = Number(r.u64());
    const ranges = Array.from(
      { length: r.u32() },
      () => [Number(r.u64()), Number(r.u64())] as const,
    );
    return { meta, rev, len, ranges, data: r.bytes(len) };
  }

  objectIds(): bigint[] {
    const bytes = this.check(this.x.phone_object_ids());
    const r = new Reader(bytes);
    return Array.from({ length: bytes.length / 8 }, () => r.u64());
  }

  /** An id as the sidecar writes it in meta (sync_core::hash_id). */
  hashId(id: string): string {
    const bytes = new TextEncoder().encode(id);
    const out = this.check(this.withInput(bytes, (ptr, len) => this.x.phone_hash_id(ptr, len)));
    return new TextDecoder().decode(out);
  }

  stats(): PhoneStats {
    const r = new Reader(this.check(this.x.phone_stats()));
    const v = Array.from({ length: 13 }, () => r.f64());
    const opt = (n: number) => (n < 0 ? null : n);
    return {
      sendRate: v[0]!,
      deliveredRate: v[1]!,
      loss: v[2]!,
      srttMs: opt(v[3]!),
      minRttMs: opt(v[4]!),
      sentPackets: v[5]!,
      sentWireBytes: v[6]!,
      receivedPackets: v[7]!,
      receivedWireBytes: v[8]!,
      lastHeardMs: opt(v[9]!),
      unsyncedBytes: v[10]!,
      inflightPackets: v[11]!,
      duplicateBytes: v[12]!,
    };
  }
}
