//! Stroke styling for [`Draw`] shapes, applied around a [`Draw`] call.

use crate::macros::builder_fns;
use crate::{Canvas, Draw};

/// Stroke and fill appearance applied when drawing a [`Styled`] shape.
pub struct Style {
    stroke: String,
    line_width: f64,
    fill: String,
}

impl Default for Style {
    /// Returns the default style: a solid black stroke, 1 pixel wide, with a solid black fill.
    fn default() -> Self {
        Self {
            stroke: String::from("black"),
            line_width: 1.0,
            fill: String::from("black"),
        }
    }
}

/// A [`Draw`] shape paired with the [`Style`] it should be drawn with.
pub struct Styled<D: Draw> {
    shape: D,
    style: Style,
}

impl<D: Draw> Styled<D> {
    /// Wraps `shape` with the default [`Style`].
    ///
    /// # Parameters
    /// - `shape`: the shape to style.
    ///
    /// # Returns
    /// A [`Styled`] wrapping `shape` with the default style.
    pub fn new(shape: D) -> Self {
        Self {
            shape,
            style: Style::default(),
        }
    }
}

builder_fns! {
    impl<D: Draw> Styled<D> {
        /// any color string accepted by `CanvasRenderingContext2d`'s stroke style (e.g. a
        /// CSS color name or hex code).
        pub stroke: &str => style.stroke,

        /// the line width in pixels.
        pub line_width: f64 => style.line_width,

        /// any color string accepted by `CanvasRenderingContext2d`'s fill style (e.g. a
        /// CSS color name or hex code).
        pub fill: &str => style.fill,
    }
}

impl<D: Draw> Draw for Styled<D> {
    /// Applies this wrapper's [`Style`] to the canvas context, draws the wrapped shape,
    /// then restores the previous context state.
    ///
    /// # Side Effects
    /// Calls `save`/`restore` on `canvas`'s 2D rendering context and temporarily overrides
    /// its stroke style, line width, and fill style while the wrapped shape is drawn.
    fn draw(&self, canvas: &Canvas) {
        canvas.ctx().save();
        canvas.ctx().set_stroke_style_str(&self.style.stroke);
        canvas.ctx().set_line_width(self.style.line_width);
        canvas.ctx().set_fill_style_str(&self.style.fill);
        canvas.draw(&self.shape);
        canvas.ctx().restore();
    }
}
