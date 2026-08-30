//! A multi-point connected line segment primitive.

use crate::macros::builder_fns;
use crate::{Canvas, Draw};

/// A connected sequence of line segments defined by an ordered list of points.
///
/// Borrows its points rather than owning them, since polylines are typically
/// constructed immediately before being drawn and discarded afterward.
pub struct Polyline<'a> {
    points: &'a [(f64, f64)],
    closed: bool,
}

impl<'a> Polyline<'a> {
    /// Creates a polyline from an ordered slice of points.
    ///
    /// # Parameters
    /// - `points`: the `(x, y)` coordinates to connect, in drawing order.
    ///
    /// # Returns
    /// A [`Polyline`] borrowing `points`, open (not closed) by default.
    pub fn new(points: &'a [(f64, f64)]) -> Self {
        Self { points, closed: false }
    }
}

builder_fns! {
    impl<'a> Polyline<'a> {
        /// if `true`, the stroked path connects the last point back to the first.
        pub closed: bool,
    }
}

impl<'a> Draw for Polyline<'a> {
    /// Strokes a single connected path through this polyline's points, using the canvas
    /// context's current stroke style and line width. If [`closed`](Polyline::closed) was
    /// set, the path is closed back to its first point before stroking.
    ///
    /// # Side Effects
    /// Issues `beginPath`/`moveTo`/`lineTo`/`closePath`/`stroke` calls on `canvas`'s 2D
    /// rendering context. Does nothing if the polyline has no points.
    fn draw(&self, canvas: &Canvas) {
        let Some((x0, y0)) = self.points.get(0) else {
            return;
        };
        let Some(rest) = self.points.get(1..) else {
            return;
        };
        canvas.ctx().begin_path();
        canvas.ctx().move_to(*x0, *y0);
        for (x, y) in rest {
            canvas.ctx().line_to(*x, *y);
        }
        if self.closed {
            canvas.ctx().close_path();
        }
        canvas.ctx().stroke();
    }
}
