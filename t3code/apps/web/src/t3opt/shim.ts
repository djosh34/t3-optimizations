// The in-page server the web client talks to (issue #32): a WebSocket to /ws
// and fetches to the page's own API paths are answered here, from synced state
// or through the sidecar. See README.md in this folder.

import type { Engine } from "./engine";
import { configOf, imageBlob, imageComing, shellSnapshot, threadSnapshot } from "./snapshots";
import type { Pairing } from "./store";

type Json = Record<string, unknown>;

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
  if (shimTrace.length > 1000) shimTrace.shift();
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

/**
 * A synced image's `blob:` URL, one per attachment for the page's life. Null
 * when its bytes did not come within `IMAGE_WAIT_MS` (the sidecar's fetch can
 * fail): the call goes to the sidecar then.
 */
const imageUrls = new Map<string, string>();
const IMAGE_WAIT_MS = 120_000;

async function imageUrl(engine: Engine, attachment: string): Promise<string | null> {
  const known = imageUrls.get(attachment);
  if (known !== undefined) return known;
  const blob = await new Promise<Blob | null>((resolve) => {
    const now = imageBlob(engine.objects, attachment);
    if (now !== null) {
      resolve(now);
      return;
    }
    const done = (b: Blob | null) => {
      stop();
      clearTimeout(timer);
      resolve(b);
    };
    const stop = engine.objects.subscribe(() => {
      const b = imageBlob(engine.objects, attachment);
      if (b !== null) done(b);
    });
    const timer = setTimeout(() => done(null), IMAGE_WAIT_MS);
  });
  if (blob === null) return null;
  const url = imageUrls.get(attachment) ?? URL.createObjectURL(blob);
  imageUrls.set(attachment, url);
  return url;
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

/**
 * Calls the client makes by itself, not for what the owner looks at or taps:
 * their replies wait behind everything the phone is syncing.
 */
const BACKGROUND_CALLS = new Set([
  "server.reportClientActivity",
  "preview.reportProfiles",
  "attachments.delete",
  "subscribeResourceTelemetry",
  "subscribeDeviceState",
  "subscribePreviewEvents",
  "subscribeTerminalMetadata",
  "subscribeDiscoveredLocalServers",
  "subscribeProjectClones",
  "pullRequests.subscribeRefreshes",
  "scheduledTasks.subscribe",
]);

/**
 * Calls the shim answers at once from their last reply, while the sidecar
 * answers them again in the background. The client refreshes its providers
 * whenever the owner types, and takes no Enter until the reply is here: on
 * case 50 a full round trip each time (issue #46).
 */
const ANSWERED_FROM_LAST_REPLY = new Set(["server.refreshProviders"]);
const lastReplies = new Map<string, unknown>();

export function isBackgroundCall(tag: string): boolean {
  return BACKGROUND_CALLS.has(tag);
}

const FOLLOW_INTERVAL_MS = 250;

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
    if (frame._tag !== "Request")
      trace(`in ${String(frame._tag)} ${String(frame.requestId ?? "")}`);
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

  /**
   * A stream answered from synced state: `values` again on every change, at
   * most every `FOLLOW_INTERVAL_MS`. A thread snapshot is tens of kB: built,
   * sent and rendered for every packet, it kept WebKit's main thread busy for
   * half a minute at a time on case 50 (issue #46).
   */
  private follow(id: RequestId, values: (first: boolean) => unknown[] | null) {
    let first = true;
    let last = "";
    let lastRun = -Infinity;
    let timer: ReturnType<typeof setTimeout> | null = null;
    const update = () => {
      if (timer !== null) return;
      const wait = lastRun + FOLLOW_INTERVAL_MS - performance.now();
      if (wait > 0) {
        timer = setTimeout(() => {
          timer = null;
          run();
        }, wait);
        return;
      }
      run();
    };
    const run = () => {
      lastRun = performance.now();
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
    const unsubscribe = this.engine.objects.subscribe(update);
    const stop = () => {
      unsubscribe();
      if (timer !== null) clearTimeout(timer);
    };
    this.streams.set(String(id), { pending: null, awaitingAck: false, replaces: true, stop });
    update();
  }

  private async request(id: RequestId, tag: string, payload: unknown) {
    const command = tag === "orchestration.dispatchCommand" ? (payload as Json | null) : null;
    trace(`rpc ${id} ${tag}${command ? ` ${String(command.type ?? "")}` : ""}`);
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
        // Images come over the sync link, the last class, so the bytes can still
        // be on their way; the client keeps the URL it gets, so wait for them.
        if (attachment !== null && imageComing(objects, attachment)) {
          const url = await imageUrl(this.engine, attachment);
          if (url !== null) {
            this.exit(id, {
              _tag: "Success",
              value: { relativeUrl: url, expiresAt: Date.now() + 3_600_000 },
            });
            return;
          }
        }
        break;
      }
      case "orchestration.dispatchCommand": {
        // Every command, also those the client sends by itself (thread.visit):
        // the client sends the next one only after the last one's reply.
        const command = payload as Json | null;
        const { seq, reply } = await this.engine.request("command", payload);
        if (command?.type === "message.dispatch") trackPrompt(this.engine, seq, reply);
        const r = readReply(await reply);
        this.exit(id, "exit" in r ? r.exit : die("a command has no stream"));
        return;
      }
    }
    // Everything else: one call to the sidecar, one reply. A call for what is
    // on screen or what a tap needs gets the reply class (#16); the client's
    // own reports, refreshes and side subscriptions go at the background class.
    const key = ANSWERED_FROM_LAST_REPLY.has(tag) ? `${tag} ${JSON.stringify(payload)}` : null;
    const last = key === null ? undefined : lastReplies.get(key);
    if (key !== null && last !== undefined) {
      // Answered; the fresh reply only replaces the last one.
      this.exit(id, last);
      void this.engine
        .request("call", { tag, payload }, true)
        .then(({ reply }) => reply)
        .then((value) => {
          const r = readReply(value);
          if ("exit" in r && (r.exit as Json | null)?._tag === "Success") lastReplies.set(key, r.exit);
        })
        .catch((error: unknown) => trace(`refresh of ${tag} failed: ${String(error)}`));
      return;
    }
    const { reply } = await this.engine.request("call", { tag, payload }, isBackgroundCall(tag));
    const r = readReply(await reply);
    if ("exit" in r && key !== null && (r.exit as Json | null)?._tag === "Success") {
      lastReplies.set(key, r.exit);
    }
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
  const stop = engine.onStatus(() => {
    const state = engine.entryState(seq);
    if (state !== null && state !== shimEvents.prompt?.state) set(state);
  });
  void reply.then((r) => {
    stop();
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
  if (ctx.engine === null)
    return json(401, { _tag: "EnvironmentAuthInvalidError", reason: "unpaired" });
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
  const thread = /^\/api\/orchestration\/threads\/([^/]+)(?:\/bounded)?$/.exec(path);
  if (thread && request.method === "GET") {
    const threadId = decodeURIComponent(thread[1] ?? "");
    // The bounded snapshot's fields are the full one's and more.
    return json(200, await whenReady(engine, () => threadSnapshot(objects, threadId)));
  }
  // Everything else goes to the forked server through the sidecar, at the
  // background class like other calls.
  const bytes = new Uint8Array(await request.arrayBuffer());
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
    true,
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
    return handleFetch(input, request).then((response) => {
      trace(`fetched ${request.method} ${url.pathname} ${response.status}`);
      return response;
    });
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
  window.XMLHttpRequest = shimXhr(input);
}

/**
 * XMLHttpRequest, which the client uploads attachments with (for their
 * progress), with requests to the server's paths answered like fetches.
 */
function shimXhr(ctx: FetchContext): typeof XMLHttpRequest {
  const Real = window.XMLHttpRequest;
  return class ShimXhr extends Real {
    private shimmed: { method: string; url: string; headers: Headers } | null = null;
    private shimStatus = 0;
    private shimText = "";
    private shimAborted = false;

    override open(method: string, url: string | URL, ...rest: unknown[]) {
      const u = new URL(url, location.href);
      if (!isServerPath(u)) {
        this.shimmed = null;
        (super.open as (...args: unknown[]) => void)(method, url, ...rest);
        return;
      }
      this.shimmed = { method, url: u.toString(), headers: new Headers() };
    }

    override setRequestHeader(name: string, value: string) {
      if (this.shimmed) this.shimmed.headers.set(name, value);
      else super.setRequestHeader(name, value);
    }

    override get status() {
      return this.shimmed ? this.shimStatus : super.status;
    }

    override get responseText() {
      return this.shimmed ? this.shimText : super.responseText;
    }

    override abort() {
      if (!this.shimmed) return super.abort();
      this.shimAborted = true;
      this.dispatchEvent(new ProgressEvent("abort"));
    }

    override send(body?: Document | XMLHttpRequestBodyInit | null) {
      const shimmed = this.shimmed;
      if (!shimmed) return super.send(body);
      const request = new Request(shimmed.url, {
        method: shimmed.method,
        headers: shimmed.headers,
        ...(body instanceof Document || body === null || body === undefined ? {} : { body }),
      });
      // The client's own timeout: without it, an upload whose reply never
      // comes holds the composer's send for good.
      if (this.timeout > 0) {
        setTimeout(() => {
          if (this.shimStatus !== 0 || this.shimAborted) return;
          this.shimAborted = true;
          this.dispatchEvent(new ProgressEvent("timeout"));
        }, this.timeout);
      }
      handleFetch(ctx, request).then(
        async (response) => {
          if (this.shimAborted) return;
          this.shimStatus = response.status;
          this.shimText = await response.text();
          this.upload.dispatchEvent(
            new ProgressEvent("progress", { lengthComputable: true, loaded: 1, total: 1 }),
          );
          this.dispatchEvent(new ProgressEvent("load"));
          this.dispatchEvent(new ProgressEvent("loadend"));
        },
        () => {
          if (this.shimAborted) return;
          // Settled: the timeout must not fire after the error.
          this.shimAborted = true;
          this.dispatchEvent(new ProgressEvent("error"));
        },
      );
    }
  };
}
