// The phone side of a pipe run (`lab run --client pipe`, `pipe-unpatched` or
// `udp`). Runs inside the t3lab-phone container with `docker exec`, so its
// own control traffic never crosses the shaped link.
//
// Argument: the path of a JSON config (see PipeConfig in
// internal/phone/phone.go).
// stdout: one JSON event per line, each with `t` (Unix ms) and `ev`.
// stdin: "go" once the lab applied the profile, "stop" at the end.
//
// Each side sends a 25-byte tick every 100 ms: sequence number (u32), the
// sender's Unix time in ms (u64), both big-endian, then zeros. The server
// side is crates/pipe/src/bin/pipe-lab.rs.
import { createSocket } from "node:dgram";
import { readFileSync } from "node:fs";
import { createInterface } from "node:readline";
import { devices, webkit } from "/opt/driver/node_modules/playwright/index.mjs";

const cfg = JSON.parse(readFileSync(process.argv[2], "utf8"));

function emit(ev, data = {}) {
  process.stdout.write(`${JSON.stringify({ t: Date.now(), ev, ...data })}\n`);
}

const commands = createInterface({ input: process.stdin })[Symbol.asyncIterator]();
async function nextCommand() {
  const { value, done } = await commands.next();
  return done ? "stop" : value.trim();
}

function encodeTick(seq) {
  const tick = new Uint8Array(25);
  const view = new DataView(tick.buffer);
  view.setUint32(0, seq);
  view.setBigUint64(4, BigInt(Date.now()));
  return tick;
}

function decodeTick(bytes) {
  if (bytes.byteLength !== 25) return undefined;
  const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  return { seq: view.getUint32(0), sent: Number(view.getBigUint64(4)) };
}

// Ticks start as soon as the pipe is open, on the clean link; the lab
// measures from the switch to the profile.
async function startWebRTC() {
  const browser = await webkit.launch();
  const context = await browser.newContext({ ...devices["iPhone 17 Pro"], ignoreHTTPSErrors: true });
  const page = await context.newPage();
  await page.exposeBinding("report", (_source, ev, data) => emit(ev, data));
  await page.goto(`${cfg.origin}/`, { timeout: 60_000 });
  // Runs in WebKit. encodeTick and decodeTick are passed as source text.
  await page.evaluate(
    async ([encodeSrc, decodeSrc]) => {
      const encodeTick = new Function(`return ${encodeSrc}`)();
      const decodeTick = new Function(`return ${decodeSrc}`)();
      const pc = new RTCPeerConnection();
      const dc = pc.createDataChannel("pipe", { ordered: false, maxRetransmits: 0 });
      dc.binaryType = "arraybuffer";
      pc.oniceconnectionstatechange = () => report("state", { text: `ice ${pc.iceConnectionState}` });
      pc.onconnectionstatechange = () => report("state", { text: `connection ${pc.connectionState}` });
      dc.onclose = () => report("state", { text: "channel closed" });
      dc.onerror = (e) => report("note", { text: `channel error: ${e.error?.message ?? e.type}` });
      dc.onmessage = (e) => {
        const at = Date.now();
        const tick = decodeTick(new Uint8Array(e.data));
        if (tick) report("recv", { t: at, seq: tick.seq, sent: tick.sent });
        else report("note", { text: `not a tick: ${e.data.byteLength} bytes` });
      };
      dc.onopen = () => {
        report("open");
        let seq = 0;
        setInterval(() => {
          if (dc.readyState !== "open") return;
          dc.send(encodeTick(seq));
          report("sent", { seq });
          seq++;
        }, 100);
      };
      await pc.setLocalDescription(await pc.createOffer());
      // No STUN servers: gathering ends with the host candidates.
      while (pc.iceGatheringState !== "complete") {
        await new Promise((resolve) => setTimeout(resolve, 20));
      }
      const response = await fetch("/offer", { method: "POST", body: pc.localDescription.sdp });
      const answer = await response.text();
      if (!response.ok) throw new Error(`offer: ${response.status} ${answer}`);
      await pc.setRemoteDescription({ type: "answer", sdp: answer });
      window.pipe = pc;
      await new Promise((resolve, reject) => {
        if (dc.readyState === "open") resolve();
        dc.addEventListener("open", resolve);
        setTimeout(() => reject(new Error(`channel not open after 30 s: ${pc.connectionState}`)), 30_000);
      });
    },
    [encodeTick.toString(), decodeTick.toString()],
  );
  return async () => browser.close();
}

async function startUDP() {
  const socket = createSocket("udp4");
  const [host, port] = [cfg.udpHost, cfg.udpPort];
  socket.on("message", (msg) => {
    const at = Date.now();
    const tick = decodeTick(msg);
    if (tick) emit("recv", { t: at, seq: tick.seq, sent: tick.sent });
    else emit("note", { text: `not a tick: ${msg.byteLength} bytes` });
  });
  socket.on("error", (e) => emit("note", { text: `udp: ${e.message}` }));
  await new Promise((resolve) => socket.bind(0, resolve));
  emit("open");
  let seq = 0;
  const timer = setInterval(() => {
    const n = seq++;
    socket.send(encodeTick(n), port, host, (err) => {
      if (err) emit("note", { text: `send tick ${n}: ${err.message}` });
    });
    emit("sent", { seq: n });
  }, 100);
  return async () => {
    clearInterval(timer);
    socket.close();
  };
}

let close;
try {
  close = cfg.mode === "udp" ? await startUDP() : await startWebRTC();
} catch (error) {
  emit("note", { text: `setup failed: ${error.message.split("\n")[0]}` });
  process.exit(1);
}
emit("ready");
const go = await nextCommand();
if (go !== "go") emit("note", { text: `expected go, got ${go}` });
await nextCommand();
await close();
process.exit(0);
