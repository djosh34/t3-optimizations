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

const device = devices["iPhone 17 Pro"];
const browser = await webkit.launch();
const context = await browser.newContext({ ...device, ignoreHTTPSErrors: true });
let page = await context.newPage();
watchSockets(page);

// Setup on the clean link: pair, open the thread, send the image turn.
async function setup() {
  await page.goto(cfg.pairUrl, { waitUntil: "domcontentloaded", timeout: 120_000 });
  await page.locator(composerOrExpand).first().waitFor({ timeout: 120_000 });
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
  await page.keyboard.press("Enter");
  await waitForText(page, "I received the image", 120_000);
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

if (!stopping) {
  const sent = await untilStop(
    (async () => {
      await focusComposer(page, 0);
      await page.keyboard.type(cfg.prompt);
      promptSentAt = Date.now();
      emit("prompt-sent");
      await page.keyboard.press("Enter");
      return "sent";
    })().catch((error) => `failed: ${error.message}`),
  );
  if (sent !== "sent") emit("note", { text: `prompt not sent: ${sent}` });
}

let shown = 0;
let approvalVisible = false;
while (!stopping) {
  try {
    const n = await untilStop(sampleText(page));
    if (typeof n === "number" && n !== shown) {
      shown = n;
      emit("text", { n });
    }
    if (shown === reply.length) {
      emit("complete");
      break;
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
emit("stats", stats);
await browser.close();
process.exit(0);
