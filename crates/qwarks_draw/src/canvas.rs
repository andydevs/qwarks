//! Wraps an HTML `<canvas>` element and its 2D rendering context.

use crate::Draw;
use wasm_bindgen::{JsCast as _, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, window};

/// An HTML canvas element paired with the 2D rendering context used to draw on it.
pub struct Canvas {
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
}

impl Canvas {
    /// Locates a `<canvas>` element in the current document and binds a 2D rendering
    /// context to it.
    ///
    /// # Parameters
    /// - `selector`: a CSS selector identifying the target `<canvas>` element.
    ///
    /// # Returns
    /// A [`Canvas`] wrapping the located element and its 2D rendering context.
    ///
    /// # Errors
    /// Returns a `JsValue` error if there is no global `window`/`document`, if no element
    /// matches `selector`, if the matched element is not an `HtmlCanvasElement`, or if a
    /// `"2d"` rendering context is unavailable.
    pub fn from_selector(selector: &str) -> Result<Self, JsValue> {
        let canvas = window()
            .ok_or(JsValue::from("Unable to load window"))?
            .document()
            .ok_or(JsValue::from("Unable to load document"))?
            .query_selector(selector)?
            .ok_or(JsValue::from(format!("Cannot find element by selector \"{selector}\"")))?
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| JsValue::from(format!("Selector \"{selector}\" can not be interpreted as canvas")))?;
        let ctx = canvas
            .get_context("2d")?
            .ok_or(JsValue::from("2d rendering context unavailable"))?
            .dyn_into::<CanvasRenderingContext2d>()
            .map_err(|_| JsValue::from("Unable to convert to CanvasRenderingContext2d"))?;
        Ok(Self { canvas, ctx })
    }

    /// Returns the canvas's pixel dimensions.
    ///
    /// # Returns
    /// A `(width, height)` tuple in pixels.
    pub fn dimensions(&self) -> (u32, u32) {
        (self.canvas.width(), self.canvas.height())
    }

    /// Clears the entire canvas.
    ///
    /// # Side Effects
    /// Erases all pixels previously drawn to the canvas.
    pub fn clear(&mut self) {
        let (width, height) = self.dimensions();
        self.ctx.clear_rect(0.0, 0.0, width as f64, height as f64);
    }

    /// Returns the underlying 2D rendering context.
    ///
    /// # Returns
    /// A reference to the canvas's `CanvasRenderingContext2d`.
    pub(crate) fn ctx(&self) -> &CanvasRenderingContext2d {
        &self.ctx
    }

    /// Draws a [`Draw`] object onto this canvas.
    ///
    /// # Parameters
    /// - `obj`: the drawable object to render.
    ///
    /// # Side Effects
    /// Invokes `obj`'s [`Draw::draw`] implementation, which issues drawing calls against
    /// this canvas's 2D rendering context.
    pub fn draw(&self, obj: &dyn Draw) {
        obj.draw(self);
    }
}
