// The WebKit part of the main sidecar's live check (issue #85): the
// home-screen web app pairs with a code and works only on the check's own
// thread. As the Safari tab it starts that thread in an existing project
// with GPT-6 Luna on pi at low thinking, sends one reply-only prompt and
// times the reply; as the home-screen app it opens the thread and sees the
// reply. Runs in the lab's phone image; see deploy/main-live-check.sh,
// which starts it.
//
// The page never sends `thread.visit` (no-visits.mjs), so no thread's read
// mark changes, whatever the app opens.
// stdout names no thread, project or message, only steps and times.
//
// Usage: node main-live-check.mjs <origin> <pairing code> <safari|home-screen> <project> <title>
//   safari: the Playwright device "iPhone 17 Pro" (402x681 at 3x); starts the thread.
//   home-screen: the same at 402x874, the home-screen app; opens the thread.
// Screenshots go to /out/<size>-<step>.png, for a local look only.
import { devices, webkit } from "/opt/driver/node_modules/playwright/index.mjs";
import { noVisits } from "./no-visits.mjs";

const [origin, code, size, project, title] = process.argv.slice(2);
if (!origin || !code || !["safari", "home-screen"].includes(size) || !project || !title) {
  console.error("usage: node main-live-check.mjs <origin> <code> <safari|home-screen> <project> <title>");
  process.exit(2);
}

// deploy/main-live-check.sh looks for this prompt, so a thread is made once.
const prompt = "Reply with the words: t3opt live check ok";
const model = /^GPT-6 Luna , Pi openai-codex\b/;
const reply = /^\s*t3opt live check ok\.?\s*$/i;

const device = devices["iPhone 17 Pro"];
const viewport = size === "safari" ? device.viewport : { width: 402, height: 874 };
const browser = await webkit.launch();
const context = await browser.newContext({ ...device, viewport });

await context.addInitScript(noVisits);

const page = await context.newPage();
const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));
let failed = false;

// Errors are not printed: their text can name threads.
async function step(name, run) {
  const t = Date.now();
  try {
    const detail = await run();
    await page.screenshot({ path: `/out/${size}-${name}.png` });
    console.log(`${size} ${name}: ok in ${((Date.now() - t) / 1000).toFixed(1)} s${detail ? ` (${detail})` : ""}`);
  } catch (error) {
    failed = true;
    await page.screenshot({ path: `/out/${size}-${name}-failed.png` }).catch(() => {});
    console.log(`${size} ${name}: failed (${error?.name ?? "error"}); see /out/${size}-${name}-failed.png`);
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

// Times until a leaf element of the conversation shows text that matches
// the regular expression `source`, on screen (in the viewport and fully opaque), polled each 16 ms.
async function shown(source, timeout) {
  const t = performance.now();
  await page.waitForFunction(
    ({ source, prompt }) => {
      const re = new RegExp(source, "i");
      const visible = (e) => {
        for (let n = e; n; n = n.parentElement) {
          const style = getComputedStyle(n);
          if (style.opacity !== "1" || style.visibility === "hidden") return false;
        }
        const r = e.getBoundingClientRect();
        return r.width > 0 && r.bottom > 0 && r.top < innerHeight && r.right > 0 && r.left < innerWidth;
      };
      const conversation = document.querySelector('[aria-label="Conversation"]');
      return [...(conversation?.querySelectorAll("p, span, div") ?? [])].some((e) => {
        const text = e.textContent ?? "";
        return e.children.length === 0 && !text.includes(prompt) && re.test(text) && visible(e);
      });
    },
    { source, prompt },
    { timeout, polling: 16 },
  );
  return Math.round(performance.now() - t);
}

const input = page.getByRole("region", { name: "Message input" });

try {
  await step("open", async () => {
    await page.goto(origin, { waitUntil: "commit", timeout: 120_000 });
    await page.locator("#pairing-token").waitFor({ timeout: 120_000 });
  });

  await step("paired", async () => {
    await page.locator("#pairing-token").fill(code);
    await page.getByRole("button", { name: "Continue" }).click();
    await page.locator("#pairing-token").waitFor({ state: "detached", timeout: 120_000 });
    const welcome = await closeWelcome(10_000);
    await page.getByRole("button", { name: "Toggle main sidebar" }).first().waitFor({ timeout: 120_000 });
    return welcome;
  });

  if (size === "safari") {
    await step("draft", async () => {
      // The app opens on a new thread's draft; it picks the project here.
      await input.getByRole("combobox").first().click({ timeout: 60_000 });
      await page.getByRole("combobox", { name: "Search projects" }).fill(project);
      await page.getByRole("option", { name: project, exact: true }).click({ timeout: 30_000 });
      await input.locator('[data-chat-provider-model-picker="true"]').first().click({ timeout: 30_000 });
      await page.getByRole("combobox", { name: "Search models..." }).fill("luna");
      await page.getByRole("option", { name: model }).first().click({ timeout: 30_000 });
      await sleep(500);
      // Choosing a model closes the picker; the thinking level is in it too.
      if (!(await page.getByRole("radiogroup", { name: "Thinking" }).isVisible())) {
        await input.locator('[data-chat-provider-model-picker="true"]').first().click({ timeout: 30_000 });
      }
      await page.getByRole("radiogroup", { name: "Thinking" }).getByRole("radio", { name: "Low" }).click();
      await page.keyboard.press("Escape");
      const label = await input.locator('[data-chat-provider-model-picker="true"]').first().getAttribute("aria-label");
      if (!/Luna/.test(label ?? "") || !/Low/.test(label ?? "")) throw new Error("the model is not GPT-6 Luna, Low");
      return label;
    });

    await step("sent", async () => {
      await input.getByRole("textbox").first().click({ timeout: 30_000 });
      await page.keyboard.type(prompt);
      const t = performance.now();
      await page.keyboard.press("Enter");
      const first = await shown("^\\s*t3", 300_000);
      await shown(reply.source, 300_000);
      return `send to first reply text ${first} ms, to the whole reply ${Math.round(performance.now() - t)} ms`;
    });

    await step("titled", async () => {
      // t3 titles a new thread from its first prompt; the check renames it.
      await sleep(5_000);
      await page.getByRole("button", { name: /^Thread actions for / }).first().dblclick({ timeout: 30_000 });
      await page.getByRole("textbox", { name: "Thread title" }).fill(title);
      await page.keyboard.press("Enter");
      await page.getByRole("button", { name: `Thread actions for ${title}`, exact: true }).first().waitFor({ timeout: 30_000 });
    });
  } else {
    // The sidebar's row; the title also sits in a screen-reader-only label.
    const row = page
      .getByText(title, { exact: true })
      .and(page.locator(":not(.sr-only)"))
      .filter({ visible: true })
      .first();
    await step("listed", async () => {
      await page.getByRole("button", { name: "Toggle main sidebar" }).first().click({ timeout: 30_000 });
      await row.waitFor({ timeout: 120_000 });
    });
    await step("opened", async () => {
      const t = performance.now();
      await row.click({ timeout: 30_000 });
      await shown(reply.source, 120_000);
      return `tap to the reply on screen ${Math.round(performance.now() - t)} ms`;
    });
  }
} catch {
  // The step said where.
}

await browser.close();
process.exit(failed ? 1 : 0);
