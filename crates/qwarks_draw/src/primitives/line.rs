//! A single straight line segment primitive.

use crate::{Canvas, Draw};

/// A straight line segment between two points.
pub struct Line {
    start: (f64, f64),
    end: (f64, f64),
}

impl Line {
    /// Creates a line segment from `start` to `end`.
    ///
    /// # Parameters
    /// - `start`: the `(x, y)` coordinates of the segment's first endpoint.
    /// - `end`: the `(x, y)` coordinates of the segment's second endpoint.
    ///
    /// # Returns
    /// A [`Line`] spanning the two points.
    pub fn new(start: (f64, f64), end: (f64, f64)) -> Self {
        Self { start, end }
    }
}

impl Draw for Line {
    /// Traces a path from this line's start point to its end point and strokes it.
    ///
    /// # Side Effects
    /// Issues `beginPath`/`moveTo`/`lineTo`/`stroke` calls on `canvas`'s 2D rendering
    /// context, using its current stroke style and line width.
    fn draw(&self, canvas: &Canvas) {
        canvas.ctx().begin_path();
        canvas.ctx().move_to(self.start.0, self.start.1);
        canvas.ctx().line_to(self.end.0, self.end.1);
        canvas.ctx().stroke();
    }
}
