// The sync engine of the page: sync-core's phone (WebAssembly) between the
// link to the sidecar and IndexedDB. Each received packet's content is
// committed to IndexedDB before the phone acks it (issue #19).

import { Link, type LinkInfo } from "./link";
import { Objects } from "./objects";
import { type EntryState, PhoneWasm, type PhoneStats } from "./phoneWasm";
import type { Pairing, Saved, Store } from "./store";

const encoder = new TextEncoder();
const decoder = new TextDecoder();

function keyName(kind: number, id: bigint) {
  return `${kind}:${id}`;
}

function randomEpoch(): bigint {
  const b = crypto.getRandomValues(new Uint8Array(8));
  return new DataView(b.buffer).getBigUint64(0, true);
}

function hexBytes(hex: string): Uint8Array {
  return Uint8Array.from(hex.match(/../g) ?? [], (b) => Number.parseInt(b, 16));
}

export interface EngineStatus {
  readonly link: LinkInfo;
  readonly stats: PhoneStats;
  /** Bytes of synced content that arrived since the page started. */
  readonly receivedContentBytes: number;
  readonly undecodable: number;
}

export class Engine {
  readonly objects = new Objects();
  private link!: Link;
  private readonly put = new Map<string, Uint8Array>();
  private readonly remove = new Set<string>();
  private stateDirty = false;
  private batch: bigint | null = null;
  private committing = false;
  private pumpTimer: ReturnType<typeof setTimeout> | null = null;
  private readonly waiting = new Map<bigint, (reply: unknown) => void>();
  private readonly statusListeners = new Set<() => void>();
  private receivedContentBytes = 0;
  private undecodable = 0;

  private constructor(
    private readonly phone: PhoneWasm,
    private readonly store: Store,
  ) {}

  static async start(input: {
    readonly wasmUrl: string;
    readonly store: Store;
    readonly saved: Saved;
    readonly pairing: Pairing;
    readonly realFetch: typeof fetch;
  }): Promise<Engine> {
    const phone = await PhoneWasm.load(input.wasmUrl);
    const now = performance.now();
    const { saved } = input;
    let loaded = false;
    if (saved.state !== null) {
      try {
        phone.load(saved.state, saved.objects, now);
        loaded = true;
      } catch (error) {
        console.warn("t3opt: stored sync state unreadable, syncing from zero", error);
      }
    }
    if (!loaded) {
      // A new epoch tells the sidecar this phone has nothing.
      phone.create(hexBytes(input.pairing.token), randomEpoch(), now);
      await input.store.clearContent();
      await input.store.set({ state: phone.saveState() });
    }
    const engine = new Engine(phone, input.store);
    for (const id of phone.objectIds()) engine.objects.set(id, phone.object(id));
    engine.link = new Link(
      input.pairing,
      saved,
      { save: (values) => input.store.set(values) },
      {
        onPacket: (data) => engine.onPacket(data),
        onNewConnection: () => {
          phone.connected();
          engine.pump();
        },
        onChange: () => engine.statusChanged(),
        onWritable: () => engine.pump(),
      },
      input.realFetch,
    );
    await engine.link.start();
    engine.pump();
    return engine;
  }

  status(): EngineStatus {
    return {
      link: this.link.info(),
      stats: this.phone.stats(),
      receivedContentBytes: this.receivedContentBytes,
      undecodable: this.undecodable,
    };
  }

  onStatus(listener: () => void): () => void {
    this.statusListeners.add(listener);
    return () => this.statusListeners.delete(listener);
  }

  private statusChanged() {
    for (const l of this.statusListeners) l();
  }

  private onPacket(data: Uint8Array) {
    let r;
    try {
      r = this.phone.receive(data, performance.now());
    } catch (error) {
      this.undecodable += 1;
      console.warn("t3opt: packet not read", error);
      return;
    }
    const changed = new Set<bigint>();
    for (const { key, saved } of r.changed) {
      const name = keyName(key.kind, key.id);
      this.put.set(name, saved);
      this.remove.delete(name);
      if (key.kind === 0) changed.add(key.id);
    }
    for (const key of r.removed) {
      const name = keyName(key.kind, key.id);
      this.put.delete(name);
      this.remove.add(name);
    }
    if (r.replies.length > 0) this.stateDirty = true;
    this.batch = r.batch;
    this.receivedContentBytes += r.newBytes;
    for (const id of changed) this.objects.set(id, this.phone.object(id));
    for (const reply of r.replies) {
      const resolve = this.waiting.get(reply.seq);
      this.waiting.delete(reply.seq);
      let value: unknown = null;
      try {
        value = JSON.parse(decoder.decode(reply.payload));
      } catch {
        value = { error: "the reply is not JSON" };
      }
      resolve?.(value);
    }
    this.objects.notify(changed);
    this.statusChanged();
    void this.flush();
    this.pump();
  }

  /** Commits what arrived, then lets the phone ack it. One commit at a time. */
  private async flush(): Promise<void> {
    if (this.committing || this.batch === null) return;
    const batch = this.batch;
    const commit = {
      put: new Map(this.put),
      remove: new Set(this.remove),
      state: this.stateDirty ? this.phone.saveState() : null,
    };
    this.put.clear();
    this.remove.clear();
    this.stateDirty = false;
    this.batch = null;
    this.committing = true;
    try {
      await this.store.commit(commit);
      this.phone.committed(batch);
    } catch (error) {
      // Not acked: the sidecar sends it again. Keep what is newer.
      console.warn("t3opt: IndexedDB commit failed", error);
      for (const [k, v] of commit.put)
        if (!this.put.has(k) && !this.remove.has(k)) this.put.set(k, v);
      for (const k of commit.remove) if (!this.put.has(k)) this.remove.add(k);
      this.stateDirty ||= commit.state !== null;
    } finally {
      this.committing = false;
    }
    if (this.batch !== null) void this.flush();
    this.pump();
  }

  /** Sends what the phone has to send now, and wakes when it next has. */
  pump(): void {
    const now = performance.now();
    let sent = false;
    while (this.link.canSend()) {
      const p = this.phone.poll(now);
      if (p === null) break;
      this.link.send(p);
      sent = true;
    }
    if (sent) this.statusChanged();
    const at = this.phone.nextWakeup(now);
    if (this.pumpTimer !== null) clearTimeout(this.pumpTimer);
    this.pumpTimer = setTimeout(() => this.pump(), Math.max(1, Math.min(1_000, at - now)));
  }

  /**
   * Sends a command (`orchestration.dispatchCommand`) or a call through the
   * phone's log. The log is stored before it goes out. Resolves with the reply.
   */
  async request(
    kind: "command" | "call",
    payload: unknown,
    background = false,
  ): Promise<{ seq: bigint; reply: Promise<unknown> }> {
    const bytes = encoder.encode(JSON.stringify(payload));
    const seq = kind === "command" ? this.phone.command(bytes) : this.phone.call(bytes, background);
    const reply = new Promise<unknown>((resolve) => this.waiting.set(seq, resolve));
    await this.store.set({ state: this.phone.saveState() });
    this.pump();
    this.statusChanged();
    return { seq, reply };
  }

  entryState(seq: bigint): EntryState | null {
    return this.phone.entryState(seq);
  }
}
