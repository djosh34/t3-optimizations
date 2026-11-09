// Sends one orchestration command to a t3 server, as its own client: the
// lab uses it to change the server while the sidecar is down.
// Usage: node t3-command.mjs <server url> <pairing code> <command JSON>
// Prints t3's exit; exits 1 unless it is a success.
import { connect, frames, request } from "./t3-rpc.mjs";

const [url, code, commandJson] = process.argv.slice(2);
const ws = await connect(url, code);
const exit = await new Promise((resolve, reject) => {
  ws.onerror = () => reject(new Error("WebSocket failed"));
  ws.onmessage = (e) => {
    const done = frames(e.data).find((f) => f._tag === "Exit" && String(f.requestId) === "1");
    if (done) resolve(done.exit);
  };
  request(ws, 1, "orchestration.dispatchCommand", JSON.parse(commandJson));
});
ws.close();
console.log(JSON.stringify(exit));
process.exit(exit._tag === "Success" ? 0 : 1);
