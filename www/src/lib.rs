//! WebAssembly entry point for the qwarks demo site.

mod console;
use qwarks_draw::{Canvas, Circle, Draw, Styled};
use qwarks_loop::{FrameCtx, RAFLoop};
use wasm_bindgen::prelude::*;

struct Orbit {
    velocity: f64,
    radius: f64,
}

impl Orbit {
    fn position(&self, t: f64) -> (f64, f64) {
        let (u, v) = (self.velocity * t).sin_cos();
        let x = u * self.radius;
        let y = v * self.radius;
        (x, y)
    }
}

#[derive(Clone, Copy)]
struct BodyShape {
    radius: f64,
    color: &'static str,
}

impl BodyShape {
    fn into_body(self, x: f64, y: f64) -> Body {
        Body { x, y, shape: self }
    }
}

struct Body {
    x: f64,
    y: f64,
    shape: BodyShape,
}

impl Body {
    fn orbiting(&self, t: f64, orbit: Orbit, shape: BodyShape) -> Self {
        let (u, v) = orbit.position(t);
        Self { x: self.x + u, y: self.y + v, shape }
    }
}

impl Draw for Body {
    fn draw(&self, canvas: &Canvas) {
        let circle = Circle::new(self.x, self.y, self.shape.radius).fill(true).stroke(true);
        let styled = Styled::new(circle).fill(self.shape.color);
        styled.draw(canvas);
    }
}

/// Runs when the module is initialized.
#[wasm_bindgen(start)]
fn start() {
    console_error_panic_hook::set_once();
    log!("Anim loaded");

    // Create loop
    let my_canvas = Canvas::from_selector("#solar-system").expect("Can't make canvas");

    let rl = RAFLoop::new(move |ctx: FrameCtx| {
        let t = ctx.timestamp / 1000.0;
        let (width, height) = my_canvas.dimensions();
        let x0 = f64::from(width) / 2.0;
        let y0 = f64::from(height) / 2.0;
        my_canvas.clear();

        // Solar System
        let sun = BodyShape { radius: 20.0, color: "#fbb800" }.into_body(x0, y0);
        let red_planet = sun.orbiting(
            t,
            Orbit { radius: 60.0, velocity: 4.0 },
            BodyShape { radius: 4.0, color: "#ff3c00" },
        );
        let blue_planet = sun.orbiting(
            t,
            Orbit { radius: 120.0, velocity: 3.0 },
            BodyShape { radius: 8.0, color: "#00aaff" },
        );
        let inner_moon = blue_planet.orbiting(
            t,
            Orbit { radius: 20.0, velocity: 8.0 },
            BodyShape { radius: 2.0, color: "#bbbbbb" },
        );
        let gas_giant = sun.orbiting(
            t,
            Orbit { radius: 200.0, velocity: 1.0 },
            BodyShape { radius: 10.0, color: "#ccbb99" },
        );
        let gg_moon_1 = gas_giant.orbiting(
            t,
            Orbit { radius: 17.0, velocity: 8.0 },
            BodyShape { radius: 2.0, color: "#c4ac9f" },
        );
        let gg_moon_2 = gas_giant.orbiting(
            t,
            Orbit { radius: 25.0, velocity: 6.0 },
            BodyShape { radius: 2.0, color: "#8a817c" },
        );
        let gg_moon_3 = gas_giant.orbiting(
            t,
            Orbit { radius: 35.0, velocity: 3.0 },
            BodyShape { radius: 2.0, color: "#bbbbbb" },
        );
        let ice_giant = sun.orbiting(
            t,
            Orbit { radius: 300.0, velocity: 0.5 },
            BodyShape { radius: 8.0, color: "#4400aa" },
        );
        let ig_moon_1 = ice_giant.orbiting(
            t,
            Orbit { radius: 15.0, velocity: 1.5 },
            BodyShape { radius: 2.0, color: "#aabbcc" },
        );

        // Draw system
        sun.draw(&my_canvas);
        red_planet.draw(&my_canvas);
        blue_planet.draw(&my_canvas);
        inner_moon.draw(&my_canvas);
        gas_giant.draw(&my_canvas);
        gg_moon_1.draw(&my_canvas);
        gg_moon_2.draw(&my_canvas);
        gg_moon_3.draw(&my_canvas);
        ice_giant.draw(&my_canvas);
        ig_moon_1.draw(&my_canvas);
    })
    .expect("Failed to create loop");

    // Leak
    Box::leak::<'static>(Box::new(rl));
}
