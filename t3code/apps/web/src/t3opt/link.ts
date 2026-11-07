// How the web app reaches the sidecar (issues #24, #27, #29): a WebRTC data
// channel first, the sidecar's WebSocket when WebRTC does not open. One
// RTCPeerConnection lives as long as the page and is never closed on
// purpose. When the sidecar goes silent the app restarts ICE on it; only
// when ICE comes back and still nothing arrives, or the channel closes, does
// it build a new connection (the next generation), without signaling.
// crates/sidecar/README.md describes the sidecar's side.

import type { Pairing } from "./store";

/** With nothing from the sidecar for this long, restart ICE, and again each time it passes. */
const RESTART_AFTER_MS = 8_000;
/** ICE is up again but no data for this long: the sidecar lost the connection. */
const DEAD_AFTER_MS = 30_000;
/** A first WebRTC connection that has not opened by then falls back to the WebSocket. */
const FALLBACK_AFTER_MS = 20_000;
/** WebRTC is tried again this often while it failed to connect. */
const RTC_RETRY_MS = 10_000;
/** And this often while the WebSocket carries packets. */
const RTC_RETRY_ON_WS_MS = 60_000;
/** A gap this long counts as a silence in the link reports. */
const SILENCE_MS = 2_000;
const CERT_LIFETIME_MS = 365 * 24 * 3600 * 1000;

export type Transport = "webrtc" | "websocket";

export interface LinkInfo {
  readonly transport: Transport;
  /** What the link is doing, for the status line. */
  readonly trying: string;
  readonly open: boolean;
  readonly generation: number;
  readonly iceRestarts: number;
  /** New connections after the first on this page. */
  readonly newConnections: number;
  /** From creating the connection to its channel opening, the last time. */
  readonly handshakeMs: number | null;
  readonly lastDataAt: number | null;
  readonly silences: number;
  readonly longestSilenceMs: number;
  /** The longest gap after which data came again on the same connection. */
  readonly longestRecoveredGapMs: number;
}

export interface LinkHooks {
  readonly onPacket: (data: Uint8Array) => void;
  /** A new connection: the phone must send its hello and log again. */
  readonly onNewConnection: () => void;
  readonly onChange: () => void;
}

/** What the link keeps in storage. */
export interface LinkSaved {
  readonly certificate: RTCCertificate | null;
  readonly keptAnswer: string | null;
  readonly generation: number;
}

export interface LinkStorage {
  readonly save: (values: Partial<LinkSaved>) => Promise<void>;
}

const enc = new TextEncoder();

function hexBytes(hex: string): Uint8Array<ArrayBuffer> {
  return Uint8Array.from(hex.match(/../g) ?? [], (b) => Number.parseInt(b, 16));
}

/** pipe::creds::derive, in WebCrypto. */
async function derive(
  key: CryptoKey,
  device: string,
  side: "p" | "s",
  generation: number,
  restart: number,
) {
  const ufrag = `${device}+${side}${generation}+${restart}`;
  const mac = new Uint8Array(await crypto.subtle.sign("HMAC", key, enc.encode(`t3opt ice ${ufrag}`)));
  const pwd = btoa(String.fromCharCode(...mac))
    .replace(/=+$/, "")
    .slice(0, 24);
  return { ufrag, pwd };
}

function withCreds(sdp: string, creds: { ufrag: string; pwd: string }): string {
  const old = /a=ice-ufrag:(.*)\r?\n/.exec(sdp)?.[1];
  let out = sdp
    .replace(/a=ice-ufrag:.*\r?\n/g, `a=ice-ufrag:${creds.ufrag}\r\n`)
    .replace(/a=ice-pwd:.*\r?\n/g, `a=ice-pwd:${creds.pwd}\r\n`);
  if (old) out = out.split(` ufrag ${old}`).join(` ufrag ${creds.ufrag}`);
  return out;
}

interface Rtc {
  readonly pc: RTCPeerConnection;
  readonly dc: RTCDataChannel;
  readonly generation: number;
  restart: number;
  readonly createdAt: number;
  iceUpSince: number | null;
  lastRestartAt: number | null;
}

export class Link {
  private key: CryptoKey | null = null;
  private certificate: RTCCertificate | null;
  private keptAnswer: string | null;
  private generation: number;
  /** The WebRTC connection: carrying packets, or being tried while the WebSocket does. */
  private rtc: Rtc | null = null;
  private ws: WebSocket | null = null;
  private transport: Transport = "webrtc";
  /** Only the WebSocket, as asked with `?transport=websocket` or without WebRTC. */
  private wsOnly = false;
  private trying = "starting";
  /** A channel opened on this page. */
  private opened = false;
  private readonly startedAt = Date.now();
  private lastRtcTry = 0;
  private iceRestarts = 0;
  private newConnections = 0;
  private handshakeMs: number | null = null;
  private lastDataAt: number | null = null;
  /** The connection the last data came on. */
  private lastDataOn: object | null = null;
  private silences = 0;
  private longestSilenceMs = 0;
  private longestRecoveredGapMs = 0;
  private busy = false;

  constructor(
    private readonly pairing: Pairing,
    saved: LinkSaved,
    private readonly storage: LinkStorage,
    private readonly hooks: LinkHooks,
    private readonly realFetch: typeof fetch,
  ) {
    this.certificate = saved.certificate;
    this.keptAnswer = saved.keptAnswer;
    this.generation = saved.generation;
  }

  info(): LinkInfo {
    return {
      transport: this.transport,
      trying: this.trying,
      open: this.isOpen(),
      generation: this.generation,
      iceRestarts: this.iceRestarts,
      newConnections: this.newConnections,
      handshakeMs: this.handshakeMs,
      lastDataAt: this.lastDataAt,
      silences: this.silences,
      longestSilenceMs: this.longestSilenceMs,
      longestRecoveredGapMs: this.longestRecoveredGapMs,
    };
  }

  private isOpen(): boolean {
    if (this.transport === "websocket") return this.ws?.readyState === WebSocket.OPEN;
    return this.rtc?.dc.readyState === "open";
  }

  private set(trying: string) {
    this.trying = trying;
    this.hooks.onChange();
  }

  /** Starts connecting; returns at once, the page does not wait for the network. */
  async start(): Promise<void> {
    const forced = new URLSearchParams(location.search).get("transport");
    this.wsOnly = forced === "websocket" || typeof RTCPeerConnection === "undefined";
    if (this.wsOnly) {
      this.openWebSocket();
    } else {
      this.key = await crypto.subtle.importKey(
        "raw",
        hexBytes(this.pairing.token),
        { name: "HMAC", hash: "SHA-256" },
        false,
        ["sign"],
      );
      this.tryRtc();
    }
    setInterval(() => void this.watch(), 1_000);
  }

  send(packet: Uint8Array<ArrayBuffer>): void {
    // A full buffer is loss, which sync-core handles; a closed one too.
    try {
      if (this.transport === "websocket") {
        if (this.ws?.readyState === WebSocket.OPEN) this.ws.send(packet);
      } else if (this.rtc?.dc.readyState === "open" && this.rtc.dc.bufferedAmount < 64_000) {
        this.rtc.dc.send(packet);
      }
    } catch (error) {
      console.warn("t3opt: send failed", error);
    }
  }

  private received(on: object, data: ArrayBuffer) {
    const now = Date.now();
    if (this.lastDataAt !== null) {
      const gap = now - this.lastDataAt;
      if (gap >= SILENCE_MS) this.silences += 1;
      this.longestSilenceMs = Math.max(this.longestSilenceMs, gap);
      // Data again on the connection that went silent: recovered without a handshake.
      if (on === this.lastDataOn) {
        this.longestRecoveredGapMs = Math.max(this.longestRecoveredGapMs, gap);
      }
    }
    this.lastDataAt = now;
    this.lastDataOn = on;
    if (this.rtc) this.rtc.iceUpSince = null;
    if (this.trying !== `${this.transport} live`) this.set(`${this.transport} live`);
    this.hooks.onPacket(new Uint8Array(data));
  }

  private async certificateFor(): Promise<RTCCertificate> {
    if (this.certificate && this.certificate.expires > Date.now() + 24 * 3600 * 1000) {
      return this.certificate;
    }
    const params = {
      name: "ECDSA",
      namedCurve: "P-256",
      expires: CERT_LIFETIME_MS,
    } as AlgorithmIdentifier;
    this.certificate = await RTCPeerConnection.generateCertificate(params);
    // A new certificate needs a signaled connection: the sidecar learns its fingerprint.
    this.keptAnswer = null;
    await this.storage.save({ certificate: this.certificate, keptAnswer: null });
    return this.certificate;
  }

  private tryRtc() {
    this.lastRtcTry = Date.now();
    void this.connect().catch((error: unknown) => {
      console.warn("t3opt: WebRTC connection failed", error);
      this.rtc?.pc.close();
      this.rtc = null;
      if (this.transport === "webrtc") this.set("webrtc failed, trying again");
    });
  }

  /** A new connection of the next generation; signaled only when no answer is kept. */
  private async connect(): Promise<void> {
    const key = this.key;
    if (!key) throw new Error("no key");
    const certificate = await this.certificateFor();
    const replaces = this.rtc !== null;
    const generation = this.generation + 1;
    this.generation = generation;
    await this.storage.save({ generation });
    if (this.transport === "webrtc") {
      this.set(this.keptAnswer ? `webrtc connecting (generation ${generation})` : "webrtc signaling");
    }
    const pc = new RTCPeerConnection({ certificates: [certificate] });
    const dc = pc.createDataChannel("sync", { ordered: false, maxRetransmits: 0 });
    dc.binaryType = "arraybuffer";
    const rtc: Rtc = {
      pc,
      dc,
      generation,
      restart: 0,
      createdAt: Date.now(),
      iceUpSince: null,
      lastRestartAt: null,
    };
    this.rtc = rtc;
    if (replaces) this.newConnections += 1;
    dc.onopen = () => {
      if (this.rtc !== rtc) return;
      this.handshakeMs = Date.now() - rtc.createdAt;
      this.opened = true;
      if (this.transport === "websocket") {
        // WebRTC works again: it takes over from the WebSocket.
        const ws = this.ws;
        this.ws = null;
        ws?.close();
        this.transport = "webrtc";
      }
      this.hooks.onNewConnection();
      this.set("webrtc open");
    };
    dc.onmessage = (e: MessageEvent<ArrayBuffer>) => {
      if (this.rtc === rtc && this.transport === "webrtc") this.received(rtc, e.data);
    };
    dc.onclose = () => {
      if (this.rtc === rtc && this.transport === "webrtc") this.set("webrtc channel closed");
    };
    pc.oniceconnectionstatechange = () => {
      if (this.rtc !== rtc) return;
      const state = pc.iceConnectionState;
      if (state === "connected" || state === "completed") rtc.iceUpSince ??= Date.now();
      else rtc.iceUpSince = null;
      this.hooks.onChange();
    };
    const offer = await pc.createOffer();
    const mine = await derive(key, this.pairing.device, "p", generation, 0);
    await pc.setLocalDescription({ type: "offer", sdp: withCreds(offer.sdp ?? "", mine) });
    if (this.keptAnswer === null) {
      const r = await this.realFetch("/api/rtc/offer", {
        method: "POST",
        headers: { authorization: `Bearer ${this.pairing.token}` },
        body: pc.localDescription?.sdp ?? "",
      });
      if (!r.ok) throw new Error(`signaling: HTTP ${r.status} ${await r.text()}`);
      this.keptAnswer = await r.text();
      await this.storage.save({ keptAnswer: this.keptAnswer });
      await pc.setRemoteDescription({ type: "answer", sdp: this.keptAnswer });
    } else {
      const theirs = await derive(key, this.pairing.device, "s", generation, 0);
      await pc.setRemoteDescription({ type: "answer", sdp: withCreds(this.keptAnswer, theirs) });
    }
  }

  private async restartIce(rtc: Rtc): Promise<void> {
    const key = this.key;
    if (!key || this.keptAnswer === null) return;
    rtc.restart += 1;
    rtc.lastRestartAt = Date.now();
    rtc.iceUpSince = null;
    this.iceRestarts += 1;
    this.set(`webrtc ICE restart ${rtc.restart}`);
    rtc.pc.restartIce();
    const offer = await rtc.pc.createOffer();
    const mine = await derive(key, this.pairing.device, "p", rtc.generation, rtc.restart);
    await rtc.pc.setLocalDescription({ type: "offer", sdp: withCreds(offer.sdp ?? "", mine) });
    const theirs = await derive(key, this.pairing.device, "s", rtc.generation, rtc.restart);
    await rtc.pc.setRemoteDescription({ type: "answer", sdp: withCreds(this.keptAnswer, theirs) });
  }

  private openWebSocket() {
    this.transport = "websocket";
    const url = new URL("/sync", location.href);
    url.protocol = url.protocol === "https:" ? "wss:" : "ws:";
    this.set("websocket connecting");
    const createdAt = Date.now();
    const ws = new WebSocket(url);
    ws.binaryType = "arraybuffer";
    this.ws = ws;
    ws.onopen = () => {
      if (this.ws !== ws) return;
      this.handshakeMs = Date.now() - createdAt;
      this.opened = true;
      this.hooks.onNewConnection();
      this.set("websocket open");
    };
    ws.onmessage = (e: MessageEvent<ArrayBuffer>) => {
      if (this.ws === ws) this.received(ws, e.data);
    };
    ws.onclose = () => {
      if (this.ws !== ws) return;
      this.ws = null;
      this.set("websocket closed");
    };
  }

  /** Once a second: restarts ICE, makes a new connection, falls back, tries WebRTC again. */
  private async watch(): Promise<void> {
    if (this.busy) return;
    this.busy = true;
    try {
      await this.step(Date.now());
    } catch (error) {
      console.warn("t3opt: link step failed", error);
    } finally {
      this.busy = false;
    }
  }

  private async step(now: number): Promise<void> {
    if (this.transport === "websocket") {
      // The WebSocket runs over TCP; a closed one is opened again.
      if (this.ws === null) this.openWebSocket();
      const probe = this.rtc;
      if (probe !== null && probe.dc.readyState !== "open" && now - probe.createdAt > FALLBACK_AFTER_MS) {
        probe.pc.close();
        this.rtc = null;
      }
      if (!this.wsOnly && this.rtc === null && now - this.lastRtcTry >= RTC_RETRY_ON_WS_MS) {
        this.tryRtc();
      }
      return;
    }
    if (!this.opened && now - this.startedAt > FALLBACK_AFTER_MS) {
      // Nothing opened on this page: the network may block WebRTC. The
      // WebSocket carries packets; WebRTC is tried again from time to time.
      this.rtc?.pc.close();
      this.rtc = null;
      this.lastRtcTry = now;
      this.openWebSocket();
      return;
    }
    const rtc = this.rtc;
    if (rtc === null) {
      if (now - this.lastRtcTry >= RTC_RETRY_MS) this.tryRtc();
      return;
    }
    const gone = rtc.dc.readyState === "closed" || rtc.pc.connectionState === "failed";
    if (gone) {
      this.tryRtc();
      return;
    }
    if (rtc.dc.readyState !== "open") return;
    const silentFor = now - (this.lastDataAt ?? rtc.createdAt);
    if (silentFor < RESTART_AFTER_MS) return;
    if (rtc.iceUpSince !== null && now - rtc.iceUpSince >= DEAD_AFTER_MS) {
      this.tryRtc();
      return;
    }
    if (
      rtc.iceUpSince === null &&
      (rtc.lastRestartAt === null || now - rtc.lastRestartAt >= RESTART_AFTER_MS)
    ) {
      await this.restartIce(rtc);
    }
  }
}
