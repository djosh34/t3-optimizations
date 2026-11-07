// Drives transport.html in Playwright WebKit against a running sidecar.
// Runs in the lab's phone image (Playwright 1.64.1) with the host's network,
// so the page at http://127.0.0.1 is a secure context.
//
// Argument: JSON {origin, device, token}. Screenshots go to /out.
// stdout: one JSON object with the result of each step.
import { devices, webkit } from "/opt/driver/node_modules/playwright/index.mjs";

const cfg = JSON.parse(process.argv[2]);
const results = {};
const browser = await webkit.launch();
const device = devices["iPhone 17 Pro"];
const context = await browser.newContext({ ...device });
const page = await context.newPage();
page.on("console", (m) => console.error(`page: ${m.text()}`));

async function step(name, text, run) {
  try {
    results[name] = { ok: true, ...(await run()) };
  } catch (error) {
    results[name] = { ok: false, error: String(error.message ?? error).split("\n")[0] };
  }
  await page.evaluate(([t, ok]) => t3.step(t, ok), [text, results[name].ok]);
}

const until = (fn, arg, timeout = 30_000) => page.waitForFunction(fn, arg, { timeout, polling: 100 });

await page.goto(`${cfg.origin}/transport.html`);
await page.evaluate((c) => t3.init(c), { device: cfg.device, token: cfg.token });

await step("signaled", "First connection, signaled, with derived ICE credentials", async () => {
  await page.evaluate(async () => {
    window.c1 = await t3.connect(1);
  });
  await until(() => window.c1.received >= 3);
  const ufrag = await page.evaluate(() => window.c1.pc.localDescription.sdp.match(/a=ice-ufrag:(.*)/)[1].trim());
  return { ufrag, received: await page.evaluate(() => window.c1.received) };
});

await step("restart", "ICE restart on the same connection, without signaling", async () => {
  const before = await page.evaluate(async () => {
    const n = window.c1.received;
    window.c1.states = [];
    await t3.restartIce(window.c1);
    return n;
  });
  await until((n) => window.c1.received >= n + 5, before);
  const ufrag = await page.evaluate(() => window.c1.pc.localDescription.sdp.match(/a=ice-ufrag:(.*)/)[1].trim());
  if (!ufrag.endsWith("+p1+1")) throw new Error(`local ufrag after the restart is ${ufrag}`);
  const pair = await page.evaluate(async () => {
    for (const s of (await window.c1.pc.getStats()).values()) {
      if (s.type === "transport") return { selectedPair: s.selectedCandidatePairId, iceState: s.iceState };
    }
    return null;
  });
  return { ufrag, states: await page.evaluate(() => window.c1.states), pair };
});

await step("cold", "New connection from the kept answer and certificate, old one still open", async () => {
  await page.evaluate(async () => {
    await t3.keepCert();
    window.c2 = await t3.connect(2, window.c1.kept);
  });
  await until(() => window.c2.received >= 3);
  return {
    oldStillOpen: await page.evaluate(() => window.c1.pc.connectionState),
    received: await page.evaluate(() => window.c2.received),
  };
});

await step("websocket", "The same packets over the WebSocket fallback", async () => {
  await page.evaluate(async () => {
    window.w = await t3.websocket();
  });
  await until(() => window.w.received >= 3);
  return { received: await page.evaluate(() => window.w.received) };
});

// The Safari tab, then the home-screen app, which has no browser bars.
await page.screenshot({ path: "/out/transport-safari.png" });
await page.setViewportSize({ width: 402, height: 874 });
await page.screenshot({ path: "/out/transport-home-screen.png" });
await browser.close();
process.stdout.write(`${JSON.stringify(results)}\n`);
