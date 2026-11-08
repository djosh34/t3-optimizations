import { describe, expect, it, vi } from "vite-plus/test";

import type { Engine } from "./engine";
import { Objects } from "./objects";
import { isBackgroundCall, RpcServer } from "./shim";

describe("isBackgroundCall", () => {
  it.each([
    // What the owner sees or taps: the reply class, ahead of history (#16).
    ["subscribeServerLifecycle", false],
    ["subscribeAuthAccess", false],
    ["subscribeVcsStatus", false],
    ["server.getSettings", false],
    ["attachments.createUploadUrl", false],
    ["git.status", false],
    // Also when the client calls them by itself: the name cannot tell a tap from that.
    ["projects.readFile", false],
    ["preview.list", false],
    ["server.refreshProviders", false],
    // What the client does by itself.
    ["server.reportClientActivity", true],
    ["preview.reportProfiles", true],
    ["subscribeResourceTelemetry", true],
  ])("%s goes at the background class: %s", (tag, background) => {
    expect(isBackgroundCall(tag)).toBe(background);
  });
});

const encoder = new TextEncoder();

function putConfig(objects: Objects, version: number) {
  const data = encoder.encode(JSON.stringify({ version }));
  objects.set(1n, {
    meta: encoder.encode(JSON.stringify({ c: "config" })),
    rev: 1n,
    len: data.length,
    ranges: [[0, data.length]],
    data,
  });
  objects.notify(new Set([1n]));
}

interface Frame {
  readonly _tag: string;
  readonly requestId?: unknown;
  readonly values?: ReadonlyArray<{ readonly config?: { readonly version?: number } }>;
  readonly exit?: { readonly value?: unknown };
}

/** An engine whose calls answer with the replies given, in order. */
function fakeEngine(replies: unknown[]) {
  const request = vi.fn((_kind: string, _payload: unknown, _background?: boolean) =>
    Promise.resolve({ seq: 1n, reply: Promise.resolve(replies.shift()) }),
  );
  return { engine: { objects: new Objects(), request } as unknown as Engine, request };
}

function server(engine: Engine) {
  const sent: Frame[] = [];
  return { rpc: new RpcServer(engine, (frame) => sent.push(frame as Frame)), sent };
}

const configVersions = (sent: Frame[]) =>
  sent.filter((f) => f._tag === "Chunk").map((f) => f.values?.[0]?.config?.version);

describe("a followed stream", () => {
  it("sends at most one update per 250 ms, the newest", async () => {
    vi.useFakeTimers({ toFake: ["setTimeout", "clearTimeout", "performance"] });
    try {
      const { engine } = fakeEngine([]);
      putConfig(engine.objects, 1);
      const { rpc, sent } = server(engine);
      rpc.handle({ _tag: "Request", id: 7, tag: "subscribeServerConfig", payload: {} });
      rpc.handle({ _tag: "Ack", requestId: 7 });
      putConfig(engine.objects, 2);
      putConfig(engine.objects, 3);
      expect(configVersions(sent)).toEqual([1]);
      await vi.advanceTimersByTimeAsync(250);
      expect(configVersions(sent)).toEqual([1, 3]);
    } finally {
      vi.useRealTimers();
    }
  });

  it("sends nothing more once closed", async () => {
    vi.useFakeTimers({ toFake: ["setTimeout", "clearTimeout", "performance"] });
    try {
      const { engine } = fakeEngine([]);
      putConfig(engine.objects, 1);
      const { rpc, sent } = server(engine);
      rpc.handle({ _tag: "Request", id: 7, tag: "subscribeServerConfig", payload: {} });
      rpc.handle({ _tag: "Ack", requestId: 7 });
      putConfig(engine.objects, 2);
      rpc.close();
      await vi.advanceTimersByTimeAsync(1_000);
      expect(configVersions(sent)).toEqual([1]);
    } finally {
      vi.useRealTimers();
    }
  });
});

describe("server.refreshProviders", () => {
  const success = (value: unknown) => ({ exit: { _tag: "Success", value } });
  const exitsFor = (sent: Frame[], id: number) =>
    sent.filter((f) => f._tag === "Exit" && f.requestId === id).map((f) => f.exit?.value);

  it("waits for the sidecar the first time, then answers at once from the last reply", async () => {
    const { engine, request } = fakeEngine([success("first"), success("second")]);
    const { rpc, sent } = server(engine);
    const payload = { cwd: "/a" };
    rpc.handle({ _tag: "Request", id: 1, tag: "server.refreshProviders", payload });
    await vi.waitFor(() => expect(exitsFor(sent, 1)).toEqual(["first"]));
    rpc.handle({ _tag: "Request", id: 2, tag: "server.refreshProviders", payload });
    // Answered before the sidecar replies, and refreshed in the background.
    expect(exitsFor(sent, 2)).toEqual(["first"]);
    expect(request).toHaveBeenLastCalledWith(
      "call",
      { tag: "server.refreshProviders", payload },
      true,
    );
    // The background reply replaces the last one once its promise chain has
    // run; no timer is involved, only queued promise steps.
    const background = await (request.mock.results[1]?.value as Promise<{ reply: unknown }>);
    await background.reply;
    for (let step = 0; step < 5; step++) await Promise.resolve();
    rpc.handle({ _tag: "Request", id: 3, tag: "server.refreshProviders", payload });
    expect(exitsFor(sent, 3)).toEqual(["second"]);
    expect(exitsFor(sent, 2)).toEqual(["first"]);
  });

  it("keeps the replies of different payloads apart", async () => {
    const { engine } = fakeEngine([success("for b"), success("for c")]);
    const { rpc, sent } = server(engine);
    rpc.handle({ _tag: "Request", id: 1, tag: "server.refreshProviders", payload: { cwd: "/b" } });
    await vi.waitFor(() => expect(exitsFor(sent, 1)).toEqual(["for b"]));
    rpc.handle({ _tag: "Request", id: 2, tag: "server.refreshProviders", payload: { cwd: "/c" } });
    await vi.waitFor(() => expect(exitsFor(sent, 2)).toEqual(["for c"]));
  });

  it("answers once when the background refresh fails", async () => {
    const { engine, request } = fakeEngine([success("ok")]);
    const { rpc, sent } = server(engine);
    const payload = { cwd: "/d" };
    rpc.handle({ _tag: "Request", id: 1, tag: "server.refreshProviders", payload });
    await vi.waitFor(() => expect(exitsFor(sent, 1)).toEqual(["ok"]));
    request.mockImplementationOnce(() => Promise.reject(new Error("link closed")));
    rpc.handle({ _tag: "Request", id: 2, tag: "server.refreshProviders", payload });
    await vi.waitFor(() => expect(request).toHaveBeenCalledTimes(2));
    expect(exitsFor(sent, 2)).toEqual(["ok"]);
  });
});
