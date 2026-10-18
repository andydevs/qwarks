# qwarks_loop

A safe `requestAnimationFrame` loop for Rust WebAssembly. `RAFLoop` calls your
closure once per browser animation frame with timing information, and stops
automatically when it is dropped.

## Installation

Inside the qwarks workspace, add the workspace dependency:

```toml
[dependencies]
qwarks_loop.workspace = true
```

From another project, depend on it via git:

```toml
[dependencies]
qwarks_loop = { git = "https://github.com/andydevs/qwarks" }
```

The crate targets `wasm32-unknown-unknown` and must run in a browser context
where `window` is defined.

## Usage

Create a `RAFLoop` with a closure that takes a `FrameCtx`:

```rust
use qwarks_loop::{FrameCtx, RAFLoop};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
fn start() {
    let rl = RAFLoop::new(|ctx: FrameCtx| {
        let seconds = ctx.timestamp / 1000.0;
        // update and draw your scene here
    })
    .expect("Failed to start animation loop");

    // The loop runs until `rl` is dropped. To keep it running for the life
    // of the page, leak it:
    Box::leak(Box::new(rl));
}
```

`FrameCtx` fields:

| Field         | Type  | Description                                               |
| ------------- | ----- | --------------------------------------------------------- |
| `frame_count` | `u32` | Frames completed before this one (`0` on the first frame) |
| `timestamp`   | `f64` | `DOMHighResTimeStamp` for this frame, in milliseconds      |
| `delta`       | `f64` | Milliseconds since the previous frame (`0.0` on the first) |

Notes:

- `RAFLoop::new` returns `Err` if the first frame cannot be requested (for
  example, when `window` is not defined).
- Dropping a `RAFLoop` cancels its pending frame request. Store it somewhere
  that outlives the animation, or leak it as above.
- `RAFLoop::new` installs `console_error_panic_hook`, so panics in your
  callback are reported to the browser console.
