//! A multi-point connected line segment primitive.

use crate::{Canvas, Draw, Shape};

/// A connected sequence of line segments defined by an ordered list of points.
pub struct Polyline {
    points: Vec<(f64, f64)>,
}

impl Polyline {
    /// Creates an empty polyline with no points.
    ///
    /// # Returns
    /// A [`Polyline`] with an empty point list.
    pub fn new() -> Self {
        Self { points: Vec::default() }
    }

    /// Creates a polyline from an ordered slice of points.
    ///
    /// # Parameters
    /// - `points`: the `(x, y)` coordinates to connect, in drawing order.
    ///
    /// # Returns
    /// A [`Polyline`] containing a copy of `points`.
    pub fn from_points(points: &[(f64, f64)]) -> Self {
        Self {
            points: points.to_vec(),
        }
    }
}

impl Draw for Polyline {
    /// Strokes a single connected path through this polyline's points, using the canvas
    /// context's current stroke style and line width.
    ///
    /// # Side Effects
    /// Issues `beginPath`/`moveTo`/`lineTo`/`stroke` calls on `canvas`'s 2D rendering
    /// context. Does nothing if the polyline has no points.
    fn draw(&self, canvas: &Canvas) {
        let Some((x0, y0)) = self.points.get(0) else {
            return;
        };
        canvas.ctx().begin_path();
        canvas.ctx().move_to(*x0, *y0);
        if let Some(rest) = self.points.get(1..) {
            for (x, y) in rest {
                canvas.ctx().line_to(*x, *y);
            }
            canvas.ctx().stroke();
        }
    }
}

/// Marks [`Polyline`] as a [`Shape`] that can be wrapped in [`Styled`](crate::Styled).
impl Shape for Polyline {}
