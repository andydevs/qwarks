//! WebAssembly entry point for the qwarks demo site.

mod console;
use qwarks_draw::{Canvas, Circle, Styled};
use qwarks_loop::{FrameCtx, RAFLoop};
use wasm_bindgen::prelude::*;

/// Runs when the module is initialized.
#[wasm_bindgen(start)]
fn start() {
    console_error_panic_hook::set_once();
    log!("Anim loaded");

    let my_canvas = Canvas::from_selector("#spooky-canvas").expect("Can't make canvas");
    let rl = RAFLoop::new(move |ctx: FrameCtx| {
        let (x, y) = (ctx.timestamp * 3.0 / 1000.0).sin_cos();
        let x = 300.0 + x * 100.0;
        let y = 200.0 + y * 100.0;
        let circle = Circle::new(x, y, 10.0).fill(true).stroke(false);
        let circle = Styled::new(circle).fill("#00aaff");

        my_canvas.clear();
        my_canvas.draw(&circle);
    })
    .expect("Failed to create loop");
    Box::leak::<'static>(Box::new(rl));
}
