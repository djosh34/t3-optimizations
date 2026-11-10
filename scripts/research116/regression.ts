// bun scripts/research116/regression.ts
// The historical-text arrival seen in the WebKit recording, using v0.2.0's
// real snapshot reconstruction. No dependency install or live server needed.
import { mkdtemp, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import assert from "node:assert/strict";
import { resolve } from "node:path";
import { Objects } from "../../t3code/apps/web/src/t3opt/objects";

const dir = await mkdtemp(resolve(tmpdir(), "research116-regression-"));
const build = await Bun.build({
  entrypoints: [resolve(import.meta.dir, "../../t3code/apps/web/src/t3opt/snapshots.ts")],
  target: "bun",
  plugins: [{
    name: "research116-shared-source",
    setup(build) {
      build.onResolve({ filter: /^@t3tools\/shared\/orchestrationV2Timeline$/ }, () => ({
        path: resolve(import.meta.dir, "../../t3code/packages/shared/src/orchestrationV2Timeline.ts"),
      }));
    },
  }],
});
assert.ok(build.success, String(build.logs));
const file = resolve(dir, "snapshot.mjs");
await writeFile(file, await build.outputs[0]!.text());
const { threadSnapshot } = await import(file);
await rm(dir, { recursive: true });
const objects = new Objects((id) => `hash:${id}`);
let id = 0n;
function put(meta: Record<string, string>, content: unknown) {
  const data = new TextEncoder().encode(typeof content === "string" ? content : JSON.stringify(content));
  objects.set(++id, { meta: new TextEncoder().encode(JSON.stringify(meta)), data,
    rev: 1n, len: data.length, ranges: [[0, data.length]] });
}
put({ t: objects.hash("thread"), c: "thread" }, { id: "thread" });
const latest = { id: "item-599", messageId: "message-599", threadId: "thread",
  type: "assistant_message", runId: null, ordinal: 599, text: "", streaming: false, status: "completed" };
put({ t: objects.hash("thread"), c: "turnItems" }, latest);
put({ t: objects.hash("thread"), k: "assistant", m: objects.hash("message-599") }, "newest reply");
// History text is queued separately from its body. It belongs at ordinal 545,
// but only its hashed message id is available in the text object's metadata.
put({ t: objects.hash("thread"), k: "assistant", m: objects.hash("message-545") }, "older reply");
const before = threadSnapshot(objects, "thread")!.projection.turnItems as Record<string, unknown>[];
put({ t: objects.hash("thread"), c: "turnItems" }, {
  ...latest, id: "item-545", messageId: "message-545", ordinal: 545,
});
const after = threadSnapshot(objects, "thread")!.projection.turnItems as Record<string, unknown>[];
const beforeOld = before.find(i => i.text === "older reply")!;
const afterOld = after.find(i => i.text === "older reply")!;
console.log(JSON.stringify({ before: before.map(i => [i.id, i.ordinal, i.text]),
  after: after.map(i => [i.id, i.ordinal, i.text]) }, null, 2));
assert.ok(Number(beforeOld.ordinal) < 599,
  "History text must not appear after the newest reply while its item is in transit");
assert.equal(beforeOld.messageId, afterOld.messageId,
  "The timeline key must not change when a placeholder becomes a real row");
