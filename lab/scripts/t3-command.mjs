// Sends one orchestration command to a t3 server, as its own client: the
// lab uses it to change the server while the sidecar is down.
// Usage: node t3-command.mjs <server url> <pairing code> <command JSON>
// Prints t3's exit; exits 1 unless it is a success.
const [url, code, commandJson] = process.argv.slice(2);

const form = new URLSearchParams({
  grant_type: "urn:ietf:params:oauth:grant-type:token-exchange",
  subject_token: code,
  subject_token_type: "urn:t3:params:oauth:token-type:environment-bootstrap",
  requested_token_type: "urn:ietf:params:oauth:token-type:access_token",
  client_label: "t3lab",
});
const token = await fetch(`${url}/oauth/token`, { method: "POST", body: form });
if (!token.ok) throw new Error(`token exchange: HTTP ${token.status} ${await token.text()}`);
const { access_token: bearer } = await token.json();
const ticketResponse = await fetch(`${url}/api/auth/websocket-ticket/rpc`, {
  method: "POST",
  headers: { authorization: `Bearer ${bearer}` },
});
const { ticket } = await ticketResponse.json();
const ws = new WebSocket(
  `${url.replace("http", "ws")}/ws?orchestrationProtocol=2&wsTicket=${encodeURIComponent(ticket)}`,
);
const exit = await new Promise((resolve, reject) => {
  ws.onerror = () => reject(new Error("WebSocket failed"));
  ws.onopen = () =>
    ws.send(
      JSON.stringify({
        _tag: "Request",
        id: "1",
        tag: "orchestration.dispatchCommand",
        payload: JSON.parse(commandJson),
        headers: [],
      }),
    );
  ws.onmessage = (e) => {
    const frames = [JSON.parse(String(e.data))].flat();
    const done = frames.find((f) => f._tag === "Exit" && String(f.requestId) === "1");
    if (done) resolve(done.exit);
  };
});
ws.close();
console.log(JSON.stringify(exit));
process.exit(exit._tag === "Success" ? 0 : 1);
