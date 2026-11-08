import { showBootError } from "./lib/bootError";

// The sidecar's build (T3OPT_BUILD=1) puts the sync shim in front of the app first.
const beforeApp =
  import.meta.env.VITE_T3OPT === "1"
    ? import("./t3opt").then(({ startT3opt }) => startT3opt())
    : Promise.resolve();

// Shared dev chunks need the React refresh preamble first. Catch failures before React mounts.
void beforeApp
  .then(() => import("./main"))
  .then(({ startup }) => startup)
  .catch(showBootError);
