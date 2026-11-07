import { describe, expect, it } from "vitest";

import { Objects } from "./objects";
import { shellSnapshot, threadSnapshot } from "./snapshots";

const encoder = new TextEncoder();

/** Puts an object as the sync engine would: meta, and the byte ranges the phone has. */
function put(
  objects: Objects,
  id: number,
  meta: object,
  content: string | object,
  ranges?: Array<[number, number]>,
) {
  const data = encoder.encode(typeof content === "string" ? content : JSON.stringify(content));
  const have = ranges ?? [[0, data.length]];
  const kept = new Uint8Array(data.length);
  for (const [s, e] of have) kept.set(data.subarray(s, e), s);
  objects.set(BigInt(id), {
    meta: encoder.encode(JSON.stringify(meta)),
    rev: 1n,
    len: data.length,
    ranges: have,
    data: kept,
  });
}

function thread(objects: Objects) {
  put(
    objects,
    1,
    { t: "t1", c: "thread", i: "thread" },
    { id: "t1", updatedAt: "2026-01-01T00:00:00.000Z" },
  );
  put(objects, 2, { t: "t1", c: "runs", i: "r1" }, { id: "r1", status: "running" });
}

function item(id: string, type: string, ordinal: number, messageId: string) {
  return { id, type, ordinal, messageId, runId: "r1", nodeId: null, text: "" };
}

describe("threadSnapshot", () => {
  it("is null until the thread body is here", () => {
    const objects = new Objects();
    expect(threadSnapshot(objects, "t1")).toBeNull();
    thread(objects);
    expect(threadSnapshot(objects, "t1")?.projection.visibleTurnItems).toEqual([]);
  });

  it("fills an assistant text in as it arrives, newest bytes first", () => {
    const objects = new Objects();
    thread(objects);
    put(objects, 3, { t: "t1", c: "turnItems", i: "a" }, item("a", "assistant_message", 2, "m-a"));
    put(objects, 4, { t: "t1", m: "m-a", k: "assistant" }, "Hello, world. Bye.", [[14, 18]]);
    const rows = threadSnapshot(objects, "t1")?.projection.visibleTurnItems as Array<{
      item: { text: string };
    }>;
    expect(rows.map((r) => r.item.text)).toEqual(["…Bye."]);
  });

  it("holds a user message back until its text is complete", () => {
    const objects = new Objects();
    thread(objects);
    put(objects, 3, { t: "t1", c: "turnItems", i: "u" }, item("u", "user_message", 1, "m-u"));
    put(objects, 4, { t: "t1", c: "messages", i: "m-u" }, { id: "m-u", role: "user", text: "" });
    const before = threadSnapshot(objects, "t1")?.projection;
    expect(before?.visibleTurnItems).toEqual([]);
    expect(before?.messages).toEqual([]);
    put(objects, 5, { t: "t1", m: "m-u", k: "user" }, "Hi there");
    const after = threadSnapshot(objects, "t1")?.projection as {
      visibleTurnItems: Array<{ item: { text: string }; position: number }>;
      messages: Array<{ text: string }>;
    };
    expect(after.visibleTurnItems.map((r) => [r.position, r.item.text])).toEqual([[0, "Hi there"]]);
    expect(after.messages.map((m) => m.text)).toEqual(["Hi there"]);
  });

  it("orders rows by ordinal and keeps the frame's inherited rows first", () => {
    const objects = new Objects();
    thread(objects);
    put(objects, 3, { t: "t1", c: "turnItems", i: "b" }, item("b", "assistant_message", 5, "m-b"));
    put(objects, 4, { t: "t1", c: "turnItems", i: "a" }, item("a", "assistant_message", 2, "m-a"));
    const inherited = {
      visibility: "inherited",
      sourceItemId: "p",
      item: { id: "p", type: "user_message" },
    };
    put(
      objects,
      5,
      { t: "t1", c: "frame" },
      { snapshotSequence: 7, visibleTurnItems: [inherited] },
    );
    const s = threadSnapshot(objects, "t1");
    const rows = s?.projection.visibleTurnItems as Array<{
      sourceItemId: string;
      position: number;
    }>;
    expect(rows.map((r) => [r.position, r.sourceItemId])).toEqual([
      [0, "p"],
      [1, "a"],
      [2, "b"],
    ]);
    expect(s?.snapshotSequence).toBe(7);
    expect(s?.hasMoreHistory).toBe(false);
  });
});

describe("shellSnapshot", () => {
  it("leaves out a missing olderSettled and splits active from archived threads", () => {
    const objects = new Objects();
    expect(shellSnapshot(objects)).toBeNull();
    put(objects, 1, { c: "shell" }, { schemaVersion: 2, snapshotSequence: 3, olderSettled: null });
    put(objects, 2, { c: "thread", i: "t1", l: "active" }, { id: "t1" });
    put(objects, 3, { c: "thread", i: "t2", l: "archive" }, { id: "t2" });
    put(objects, 4, { c: "project", i: "p1" }, { id: "p1" });
    put(objects, 5, { c: "thread", i: "t3", l: "active" }, "");
    expect(shellSnapshot(objects)).toEqual({
      schemaVersion: 2,
      snapshotSequence: 3,
      projects: [{ id: "p1" }],
      threads: [{ id: "t1" }],
      archivedThreads: [{ id: "t2" }],
    });
  });
});
