import { describe, expect, it, vi } from "vite-plus/test";

import { Engine } from "./engine";

/** A phone whose log holds one entry per request; poll sends each once. */
function fakePhone() {
  const out: Uint8Array[] = [];
  let seq = 0n;
  return {
    command: () => {
      seq += 1n;
      out.push(new Uint8Array([Number(seq)]));
      return seq;
    },
    call: () => 0n,
    poll: () => out.shift() ?? null,
    nextWakeup: (now: number) => now + 60_000,
    saveState: () => new Uint8Array([Number(seq)]),
  };
}

/** A store whose writes the test settles one by one. */
function heldStore() {
  const writes: Array<{ resolve: () => void; reject: (e: Error) => void }> = [];
  return {
    writes,
    set: () => new Promise<void>((resolve, reject) => writes.push({ resolve, reject })),
  };
}

function engineWith(store: ReturnType<typeof heldStore>) {
  const sent: Uint8Array[] = [];
  const make = Engine as unknown as new (phone: unknown, store: unknown) => Engine;
  const engine = new make(fakePhone(), store);
  (engine as unknown as { link: unknown }).link = {
    canSend: () => true,
    send: (p: Uint8Array) => sent.push(p),
  };
  return { engine, sent };
}

describe("Engine.request", () => {
  vi.useFakeTimers();

  it("sends a command only once the log that holds it is stored", async () => {
    const store = heldStore();
    const { engine, sent } = engineWith(store);
    const request = engine.request("command", { type: "message.dispatch" });
    await vi.advanceTimersByTimeAsync(0);
    // A packet or the timer pumps while IndexedDB has not committed the log.
    engine.pump();
    expect(sent).toHaveLength(0);
    store.writes[0]?.resolve();
    await request;
    expect(sent).toHaveLength(1);
  });

  it("keeps it back while a failed write is tried again", async () => {
    const store = heldStore();
    const { engine, sent } = engineWith(store);
    const request = engine.request("command", { type: "message.dispatch" });
    await vi.advanceTimersByTimeAsync(0);
    store.writes[0]?.reject(new Error("quota"));
    await vi.advanceTimersByTimeAsync(150);
    engine.pump();
    expect(sent).toHaveLength(0);
    store.writes[1]?.resolve();
    await request;
    expect(sent).toHaveLength(1);
  });
});
