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
        my_canvas.clear();

        // Circle 0
        let circle0 = Circle::new(x0, y0, 20.0).fill(true).stroke(false);
        let circle0 = Styled::new(circle0).fill("#aaaa00");
        circle0.draw(&my_canvas);

        // Circle 1
        let (x1, y1) = (3.0 * t).sin_cos();
        let x1 = x0 + x1 * 120.0;
        let y1 = y0 + y1 * 120.0;
        let circle1 = Circle::new(x1, y1, 8.0).fill(true).stroke(false);
        let circle1 = Styled::new(circle1).fill("#00aaff");
        circle1.draw(&my_canvas);

        // Circle 2
        let (x2, y2) = (4.0 * t + 1.0).sin_cos();
        let x2 = x0 + x2 * 60.0;
        let y2 = y0 + y2 * 60.0;
        let circle2 = Circle::new(x2, y2, 4.0).fill(true).stroke(false);
        let circle2 = Styled::new(circle2).fill("#ff00aa");
        circle2.draw(&my_canvas);

        // Circle 3
        let (x3, y3) = (8.0 * t + 1.0).sin_cos();
        let x3 = x1 + x3 * 20.0;
        let y3 = y1 + y3 * 20.0;
        let circle3 = Circle::new(x3, y3, 2.0).fill(true).stroke(false);
        let circle3 = Styled::new(circle3).fill("#bbbbbb");
        circle3.draw(&my_canvas);

        // Circle 4
        let (x4, y4) = (1.0 * t + 1.0).sin_cos();
        let x4 = x0 + x4 * 200.0;
        let y4 = y0 + y4 * 200.0;
        let circle4 = Circle::new(x4, y4, 10.0).fill(true).stroke(false);
        let circle4 = Styled::new(circle4).fill("#ccbb99");
        circle4.draw(&my_canvas);

        // Circle 5
        let (x5, y5) = (8.0 * t + 1.0).sin_cos();
        let x5 = x4 + x5 * 17.0;
        let y5 = y4 + y5 * 17.0;
        let circle5 = Circle::new(x5, y5, 1.0).fill(true).stroke(false);
        let circle5 = Styled::new(circle5).fill("#bbbbbb");
        circle5.draw(&my_canvas);

        // Circle 6
        let (x6, y6) = (6.0 * t + 1.0).sin_cos();
        let x6 = x4 + x6 * 25.0;
        let y6 = y4 + y6 * 25.0;
        let circle6 = Circle::new(x6, y6, 1.0).fill(true).stroke(false);
        let circle6 = Styled::new(circle6).fill("#bbbbbb");
        circle6.draw(&my_canvas);

        // Circle 7
        let (x7, y7) = (3.0 * t + 1.0).sin_cos();
        let x7 = x4 + x7 * 35.0;
        let y7 = y4 + y7 * 35.0;
        let circle7 = Circle::new(x7, y7, 1.0).fill(true).stroke(false);
        let circle7 = Styled::new(circle7).fill("#bbbbbb");
        circle7.draw(&my_canvas);
    })
    .expect("Failed to create loop");

    // Leak
    Box::leak::<'static>(Box::new(rl));
}
