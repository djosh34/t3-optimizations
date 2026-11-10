// Fetches real responses from a throwaway stock t3 for the #119 cache measurements.
// node fetch.mjs <t3 base dir under /tmp> <port> <out dir>
import { execFileSync } from "node:child_process";
import { writeFileSync } from "node:fs";

const [base, port, outDir] = process.argv.slice(2);
if (!base?.startsWith("/tmp/")) throw new Error("throwaway t3 under /tmp only");
const url = `http://127.0.0.1:${port}`;
const t3 = `${process.env.HOME}/.local/share/t3opt/t3-0.1.7/t3`;

const printed = execFileSync(t3, ["pair", "--base-dir", base, "--label", "research119"]).toString();
const code = printed.split("\n").find((l) => l.startsWith("Token: ")).slice(7).trim();
const form = new URLSearchParams({
  grant_type: "urn:ietf:params:oauth:grant-type:token-exchange",
  subject_token: code,
  subject_token_type: "urn:t3:params:oauth:token-type:environment-bootstrap",
  requested_token_type: "urn:ietf:params:oauth:token-type:access_token",
  client_label: "research119",
});
const token = (await (await fetch(`${url}/oauth/token`, { method: "POST", body: form })).json())
  .access_token;
const auth = { authorization: `Bearer ${token}`, "x-t3-orchestration-protocol": "2" };

// Config: the subscription's first snapshot, then server.getConfig, over t3's RPC socket.
const ticket = (
  await (await fetch(`${url}/api/auth/websocket-ticket/rpc`, { method: "POST", headers: auth })).json()
).ticket;
const ws = new WebSocket(
  `ws://127.0.0.1:${port}/ws?orchestrationProtocol=2&wsTicket=${encodeURIComponent(ticket)}`,
);
await new Promise((r) => ws.addEventListener("open", r));
const config = {};
await new Promise((resolve) => {
  ws.addEventListener("message", (e) => {
    const f = JSON.parse(e.data);
    if (f._tag === "Ping") ws.send(JSON.stringify({ _tag: "Pong" }));
    if (f._tag === "Chunk" && f.requestId === "1") config.stream = JSON.stringify(f.values[0]);
    if (f._tag === "Exit" && f.requestId === "2") config.get = JSON.stringify(f.exit.value);
    if (config.stream && config.get) resolve();
  });
  const req = (id, tag, payload) =>
    ws.send(JSON.stringify({ _tag: "Request", id, tag, payload, headers: [] }));
  req("1", "subscribeServerConfig", { workspaceSnapshotsOnDemand: true });
  req("2", "server.getConfig", {});
});
ws.close();
writeFileSync(`${outDir}/config.json`, JSON.stringify(config));

// One thread: its bounded snapshot, every history page, the bounded snapshot again.
const get = async (path) => {
  const r = await fetch(`${url}${path}`, { headers: auth });
  if (r.status !== 200) throw new Error(`${path}: HTTP ${r.status}`);
  return r.text();
};
const thread = "research116-long";
const rows = [{ label: "bounded", text: await get(`/api/orchestration/threads/${thread}/bounded`) }];
let cursor = JSON.parse(rows[0].text).historyCursor;
for (let page = 1; cursor; page++) {
  const text = await get(
    `/api/orchestration/threads/${thread}/history?cursor=${encodeURIComponent(cursor)}&deferWork=1`,
  );
  rows.push({ label: `history page ${page}`, text });
  const p = JSON.parse(text);
  cursor = p.hasMoreHistory ? p.nextCursor : null;
}
rows.push({ label: "bounded again", text: await get(`/api/orchestration/threads/${thread}/bounded`) });
writeFileSync(`${outDir}/thread.json`, JSON.stringify(rows));
console.log(`wrote ${outDir}/config.json and ${outDir}/thread.json`);
