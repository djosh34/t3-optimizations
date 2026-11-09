// A t3 server's Effect RPC over its WebSocket, for the lab's own clients:
// pairs with a one-time code from `t3 pair` and opens the socket.
export async function connect(url, code) {
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
  if (!ticketResponse.ok) throw new Error(`websocket ticket: HTTP ${ticketResponse.status}`);
  const { ticket } = await ticketResponse.json();
  const ws = new WebSocket(
    `${url.replace("http", "ws")}/ws?orchestrationProtocol=2&wsTicket=${encodeURIComponent(ticket)}`,
  );
  await new Promise((resolve, reject) => {
    ws.onopen = resolve;
    ws.onerror = () => reject(new Error("WebSocket failed"));
  });
  return ws;
}

export function request(ws, id, tag, payload) {
  ws.send(JSON.stringify({ _tag: "Request", id: String(id), tag, payload, headers: [] }));
}

// The frames of one message: t3 sends one frame or an array of them.
export function frames(data) {
  return [JSON.parse(String(data))].flat();
}
