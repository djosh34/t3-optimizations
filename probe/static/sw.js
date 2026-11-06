// Probe service worker: precache the page, serve navigations network-first
// with a cache fallback so the page opens with no network.
const CACHE = "probe-v1";
const ASSETS = ["/", "/manifest.json", "/icon.png"];

self.addEventListener("install", (e) => {
  e.waitUntil(caches.open(CACHE).then((c) => c.addAll(ASSETS)).then(() => self.skipWaiting()));
});
self.addEventListener("activate", (e) => e.waitUntil(self.clients.claim()));

self.addEventListener("fetch", (e) => {
  const req = e.request;
  if (req.method !== "GET") return;
  const url = new URL(req.url);
  const isPage = req.mode === "navigate";
  if (url.origin !== location.origin || !(isPage || ASSETS.includes(url.pathname))) return;
  const key = isPage ? "/" : url.pathname;
  e.respondWith((async () => {
    const cache = await caches.open(CACHE);
    try {
      const ctrl = new AbortController();
      const t = setTimeout(() => ctrl.abort(), 4000);
      const resp = await fetch(req, { signal: ctrl.signal });
      clearTimeout(t);
      if (resp.ok) cache.put(key, resp.clone());
      return resp;
    } catch (err) {
      const hit = await cache.match(key);
      if (hit) return hit;
      throw err;
    }
  })());
});
