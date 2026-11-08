// The phone side of a lab run. Runs inside the t3lab-phone container with
// `docker exec`, so its own control traffic never crosses the shaped link.
//
// Argument: the path of a JSON config (see Config in internal/phone/phone.go).
// stdout: one JSON event per line, each with `t` (Unix ms) and `ev`.
// stdin: "go" once the lab applied the profile, "stop" at the deadline.
import { readFileSync } from "node:fs";
import { createInterface } from "node:readline";
import { devices, webkit } from "/opt/driver/node_modules/playwright/index.mjs";

const cfg = JSON.parse(readFileSync(process.argv[2], "utf8"));
const reply = cfg.reply;

function emit(ev, data = {}) {
  process.stdout.write(`${JSON.stringify({ t: Date.now(), ev, ...data })}\n`);
}

const commands = createInterface({ input: process.stdin })[Symbol.asyncIterator]();
async function nextCommand() {
  const { value, done } = await commands.next();
  return done ? "stop" : value.trim();
}

function normalize(text) {
  return text.replace(/[^A-Za-z0-9]/g, "");
}

// What the application received over WebSocket, after decompression. The
// byte counts start again when the profile is applied, like the lab's wire
// counts.
const stats = { rxBytes: 0, txBytes: 0, replyBytes: 0, duplicateReplyBytes: 0 };
const received = new Uint8Array(reply.length);
const promptRequestIds = new Set();
let promptSentAt;
let confirmed = false;

// Counts reply text the application received more than once. t3 sends each
// assistant message as an object with role "assistant" and its whole text so
// far; only messages created after the prompt was sent are the measured
// reply. The first copy of each character is new, later copies are
// duplicates.
function countReplyText(value) {
  if (Array.isArray(value)) {
    value.forEach(countReplyText);
    return;
  }
  if (!value || typeof value !== "object") return;
  if (
    value.role === "assistant" &&
    typeof value.text === "string" &&
    promptSentAt !== undefined &&
    Date.parse(value.createdAt) >= promptSentAt
  ) {
    for (const [at, n] of replyPieces(normalize(value.text))) {
      stats.replyBytes += n;
      for (let i = at; i < at + n; i++) {
        if (received[i]) stats.duplicateReplyBytes++;
        received[i] = 1;
      }
    }
    return;
  }
  Object.values(value).forEach(countReplyText);
}

function frameText(payload) {
  return typeof payload === "string" ? payload : payload.toString("utf8");
}

function parseFrames(text) {
  try {
    const value = JSON.parse(text);
    return Array.isArray(value) ? value : [value];
  } catch {
    return [];
  }
}

function watchSockets(page) {
  page.on("websocket", (ws) => {
    emit("ws-open");
    ws.on("close", () => emit("ws-close"));
    ws.on("framesent", ({ payload }) => {
      const text = frameText(payload);
      stats.txBytes += Buffer.byteLength(text);
      if (!text.includes(cfg.prompt)) return;
      for (const frame of parseFrames(text)) {
        if (frame.id !== undefined) promptRequestIds.add(String(frame.id));
      }
    });
    ws.on("framereceived", ({ payload }) => {
      const text = frameText(payload);
      stats.rxBytes += Buffer.byteLength(text);
      for (const frame of parseFrames(text)) {
        if (
          !confirmed &&
          frame._tag === "Exit" &&
          promptRequestIds.has(String(frame.requestId)) &&
          frame.exit?._tag === "Success"
        ) {
          confirmed = true;
          emit("prompt-confirmed");
        }
        countReplyText(frame);
      }
    });
  });
}

// Visible text: which characters of the reply the phone has shown. Rows can
// unmount when they scroll away, so what was seen once stays counted.
const seen = new Uint8Array(reply.length);
const lastRowText = new Map();

// The pieces of text that appear in the reply, in order, as [position,
// length]. Pieces under 8 characters are skipped: too short to place
// reliably. Text the reply does not have, such as t3's image marker, is
// stepped over.
function replyPieces(text) {
  const pieces = [];
  let rest = text;
  let from = 0;
  while (rest.length >= 8) {
    let lo = 0;
    let hi = rest.length;
    while (lo < hi) {
      const mid = (lo + hi + 1) >> 1;
      if (reply.indexOf(rest.slice(0, mid), from) >= 0) lo = mid;
      else hi = mid - 1;
    }
    if (lo >= 8) {
      const at = reply.indexOf(rest.slice(0, lo), from);
      pieces.push([at, lo]);
      from = at + lo;
      rest = rest.slice(lo);
    } else {
      rest = rest.slice(1);
    }
  }
  return pieces;
}

function markRow(text) {
  for (const [at, n] of replyPieces(text)) {
    for (let i = at; i < at + n; i++) seen[i] = 1;
  }
}

function visiblePrefix() {
  let n = 0;
  while (n < seen.length && seen[n]) n++;
  return n;
}

async function readAssistantRows(page) {
  return page.evaluate(() =>
    [...document.querySelectorAll('[data-message-role="assistant"]')].map((row) => {
      const clone = row.cloneNode(true);
      clone.querySelectorAll("button, time").forEach((node) => node.remove());
      return [row.dataset.timelineRowId ?? "", clone.textContent ?? ""];
    }),
  );
}

async function sampleText(page) {
  for (const [id, raw] of await readAssistantRows(page)) {
    const text = normalize(raw);
    if (lastRowText.get(id) === text) continue;
    lastRowText.set(id, text);
    markRow(text);
  }
  return visiblePrefix();
}

let stopping = false;
let stopRequested;
const stopped = new Promise((resolve) => {
  stopRequested = resolve;
});

function untilStop(promise) {
  return Promise.race([promise, stopped.then(() => "stopped")]);
}

async function screenshot(page, name) {
  try {
    await page.screenshot({ path: `/out/${name}.png`, timeout: 15_000 });
    emit("screenshot", { name });
  } catch (error) {
    emit("note", { text: `screenshot ${name} failed: ${error.message}` });
  }
}

async function openThread(page) {
  const threadUrl = `${cfg.origin}/${cfg.environmentId}/${encodeURIComponent(cfg.threadId)}`;
  await page.goto(threadUrl, { waitUntil: "commit", timeout: 0 });
}

async function waitForText(page, text, timeout) {
  await page.getByText(text, { exact: false }).first().waitFor({ state: "visible", timeout });
}

// The phone layout collapses the composer to one row until it is tapped.
const composerOrExpand = '[data-testid="composer-editor"]:visible, button[aria-label="Expand composer"]:visible';

async function focusComposer(page, timeout) {
  await page.locator(composerOrExpand).first().waitFor({ timeout });
  const expand = page.getByRole("button", { name: "Expand composer" });
  if (await expand.isVisible()) await expand.click({ timeout });
  await page.locator('[data-testid="composer-editor"]:visible').first().click({ timeout });
}

// The sidecar's web app has no WebSocket to watch: its sync shim says where
// the prompt is, and its engine counts the bytes.
let lastPromptState = "";
let lastSummary = 0;
async function watchWebapp(page) {
  if (Date.now() - lastSummary >= 10_000) {
    lastSummary = Date.now();
    const summary = await page.evaluate(
      (id) => {
        const status = window.t3opt?.engine?.status();
        return { objects: window.t3opt?.summary?.(id), link: status?.link, stats: status?.stats };
      },
      cfg.threadId,
    );
    emit("note", { text: `webapp ${JSON.stringify(summary)}` });
  }
  const p = await page.evaluate(() => window.t3opt?.prompt?.() ?? null);
  if (p === null || promptSentAt === undefined || p.at < promptSentAt - 1_000) return;
  if (p.state !== lastPromptState) {
    lastPromptState = p.state;
    emit("note", { text: `prompt ${p.state}` });
  }
  if (!confirmed && p.state === "confirmed") {
    confirmed = true;
    emit("prompt-confirmed");
  }
}

async function webappBytes(page) {
  return page.evaluate(() => {
    const status = window.t3opt?.engine?.status();
    const s = status?.stats;
    return {
      rx: s?.receivedWireBytes ?? 0,
      tx: s?.sentWireBytes ?? 0,
      content: status?.receivedContentBytes ?? 0,
      duplicates: s?.duplicateBytes ?? 0,
    };
  });
}

const device = devices["iPhone 17 Pro"];
const browser = await webkit.launch();
const context = await browser.newContext({ ...device, ignoreHTTPSErrors: true });
let page = await context.newPage();
watchSockets(page);

const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

// A fresh server's sole startup project counts as a first run, and t3code
// shows its welcome wizard over the app; a real owner's server does not.
async function closeWelcomeThenWait(ready, timeout) {
  const until = Date.now() + timeout;
  const welcome = page.getByText("Connect your computers").first();
  while (Date.now() < until) {
    if (await welcome.isVisible().catch(() => false)) {
      await page.getByRole("button", { name: "Close" }).first().click({ timeout: 5_000 }).catch(() => {});
    } else if (await ready.isVisible().catch(() => false)) {
      return;
    }
    await sleep(500);
  }
  await ready.waitFor({ timeout: 1_000 });
}

// Setup on the clean link: pair, open the thread, send the image turn.
async function setup() {
  await page.goto(cfg.pairUrl, { waitUntil: "domcontentloaded", timeout: 120_000 });
  await closeWelcomeThenWait(page.locator(composerOrExpand).first(), 120_000);
  emit("paired");
  const descriptor = await page.evaluate(async () => {
    const response = await fetch("/.well-known/t3/environment");
    return response.json();
  });
  cfg.environmentId = descriptor.environmentId;
  await openThread(page);
  await waitForText(page, "The lab thread is set up", 120_000);
  emit("thread-open");

  await focusComposer(page, 60_000);
  const [chooser] = await Promise.all([
    page.waitForEvent("filechooser", { timeout: 60_000 }),
    page.getByRole("button", { name: "Attach files" }).first().click({ timeout: 60_000 }),
  ]);
  await chooser.setFiles({
    name: "lab.png",
    mimeType: "image/png",
    buffer: Buffer.from(cfg.image, "base64"),
  });
  await focusComposer(page, 60_000);
  await page.keyboard.type(cfg.imagePrompt);
  // The composer does not send while the image is still uploading, which
  // through the sidecar takes a moment: Enter again until it is sent.
  const editor = page.locator('[data-testid="composer-editor"]:visible').first();
  for (let i = 0; i < 60; i++) {
    await page.keyboard.press("Enter");
    await page.waitForTimeout(2_000);
    if (!((await editor.textContent().catch(() => "")) ?? "").includes(cfg.imagePrompt)) break;
    await editor.click({ timeout: 10_000 }).catch(() => {});
  }
  await waitForText(page, "I received the image", 120_000);
  // The web app shows a reply's text before its turn has ended, with a
  // "running" pill. A prompt typed then waits as a follow-up until the turn
  // ends, so the run would measure the setup turn's sync.
  await page.getByText("running", { exact: true }).first().waitFor({ state: "hidden", timeout: 120_000 });
  // Until the client has the turn's end, Enter steers or queues instead of
  // starting a turn; the send button's label says which.
  await focusComposer(page, 60_000);
  await page.locator('button[aria-label="Submit message"]').first().waitFor({ state: "attached", timeout: 120_000 });
  await screenshot(page, "ready");
}

try {
  await setup();
} catch (error) {
  emit("note", { text: `setup failed: ${error.message.split("\n")[0]}` });
  await screenshot(page, "setup-failed");
  await browser.close();
  process.exit(1);
}
if (cfg.kind === "cold") await page.close();
emit("ready");

const go = await nextCommand();
if (go !== "go") {
  emit("note", { text: `expected go, got ${go}` });
}
stats.rxBytes = 0;
stats.txBytes = 0;
const webappStart = cfg.webapp
  ? await webappBytes(page).catch(() => ({ rx: 0, tx: 0, content: 0, duplicates: 0 }))
  : null;
void nextCommand().then(() => {
  stopping = true;
  stopRequested();
});

if (cfg.kind === "cold") {
  page = await context.newPage();
  watchSockets(page);
  emit("app-loading");
  const loaded = await untilStop(
    openThread(page)
      .then(() => page.locator(composerOrExpand).first().waitFor({ timeout: 0 }))
      .then(() => "loaded")
      .catch((error) => `failed: ${error.message}`),
  );
  if (loaded === "loaded") emit("app-loaded");
  else emit("note", { text: `app did not load: ${loaded}` });
}

// An Enter the web app's composer did not take (it stays full) is pressed
// again, as the owner would; the measure still starts at the first one. Not
// for the baseline client, whose composer may hold the text while its socket
// is stalled, where a second Enter could send the prompt twice.
let lastEnterAt = 0;
async function enterAgainIfIgnored(page) {
  if (!cfg.webapp || promptSentAt === undefined || Date.now() - lastEnterAt < 3_000) return;
  const editor = page.locator('[data-testid="composer-editor"]:visible').first();
  const text = (await editor.textContent({ timeout: 1_000 }).catch(() => "")) ?? "";
  if (!text.includes(cfg.prompt.slice(0, 24))) return;
  lastEnterAt = Date.now();
  emit("note", { text: "the prompt is still in the composer: Enter again" });
  await editor.click({ timeout: 2_000 }).catch(() => {});
  await page.keyboard.press("Enter");
}

if (!stopping) {
  const sent = await untilStop(
    (async () => {
      await focusComposer(page, 0);
      await page.keyboard.type(cfg.prompt);
      promptSentAt = Date.now();
      lastEnterAt = promptSentAt;
      emit("prompt-sent");
      await page.keyboard.press("Enter");
      return "sent";
    })().catch((error) => `failed: ${error.message}`),
  );
  if (sent !== "sent") emit("note", { text: `prompt not sent: ${sent}` });
}

let shown = 0;
let approvalVisible = false;
let scrollToEndShown = false;
let streamingShot = false;
while (!stopping) {
  try {
    if (cfg.webapp) await untilStop(watchWebapp(page));
    await untilStop(enterAgainIfIgnored(page));
    const n = await untilStop(sampleText(page));
    if (typeof n === "number" && n !== shown) {
      shown = n;
      emit("text", { n });
    }
    if (shown === reply.length) {
      emit("complete");
      break;
    }
    // The timeline stopped following new text: the owner would have to tap.
    const toEnd = page.getByRole("button", { name: "Scroll to end" }).first();
    if (await toEnd.isVisible().catch(() => false)) {
      if (!scrollToEndShown) emit("note", { text: "Scroll to end shows" });
      scrollToEndShown = true;
    } else {
      scrollToEndShown = false;
    }
    // One picture of a long reply streaming at the bottom of the timeline.
    if (!streamingShot && shown >= reply.length / 2) {
      streamingShot = true;
      await screenshot(page, "streaming");
    }
    const approve = page.getByRole("button", { name: /^(Approve|Yes)$/ }).first();
    if (await approve.isVisible()) {
      if (!approvalVisible) emit("approval-shown");
      approvalVisible = true;
      // The button stays disabled while the answer is on its way.
      if (await approve.isEnabled()) {
        await approve.click({ timeout: 2_000 });
        emit("approval-clicked");
      }
    } else {
      approvalVisible = false;
    }
  } catch (error) {
    emit("note", { text: `sample failed: ${error.message.split("\n")[0]}` });
  }
  await untilStop(new Promise((resolve) => setTimeout(resolve, 200)));
}

await screenshot(page, "end");
if (webappStart !== null) {
  const end = await webappBytes(page).catch(() => webappStart);
  stats.rxBytes = end.rx - webappStart.rx;
  stats.txBytes = end.tx - webappStart.tx;
  // The app gets content from sync-core, not reply text over a WebSocket:
  // all content it received, and of it what it had received before.
  stats.replyBytes = end.content - webappStart.content;
  stats.duplicateReplyBytes = end.duplicates - webappStart.duplicates;
  // The shim's last calls and fetches, to see what shared the link with the prompt.
  const trace = await page.evaluate(() => window.t3opt?.trace ?? []).catch(() => []);
  emit("note", { text: `shim trace\n${trace.join("\n")}` });
}
emit("stats", stats);
await browser.close();
process.exit(0);
