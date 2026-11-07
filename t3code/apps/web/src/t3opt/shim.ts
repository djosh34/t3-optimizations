// The in-page server the web client talks to (issue #32). The client keeps
// its own RPC and HTTP code; the shim takes the place of the forked server
// behind it:
// - a WebSocket to `/ws` is a `ShimSocket` that speaks the same Effect RPC
//   JSON. Subscriptions to the thread list, a thread and the server config
//   are answered from synced state; `orchestration.dispatchCommand` goes out
//   as a sync-core command; every other call goes to the sidecar as one call
//   and one reply.
// - fetch to the page's own `/api`, `/oauth` and `/.well-known` paths:
//   pairing and the session are answered here, the snapshot loaders from
//   synced state, and every other request goes through the sidecar
//   (`sidecar.http`), uploads at the background class.

import type { Engine } from "./engine";
import { configOf, imageBlob, shellSnapshot, threadSnapshot } from "./snapshots";
import type { Pairing } from "./store";

type Json = Record<string, unknown>;

/** Streams that are subscriptions or background work; their calls go at the background class. */
const BACKGROUND_STREAMS = new Set([
  "providerAuthSubscribe",
  "providerInstallSubscribe",
  "orchestration.subscribeCommandOutput",
  "subscribeAuthAccess",
  "subscribeServerLifecycle",
  "scheduledTasksSubscribe",
  "server.getStorageCleanupReport",
  "server.getDatabaseStorage",
  "subscribeTerminalMetadata",
  "subscribePreviewEvents",
  "subscribeDiscoveredLocalServers",
  "subscribeDeviceState",
  "subscribeResourceTelemetry",
  "pullRequestsSubscribeRefreshes",
  "subscribeVcsStatus",
  "subscribeWorktreeSetup",
  "subscribeProjectClones",
]);

/** How the shim answers the client before the app is paired. */
const UNPAIRED_AUTH = {
  policy: "remote-reachable",
  bootstrapMethods: ["one-time-token"],
  sessionMethods: ["browser-session-cookie"],
  sessionCookieName: "t3opt",
};

/** The session the page reports between pairing and its reload. */
const JUST_PAIRED = { scopes: ["orchestration:read"], permissions: ["orchestration:read"] };
let pairedNow = false;

export type PromptState = "queued" | "sent" | "server-has" | "confirmed" | "failed";

export interface PromptInfo {
  readonly seq: bigint;
  readonly state: PromptState;
  readonly at: number;
}

export interface ShimEvents {
  /** The newest prompt the owner sent and where it is. */
  prompt: PromptInfo | null;
  readonly listeners: Set<() => void>;
}

export const shimEvents: ShimEvents = { prompt: null, listeners: new Set() };

/** The last RPC calls and fetches and how they ended, for the details panel and the lab. */
export const shimTrace: string[] = [];

function trace(line: string) {
  shimTrace.push(`${new Date().toISOString().slice(11, 23)} ${line}`);
  if (shimTrace.length > 200) shimTrace.shift();
}

function emitShimEvent() {
  for (const l of shimEvents.listeners) l();
}

function base64(bytes: Uint8Array): string {
  let s = "";
  for (let i = 0; i < bytes.length; i += 0x8000) {
    s += String.fromCharCode(...bytes.subarray(i, i + 0x8000));
  }
  return btoa(s);
}

function fromBase64(s: string): Uint8Array {
  return Uint8Array.from(atob(s), (c) => c.charCodeAt(0));
}

function json(status: number, body: unknown): Response {
  return new Response(JSON.stringify(body), {
    status,
    headers: { "content-type": "application/json" },
  });
}

function die(message: string) {
  return { _tag: "Failure", cause: [{ _tag: "Die", defect: message }] };
}

/** The sidecar's reply to a call as an RPC exit, or chunk values for a stream. */
function readReply(reply: unknown): { exit: unknown } | { chunk: unknown[] } {
  if (typeof reply === "object" && reply !== null) {
    if ("exit" in reply) return { exit: reply.exit };
    if ("chunk" in reply && Array.isArray(reply.chunk)) return { chunk: reply.chunk };
    if ("error" in reply) return { exit: die(String(reply.error)) };
  }
  return { exit: die("the sidecar sent no reply") };
}

/** Resolves once `ready` returns a value; it is checked again on every change of synced state. */
function whenReady<T>(engine: Engine, ready: () => T | null): Promise<T> {
  const now = ready();
  if (now !== null) return Promise.resolve(now);
  return new Promise((resolve) => {
    const stop = engine.objects.subscribe(() => {
      const v = ready();
      if (v === null) return;
      stop();
      resolve(v);
    });
  });
}

// The RPC side.

/** A request id as the client sent it; replies carry it back unchanged. */
type RequestId = string | number;

interface Stream {
  /** The values waiting for the client's ack of the last chunk. */
  pending: unknown[] | null;
  awaitingAck: boolean;
  /** Snapshots replace each other: only the newest waiting one is sent. */
  readonly replaces: boolean;
  stop: () => void;
}

class RpcServer {
  private readonly streams = new Map<string, Stream>();

  constructor(
    private readonly engine: Engine,
    private readonly send: (frame: unknown) => void,
  ) {}

  close() {
    for (const s of this.streams.values()) s.stop();
    this.streams.clear();
  }

  handle(frame: Json) {
    if (frame._tag !== "Request") trace(`in ${String(frame._tag)} ${String(frame.requestId ?? "")}`);
    switch (frame._tag) {
      case "Ping":
        this.send({ _tag: "Pong" });
        return;
      case "Ack": {
        const s = this.streams.get(String(frame.requestId));
        if (!s) return;
        s.awaitingAck = false;
        if (s.pending !== null) {
          const values = s.pending;
          s.pending = null;
          this.chunk(frame.requestId as RequestId, values);
        }
        return;
      }
      case "Interrupt": {
        const id = String(frame.requestId);
        this.streams.get(id)?.stop();
        this.streams.delete(id);
        return;
      }
      case "Request":
        this.request(frame.id as RequestId, String(frame.tag), frame.payload).catch(
          (error: unknown) => {
            trace(`failed ${String(frame.id)} ${String(error)}`);
            this.exit(frame.id as RequestId, die(String(error)));
          },
        );
        return;
    }
  }

  private exit(id: RequestId, exit: unknown) {
    trace(`exit ${id} ${JSON.stringify(exit).slice(0, 160)}`);
    this.streams.get(String(id))?.stop();
    this.streams.delete(String(id));
    this.send({ _tag: "Exit", requestId: id, exit });
  }

  private chunk(id: RequestId, values: unknown[]) {
    const s = this.streams.get(String(id));
    if (!s) return;
    if (s.awaitingAck) {
      s.pending = s.replaces || s.pending === null ? values : [...s.pending, ...values];
      return;
    }
    s.awaitingAck = true;
    trace(`chunk ${id} ${JSON.stringify(values).length} bytes`);
    this.send({ _tag: "Chunk", requestId: id, values });
  }

  /** A stream answered from synced state: `values` again on every change. */
  private follow(id: RequestId, values: (first: boolean) => unknown[] | null) {
    let first = true;
    let last = "";
    const update = () => {
      let v: unknown[] | null;
      try {
        v = values(first);
      } catch (error) {
        trace(`stream ${id} failed: ${String(error)}`);
        return;
      }
      if (v === null) return;
      const text = JSON.stringify(v);
      if (text === last) return;
      last = text;
      first = false;
      this.chunk(id, v);
    };
    const stop = this.engine.objects.subscribe(update);
    this.streams.set(String(id), { pending: null, awaitingAck: false, replaces: true, stop });
    update();
  }

  private async request(id: RequestId, tag: string, payload: unknown) {
    trace(`rpc ${id} ${tag}`);
    const objects = this.engine.objects;
    switch (tag) {
      case "server.probe":
        this.exit(id, { _tag: "Success", value: {} });
        return;
      case "server.getConfig": {
        const config = await whenReady(this.engine, () => configOf(objects, "config"));
        this.exit(id, { _tag: "Success", value: config });
        return;
      }
      case "subscribeServerConfig":
        this.follow(id, () => {
          const config = configOf(objects, "config");
          return config === null ? null : [{ version: 1, type: "snapshot", config }];
        });
        return;
      case "orchestration.subscribeShell":
        this.follow(id, (first) => {
          const snapshot = shellSnapshot(objects);
          if (snapshot === null) return null;
          const item = { kind: "snapshot", snapshot };
          return first ? [item, { kind: "synchronized" }] : [item];
        });
        return;
      case "orchestration.subscribeThread": {
        const threadId = String((payload as Json | null)?.threadId ?? "");
        // The sidecar sends the open thread first.
        void this.engine.request("call", { tag: "sidecar.focus", payload: { threadId } });
        this.follow(id, (first) => {
          const snapshot = threadSnapshot(objects, threadId);
          if (snapshot === null) return null;
          const item = { kind: "snapshot", ...snapshot };
          return first ? [item, { kind: "synchronized" }] : [item];
        });
        return;
      }
      case "assets.createUrl": {
        const resource = (payload as Json | null)?.resource as Json | undefined;
        const attachment = resource?._tag === "attachment" ? String(resource.attachmentId) : null;
        const blob = attachment === null ? null : imageBlob(objects, attachment);
        if (blob !== null) {
          this.exit(id, {
            _tag: "Success",
            value: { relativeUrl: URL.createObjectURL(blob), expiresAt: Date.now() + 3_600_000 },
          });
          return;
        }
        break;
      }
      case "orchestration.dispatchCommand": {
        const command = payload as Json | null;
        const isPrompt = command?.type === "message.dispatch";
        const { seq, reply } = await this.engine.request("command", payload);
        if (isPrompt) trackPrompt(this.engine, seq, reply);
        const r = readReply(await reply);
        this.exit(id, "exit" in r ? r.exit : die("a command has no stream"));
        return;
      }
    }
    // Everything else: one call to the sidecar, one reply.
    const stream = BACKGROUND_STREAMS.has(tag);
    if (stream) {
      this.streams.set(String(id), { pending: null, awaitingAck: false, replaces: false, stop() {} });
    }
    const { reply } = await this.engine.request("call", { tag, payload }, stream);
    const r = readReply(await reply);
    if ("chunk" in r) {
      // The sidecar takes a stream's first chunk; the stream stays open here.
      if (!this.streams.has(String(id))) {
        this.streams.set(String(id), {
          pending: null,
          awaitingAck: false,
          replaces: false,
          stop() {},
        });
      }
      this.chunk(id, r.chunk);
    } else {
      this.exit(id, r.exit);
    }
  }
}

function trackPrompt(engine: Engine, seq: bigint, reply: Promise<unknown>) {
  const set = (state: PromptState) => {
    if (shimEvents.prompt !== null && shimEvents.prompt.seq !== seq) return;
    shimEvents.prompt = { seq, state, at: Date.now() };
    emitShimEvent();
  };
  shimEvents.prompt = null;
  set("queued");
  const poll = setInterval(() => {
    const state = engine.entryState(seq);
    if (state !== null && state !== shimEvents.prompt?.state) set(state);
  }, 200);
  void reply.then((r) => {
    clearInterval(poll);
    const exit = readReply(r);
    const ok = "exit" in exit && (exit.exit as Json | null)?._tag === "Success";
    set(ok ? "confirmed" : "failed");
  });
}

/** A WebSocket to `/ws` that the shim answers. */
class ShimSocket extends EventTarget {
  static readonly CONNECTING = 0;
  static readonly OPEN = 1;
  static readonly CLOSING = 2;
  static readonly CLOSED = 3;
  readonly CONNECTING = 0;
  readonly OPEN = 1;
  readonly CLOSING = 2;
  readonly CLOSED = 3;
  readyState = 0;
  binaryType: BinaryType = "blob";
  readonly bufferedAmount = 0;
  readonly extensions = "";
  readonly protocol = "";
  onopen: ((e: Event) => void) | null = null;
  onmessage: ((e: MessageEvent) => void) | null = null;
  onclose: ((e: CloseEvent) => void) | null = null;
  onerror: ((e: Event) => void) | null = null;
  private server: RpcServer | null = null;

  constructor(
    readonly url: string,
    engine: Promise<Engine>,
  ) {
    super();
    void engine.then((e) => {
      if (this.readyState !== 0) return;
      this.server = new RpcServer(e, (frame) => this.deliver(frame));
      this.readyState = 1;
      this.fire(new Event("open"));
    });
  }

  private fire(e: Event) {
    this.dispatchEvent(e);
    const handler = (this as unknown as Record<string, unknown>)[`on${e.type}`];
    if (typeof handler === "function") handler.call(this, e);
  }

  private deliver(frame: unknown) {
    if (this.readyState !== 1) return;
    // Asynchronous, like a real socket.
    setTimeout(() => this.fire(new MessageEvent("message", { data: JSON.stringify(frame) })), 0);
  }

  send(data: string | ArrayBufferLike | Blob | ArrayBufferView) {
    if (this.readyState !== 1 || typeof data !== "string") return;
    let parsed: unknown;
    try {
      parsed = JSON.parse(data);
    } catch {
      return;
    }
    for (const frame of Array.isArray(parsed) ? parsed : [parsed]) {
      if (typeof frame === "object" && frame !== null) this.server?.handle(frame as Json);
    }
  }

  close(code = 1000, reason = "") {
    if (this.readyState >= 2) return;
    this.readyState = 3;
    this.server?.close();
    this.fire(new CloseEvent("close", { code, reason, wasClean: true }));
  }
}

// The HTTP side.

function isServerPath(url: URL) {
  return (
    url.origin === location.origin &&
    (url.pathname.startsWith("/api/") ||
      url.pathname.startsWith("/oauth/") ||
      url.pathname.startsWith("/.well-known/") ||
      url.pathname.startsWith("/attachments/"))
  );
}

interface FetchContext {
  readonly realFetch: typeof fetch;
  readonly pairing: Pairing | null;
  readonly engine: Promise<Engine> | null;
  readonly pair: (code: string) => Promise<void>;
}

async function handleFetch(ctx: FetchContext, request: Request): Promise<Response> {
  const url = new URL(request.url);
  const path = url.pathname;
  trace(`fetch ${request.method} ${path}`);
  if (path === "/api/auth/session") {
    if (pairedNow) {
      // Paired on this page; the app reloads and the next page syncs.
      return json(200, { authenticated: true, auth: UNPAIRED_AUTH, ...JUST_PAIRED });
    }
    if (ctx.pairing === null || ctx.engine === null) {
      return json(200, { authenticated: false, auth: UNPAIRED_AUTH, permissions: [] });
    }
    const engine = await ctx.engine;
    return json(200, await whenReady(engine, () => configOf(engine.objects, "session")));
  }
  if (path === "/api/auth/browser-session" && request.method === "POST") {
    try {
      const body = (await request.json()) as Json;
      await ctx.pair(String(body.credential ?? ""));
      pairedNow = true;
    } catch (error) {
      return json(401, {
        _tag: "EnvironmentAuthInvalidError",
        reason: "invalid_credential",
        message: error instanceof Error ? error.message : String(error),
      });
    }
    return json(200, {
      authenticated: true,
      ...JUST_PAIRED,
      sessionMethod: "browser-session-cookie",
      expiresAt: new Date(Date.now() + 365 * 24 * 3600 * 1000).toISOString(),
    });
  }
  if (ctx.engine === null) return json(401, { _tag: "EnvironmentAuthInvalidError", reason: "unpaired" });
  const engine = await ctx.engine;
  const objects = engine.objects;
  if (path === "/.well-known/t3/environment") {
    return json(200, await whenReady(engine, () => configOf(objects, "descriptor")));
  }
  // The client's own traces would cost the link more than they are worth.
  if (path.startsWith("/api/observability/")) return new Response(null, { status: 204 });
  if (path.startsWith("/api/auth/websocket-ticket")) {
    return json(200, { ticket: "t3opt", expiresAt: new Date(Date.now() + 300_000).toISOString() });
  }
  if (path === "/api/orchestration/shell" && request.method === "GET") {
    return json(200, await whenReady(engine, () => shellSnapshot(objects)));
  }
  const thread = /^\/api\/orchestration\/threads\/([^/]+)(\/bounded)?$/.exec(path);
  if (thread && request.method === "GET") {
    const threadId = decodeURIComponent(thread[1] ?? "");
    const snapshot = await whenReady(engine, () => threadSnapshot(objects, threadId));
    if (thread[2] !== undefined) return json(200, snapshot);
    const { turnItemsOmitLocalVisible: _, ...rest } = snapshot;
    return json(200, rest);
  }
  // Everything else goes to the forked server through the sidecar.
  const bytes = new Uint8Array(await request.arrayBuffer());
  const upload = bytes.length > 0 && request.method !== "GET";
  const { reply } = await engine.request(
    "call",
    {
      tag: "sidecar.http",
      payload: {
        method: request.method,
        path: path + url.search,
        contentType: request.headers.get("content-type"),
        ...(bytes.length > 0 ? { body: base64(bytes) } : {}),
      },
    },
    upload,
  );
  const r = (await reply) as Json | null;
  const http = r?.http as Json | undefined;
  if (!http) return json(502, { error: String(r?.error ?? "no reply") });
  const headers: Record<string, string> = {};
  if (typeof http.contentType === "string") headers["content-type"] = http.contentType;
  const body = typeof http.body === "string" ? fromBase64(http.body) : new Uint8Array();
  return new Response(Number(http.status) === 204 ? null : body.slice(), {
    status: Number(http.status),
    headers,
  });
}

export interface ShimInput {
  readonly realFetch: typeof fetch;
  readonly pairing: Pairing | null;
  readonly engine: Promise<Engine> | null;
  readonly pair: (code: string) => Promise<void>;
}

/** Puts the shim in front of the page's fetch and WebSocket. */
export function installShim(input: ShimInput): void {
  const RealWebSocket = window.WebSocket;
  window.fetch = (resource: RequestInfo | URL, init?: RequestInit) => {
    const request = new Request(resource, init);
    const url = new URL(request.url);
    if (!isServerPath(url)) return input.realFetch(resource, init);
    return handleFetch(input, request);
  };
  const Shimmed = function (url: string | URL, protocols?: string | string[]) {
    const u = new URL(url, location.href);
    if (u.host === location.host && u.pathname === "/ws" && input.engine !== null) {
      return new ShimSocket(u.toString(), input.engine);
    }
    return new RealWebSocket(url, protocols);
  } as unknown as typeof WebSocket;
  Object.assign(Shimmed, { CONNECTING: 0, OPEN: 1, CLOSING: 2, CLOSED: 3 });
  window.WebSocket = Shimmed;
}
