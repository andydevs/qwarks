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

    // Create loop
    let my_canvas = Canvas::from_selector("#spooky-canvas").expect("Can't make canvas");
    let (width, height) = my_canvas.dimensions();
    let center_x = f64::from(width) / 2.0;
    let center_y = f64::from(height) / 2.0;
    let rl = RAFLoop::new(move |ctx: FrameCtx| {
        let t = ctx.timestamp / 1000.0;

        // Circle 1
        let (x0, y0) = (3.0 * t).sin_cos();
        let x0 = center_x + x0 * 100.0;
        let y0 = center_y + y0 * 100.0;
        let circle0 = Circle::new(x0, y0, 10.0).fill(true).stroke(false);
        let circle0 = Styled::new(circle0).fill("#00aaff");

        // Circle 2
        let (x1, y1) = (4.0 * t + 1.0).sin_cos();
        let x1 = center_x + x1 * 120.0;
        let y1 = center_y + y1 * 120.0;
        let circle1 = Circle::new(x1, y1, 10.0).fill(true).stroke(false);
        let circle1 = Styled::new(circle1).fill("#ff00aa");

        my_canvas.clear();
        my_canvas.draw(&circle0);
        my_canvas.draw(&circle1);
    })
    .expect("Failed to create loop");

    // Leak
    Box::leak::<'static>(Box::new(rl));
}
