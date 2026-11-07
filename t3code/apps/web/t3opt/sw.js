// The service worker of the sidecar's web app (issue #23); see
// src/t3opt/README.md. scripts/t3opt-sw.mjs puts VERSION and PRECACHE in front
// of this file at build time.

/* global VERSION, PRECACHE */
const SHELL = `t3opt-shell-${VERSION}`;
const RUNTIME = "t3opt-runtime";

self.addEventListener("install", (event) => {
  event.waitUntil(
    (async () => {
      const cache = await caches.open(SHELL);
      await Promise.all(
        PRECACHE.map(async (url) => {
          const old = url.startsWith("/assets/") ? await caches.match(url) : undefined;
          if (old) return cache.put(url, old);
          const response = await fetch(new Request(url, { cache: "no-cache" }));
          if (!response.ok) throw new Error(`${url}: HTTP ${response.status}`);
          return cache.put(url, response);
        }),
      );
    })(),
  );
});

self.addEventListener("activate", (event) => {
  event.waitUntil(
    (async () => {
      for (const name of await caches.keys()) {
        if (name !== SHELL && name !== RUNTIME) await caches.delete(name);
      }
      await self.clients.claim();
    })(),
  );
});

self.addEventListener("message", (event) => {
  if (event.data === "skip-waiting") self.skipWaiting();
});

self.addEventListener("fetch", (event) => {
  const request = event.request;
  const url = new URL(request.url);
  if (request.method !== "GET" || url.origin !== self.location.origin) return;
  // Which version serves this page, for the status details and the lab.
  if (url.pathname === "/__sw") {
    event.respondWith(Response.json({ version: VERSION, shell: SHELL }));
    return;
  }
  // The sidecar's API and the sync WebSocket never come from a cache.
  if (url.pathname.startsWith("/api/") || url.pathname === "/sync" || url.pathname === "/sw.js") {
    return;
  }
  if (request.mode === "navigate") {
    event.respondWith(
      caches
        .open(SHELL)
        .then((cache) => cache.match("/index.html"))
        .then((hit) => hit ?? fetch(request)),
    );
    return;
  }
  event.respondWith(
    (async () => {
      const hit = await caches.match(request);
      if (hit) return hit;
      const response = await fetch(request);
      if (response.ok && url.pathname.startsWith("/assets/")) {
        const cache = await caches.open(RUNTIME);
        await cache.put(request, response.clone());
      }
      return response;
    })(),
  );
});
