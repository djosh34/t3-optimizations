// Opening a thread the phone does not have yet, in Playwright WebKit (issue
// #84): the home-screen web app pairs with a code, then a tap on one of the
// tests' own threads (`t3opt test …`, made by crates/native-phone's
// test-thread) and the time until its newest message is on screen. Runs in
// the lab's phone image; see deploy/open-test-thread.sh, which starts it.
//
// The page never sends `thread.visit`: an init script answers those
// commands itself, so no thread's read mark changes, whatever the app opens.
//
// Usage: node open-test-thread.mjs <origin> <pairing code> <safari|home-screen> <title> <reply>
//   safari: the Playwright device "iPhone 17 Pro" (402x681 at 3x).
//   home-screen: the same at 402x874, the home-screen app without browser bars.
// Screenshots go to /out/<size>-<step>.png; stdout gets one line per step.
import { devices, webkit } from "/opt/driver/node_modules/playwright/index.mjs";

const [origin, code, size, title, reply] = process.argv.slice(2);
if (!origin || !code || !["safari", "home-screen"].includes(size) || !title || !reply) {
  console.error("usage: node open-test-thread.mjs <origin> <code> <safari|home-screen> <title> <reply>");
  process.exit(2);
}

const device = devices["iPhone 17 Pro"];
const viewport = size === "safari" ? device.viewport : { width: 402, height: 874 };
const browser = await webkit.launch();
const context = await browser.newContext({ ...device, viewport });

await context.addInitScript(() => {
  // The shim puts its WebSocket on window; each one it makes is wrapped so a
  // visit is answered here and never reaches the sidecar.
  let current = window.WebSocket;
  Object.defineProperty(window, "WebSocket", {
    configurable: true,
    get: () => current,
    set: (Base) => {
      current = new Proxy(Base, {
        construct(target, args) {
          const ws = Reflect.construct(target, args);
          const send = ws.send.bind(ws);
          ws.send = (data) => {
            if (typeof data !== "string" || !data.includes("thread.visit")) return send(data);
            const parsed = JSON.parse(data);
            const frames = Array.isArray(parsed) ? parsed : [parsed];
            const isVisit = (f) =>
              f?._tag === "Request" &&
              f.tag === "orchestration.dispatchCommand" &&
              f.payload?.type === "thread.visit";
            for (const f of frames.filter(isVisit)) {
              const exit = { _tag: "Exit", requestId: f.id, exit: { _tag: "Success", value: { sequence: 0 } } };
              setTimeout(() => {
                // As the shim's socket delivers: listeners and onmessage.
                const e = new MessageEvent("message", { data: JSON.stringify(exit) });
                ws.dispatchEvent(e);
                if (typeof ws.onmessage === "function") ws.onmessage(e);
              }, 0);
            }
            const kept = frames.filter((f) => !isVisit(f));
            if (kept.length > 0) send(JSON.stringify(Array.isArray(parsed) ? kept : kept[0]));
          };
          return ws;
        },
      });
    },
  });
});

const page = await context.newPage();
page.on("console", (m) => {
  if (m.type() === "error") console.error(`page error: ${m.text().slice(0, 300)}`);
});

const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));
let failed = false;

async function step(name, run) {
  const t = Date.now();
  try {
    const detail = await run();
    await page.screenshot({ path: `/out/${size}-${name}.png` });
    console.log(`${size} ${name}: ok in ${((Date.now() - t) / 1000).toFixed(1)} s${detail ? ` (${detail})` : ""}`);
  } catch (error) {
    failed = true;
    await page.screenshot({ path: `/out/${size}-${name}-failed.png` }).catch(() => {});
    // Playwright's call log says what held an action back.
    console.log(`${size} ${name}: failed: ${String(error.message ?? error).split("\n").slice(0, 12).join("\n  ")}`);
    throw error;
  }
}

async function closeWelcome(timeout) {
  const welcome = page.getByText("Connect your computers").first();
  const until = Date.now() + timeout;
  while (Date.now() < until) {
    if (await welcome.isVisible().catch(() => false)) {
      await page.getByRole("button", { name: "Close" }).first().click({ timeout: 5_000 });
      return "closed the welcome";
    }
    await sleep(500);
  }
  return "no welcome";
}

// The sidebar's row; the title also sits in a screen-reader-only label.
const row = page
  .getByText(title, { exact: true })
  .and(page.locator(":not(.sr-only)"))
  .filter({ visible: true })
  .first();

try {
  await step("open", async () => {
    await page.goto(origin, { waitUntil: "commit", timeout: 120_000 });
    await page.locator("#pairing-token").waitFor({ timeout: 120_000 });
  });

  await step("paired", async () => {
    await page.locator("#pairing-token").fill(code);
    await page.getByRole("button", { name: "Continue" }).click();
    await page.locator("#pairing-token").waitFor({ state: "detached", timeout: 120_000 });
    await page.getByRole("button", { name: "Toggle main sidebar" }).first().waitFor({ timeout: 120_000 });
    return await closeWelcome(10_000);
  });

  await step("listed", async () => {
    await page.getByRole("button", { name: "Toggle main sidebar" }).first().click({ timeout: 30_000 });
    await row.waitFor({ timeout: 120_000 });
  });

  await step("opened", async () => {
    const t = performance.now();
    await row.click({ timeout: 30_000 });
    await page
      .locator(`text=/^\\s*${reply}\\.?\\s*$/i`)
      .filter({ visible: true })
      .first()
      .waitFor({ timeout: 120_000 });
    const ms = Math.round(performance.now() - t);
    // The shim's trace has ids and paths: for a local look only.
    if (process.env.TRACE) {
      const trace = await page.evaluate(() => window.t3opt.trace.slice(-80));
      console.log(trace.join("\n"));
    }
    return `tap to newest message on screen ${ms} ms`;
  });
} catch {
  // The step said why.
}

await browser.close();
process.exit(failed ? 1 : 0);
