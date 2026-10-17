import init, { version } from "./pkg/qwarks_www.js";

await init();

document.getElementById("version").textContent = `qwarks v${version()}`;
