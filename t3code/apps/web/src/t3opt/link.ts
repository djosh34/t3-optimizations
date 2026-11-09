// How the web app reaches the sidecar: a WebRTC data channel on one
// RTCPeerConnection for the life of the page, the WebSocket as fallback.
// See README.md in this folder and crates/sidecar/README.md.

import type { Pairing } from "./store";

/**
 * With nothing from the sidecar for this long, restart ICE. Each restart in the same
 * silence waits twice as long, up to `RESTART_MAX_MS`: a restart must have time to
 * finish, and on a 1 kbit/s link at 75% loss a reply took longer than 8 s (issue #46).
 */
const RESTART_AFTER_MS = 8_000;
const RESTART_MAX_MS = 30_000;
/**
 * Silent this long with ICE up for `DEAD_ICE_UP_MS` of it: the sidecar lost the
 * connection. Both are long: a starved link at 70% loss can be silent with ICE up,
 * and a new connection there costs minutes of handshake (README.md).
 */
const DEAD_AFTER_MS = 60_000;
const DEAD_ICE_UP_MS = 20_000;
/** A first WebRTC connection whose ICE has not connected by then falls back to the WebSocket. */
const FALLBACK_AFTER_MS = 20_000;
/** The same for a later connection, once WebRTC worked on this page: a bad cell is likelier. */
const FALLBACK_AFTER_WORKED_MS = 180_000;
/** A connection whose ICE got nothing back is replaced this often: WebKit stops its checks. */
const REPLACE_UNANSWERED_MS = 60_000;
/**
 * Silent this long with ICE down for the last `PROBE_ICE_DOWN_MS`: a WebSocket
 * probe opens; open and still silent `SWITCH_AFTER_PROBE_MS` later, it takes over.
 * Long: UDP worked on this page, so a bad cell is likelier than UDP being blocked.
 */
const PROBE_WS_AFTER_MS = 180_000;
const PROBE_ICE_DOWN_MS = 150_000;
const SWITCH_AFTER_PROBE_MS = 20_000;
/** WebRTC is tried again this often while it failed to connect. */
const RTC_RETRY_MS = 10_000;
/** And this often while the WebSocket carries packets. */
const RTC_RETRY_ON_WS_MS = 60_000;
/** A gap this long counts as a silence in the link reports. */
const SILENCE_MS = 2_000;
/** Bytes the browser may hold unsent before the phone makes its next packet. */
const SEND_BUFFER = 1_200;
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
  /** The send buffer drained: the phone may send again. */
  readonly onWritable: () => void;
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
  const mac = new Uint8Array(
    await crypto.subtle.sign("HMAC", key, enc.encode(`t3opt ice ${ufrag}`)),
  );
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
  /** Since when ICE is not connected, without a break. */
  iceDownSince: number | null;
  /** ICE connected once: UDP gets through, a slow handshake is loss, not a block. */
  iceEverUp: boolean;
  /** How long ICE was connected while the sidecar was silent, up to `iceUpSince`. */
  iceUpInSilenceMs: number;
  lastRestartAt: number | null;
  /** ICE restarts since the sidecar was last heard. */
  restartsInSilence: number;
}

export class Link {
  private key: CryptoKey | null = null;
  private certificate: RTCCertificate | null;
  private keptAnswer: string | null;
  private generation: number;
  /** The WebRTC connection: carrying packets, or being tried while the WebSocket does. */
  private rtc: Rtc | null = null;
  private ws: WebSocket | null = null;
  /** A WebSocket opened while WebRTC is silent, to tell blocked UDP from an outage. */
  private probe: WebSocket | null = null;
  private probeOpenAt: number | null = null;
  private transport: Transport = "webrtc";
  /** Only the WebSocket, as asked with `?transport=websocket` or without WebRTC. */
  private wsOnly = false;
  private trying = "starting";
  /** A connection opened on this page, on either transport. */
  private opened = false;
  /** WebRTC carried a channel on this page. */
  private rtcWorked = false;
  /** Since when WebRTC connections are tried without a channel opening. */
  private rtcTryingSince: number | null = null;
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

  /**
   * Whether a packet can go out now. WebKit's SCTP holds what its congestion
   * window does not let out; packets that wait there only get older, so the
   * phone makes the next one when the buffer is (nearly) empty again.
   */
  canSend(): boolean {
    if (this.transport === "websocket") {
      return this.ws?.readyState === WebSocket.OPEN && this.ws.bufferedAmount < SEND_BUFFER;
    }
    return this.rtc?.dc.readyState === "open" && this.rtc.dc.bufferedAmount < SEND_BUFFER;
  }

  send(packet: Uint8Array<ArrayBuffer>): void {
    // A closed channel is loss, which sync-core handles.
    try {
      if (this.transport === "websocket") this.ws?.send(packet);
      else this.rtc?.dc.send(packet);
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
    if (this.probe !== null && on !== this.probe) this.closeProbe();
    if (this.rtc) {
      if (this.rtc.iceUpSince !== null) this.rtc.iceUpSince = now;
      this.rtc.iceUpInSilenceMs = 0;
      this.rtc.restartsInSilence = 0;
    }
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
      if (this.transport === "webrtc") this.set("webrtc retrying");
    });
  }

  /** A new connection of the next generation; signaled only when no answer is kept. */
  private async connect(): Promise<void> {
    const key = this.key;
    if (!key) throw new Error("no key");
    const certificate = await this.certificateFor();
    const old = this.rtc;
    const replaces = old !== null;
    const generation = this.generation + 1;
    this.generation = generation;
    this.rtcTryingSince ??= Date.now();
    await this.storage.save({ generation });
    if (this.transport === "webrtc") {
      this.set(this.keptAnswer ? "webrtc connecting" : "webrtc signaling");
    }

    const pc = new RTCPeerConnection({ certificates: [certificate] });
    const dc = pc.createDataChannel("sync", { ordered: false, maxRetransmits: 0 });
    dc.binaryType = "arraybuffer";
    dc.bufferedAmountLowThreshold = SEND_BUFFER;
    dc.onbufferedamountlow = () => this.hooks.onWritable();
    const rtc: Rtc = {
      pc,
      dc,
      generation,
      restart: 0,
      createdAt: Date.now(),
      iceUpSince: null,
      iceDownSince: null,
      iceEverUp: false,
      iceUpInSilenceMs: 0,
      lastRestartAt: null,
      restartsInSilence: 0,
    };
    this.rtc = rtc;
    // The connection it replaces is dead; closed, its ICE agent stops.
    old?.pc.close();
    if (replaces) this.newConnections += 1;
    dc.onopen = () => {
      if (this.rtc !== rtc) return;
      this.handshakeMs = Date.now() - rtc.createdAt;
      this.opened = true;
      this.rtcWorked = true;
      this.rtcTryingSince = null;
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
      if (state === "connected" || state === "completed") {
        rtc.iceUpSince ??= Date.now();
        rtc.iceDownSince = null;
        rtc.iceEverUp = true;
        // UDP gets through: the wait for blocked UDP starts again.
        this.rtcTryingSince = null;
      } else {
        rtc.iceDownSince ??= Date.now();
        if (rtc.iceUpSince !== null) rtc.iceUpInSilenceMs += Date.now() - rtc.iceUpSince;
        rtc.iceUpSince = null;
      }
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
    rtc.restartsInSilence += 1;
    rtc.lastRestartAt = Date.now();
    if (rtc.iceUpSince !== null) rtc.iceUpInSilenceMs += rtc.lastRestartAt - rtc.iceUpSince;
    rtc.iceUpSince = null;
    this.iceRestarts += 1;
    this.set(`ICE restart ${rtc.restart}`);
    rtc.pc.restartIce();
    const offer = await rtc.pc.createOffer();
    const mine = await derive(key, this.pairing.device, "p", rtc.generation, rtc.restart);
    await rtc.pc.setLocalDescription({ type: "offer", sdp: withCreds(offer.sdp ?? "", mine) });
    const theirs = await derive(key, this.pairing.device, "s", rtc.generation, rtc.restart);
    await rtc.pc.setRemoteDescription({ type: "answer", sdp: withCreds(this.keptAnswer, theirs) });
  }

  private openWebSocket() {
    this.transport = "websocket";
    this.set("websocket connecting");
    this.ws = this.newWebSocket();
  }

  private newWebSocket(): WebSocket {
    const url = new URL("/sync", location.href);
    url.protocol = url.protocol === "https:" ? "wss:" : "ws:";
    const createdAt = Date.now();
    const ws = new WebSocket(url);
    ws.binaryType = "arraybuffer";
    ws.onopen = () => {
      if (this.probe === ws) {
        this.probeOpenAt = Date.now();
        return;
      }
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
      if (this.probe === ws) {
        this.probe = null;
        this.probeOpenAt = null;
        return;
      }
      if (this.ws !== ws) return;
      this.ws = null;
      this.set("websocket closed");
    };
    return ws;
  }

  private closeProbe() {
    const probe = this.probe;
    this.probe = null;
    this.probeOpenAt = null;
    probe?.close();
  }

  /** The probe got through while WebRTC stayed silent: UDP is blocked, the WebSocket carries packets. */
  private switchToProbe(rtc: Rtc, now: number) {
    const ws = this.probe;
    if (ws === null) return;
    this.probe = null;
    this.probeOpenAt = null;
    rtc.pc.close();
    this.rtc = null;
    this.lastRtcTry = now;
    this.transport = "websocket";
    this.ws = ws;
    this.handshakeMs = null;
    this.hooks.onNewConnection();
    this.set("websocket open");
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

  /** How long a connection whose ICE never got through is waited for. */
  private fallbackAfter(): number {
    return this.rtcWorked ? FALLBACK_AFTER_WORKED_MS : FALLBACK_AFTER_MS;
  }

  private async step(now: number): Promise<void> {
    if (this.transport === "websocket") {
      // The WebSocket runs over TCP; a closed one is opened again.
      if (this.ws === null) this.openWebSocket();
      // A retry whose ICE got nothing back is replaced, as on WebRTC.
      const probe = this.rtc;
      const replaceAfter = this.rtcWorked ? REPLACE_UNANSWERED_MS : FALLBACK_AFTER_MS;
      if (
        probe !== null &&
        probe.dc.readyState !== "open" &&
        !probe.iceEverUp &&
        now - probe.createdAt > replaceAfter
      ) {
        probe.pc.close();
        this.rtc = null;
      }
      if (!this.wsOnly && this.rtc === null && now - this.lastRtcTry >= RTC_RETRY_ON_WS_MS) {
        this.tryRtc();
      }
      return;
    }
    // ICE never got through on this connection (the page's first, or one after
    // the channel closed): UDP is blocked, the WebSocket carries packets. ICE
    // up means only a slow handshake (minutes at 70% loss), where TCP does worse.
    const pending = this.rtc;
    const blocked =
      pending === null
        ? !this.opened && now - this.startedAt > FALLBACK_AFTER_MS
        : pending.dc.readyState !== "open" &&
          !pending.iceEverUp &&
          now - (this.rtcTryingSince ?? pending.createdAt) > this.fallbackAfter();
    if (blocked) {
      this.rtc?.pc.close();
      this.rtc = null;
      this.rtcTryingSince = null;
      this.lastRtcTry = now;
      this.openWebSocket();
      return;
    }
    const rtc = this.rtc;
    if (rtc === null) {
      if (now - this.lastRtcTry >= RTC_RETRY_MS) this.tryRtc();
      return;
    }
    // Only a closed channel is gone. At heavy loss WebKit calls the
    // connection failed when ICE checks are lost, while data still arrives;
    // an ICE restart below handles that, a new connection would need a DTLS
    // handshake that takes minutes at 70% loss.
    if (rtc.dc.readyState === "closed" || rtc.dc.readyState === "closing") {
      this.tryRtc();
      return;
    }
    if (
      this.rtcWorked &&
      rtc.dc.readyState !== "open" &&
      !rtc.iceEverUp &&
      now - rtc.createdAt >= REPLACE_UNANSWERED_MS
    ) {
      this.tryRtc();
      return;
    }
    // A new connection's handshake can take minutes at heavy loss.
    if (rtc.dc.readyState !== "open") return;
    const silentFor = now - (this.lastDataAt ?? rtc.createdAt);
    if (silentFor < RESTART_AFTER_MS) return;
    const iceUp = rtc.iceUpInSilenceMs + (rtc.iceUpSince === null ? 0 : now - rtc.iceUpSince);
    const iceDown = rtc.iceDownSince === null ? 0 : now - rtc.iceDownSince;
    if (!this.wsOnly && silentFor >= PROBE_WS_AFTER_MS && iceDown >= PROBE_ICE_DOWN_MS) {
      if (this.probe === null) this.probe = this.newWebSocket();
      else if (this.probeOpenAt !== null && now - this.probeOpenAt >= SWITCH_AFTER_PROBE_MS) {
        this.switchToProbe(rtc, now);
        return;
      }
    } else if (this.probe !== null) {
      this.closeProbe();
    }
    if (silentFor >= DEAD_AFTER_MS && iceUp >= DEAD_ICE_UP_MS) {
      this.tryRtc();
      return;
    }
    const restartWait = Math.min(
      RESTART_AFTER_MS * 2 ** Math.max(0, rtc.restartsInSilence - 1),
      RESTART_MAX_MS,
    );
    if (
      rtc.iceUpSince === null &&
      (rtc.lastRestartAt === null || now - rtc.lastRestartAt >= restartWait)
    ) {
      await this.restartIce(rtc);
    }
  }
}
