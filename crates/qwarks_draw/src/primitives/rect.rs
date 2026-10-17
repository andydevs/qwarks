//! A filled/stroked axis-aligned rectangle primitive.

use crate::macros::builder_fns;
use crate::{Canvas, Draw};

/// A rectangle defined by its top-left corner and dimensions, with independent
/// fill/stroke toggles.
pub struct Rect {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    fill: bool,
    stroke: bool,
}

impl Rect {
    /// Creates a rectangle with its top-left corner at `(x, y)` and the given dimensions.
    ///
    /// # Parameters
    /// - `x`: the top-left corner's x-coordinate.
    /// - `y`: the top-left corner's y-coordinate.
    /// - `width`: the rectangle's width, in pixels.
    /// - `height`: the rectangle's height, in pixels.
    ///
    /// # Returns
    /// A [`Rect`] with stroke enabled and fill disabled by default.
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self { x, y, width, height, fill: false, stroke: true }
    }
}

builder_fns! {
    impl Rect {
        /// if `true`, the rectangle's interior is filled using the canvas context's
        /// current fill style.
        pub fill: bool,

        /// if `true`, the rectangle's outline is stroked using the canvas context's
        /// current stroke style and line width.
        pub stroke: bool,
    }
}

impl Draw for Rect {
    /// Traces the rectangle's path and fills and/or strokes it according to this rect's
    /// [`fill`](Rect::fill) and [`stroke`](Rect::stroke) flags, using the canvas context's
    /// current fill style, stroke style, and line width.
    ///
    /// # Side Effects
    /// Issues `beginPath`/`rect` calls, followed by `fill` and/or `stroke`, on `canvas`'s
    /// 2D rendering context. Does nothing if both flags are `false`.
    fn draw(&self, canvas: &Canvas) {
        if !self.fill && !self.stroke {
            return;
        }
        canvas.ctx().begin_path();
        canvas.ctx().rect(self.x, self.y, self.width, self.height);
        if self.fill {
            canvas.ctx().fill();
        }
        if self.stroke {
            canvas.ctx().stroke();
        }
    }
}
