import { readFileSync, existsSync } from "node:fs";
import { resolve } from "node:path";

const root = resolve("web");
const htmlPath = resolve(root, "index.html");
const jsPath = resolve(root, "app.js");
const cssPath = resolve(root, "styles.css");

function fail(message) {
  console.error(`FAIL: ${message}`);
  process.exitCode = 1;
}

for (const [label, path] of [["HTML entrypoint", htmlPath], ["JavaScript entrypoint", jsPath], ["stylesheet", cssPath]]) {
  if (!existsSync(path)) fail(`Missing ${label}: ${path}`);
}

if (process.exitCode) process.exit();

const html = readFileSync(htmlPath, "utf8");
const js = readFileSync(jsPath, "utf8");

if (!/<html\s+lang=["']es["']/.test(html)) fail("index.html must declare Spanish as its document language.");
if (!/<link\b[^>]*href=["']\.\/styles\.css["']/.test(html)) fail("index.html must link ./styles.css.");
if (!/<script\b[^>]*src=["']\.\/app\.js["']/.test(html)) fail("index.html must load ./app.js.");
for (const id of ["resource-tree", "structure-panel", "map-panel", "details-content", "toast-region"]) {
  if (!html.includes(`id="${id}"`)) fail(`Missing required UI container #${id}.`);
}
if (!js.includes("localStorage.setItem(KEY")) fail("Expected explicit browser-local persistence.");
if (!js.includes("function normalizeWorkspace")) fail("Expected workspace import validation.");
if (!html.includes("no se conecta todavía con el núcleo Rust")) {
  fail("The help dialog must disclose that this prototype is not connected to the Rust core.");
}

if (!process.exitCode) console.log("Frontend smoke checks passed.");
