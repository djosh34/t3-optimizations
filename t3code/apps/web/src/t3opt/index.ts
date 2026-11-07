// The home-screen web app's start (issue #45): the t3code web client served
// by the sidecar, talking to it through sync-core. Built with T3OPT_BUILD=1
// (lab/scripts/build-webapp.sh); a normal build never loads this module.

import "./t3opt.css";

import { createElement } from "react";
import { createRoot } from "react-dom/client";

import { Engine } from "./engine";
import { registerServiceWorker } from "./serviceWorker";
import { installShim, shimEvents, shimTrace } from "./shim";
import { StatusLine } from "./StatusLine";
import { Store } from "./store";

/** Link statistics go to the sidecar this often, and when the page is hidden. */
const REPORT_EVERY_MS = 60_000;

declare global {
  interface Window {
    /** For the lab's driver and tests. */
    t3opt?: {
      engine: Engine | null;
      store: Store;
      trace: string[];
      /** The newest prompt's state, its sequence number as a string. */
      prompt: () => { seq: string; state: string; at: number } | null;
      /** What the phone has of a thread's objects, by collection or text kind. */
      summary: (threadId: string) => Record<string, string>;
    };
  }
}

function startReports(engine: Engine, token: string, realFetch: typeof fetch) {
  const report = () => {
    const { link, stats } = engine.status();
    const body = {
      transport: link.transport,
      generation: link.generation,
      handshakeMs: link.handshakeMs,
      loss: stats.loss,
      srttMs: stats.srttMs,
      silences: link.silences,
      longestSilenceMs: link.longestSilenceMs,
      longestRecoveredGapMs: link.longestRecoveredGapMs,
      iceRestarts: link.iceRestarts,
      newConnections: link.newConnections,
      userAgent: navigator.userAgent,
    };
    // Best effort over HTTPS; a lost report is replaced by the next one.
    void realFetch("/api/link-report", {
      method: "POST",
      headers: { authorization: `Bearer ${token}`, "content-type": "application/json" },
      body: JSON.stringify(body),
      keepalive: true,
    }).catch(() => {});
  };
  setInterval(report, REPORT_EVERY_MS);
  document.addEventListener("visibilitychange", () => {
    if (document.visibilityState === "hidden") report();
  });
}

function mountStatusLine(engine: Engine | null) {
  const host = document.createElement("div");
  host.id = "t3opt-status-root";
  document.body.prepend(host);
  document.documentElement.classList.add("t3opt");
  createRoot(host).render(createElement(StatusLine, { engine }));
}

export async function startT3opt(): Promise<void> {
  const realFetch = window.fetch.bind(window);
  // The client reloads once when a lazy chunk fails; the lab's driver logs this.
  window.addEventListener("vite:preloadError", (event) => {
    console.warn("t3opt: a chunk failed to load", (event as Event & { payload?: unknown }).payload);
  });
  void registerServiceWorker().catch((error: unknown) =>
    console.warn("t3opt: no service worker", error),
  );
  const store = await Store.open();
  const saved = await store.load();
  const pairing = saved.pairing;
  const engine =
    pairing === null
      ? null
      : Engine.start({ wasmUrl: "/phone.wasm", store, saved, pairing, realFetch });
  installShim({
    realFetch,
    pairing,
    engine,
    pair: async (code) => {
      const r = await realFetch("/api/pair", {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify({ code }),
      });
      if (!r.ok) throw new Error(await r.text());
      const p = (await r.json()) as { device: string; token: string };
      await store.clearAll();
      await store.set({ pairing: { device: p.device, token: p.token } });
    },
  });
  const started = engine === null ? null : await engine;
  window.t3opt = {
    engine: started,
    store,
    trace: shimTrace,
    prompt: () => {
      const p = shimEvents.prompt;
      return p === null ? null : { seq: String(p.seq), state: p.state, at: p.at };
    },
    summary: (threadId) => {
      const out: Record<string, string> = {};
      for (const o of started?.objects.ofThread(threadId) ?? []) {
        const name = o.metaJson.k ?? o.metaJson.c ?? "?";
        const have = o.ranges.reduce((n, [a, b]) => n + (b - a), 0);
        out[name] = `${out[name] ? `${out[name]} ` : ""}${have}/${o.len}`;
      }
      return out;
    },
  };
  mountStatusLine(started);
  if (started !== null && pairing !== null) startReports(started, pairing.token, realFetch);
}
