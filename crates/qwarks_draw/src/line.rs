//! A single straight line segment primitive.

use crate::macros::builder_fns;
use crate::{Canvas, Draw};

/// A straight line segment between two points, with independent fill/stroke toggles.
pub struct Line {
    start: (f64, f64),
    end: (f64, f64),
    fill: bool,
    stroke: bool,
}

impl Line {
    /// Creates a line segment from `start` to `end`.
    ///
    /// # Parameters
    /// - `start`: the `(x, y)` coordinates of the segment's first endpoint.
    /// - `end`: the `(x, y)` coordinates of the segment's second endpoint.
    ///
    /// # Returns
    /// A [`Line`] with stroke enabled and fill disabled by default.
    pub fn new(start: (f64, f64), end: (f64, f64)) -> Self {
        Self { start, end, fill: false, stroke: true }
    }
}

builder_fns! {
    impl Line {
        /// if `true`, the path is filled using the canvas context's current fill style.
        pub fill: bool,

        /// if `true`, the segment is stroked using the canvas context's current stroke
        /// style and line width.
        pub stroke: bool,
    }
}

impl Draw for Line {
    /// Traces a path from this line's start point to its end point and fills and/or
    /// strokes it according to this line's [`fill`](Line::fill) and [`stroke`](Line::stroke)
    /// flags.
    ///
    /// # Side Effects
    /// Issues `beginPath`/`moveTo`/`lineTo` calls, followed by `fill` and/or `stroke`, on
    /// `canvas`'s 2D rendering context. Does nothing if both flags are `false`.
    fn draw(&self, canvas: &Canvas) {
        if !self.fill && !self.stroke {
            return;
        }
        canvas.ctx().begin_path();
        canvas.ctx().move_to(self.start.0, self.start.1);
        canvas.ctx().line_to(self.end.0, self.end.1);
        if self.fill {
            canvas.ctx().fill();
        }
        if self.stroke {
            canvas.ctx().stroke();
        }
    }
}
