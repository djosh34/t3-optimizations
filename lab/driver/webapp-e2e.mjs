// The end-to-end test of the home-screen web app (issue #45), in WebKit at the
// iPhone 17 Pro's size. Runs inside the t3lab-phone container with `docker
// exec`; `lab webapp-e2e` starts it and does what it asks for.
//
// Argument: the path of a JSON config (E2EConfig in internal/phone/phone.go).
// stdout: one JSON event per line: "step" with its name and "ok" or why it
// failed, "screenshot", "note", and "ask" when the lab has to act (shape the
// link, cut it, block a kind of traffic, swap the build, restart the
// sidecar). stdin: the lab's answer to each ask, "ok [value]" or "error ...".
import { readFileSync } from "node:fs";
import { createInterface } from "node:readline";
import { devices, webkit } from "/opt/driver/node_modules/playwright/index.mjs";

const cfg = JSON.parse(readFileSync(process.argv[2], "utf8"));
const lines = createInterface({ input: process.stdin })[Symbol.asyncIterator]();

function emit(ev, data = {}) {
  process.stdout.write(`${JSON.stringify({ t: Date.now(), ev, ...data })}\n`);
}

async function ask(text) {
  emit("ask", { text });
  const { value, done } = await lines.next();
  const answer = done ? "error the lab is gone" : value.trim();
  if (!answer.startsWith("ok")) throw new Error(`${text}: ${answer}`);
  return answer.slice(2).trim();
}

const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

// The Safari tab (402x681), then the home-screen app (402x874, no browser bars).
const device = devices["iPhone 17 Pro"];
const browser = await webkit.launch();
const context = await browser.newContext({ ...device, ignoreHTTPSErrors: true });
const page = await context.newPage();
page.on("console", (m) => {
  if (m.type() === "error" || m.type() === "warning") console.error(`page ${m.type()}: ${m.text().slice(0, 500)}`);
});

async function shot(name) {
  try {
    await page.screenshot({ path: `/out/${name}-safari.png`, timeout: 15_000 });
    await page.setViewportSize({ width: 402, height: 874 });
    await sleep(300);
    await page.screenshot({ path: `/out/${name}-home-screen.png`, timeout: 15_000 });
    emit("screenshot", { name });
  } catch (error) {
    emit("note", { text: `screenshot ${name} failed: ${error.message.split("\n")[0]}` });
  } finally {
    await page.setViewportSize(device.viewport).catch(() => {});
  }
}

const results = [];
async function step(name, run) {
  const started = Date.now();
  try {
    const detail = await run();
    results.push({ name, ok: true });
    emit("step", { name, text: `ok in ${((Date.now() - started) / 1000).toFixed(1)} s${detail ? `: ${detail}` : ""}` });
  } catch (error) {
    results.push({ name, ok: false });
    emit("step", { name, text: `failed: ${String(error.message ?? error).split("\n")[0]}` });
    await shot(`failed-${name}`);
  }
}

// The app.

const composer = '[data-testid="composer-editor"]:visible';
const composerOrExpand = `${composer}, button[aria-label="Expand composer"]:visible`;

async function appReady(timeout = 120_000) {
  await page.locator(composerOrExpand).first().waitFor({ timeout });
}

async function focusComposer(timeout = 60_000) {
  await page.locator(composerOrExpand).first().waitFor({ timeout });
  const expand = page.getByRole("button", { name: "Expand composer" });
  if (await expand.isVisible()) await expand.click({ timeout });
  await page.locator(composer).first().click({ timeout });
}

async function send(text) {
  await focusComposer();
  await page.keyboard.type(text);
  await page.keyboard.press("Enter");
}

async function waitText(text, timeout = 120_000) {
  await page.getByText(text, { exact: false }).first().waitFor({ state: "visible", timeout });
}

async function openThread(threadId) {
  await page.goto(`${cfg.origin}/${cfg.environmentId}/${encodeURIComponent(threadId)}`, {
    waitUntil: "commit",
    timeout: 60_000,
  });
  await appReady();
}

async function reload() {
  await page.reload({ waitUntil: "commit", timeout: 60_000 });
}

const link = () => page.evaluate(() => window.t3opt?.engine?.status().link ?? null);
const promptState = () => page.evaluate(() => window.t3opt?.prompt?.() ?? null);

/** Sends a prompt and takes a screenshot at each state the status line shows. */
async function promptWithStates(text, name, timeout = 120_000) {
  const before = Date.now();
  await send(text);
  const seen = new Set();
  const until = Date.now() + timeout;
  while (Date.now() < until) {
    const p = await promptState();
    if (p !== null && p.at >= before - 1_000 && !seen.has(p.state)) {
      seen.add(p.state);
      await shot(`${name}-${p.state}`);
      if (p.state === "confirmed") return `${[...seen].join(" > ")} in ${((Date.now() - before) / 1000).toFixed(1)} s`;
      if (p.state === "failed") throw new Error("the server refused the prompt");
    }
    await sleep(100);
  }
  throw new Error(`not confirmed in ${timeout / 1000} s (states seen: ${[...seen].join(", ")})`);
}

async function openSidebar() {
  const toggle = page.getByRole("button", { name: "Toggle main sidebar" }).first();
  await toggle.click({ timeout: 30_000 });
  await sleep(800);
}

async function swVersion() {
  return page.evaluate(async () => {
    const r = await fetch("/__sw");
    return r.ok ? (await r.json()).version : null;
  });
}

// The steps.

let newThreadId = null;

await step("pairing", async () => {
  await page.goto(cfg.origin, { waitUntil: "commit", timeout: 120_000 });
  await page.locator("#pairing-token").waitFor({ timeout: 120_000 });
  await shot("pairing");
  await page.locator("#pairing-token").fill(cfg.code);
  await page.getByRole("button", { name: "Continue" }).click();
  await appReady();
  const descriptor = await page.evaluate(async () => (await fetch("/.well-known/t3/environment")).json());
  cfg.environmentId = descriptor.environmentId;
  await shot("paired");
});

await step("service-worker", async () => {
  await page.waitForFunction(() => navigator.serviceWorker.controller !== null, null, { timeout: 120_000 });
  return `version ${await swVersion()}`;
});

await step("add-project-and-thread", async () => {
  await openSidebar();
  await page.getByRole("button", { name: "Add project" }).first().click({ timeout: 30_000 });
  await page.getByText("New project", { exact: true }).first().click({ timeout: 30_000 });
  await page.keyboard.type("phone-project");
  await page.keyboard.press("Enter");
  await sleep(4_000);
  await page.keyboard.press("Escape");
  await send("lab:warmup from the phone");
  await waitText("The lab thread is set up");
  newThreadId = decodeURIComponent(new URL(page.url()).pathname.split("/").pop() ?? "");
  await shot("new-thread");
  return `thread ${newThreadId}`;
});

await step("stream-and-approval", async () => {
  await send("Explain the sync loop");
  await waitText(cfg.replyStart.slice(0, 30), 60_000);
  await shot("streaming");
  const approve = page.getByRole("button", { name: /^(Approve|Yes)$/ }).first();
  await approve.waitFor({ timeout: 120_000 });
  await shot("approval");
  await approve.click();
  await waitText(cfg.replyEnd, 120_000);
  await shot("reply-done");
});

await step("interrupt", async () => {
  await send("Explain it once more");
  await waitText(cfg.replyStart.slice(0, 30), 60_000);
  const stop = page.getByRole("button", { name: /stop|interrupt/i }).first();
  await stop.click({ timeout: 30_000 });
  await page.getByRole("button", { name: "Submit message" }).first().waitFor({ timeout: 60_000 });
  await shot("interrupted");
});

await step("upload", async () => {
  await openThread(cfg.threadId);
  await focusComposer();
  const [chooser] = await Promise.all([
    page.waitForEvent("filechooser", { timeout: 60_000 }),
    page.getByRole("button", { name: "Attach files" }).first().click({ timeout: 60_000 }),
  ]);
  await chooser.setFiles({ name: "phone.png", mimeType: "image/png", buffer: Buffer.from(cfg.image, "base64") });
  await focusComposer();
  await page.keyboard.type("lab:image from the phone");
  const editor = page.locator(composer).first();
  for (let i = 0; i < 60; i++) {
    await page.keyboard.press("Enter");
    await sleep(2_000);
    if (!((await editor.textContent().catch(() => "")) ?? "").includes("lab:image from the phone")) break;
    await editor.click({ timeout: 10_000 }).catch(() => {});
  }
  await waitText("I received the image", 120_000);
  await shot("upload");
});

await step("prompt-on-case70", async () => {
  await ask("shape case70");
  const states = await promptWithStates(cfg.prompt, "prompt", 120_000);
  await sleep(10_000);
  await shot("status-case70");
  await page.getByTestId("t3opt-status").click();
  await sleep(500);
  await shot("details-case70");
  await page.getByTestId("t3opt-details").click();
  await waitText(cfg.replyStart.slice(0, 30), 600_000);
  await shot("text-case70");
  const approve = page.getByRole("button", { name: /^(Approve|Yes)$/ }).first();
  await approve.waitFor({ timeout: 600_000 });
  await approve.click();
  await ask("shape clean");
  await waitText(cfg.replyEnd, 300_000);
  return states;
});

await step("thread-fills-newest-first", async () => {
  await ask("shape case50");
  await page.evaluate(() => window.t3opt.store.clearContent());
  await reload();
  // The newest assistant text shows before the thread's first prompt.
  const order = [];
  const until = Date.now() + 300_000;
  let n = 0;
  while (Date.now() < until && order.length < 2) {
    for (const [label, text] of [
      ["newest", cfg.replyEnd],
      ["oldest", "The lab thread is set up"],
    ]) {
      if (order.includes(label)) continue;
      if (await page.getByText(text, { exact: false }).first().isVisible().catch(() => false)) order.push(label);
    }
    if (n < 4) await shot(`filling-${n++}`);
    await sleep(1_500);
  }
  await ask("shape clean");
  if (order[0] !== "newest") throw new Error(`shown in the order ${order.join(", ")}`);
  return order.join(" before ");
});

await step("reload-old-connection-alive", async () => {
  const before = (await link())?.generation;
  await reload();
  await appReady();
  await page.waitForFunction(() => window.t3opt?.engine?.status().link.trying.endsWith("live"), null, {
    timeout: 60_000,
  });
  const after = (await link())?.generation;
  if (!(after > before)) throw new Error(`generation ${before} then ${after}`);
  await promptWithStates("lab:warmup after the reload", "after-reload", 60_000);
  return `generation ${before} to ${after}`;
});

await step("outage-recovered-on-the-same-connection", async () => {
  const before = await link();
  await ask("outage on");
  await sleep(cfg.outageSeconds * 1000);
  await shot("outage");
  await ask("outage off");
  await page.waitForFunction((t) => (window.t3opt?.engine?.status().link.lastDataAt ?? 0) > t, Date.now(), {
    timeout: 300_000,
  });
  const after = await link();
  if (after.newConnections !== before.newConnections) {
    throw new Error(`made ${after.newConnections - before.newConnections} new connections`);
  }
  if (after.longestRecoveredGapMs < cfg.outageSeconds * 800) {
    throw new Error(`longest gap recovered on the same connection ${after.longestRecoveredGapMs} ms`);
  }
  await shot("after-outage");
  return `${after.iceRestarts - before.iceRestarts} ICE restarts, gap ${(after.longestRecoveredGapMs / 1000).toFixed(1)} s`;
});

await step("history-and-images-without-http", async () => {
  await ask("block https on");
  try {
    await reload();
    await appReady();
    await openThread(cfg.threadId);
    await waitText("The lab thread is set up", 60_000);
    await waitText("I received the image", 60_000);
    const image = page.locator('img[src^="blob:"]').first();
    await image.waitFor({ timeout: 60_000 });
    await image.scrollIntoViewIfNeeded();
    await shot("no-http");
  } finally {
    await ask("block https off");
  }
});

await step("offline-open", async () => {
  await ask("outage on");
  try {
    await reload();
    await appReady();
    await waitText(cfg.replyEnd, 60_000);
    await shot("offline");
  } finally {
    await ask("outage off");
  }
});

await step("websocket-fallback", async () => {
  await ask("block udp on");
  try {
    await reload();
    await appReady();
    await page.waitForFunction(() => window.t3opt?.engine?.status().link.transport === "websocket", null, {
      timeout: 90_000,
    });
    const states = await promptWithStates("lab:warmup over the WebSocket", "websocket", 60_000);
    await page.getByTestId("t3opt-status").click();
    await shot("websocket-details");
    await page.getByTestId("t3opt-details").click();
    return states;
  } finally {
    await ask("block udp off");
  }
});

await step("update", async () => {
  const before = await swVersion();
  await ask("swap build");
  await page.evaluate(async () => (await navigator.serviceWorker.getRegistration())?.update());
  const button = page.getByTestId("t3opt-update");
  await button.waitFor({ timeout: 180_000 });
  await shot("update-offered");
  await button.click();
  await page.waitForFunction((v) => fetch("/__sw").then((r) => r.json()).then((j) => j.version !== v), before, {
    timeout: 120_000,
    polling: 1_000,
  });
  await appReady();
  await shot("updated");
  return `${before} to ${await swVersion()}`;
});

await step("deleted-while-sidecar-down", async () => {
  if (!newThreadId) throw new Error("no thread from add-project-and-thread");
  await openThread(cfg.threadId);
  await ask(`delete-offline ${newThreadId}`);
  const until = Date.now() + 300_000;
  while (Date.now() < until) {
    const listed = await page.evaluate(async (id) => {
      const shell = await (await fetch("/api/orchestration/shell")).json();
      return [...shell.threads, ...shell.archivedThreads].some((t) => t.id === id);
    }, newThreadId);
    if (!listed) {
      await openSidebar();
      await shot("deleted");
      return;
    }
    await sleep(2_000);
  }
  throw new Error("the deleted thread is still on the phone");
});

await step("cache-lost-token-kept", async () => {
  await page.evaluate(() => window.t3opt.store.clearContent());
  await openThread(cfg.threadId);
  await waitText(cfg.replyEnd, 120_000);
});

await step("token-lost", async () => {
  await page.evaluate(() => window.t3opt.store.clearAll());
  await page.goto(cfg.origin, { waitUntil: "commit", timeout: 60_000 });
  await page.locator("#pairing-token").waitFor({ timeout: 60_000 });
  await shot("pairing-again");
  const code = await ask("mint-code");
  await page.locator("#pairing-token").fill(code);
  await page.getByRole("button", { name: "Continue" }).click();
  await appReady();
  await openThread(cfg.threadId);
  await waitText(cfg.replyEnd, 120_000);
});

const failed = results.filter((r) => !r.ok).map((r) => r.name);
emit("note", { text: failed.length === 0 ? "all steps passed" : `failed: ${failed.join(", ")}` });
await browser.close();
process.exit(failed.length === 0 ? 0 : 1);
