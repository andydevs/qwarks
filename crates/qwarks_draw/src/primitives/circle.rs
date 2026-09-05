//! A filled/stroked circle primitive.

use crate::macros::builder_fns;
use crate::{Canvas, Draw};
use std::f64::consts::TAU;

/// A circle defined by a center point and radius, with independent fill/stroke toggles.
pub struct Circle {
    cx: f64,
    cy: f64,
    radius: f64,
    fill: bool,
    stroke: bool,
}

impl Circle {
    /// Creates a circle centered at `(cx, cy)` with the given `radius`.
    ///
    /// # Parameters
    /// - `cx`: the center point's x-coordinate.
    /// - `cy`: the center point's y-coordinate.
    /// - `radius`: the circle's radius, in pixels.
    ///
    /// # Returns
    /// A [`Circle`] with stroke enabled and fill disabled by default.
    pub fn new(cx: f64, cy: f64, radius: f64) -> Self {
        Self { cx, cy, radius, fill: false, stroke: true }
    }
}

builder_fns! {
    impl Circle {
        /// if `true`, the circle's interior is filled using the canvas context's current
        /// fill style.
        pub fill: bool,

        /// if `true`, the circle's outline is stroked using the canvas context's current
        /// stroke style and line width.
        pub stroke: bool,
    }
}

impl Draw for Circle {
    /// Traces a full circular path and fills and/or strokes it according to this circle's
    /// [`fill`](Circle::fill) and [`stroke`](Circle::stroke) flags, using the canvas
    /// context's current fill style, stroke style, and line width.
    ///
    /// # Side Effects
    /// Issues `beginPath`/`arc` calls, followed by `fill` and/or `stroke`, on `canvas`'s 2D
    /// rendering context. Does nothing if both flags are `false`.
    fn draw(&self, canvas: &Canvas) {
        if !self.fill && !self.stroke {
            return;
        }
        canvas.ctx().begin_path();
        let _ = canvas.ctx().arc(self.cx, self.cy, self.radius, 0.0, TAU);
        if self.fill {
            canvas.ctx().fill();
        }
        if self.stroke {
            canvas.ctx().stroke();
        }
    }
}
