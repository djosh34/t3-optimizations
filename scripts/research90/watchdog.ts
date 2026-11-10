// Run with `bun scripts/research90/watchdog.ts` at v0.2.0.
// This checks a recovery defect, not the cause of the owner's iOS silence.
import assert from "node:assert/strict";
import { Link } from "../../t3code/apps/web/src/t3opt/link";

class Channel {
  readyState = "open";
  bufferedAmount = 1_200;
  bufferedAmountLowThreshold = 0;
  onopen?: () => void;
  onmessage?: (e: { data: ArrayBuffer }) => void;
  send() {}
}

class Peer {
  static last: Peer;
  iceConnectionState = "connected";
  oniceconnectionstatechange?: () => void;
  readonly channel = new Channel();
  localDescription = { sdp: "" };
  constructor() {
    Peer.last = this;
  }
  createDataChannel() {
    return this.channel;
  }
  async createOffer() {
    return { sdp: "a=ice-ufrag:p\r\na=ice-pwd:p\r\n" };
  }
  async setLocalDescription() {}
  async setRemoteDescription() {}
  restartIce() {}
  close() {}
}

const oldPeer = globalThis.RTCPeerConnection;
const oldLocation = globalThis.location;
const oldInterval = globalThis.setInterval;
const oldNow = Date.now;
let now = 10_000;
try {
  Object.defineProperty(globalThis, "RTCPeerConnection", { configurable: true, value: Peer });
  Object.defineProperty(globalThis, "location", {
    configurable: true,
    value: { search: "", href: "http://localhost/" },
  });
  // Advance the real Link.step ourselves. No browser or network is involved.
  globalThis.setInterval = (() => 0) as unknown as typeof setInterval;
  Date.now = () => now;
  const link = new Link(
    { device: "test", token: "00112233445566778899aabbccddeeff" },
    {
      certificate: { expires: Number.MAX_SAFE_INTEGER } as RTCCertificate,
      keptAnswer: "a=ice-ufrag:s\r\na=ice-pwd:s\r\n",
      generation: 0,
    },
    { save: async () => {} },
    { onPacket() {}, onNewConnection() {}, onChange() {}, onWritable() {} },
    fetch,
  );
  await link.start();
  await Bun.sleep(20);
  const peer = Peer.last;
  assert.ok(peer);
  peer.oniceconnectionstatechange?.();
  peer.channel.onopen?.();
  const step = link as unknown as { step: (at: number) => Promise<void> };
  // The peer's send path remains blocked. The sidecar's small packets arrive
  // more often than the receive-silence timer, as at its 50 B/s floor.
  for (let second = 0; second <= 120; second++) {
    now = 10_000 + second * 1_000;
    if (second % 5 === 0) peer.channel.onmessage?.({ data: new ArrayBuffer(176) });
    await step.step(now);
    assert.equal(link.canSend(), false);
  }
  const info = link.info();
  console.log(JSON.stringify({
    secondsBlocked: 120,
    receiveGapSeconds: 5,
    iceRestarts: info.iceRestarts,
    newConnections: info.newConnections,
  }));
  assert.ok(info.iceRestarts + info.newConnections > 0,
    "the watchdog never tries to repair a persistently blocked send path while data arrives");
} finally {
  Date.now = oldNow;
  globalThis.setInterval = oldInterval;
  Object.defineProperty(globalThis, "RTCPeerConnection", { configurable: true, value: oldPeer });
  Object.defineProperty(globalThis, "location", { configurable: true, value: oldLocation });
}
