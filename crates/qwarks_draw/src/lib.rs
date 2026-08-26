//! Jarsdraw — a Rust/WebAssembly canvas drawing library for the browser.
//!
//! Compiled to WASM via `wasm-bindgen`, this crate exposes drawing primitives
//! that operate on the HTML5 Canvas API through `web-sys`.

use wasm_bindgen::{JsCast as _, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, window};

pub struct Canvas {
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
}

impl Canvas {
    pub fn from_selector(selector: &str) -> Result<Self, JsValue> {
        let canvas = window()
            .ok_or(JsValue::from("Unable to load window"))?
            .document()
            .ok_or(JsValue::from("Unable to load document"))?
            .query_selector(selector)?
            .ok_or(JsValue::from(format!(
                "Could not find html element by selector \"{selector}\""
            )))?
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| {
                JsValue::from(format!(
                    "Element with selector \"{selector}\" can not be interpreted as canvas"
                ))
            })?;
        let ctx = canvas
            .get_context("2d")?
            .ok_or(JsValue::from("2d rendering context unavailable"))?
            .dyn_into::<CanvasRenderingContext2d>()
            .map_err(|_| JsValue::from("Unable to convert to CanvasRenderingContext2d"))?;
        Ok(Self { canvas, ctx })
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.canvas.width(), self.canvas.height())
    }

    pub fn clear(&mut self) {
        web_sys::console::log_1(&"Clearing canvas".into());
        let (width, height) = self.dimensions();
        self.ctx.clear_rect(0.0, 0.0, width as f64, height as f64);
    }

    /// TEMPORARILY PUBLIC until I implement the draw trait
    pub fn ctx(&self) -> &CanvasRenderingContext2d {
        &self.ctx
    }
}
