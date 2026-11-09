// Research #69: WebKit + the released phone.wasm against webrtc_clean_rate.rs.
// This tests the actual Pipe/str0m path, not the app UI or IndexedDB. A synthetic
// 10 ms storage delay precedes each commit, as in the simulator.
//
// Build: cargo build --release -p sidecar --example webrtc_clean_rate
// Run: node webrtc_clean_rate.mjs <binary> <phone.wasm> [MB=20] [object-bytes=20000]
// Needs Playwright 1.64.1. PLAYWRIGHT_MODULE can name its absolute index.mjs;
// PLAYWRIGHT_WS optionally connects to a Playwright server with WebKit installed.
import { readFileSync } from "node:fs";
import { spawn } from "node:child_process";
import { createInterface } from "node:readline";

const { webkit } = await import(process.env.PLAYWRIGHT_MODULE || "playwright");
const [bin, wasmPath, mb = "20", size = "20000"] = process.argv.slice(2);
if (!bin || !wasmPath) throw new Error("usage: <binary> <phone.wasm> [MB] [object-bytes]");
const wasm = readFileSync(wasmPath).toString("base64");
const browser = process.env.PLAYWRIGHT_WS
  ? await webkit.connect(process.env.PLAYWRIGHT_WS)
  : await webkit.launch({ headless: true });
const server = spawn(bin, [mb, size]);
let timeout;
try {
  const page = await browser.newPage();
  // Fulfilled in Playwright; no HTTP listener or t3 server is started.
  await page.route("http://localhost:18069/", r =>
    r.fulfill({ body: "<!doctype html><title>research69</title>", contentType: "text/html" }));
  await page.goto("http://localhost:18069/");
  page.on("console", m => console.log("PHONE", m.text()));
  page.on("pageerror", e => console.error("PHONE ERROR", String(e)));
  const answer = new Promise((resolve, reject) => {
    createInterface({ input: server.stdout }).on("line", line => {
      if (line.startsWith("ANSWER ")) resolve(JSON.parse(line.slice(7)));
      else console.log(line);
    });
    server.on("error", reject);
    server.on("exit", code => reject(new Error(`server exited before SDP answer: ${code}`)));
  });
  server.stderr.pipe(process.stderr);
  const done = new Promise(resolve => server.on("exit", resolve));
  const offer = await page.evaluate(async wasm => {
    const { instance } = await WebAssembly.instantiate(
      Uint8Array.from(atob(wasm), c => c.charCodeAt(0)));
    const x = instance.exports;
    const input = (bytes, call) => {
      const ptr = x.phone_alloc(bytes.length);
      new Uint8Array(x.memory.buffer, ptr, bytes.length).set(bytes);
      try { return call(ptr, bytes.length); }
      finally { x.phone_dealloc(ptr, bytes.length); }
    };
    const out = () => new Uint8Array(x.memory.buffer, x.phone_out_ptr(), x.phone_out_len()).slice();
    const check = status => {
      if (status < 0) throw new Error(new TextDecoder().decode(out()));
      return status;
    };
    check(input(new TextEncoder().encode("sim-token-000001"),
      ptr => x.phone_create(ptr, 1n, performance.now())));
    const pc = window.pc = new RTCPeerConnection({ iceServers: [] });
    const dc = pc.createDataChannel("sync", { ordered: false, maxRetransmits: 0 });
    dc.binaryType = "arraybuffer";
    let timer;
    const pump = () => {
      const now = performance.now();
      while (dc.readyState === "open" && dc.bufferedAmount < 1200) {
        if (check(x.phone_poll(now)) !== 1) break;
        dc.send(out());
      }
      clearTimeout(timer);
      timer = setTimeout(pump, Math.max(1, Math.min(1000, x.phone_next_wakeup(now) - now)));
    };
    dc.onopen = () => { console.log("open"); pump(); };
    dc.onbufferedamountlow = pump;
    dc.bufferedAmountLowThreshold = 1200;
    dc.onmessage = e => {
      check(input(new Uint8Array(e.data), (p, n) => x.phone_receive(p, n, performance.now())));
      const batch = new DataView(x.memory.buffer, x.phone_out_ptr(), x.phone_out_len())
        .getBigUint64(0, true);
      setTimeout(() => { check(x.phone_committed(batch)); pump(); }, 10);
      pump();
    };
    await pc.setLocalDescription(await pc.createOffer());
    if (pc.iceGatheringState !== "complete") {
      await new Promise(resolve => {
        pc.onicegatheringstatechange = () => {
          if (pc.iceGatheringState === "complete") resolve();
        };
      });
    }
    return pc.localDescription.sdp;
  }, wasm);
  server.stdin.write(JSON.stringify(offer) + "\n");
  const sdp = await answer;
  await page.evaluate(sdp => window.pc.setRemoteDescription({ type: "answer", sdp }), sdp);
  await Promise.race([done, new Promise((_, reject) => {
    timeout = setTimeout(() => reject(new Error("transfer timeout")), 210000);
  })]);
} finally {
  clearTimeout(timeout);
  await browser.close();
  server.kill("SIGTERM");
}
