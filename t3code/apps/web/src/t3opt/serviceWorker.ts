// The service worker that keeps the app shell (issue #23): it is fetched and
// updated over HTTPS like any page, never over the sync link. A new build
// installs in the background; the status line then offers a reload.

export const updateEvents = {
  waiting: false,
  listeners: new Set<() => void>(),
  apply: () => {},
};

function setWaiting(worker: ServiceWorker) {
  updateEvents.waiting = true;
  updateEvents.apply = () => worker.postMessage("skip-waiting", []);
  for (const l of updateEvents.listeners) l();
}

export async function registerServiceWorker(): Promise<void> {
  if (!("serviceWorker" in navigator)) return;
  let reloading = false;
  // The first install takes control without a reload; only an update reloads.
  let controlled = navigator.serviceWorker.controller !== null;
  navigator.serviceWorker.addEventListener("controllerchange", () => {
    if (!controlled) {
      controlled = true;
      return;
    }
    if (reloading) return;
    reloading = true;
    location.reload();
  });
  const reg = await navigator.serviceWorker.register("/sw.js", { updateViaCache: "none" });
  const watch = (worker: ServiceWorker | null) => {
    if (!worker) return;
    worker.addEventListener("statechange", () => {
      // Installed with an older worker in control: an update, not the first install.
      if (worker.state === "installed" && navigator.serviceWorker.controller) setWaiting(worker);
    });
  };
  if (reg.waiting && navigator.serviceWorker.controller) setWaiting(reg.waiting);
  watch(reg.installing);
  reg.addEventListener("updatefound", () => watch(reg.installing));
  void dropOldShells(reg).catch(() => {});
  const check = () => void reg.update().catch(() => {});
  setInterval(check, 15 * 60_000);
  document.addEventListener("visibilitychange", () => {
    if (document.visibilityState === "visible") check();
  });
}

/**
 * Deletes the shells of older versions. The worker does it when it activates,
 * but WebKit can activate a waiting worker as the app starts without that
 * running to the end; with no update on its way, every other shell is unused.
 */
async function dropOldShells(reg: ServiceWorkerRegistration): Promise<void> {
  if (reg.waiting || reg.installing || !navigator.serviceWorker.controller) return;
  const r = await fetch("/__sw");
  if (!r.ok) return;
  const { shell } = (await r.json()) as { shell?: string };
  if (!shell) return;
  for (const name of await caches.keys()) {
    if (name.startsWith("t3opt-shell-") && name !== shell) await caches.delete(name);
  }
}
