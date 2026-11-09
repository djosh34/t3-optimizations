// Logs when a t3 server releases assistant text to its clients: a client of
// the server on its own host, subscribed to the given threads like any
// client. The lab grades the newest-text lag from these times (issue #59).
// Usage: node t3-watch.mjs <server url> <pairing code> <thread id>...
// stdout: one JSON line per change of an assistant message's text:
// {"t": Unix ms, "thread", "item", "ordinal", "text"}.
import { connect, frames, request } from "./t3-rpc.mjs";

const [url, code, ...threads] = process.argv.slice(2);
const ws = await connect(url, code);
const threadOf = new Map();
threads.forEach((thread, i) => {
  threadOf.set(String(i + 1), thread);
  request(ws, i + 1, "orchestration.subscribeThread", { threadId: thread });
});

const last = new Map();
function note(thread, item) {
  if (item?.type !== "assistant_message" || typeof item.text !== "string") return;
  if (last.get(item.id) === item.text) return;
  last.set(item.id, item.text);
  const line = { t: Date.now(), thread, item: item.id, ordinal: item.ordinal, text: item.text };
  process.stdout.write(`${JSON.stringify(line)}\n`);
}

// The server drops a socket that is silent for 15 s.
setInterval(() => ws.send(JSON.stringify({ _tag: "Ping" })), 5_000);
ws.onclose = () => {
  console.error("t3-watch: the server closed the socket");
  process.exit(1);
};
ws.onmessage = (e) => {
  for (const f of frames(e.data)) {
    if (f._tag === "Exit") {
      console.error(`t3-watch: subscription ${f.requestId} ended: ${JSON.stringify(f.exit)}`);
      process.exit(1);
    }
    if (f._tag !== "Chunk") continue;
    // Streams wait for an ack before the next chunk.
    ws.send(JSON.stringify({ _tag: "Ack", requestId: String(f.requestId) }));
    const thread = threadOf.get(String(f.requestId));
    for (const v of f.values ?? []) {
      if (v.kind === "snapshot") v.projection?.turnItems?.forEach((item) => note(thread, item));
      if (v.kind === "event" && v.event?.type === "turn-item.updated") note(thread, v.event.payload);
    }
  }
};
