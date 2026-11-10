// The page script that keeps a Playwright page from sending `thread.visit`,
// for the checks that run the web app against a real t3code (issues #84 and
// #85): no thread's read mark changes, whatever the app opens. Use it with
// `context.addInitScript(noVisits)`.
//
// The shim puts its WebSocket on window; each one it makes is wrapped so a
// visit is answered here and never reaches the sidecar.
export function noVisits() {
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
}
