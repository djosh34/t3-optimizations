// The live check of an installed t3opt (issue #48): Playwright WebKit opens
// the home-screen web app at its tailnet URL, pairs with a code, adds a
// project, starts a thread in t3opt-server, sends a short prompt to a real
// agent and waits for its reply. Runs in the lab's phone image; see
// deploy/live-check.sh, which starts it.
//
// Usage: node live-check.mjs <origin> <pairing code> <safari|home-screen>
//   safari: the Playwright device "iPhone 17 Pro" (402x681 at 3x).
//   home-screen: the same at 402x874, the home-screen app without browser bars.
// Screenshots go to /out/<size>-<step>.png; stdout gets one line per step.
import { devices, webkit } from "/opt/driver/node_modules/playwright/index.mjs";

const [origin, code, size] = process.argv.slice(2);
if (!origin || !code || !["safari", "home-screen"].includes(size)) {
  console.error("usage: node live-check.mjs <origin> <pairing code> <safari|home-screen>");
  process.exit(2);
}

const device = devices["iPhone 17 Pro"];
const viewport = size === "safari" ? device.viewport : { width: 402, height: 874 };
const browser = await webkit.launch();
const context = await browser.newContext({ ...device, viewport });
const page = await context.newPage();
page.on("console", (m) => {
  if (m.type() === "error") console.error(`page error: ${m.text().slice(0, 300)}`);
});

const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));
const started = Date.now();
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
    console.log(`${size} ${name}: failed: ${String(error.message ?? error).split("\n")[0]}`);
    throw error;
  }
}

// A server without projects shows t3code's welcome over the app, some
// seconds after the app is up. Waits for it up to `timeout` and closes it.
async function closeWelcome(timeout) {
  const welcome = page.getByText("Connect your computers").first();
  const until = Date.now() + timeout;
  while (Date.now() < until) {
    if (await welcome.isVisible().catch(() => false)) {
      await page.getByRole("button", { name: "Close" }).first().click({ timeout: 5_000 });
      await welcome.waitFor({ state: "hidden", timeout: 10_000 });
      return "closed the welcome";
    }
    await sleep(500);
  }
  return "no welcome";
}

const composer = '[data-testid="composer-editor"]:visible';
const project = `live-check-${size}-${new Date().toISOString().slice(0, 16).replace(/[-:T]/g, "")}`;
// A question whose answer is not in the prompt, so the reply is the agent's.
const prompt = "What is 17 times 23? Answer with the number only.";
const answer = "391";

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
    return await closeWelcome(30_000);
  });

  await step("project-added", async () => {
    await page.getByRole("button", { name: "Toggle main sidebar" }).first().click({ timeout: 30_000 });
    await sleep(800);
    await page.getByRole("button", { name: "Add project" }).first().click({ timeout: 30_000 });
    await page.getByText("New project", { exact: true }).first().click({ timeout: 30_000 });
    await page.keyboard.type(project);
    await page.keyboard.press("Enter");
    await page.getByText(`Created ${project}`).first().waitFor({ timeout: 60_000 });
    // The phone's sidebar is a sheet over the thread: close it, and the toast.
    await sleep(1_000);
    await page.keyboard.press("Escape");
    await page.locator(composer).first().click({ timeout: 30_000 });
    return project;
  });

  let threadUrl = "";
  await step("prompt-sent", async () => {
    await page.keyboard.type(prompt);
    await page.keyboard.press("Enter");
    await page.getByText(prompt).first().waitFor({ timeout: 60_000 });
    threadUrl = page.url();
    return new URL(threadUrl).pathname;
  });

  await step("reply", async () => {
    // The answer shows in the agent's message, apart from the prompt.
    await page
      .locator(`text=/^\\s*${answer}\\.?\\s*$/`)
      .first()
      .waitFor({ timeout: 300_000 });
    return `the agent answered ${answer}`;
  });
} catch {
  // The step said why.
}

console.log(`${size}: ${failed ? "failed" : "all steps passed"} in ${((Date.now() - started) / 1000).toFixed(0)} s`);
await browser.close();
process.exit(failed ? 1 : 0);
