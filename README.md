# qwarks
Quick Webassembly Animation Rigidbody Kinematics Simulator

## Layout

- `crates/` — public library crates (`qwarks_draw`, `qwarks_loop`)
- `www/` — wasm-bindgen crate (`qwarks_www`) plus a Vite site that loads it

## Development

Requires Rust with the `wasm32-unknown-unknown` target, [wasm-pack](https://rustwasm.github.io/wasm-pack/), and Node.js.

```sh
cargo test              # test the workspace
cd www
npm install
npm run dev             # build wasm (dev) and start the Vite dev server;
                        # Rust changes rebuild the wasm and reload the page
npm run build           # build wasm (release) and bundle into www/dist
```
