// Writes <dist>/sw.js for the sidecar's web app: t3opt/sw.js with the
// precache list and a version that changes with any precached file.
// Usage: node scripts/t3opt-sw.mjs <dist>
import * as NodeCrypto from "node:crypto";
import * as NodeFS from "node:fs";
import * as NodePath from "node:path";

const dist = process.argv[2];
if (!dist) {
  console.error("usage: t3opt-sw.mjs <dist>");
  process.exit(2);
}
const manifest = JSON.parse(
  NodeFS.readFileSync(NodePath.join(dist, ".vite/manifest.json"), "utf8"),
);

// The entry and every route, with what they import, statically and lazily:
// what the app needs to open any screen without the network. A lazy chunk
// missing offline makes the client reload itself (src/main.tsx). Left to load
// on first use: the file viewers, HEIC decoding and the code grammars.
const lazyOnly = (key) => key.includes("src/components/files/") || key.includes("/heic-to/");
const GRAMMARS = "/shiki/dist/langs.mjs";
const seen = new Set();
const visit = (key) => {
  if (seen.has(key) || !manifest[key]) return;
  seen.add(key);
  const entry = manifest[key];
  for (const i of entry.imports ?? []) visit(i);
  if (key.endsWith(GRAMMARS)) return;
  for (const i of entry.dynamicImports ?? []) if (!lazyOnly(i)) visit(i);
};
for (const [key, entry] of Object.entries(manifest)) {
  if (entry.isEntry || key.includes("src/routes/") || key.startsWith("src/t3opt/")) visit(key);
}
const files = new Set(["index.html", "phone.wasm", "manifest.webmanifest", "apple-touch-icon.png"]);
for (const key of seen) {
  const entry = manifest[key];
  files.add(entry.file);
  for (const f of [...(entry.css ?? []), ...(entry.assets ?? [])]) files.add(f);
}

// The worker that highlights diffs, with the chunks it imports (its own copy
// of the highlighter, outside the manifest), but for its grammars.
const IMPORT = /(?:import\(|from|import)\s*[`"'](\.\/[^`"']+\.(?:js|mjs|wasm))[`"']/g;
const scanned = new Set();
const addWithImports = (file) => {
  if (scanned.has(file)) return;
  scanned.add(file);
  files.add(file);
  if (NodePath.posix.basename(file).startsWith("langs-")) return;
  const refs = [...NodeFS.readFileSync(NodePath.join(dist, file), "utf8").matchAll(IMPORT)];
  for (const [, ref] of refs)
    addWithImports(NodePath.posix.join(NodePath.posix.dirname(file), ref));
};
const pool = Object.entries(manifest).find(([key]) => key.includes("DiffWorkerPoolProvider"));
const highlighter = pool
  ? /new Worker\(`\/(assets\/[^`]+)`/.exec(
      NodeFS.readFileSync(NodePath.join(dist, pool[1].file), "utf8"),
    )?.[1]
  : undefined;
if (!highlighter) {
  console.error("t3opt-sw: the highlighter worker was not found");
  process.exit(1);
}
addWithImports(highlighter);

const hash = NodeCrypto.createHash("sha256");
const template = NodeFS.readFileSync(new URL("../t3opt/sw.js", import.meta.url));
hash.update(template);
const precache = [...files].sort();
for (const f of precache) hash.update(f).update(NodeFS.readFileSync(NodePath.join(dist, f)));
const version = hash.digest("hex").slice(0, 16);
const urls = precache.map((f) => (f === "index.html" ? "/index.html" : `/${f}`));

NodeFS.writeFileSync(
  NodePath.join(dist, "sw.js"),
  `const VERSION = ${JSON.stringify(version)};\nconst PRECACHE = ${JSON.stringify(urls)};\n${template}`,
);
console.log(`sw.js: version ${version}, ${urls.length} files precached`);
