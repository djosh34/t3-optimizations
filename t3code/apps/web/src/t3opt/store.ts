// What the web app keeps between page loads, in IndexedDB: the pairing (device
// and token), the WebRTC certificate and kept answer, and sync-core's saved
// state and objects. The phone acks a packet only after its content is
// committed here, so a page killed in between gets it again.

const DB_NAME = "t3opt";
const KV = "kv";
const OBJECTS = "objects";

export interface Pairing {
  readonly device: string;
  /** 32 hex characters. */
  readonly token: string;
}

export interface Saved {
  readonly pairing: Pairing | null;
  readonly certificate: RTCCertificate | null;
  readonly keptAnswer: string | null;
  readonly generation: number;
  readonly state: Uint8Array | null;
  readonly objects: ReadonlyArray<Uint8Array>;
}

/** One commit: objects to write and delete, and the phone's state. */
export interface Commit {
  readonly put: ReadonlyMap<string, Uint8Array>;
  readonly remove: ReadonlySet<string>;
  readonly state: Uint8Array | null;
}

function request<T>(r: IDBRequest<T>): Promise<T> {
  return new Promise((resolve, reject) => {
    r.onsuccess = () => resolve(r.result);
    r.onerror = () => reject(r.error);
  });
}

function done(t: IDBTransaction): Promise<void> {
  return new Promise((resolve, reject) => {
    t.oncomplete = () => resolve();
    t.onerror = () => reject(t.error);
    t.onabort = () => reject(t.error ?? new Error("IndexedDB transaction aborted"));
  });
}

export class Store {
  private constructor(private readonly db: IDBDatabase) {}

  static async open(): Promise<Store> {
    const open = indexedDB.open(DB_NAME, 1);
    open.onupgradeneeded = () => {
      open.result.createObjectStore(KV);
      open.result.createObjectStore(OBJECTS);
    };
    return new Store(await request(open));
  }

  async load(): Promise<Saved> {
    const t = this.db.transaction([KV, OBJECTS]);
    const kv = t.objectStore(KV);
    const get = <T>(key: string) => request(kv.get(key) as IDBRequest<T | undefined>);
    const [pairing, certificate, keptAnswer, generation, state, objects] = await Promise.all([
      get<Pairing>("pairing"),
      get<RTCCertificate>("certificate"),
      get<string>("keptAnswer"),
      get<number>("generation"),
      get<Uint8Array>("state"),
      request(t.objectStore(OBJECTS).getAll() as IDBRequest<Uint8Array[]>),
    ]);
    return {
      pairing: pairing ?? null,
      certificate: certificate ?? null,
      keptAnswer: keptAnswer ?? null,
      generation: generation ?? 0,
      state: state ?? null,
      objects,
    };
  }

  async set(values: Record<string, unknown>): Promise<void> {
    const t = this.db.transaction(KV, "readwrite");
    for (const [key, value] of Object.entries(values)) t.objectStore(KV).put(value, key);
    await done(t);
  }

  /** Resolves once the commit is durable. */
  async commit(c: Commit): Promise<void> {
    const t = this.db.transaction([KV, OBJECTS], "readwrite");
    const objects = t.objectStore(OBJECTS);
    for (const [key, bytes] of c.put) objects.put(bytes, key);
    for (const key of c.remove) objects.delete(key);
    if (c.state !== null) t.objectStore(KV).put(c.state, "state");
    await done(t);
  }

  /** Drops the synced content but keeps the pairing: the next start syncs from zero. */
  async clearContent(): Promise<void> {
    const t = this.db.transaction([KV, OBJECTS], "readwrite");
    t.objectStore(OBJECTS).clear();
    t.objectStore(KV).delete("state");
    await done(t);
  }

  /** Forgets everything; the app pairs again. */
  async clearAll(): Promise<void> {
    const t = this.db.transaction([KV, OBJECTS], "readwrite");
    t.objectStore(OBJECTS).clear();
    t.objectStore(KV).clear();
    await done(t);
  }
}
