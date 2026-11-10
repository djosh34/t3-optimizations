// Run inside t3lab-phone:1.64.1, built from lab/driver/Dockerfile.
// Usage: node webkit-probe.mjs <pairing-code-file> <safari|home> [pause]
// Only the throwaway instance on this ticket's HTTP port is allowed.
import fs from "node:fs";
import { devices, webkit } from "/opt/driver/node_modules/playwright/index.mjs";

const [codeFile, size, pause] = process.argv.slice(2);
if (!codeFile || !["safari", "home"].includes(size)) {
  throw new Error("usage: webkit-probe.mjs <pairing-code-file> <safari|home> [pause]");
}
const code = fs.readFileSync(codeFile, "utf8").match(/^Token: (.*)$/m)?.[1];
if (!code) throw new Error("no pairing code in the input file");
const browser = await webkit.launch();
const context = await browser.newContext({
  ...devices["iPhone 17 Pro"],
  ...(size === "home" ? { viewport: { width: 402, height: 874 } } : {}),
});
await context.addInitScript(() => {
  window.research90 = { sends: 0, receives: 0, maxBuffered: 0, restarts: 0 };
  const create = RTCPeerConnection.prototype.createDataChannel;
  RTCPeerConnection.prototype.createDataChannel = function (...args) {
    const dc = create.apply(this, args);
    window.channel90 = dc;
    window.peer90 = this;
    const send = dc.send.bind(dc);
    dc.send = (packet) => {
      window.research90.sends++;
      send(packet);
      window.research90.maxBuffered = Math.max(window.research90.maxBuffered, dc.bufferedAmount);
    };
    dc.addEventListener("message", () => window.research90.receives++);
    return dc;
  };
  const restart = RTCPeerConnection.prototype.restartIce;
  RTCPeerConnection.prototype.restartIce = function () {
    window.research90.restarts++;
    return restart.call(this);
  };
});
const page = await context.newPage();
let stoppedPid;
try {
  await page.goto("http://127.0.0.1:12501");
  await page.locator("#pairing-token").fill(code);
  await page.getByRole("button", { name: "Continue" }).click();
  for (let sample = 0; sample < 90; sample++) {
    if (pause && sample === 10) {
      const pid = fs.readdirSync("/proc").find((name) => {
        if (!/^\d+$/.test(name)) return false;
        try {
          return fs.readFileSync(`/proc/${name}/comm`, "utf8").trim() === "WPEWebProcess";
        } catch {
          return false;
        }
      });
      if (!pid) throw new Error("no WebKit content process");
      stoppedPid = Number(pid);
      console.log(JSON.stringify({ suspendedAt: Date.now() }));
      process.kill(stoppedPid, "SIGSTOP");
      // The Node driver remains alive. This models stopped event processing,
      // not all of iOS's background networking and process policies.
      setTimeout(() => {
        process.kill(stoppedPid, "SIGCONT");
        stoppedPid = undefined;
        console.log(JSON.stringify({ resumedAt: Date.now() }));
      }, 45_000);
    }
    await page.waitForTimeout(2_000);
    console.log(JSON.stringify(await page.evaluate(() => {
      const engine = window.t3opt?.engine;
      return {
        at: Date.now(),
        counters: window.research90,
        buffered: window.channel90?.bufferedAmount,
        ready: window.channel90?.readyState,
        ice: window.peer90?.iceConnectionState,
        status: engine?.status(),
        committing: engine?.committing,
        unsavedLog: engine?.unsavedLog,
      };
    })));
  }
} finally {
  if (stoppedPid) process.kill(stoppedPid, "SIGCONT");
  await browser.close();
}
