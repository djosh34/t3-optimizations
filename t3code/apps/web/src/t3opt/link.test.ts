import { describe, expect, it, vi } from "vite-plus/test";

import { Link } from "./link";

class FakeChannel {
  readyState: RTCDataChannelState = "connecting";
  bufferedAmount = 0;
  bufferedAmountLowThreshold = 0;
  binaryType = "arraybuffer";
  onopen: (() => void) | null = null;
  onmessage: ((e: { data: ArrayBuffer }) => void) | null = null;
  onclose: (() => void) | null = null;
  onbufferedamountlow: (() => void) | null = null;
  send() {}
}

class FakePeer {
  static all: FakePeer[] = [];
  iceConnectionState: RTCIceConnectionState = "new";
  oniceconnectionstatechange: (() => void) | null = null;
  channel = new FakeChannel();
  closed = false;
  localDescription = { sdp: "" };
  constructor() {
    FakePeer.all.push(this);
  }
  createDataChannel() {
    return this.channel;
  }
  async createOffer() {
    return { sdp: "a=ice-ufrag:a\r\na=ice-pwd:b\r\n" };
  }
  async setLocalDescription() {}
  async setRemoteDescription() {}
  restartIce() {}
  close() {
    this.closed = true;
  }
  ice(state: RTCIceConnectionState) {
    this.iceConnectionState = state;
    this.oniceconnectionstatechange?.();
  }
}

class FakeSocket {
  static readonly OPEN = 1;
  static all: FakeSocket[] = [];
  readyState = 0;
  bufferedAmount = 0;
  binaryType = "arraybuffer";
  onopen: (() => void) | null = null;
  onmessage: ((e: { data: ArrayBuffer }) => void) | null = null;
  onclose: (() => void) | null = null;
  constructor() {
    FakeSocket.all.push(this);
  }
  send() {}
  open() {
    this.readyState = 1;
    this.onopen?.();
  }
  close() {
    this.readyState = 3;
    this.onclose?.();
  }
}

function setup() {
  // An earlier test's link still steps on the shared fake clock.
  vi.clearAllTimers();
  FakePeer.all = [];
  FakeSocket.all = [];
  vi.stubGlobal("RTCPeerConnection", FakePeer);
  vi.stubGlobal("WebSocket", FakeSocket);
  vi.stubGlobal("location", { href: "https://phone.test/", search: "" });
  // WebCrypto answers in real time, which fake timers would run past.
  vi.stubGlobal("crypto", {
    subtle: { importKey: async () => ({}), sign: async () => new ArrayBuffer(32) },
  });
  const link = new Link(
    { device: "d1", token: "00112233445566778899aabbccddeeff" },
    {
      certificate: { expires: Number.MAX_SAFE_INTEGER } as RTCCertificate,
      keptAnswer: "a=ice-ufrag:s\r\na=ice-pwd:t\r\n",
      generation: 0,
    },
    { save: async () => {} },
    { onPacket: () => {}, onNewConnection: () => {}, onChange: () => {}, onWritable: () => {} },
    (async () => new Response("")) as typeof fetch,
  );
  return link;
}

/** An open channel with ICE up and one packet in. */
async function openLink() {
  const link = setup();
  await link.start();
  await vi.advanceTimersByTimeAsync(10);
  const peer = FakePeer.all[0];
  if (!peer) throw new Error("no connection");
  peer.ice("connected");
  peer.channel.readyState = "open";
  peer.channel.onopen?.();
  peer.channel.onmessage?.({ data: new ArrayBuffer(1) });
  return { link, peer };
}

describe("Link", () => {
  vi.useFakeTimers();

  it("moves to the WebSocket when UDP goes away under an open page", async () => {
    const { link, peer } = await openLink();
    // ICE notices a few seconds after the last packet.
    await vi.advanceTimersByTimeAsync(5_000);
    peer.ice("disconnected");
    await vi.advanceTimersByTimeAsync(60_000);
    expect(FakeSocket.all).toHaveLength(0);
    await vi.advanceTimersByTimeAsync(130_000);
    const probe = FakeSocket.all[0];
    expect(probe).toBeDefined();
    probe?.open();
    await vi.advanceTimersByTimeAsync(25_000);
    expect(link.info().transport).toBe("websocket");
    expect(peer.closed).toBe(true);
  });

  it("gives a later connection three minutes before calling UDP blocked", async () => {
    const { link, peer } = await openLink();
    // The channel closes; its replacement never gets ICE through, as at 70% loss.
    peer.channel.readyState = "closed";
    await vi.advanceTimersByTimeAsync(2_000);
    expect(FakePeer.all).toHaveLength(2);
    await vi.advanceTimersByTimeAsync(30_000);
    expect(link.info().transport).toBe("webrtc");
    // WebKit stops sending unanswered checks: a fresh connection every minute.
    await vi.advanceTimersByTimeAsync(31_000);
    expect(FakePeer.all).toHaveLength(3);
    expect(FakePeer.all[1]?.closed).toBe(true);
    // The three minutes count from the first try, not the newest connection.
    await vi.advanceTimersByTimeAsync(119_000);
    expect(link.info().transport).toBe("websocket");
    // WebRTC is tried again from the WebSocket, a fresh connection every minute.
    const before = FakePeer.all.length;
    await vi.advanceTimersByTimeAsync(60_000);
    expect(FakePeer.all).toHaveLength(before + 1);
    const retry = FakePeer.all.at(-1);
    await vi.advanceTimersByTimeAsync(30_000);
    expect(retry?.closed).toBe(false);
    await vi.advanceTimersByTimeAsync(32_000);
    expect(retry?.closed).toBe(true);
    expect(FakePeer.all).toHaveLength(before + 2);
  });

  it("starts the wait for blocked UDP again once ICE got through", async () => {
    const { link, peer } = await openLink();
    peer.channel.readyState = "closed";
    await vi.advanceTimersByTimeAsync(2_000);
    // ICE gets through on the new connection, its channel never opens, and
    // three minutes later it closes.
    const second = FakePeer.all[1];
    if (!second) throw new Error("no second connection");
    second.ice("connected");
    await vi.advanceTimersByTimeAsync(180_000);
    second.channel.readyState = "closed";
    await vi.advanceTimersByTimeAsync(30_000);
    expect(FakePeer.all).toHaveLength(3);
    expect(link.info().transport).toBe("webrtc");
  });

  it("replaces a retry from the WebSocket whose channel closed or never opened", async () => {
    const link = setup();
    await link.start();
    // UDP blocked from the start: the WebSocket after 20 s.
    await vi.advanceTimersByTimeAsync(21_000);
    expect(link.info().transport).toBe("websocket");
    await vi.advanceTimersByTimeAsync(60_000);
    expect(FakePeer.all).toHaveLength(2);
    const retry = FakePeer.all[1];
    if (!retry) throw new Error("no retry");
    // ICE gets through, the handshake does not, and the channel closes.
    retry.ice("connected");
    retry.channel.readyState = "closed";
    await vi.advanceTimersByTimeAsync(61_000);
    expect(FakePeer.all).toHaveLength(3);
    // ICE gets through on the next, and its channel never opens.
    FakePeer.all[2]?.ice("connected");
    await vi.advanceTimersByTimeAsync(181_000);
    expect(FakePeer.all).toHaveLength(4);
  });

  it("replaces a connection whose handshake stalled after ICE got through", async () => {
    const { peer } = await openLink();
    peer.channel.readyState = "closed";
    await vi.advanceTimersByTimeAsync(2_000);
    const second = FakePeer.all[1];
    if (!second) throw new Error("no second connection");
    second.ice("connected");
    // At heavy loss a handshake takes minutes: it is waited for.
    await vi.advanceTimersByTimeAsync(170_000);
    expect(FakePeer.all).toHaveLength(2);
    await vi.advanceTimersByTimeAsync(15_000);
    expect(FakePeer.all).toHaveLength(3);
    expect(second.closed).toBe(true);
    // The next stalls too: it gets twice as long, six minutes.
    FakePeer.all[2]?.ice("connected");
    await vi.advanceTimersByTimeAsync(350_000);
    expect(FakePeer.all).toHaveLength(3);
    await vi.advanceTimersByTimeAsync(15_000);
    expect(FakePeer.all).toHaveLength(4);
    // Twelve minutes for the next, and no longer after that.
    for (const peers of [5, 6]) {
      FakePeer.all.at(-1)?.ice("connected");
      await vi.advanceTimersByTimeAsync(710_000);
      expect(FakePeer.all).toHaveLength(peers - 1);
      await vi.advanceTimersByTimeAsync(15_000);
      expect(FakePeer.all).toHaveLength(peers);
    }
  });

  it("stays on WebRTC in an outage, where the probe cannot connect", async () => {
    const { link, peer } = await openLink();
    await vi.advanceTimersByTimeAsync(5_000);
    peer.ice("disconnected");
    // Restarts back off, so one can finish on a slow link: 8, 8, 16, then
    // every 30 s (every 8 s was 38 in this outage).
    await vi.advanceTimersByTimeAsync(200_000);
    const at200 = link.info().iceRestarts;
    await vi.advanceTimersByTimeAsync(90_000);
    expect(link.info().iceRestarts - at200).toBe(3);
    await vi.advanceTimersByTimeAsync(10_000);
    expect(link.info().transport).toBe("webrtc");
    // The outage ends: an ICE restart brings the data back, the probe goes.
    peer.ice("connected");
    peer.channel.onmessage?.({ data: new ArrayBuffer(1) });
    expect(FakeSocket.all.every((s) => s.readyState === 3 || s.readyState === 0)).toBe(true);
    expect(link.info().transport).toBe("webrtc");
    expect(link.info().newConnections).toBe(0);
    // Data came: the next silence restarts after 8 s again.
    const before = link.info().iceRestarts;
    peer.ice("disconnected");
    await vi.advanceTimersByTimeAsync(9_000);
    expect(link.info().iceRestarts).toBe(before + 1);
  });
});
