//! WebAssembly entry point for the qwarks demo site.

mod console;
use qwarks_draw::{Canvas, Circle, Draw, Styled};
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
    let x0 = f64::from(width) / 2.0;
    let y0 = f64::from(height) / 2.0;
    let rl = RAFLoop::new(move |ctx: FrameCtx| {
        let t = ctx.timestamp / 1000.0;

        // Circle 0
        let circle0 = Circle::new(x0, y0, 20.0).fill(true).stroke(false);
        let circle0 = Styled::new(circle0).fill("#aaaa00");

        // Circle 1
        let (x1, y1) = (3.0 * t).sin_cos();
        let x1 = x0 + x1 * 100.0;
        let y1 = y0 + y1 * 100.0;
        let circle1 = Circle::new(x1, y1, 10.0).fill(true).stroke(false);
        let circle1 = Styled::new(circle1).fill("#00aaff");

        // Circle 2
        let (x2, y2) = (4.0 * t + 1.0).sin_cos();
        let x2 = x0 + x2 * 80.0;
        let y2 = y0 + y2 * 80.0;
        let circle2 = Circle::new(x2, y2, 10.0).fill(true).stroke(false);
        let circle2 = Styled::new(circle2).fill("#ff00aa");

        my_canvas.clear();
        circle0.draw(&my_canvas);
        circle1.draw(&my_canvas);
        circle2.draw(&my_canvas);
    })
    .expect("Failed to create loop");

    // Leak
    Box::leak::<'static>(Box::new(rl));
}
