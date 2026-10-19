# CLAUDE.md

Guidance for Claude Code when working in this repository.

## Project

qwarks (Quick Webassembly Animation Rigidbody Kinematics Simulator) is a Rust
workspace that compiles to WebAssembly and runs in the browser.

## Layout

- `crates/qwarks_draw/` — canvas drawing library (`Canvas`, `Draw` trait,
  `Circle`/`Line`/`Polyline`/`Rect` primitives, `Styled` wrapper). `rlib` only.
- `crates/qwarks_loop/` — `requestAnimationFrame` loop (`RAFLoop`, `FrameCtx`).
  Frame scheduling goes through the internal `FrameAdapter` trait so it can be
  mocked; `BrowserAdapter` is the real implementation. `rlib` only.
- `www/` — `qwarks-www` `cdylib` crate (wasm-bindgen entry point in
  `src/lib.rs`, `log!`/`warn!`/`error!` console macros in `src/console.rs`)
  plus a Vite site (`index.html`, `main.js`, `vite.config.js`, and the
  wasm-pack build/watch plugin in `plugins/wasm-pack.js`).
- `www/pkg/`, `www/dist/`, `target/` are build output and are gitignored.

Shared dependency versions and package metadata live in the root `Cargo.toml`
under `[workspace.dependencies]` / `[workspace.package]`; member crates use
`.workspace = true`.

## Commands

```sh
cargo build                          # build the workspace
cargo test                           # run tests
cargo doc -p <crate> --no-deps       # build docs for a crate
cd www && npm install                # install site dependencies
cd www && npm run dev                # wasm-pack (dev) + Vite dev server, rebuilds on Rust changes
cd www && npm run build              # wasm-pack (release) + bundle into www/dist
```

Requires the `wasm32-unknown-unknown` target, `wasm-pack`, and Node.js.
`plugins/wasm-pack.js` runs `wasm-pack build --target web --out-dir pkg --out-name qwarks_www`
from `www/` and watches the workspace's Rust sources.

## Conventions

- Rust edition 2024, MIT license.
- Every source file starts with a `//!` header; public and crate-internal
  items have `///` docs with `# Parameters`, `# Returns`, `# Side Effects`,
  and `# Panics` sections where applicable. Trait methods and their impls are
  both documented.
- Crate `README.md` files follow a Title / Installation / Usage layout
  (see `crates/qwarks_loop/README.md`).
