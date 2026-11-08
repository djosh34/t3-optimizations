import { describe, expect, it } from "vite-plus/test";

import { Objects } from "./objects";
import { imageComing, shellSnapshot, threadSnapshot } from "./snapshots";

const encoder = new TextEncoder();

/** Puts an object as the sync engine would: meta, and the byte ranges the phone has. */
function put(
  objects: Objects,
  id: number,
  meta: object,
  content: string | object,
  ranges?: Array<[number, number]>,
  rev = 1n,
) {
  const data = encoder.encode(typeof content === "string" ? content : JSON.stringify(content));
  const have = ranges ?? [[0, data.length]];
  const kept = new Uint8Array(data.length);
  for (const [s, e] of have) kept.set(data.subarray(s, e), s);
  objects.set(BigInt(id), {
    meta: encoder.encode(JSON.stringify(meta)),
    rev,
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
  return { id, type, ordinal, messageId, runId: "r1", nodeId: null, text: "", streaming: true };
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

  it("shows a newest text whose turn item has not arrived, in a stand-in", () => {
    const objects = new Objects();
    thread(objects);
    put(objects, 3, { t: "t1", c: "turnItems", i: "a" }, item("a", "assistant_message", 2, "m-a"));
    put(objects, 4, { t: "t1", m: "m-a", k: "assistant" }, "Older reply.");
    put(objects, 5, { t: "t1", m: "m-b", k: "assistant" }, "Newest text", [[7, 11]]);
    const rows = threadSnapshot(objects, "t1")?.projection.visibleTurnItems as Array<{
      sourceItemId: string;
      item: { text: string; streaming: boolean; messageId: string };
    }>;
    expect(rows.map((r) => [r.sourceItemId, r.item.text, r.item.streaming])).toEqual([
      ["a", "Older reply.", true],
      ["turn-item:message:m-b", "…text", true],
    ]);
    // The real item takes its place.
    put(
      objects,
      6,
      { t: "t1", c: "turnItems", i: "turn-item:message:m-b" },
      {
        ...item("turn-item:message:m-b", "assistant_message", 3, "m-b"),
        streaming: false,
      },
    );
    const after = threadSnapshot(objects, "t1")?.projection.visibleTurnItems as Array<{
      item: { streaming: boolean };
    }>;
    expect(after).toHaveLength(2);
    expect(after[1]?.item.streaming).toBe(false);
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

  it("streams a growing text, not its shorter complete self", () => {
    const objects = new Objects();
    thread(objects);
    put(objects, 3, { t: "t1", c: "turnItems", i: "a" }, item("a", "assistant_message", 1, "m-a"));
    put(objects, 4, { t: "t1", m: "m-a", k: "assistant" }, "Hello");
    put(objects, 4, { t: "t1", m: "m-a", k: "assistant" }, "Hello world", [[0, 8]]);
    const texts = () => {
      const rows = threadSnapshot(objects, "t1")?.projection.visibleTurnItems as
        | Array<{ item: { text: string } }>
        | undefined;
      return (rows ?? []).map((r) => r.item.text);
    };
    expect(texts()).toEqual(["Hello wo…"]);
    // Also in a new revision after a complete one, partly here and growing:
    // its newest bytes show, not the old revision.
    put(objects, 4, { t: "t1", m: "m-a", k: "assistant" }, "Hello world");
    put(objects, 4, { t: "t1", m: "m-a", k: "assistant" }, "Hallo world!", [[0, 3]], 2n);
    put(
      objects,
      4,
      { t: "t1", m: "m-a", k: "assistant" },
      "Hallo world!",
      [
        [0, 3],
        [6, 12],
      ],
      2n,
    );
    expect(texts()).toEqual(["Hal…world!"]);
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

  it("keeps a thread's row while its new revision arrives", () => {
    const objects = new Objects();
    put(objects, 1, { c: "shell" }, { schemaVersion: 2, snapshotSequence: 3 });
    put(objects, 2, { c: "thread", i: "t1", l: "active" }, { id: "t1", visitedAt: "a" });
    const next = { id: "t1", visitedAt: "b" };
    put(objects, 2, { c: "thread", i: "t1", l: "active" }, next, [[0, 5]], 2n);
    expect(shellSnapshot(objects)?.threads).toEqual([{ id: "t1", visitedAt: "a" }]);
    put(objects, 2, { c: "thread", i: "t1", l: "active" }, next, undefined, 2n);
    expect(shellSnapshot(objects)?.threads).toEqual([next]);
  });
});

describe("imageComing", () => {
  it("is true for an image a turn item refers to, before any of its bytes", () => {
    const objects = new Objects();
    thread(objects);
    expect(imageComing(objects, "img1")).toBe(false);
    put(
      objects,
      3,
      { t: "t1", c: "turnItems", i: "u" },
      {
        ...item("u", "user_message", 1, "m-u"),
        attachments: [
          { type: "file", id: "doc1" },
          { type: "image", id: "img1", mimeType: "image/png" },
        ],
      },
    );
    expect(imageComing(objects, "img1")).toBe(true);
    expect(imageComing(objects, "doc1")).toBe(false);
  });
});
