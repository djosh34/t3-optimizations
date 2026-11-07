// Writes <dist>/sw.js for the sidecar's web app: t3opt/sw.js with the
// precache list and a version that changes with any precached file.
// Usage: node scripts/t3opt-sw.mjs <dist>
import { createHash } from "node:crypto";
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const dist = process.argv[2];
if (!dist) {
  console.error("usage: t3opt-sw.mjs <dist>");
  process.exit(2);
}
const manifest = JSON.parse(readFileSync(join(dist, ".vite/manifest.json"), "utf8"));

// The entry and every route, with what they import statically: what the app
// needs to open any screen without the network.
const seen = new Set();
const visit = (key) => {
  if (seen.has(key) || !manifest[key]) return;
  seen.add(key);
  for (const i of manifest[key].imports ?? []) visit(i);
};
for (const [key, entry] of Object.entries(manifest)) {
  if (entry.isEntry || key.includes("src/routes/")) visit(key);
}
const files = new Set(["index.html", "phone.wasm", "manifest.webmanifest", "apple-touch-icon.png"]);
for (const key of seen) {
  const entry = manifest[key];
  files.add(entry.file);
  for (const f of [...(entry.css ?? []), ...(entry.assets ?? [])]) files.add(f);
}

const hash = createHash("sha256");
const template = readFileSync(new URL("../t3opt/sw.js", import.meta.url));
hash.update(template);
const precache = [...files].sort();
for (const f of precache) hash.update(f).update(readFileSync(join(dist, f)));
const version = hash.digest("hex").slice(0, 16);
const urls = precache.map((f) => (f === "index.html" ? "/index.html" : `/${f}`));

writeFileSync(
  join(dist, "sw.js"),
  `const VERSION = ${JSON.stringify(version)};\nconst PRECACHE = ${JSON.stringify(urls)};\n${template}`,
);
console.log(`sw.js: version ${version}, ${urls.length} files precached`);
