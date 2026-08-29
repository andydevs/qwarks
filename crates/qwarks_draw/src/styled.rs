//! Stroke styling for [`Shape`]s, applied around a [`Draw`] call.

use crate::{Canvas, Draw};

/// Stroke appearance applied when drawing a [`Styled`] shape.
pub struct Style {
    stroke: String,
    line_width: f64,
}

impl Default for Style {
    /// Returns the default style: a solid black stroke, 1 pixel wide.
    fn default() -> Self {
        Self {
            stroke: String::from("black"),
            line_width: 1.0,
        }
    }
}

/// A [`Draw`]able primitive that can be wrapped in a [`Styled`] to customize its stroke
/// appearance.
pub trait Shape: Draw + Sized {
    /// Wraps this shape in a [`Styled`] with the default [`Style`].
    ///
    /// # Returns
    /// A [`Styled`] wrapping `self`, ready for further style customization.
    fn into_styled(self) -> Styled<Self> {
        Styled::new(self)
    }
}

/// A [`Shape`] paired with the [`Style`] it should be drawn with.
pub struct Styled<S: Shape> {
    shape: S,
    style: Style,
}

impl<D: Shape> Styled<D> {
    /// Wraps `shape` with the default [`Style`].
    ///
    /// # Parameters
    /// - `shape`: the shape to style.
    ///
    /// # Returns
    /// A [`Styled`] wrapping `shape` with the default style.
    fn new(shape: D) -> Self {
        Self {
            shape,
            style: Style::default(),
        }
    }

    /// Sets the stroke color.
    ///
    /// # Parameters
    /// - `stroke_style`: any color string accepted by `CanvasRenderingContext2d`'s stroke
    ///   style (e.g. a CSS color name or hex code).
    ///
    /// # Returns
    /// `Self`, for method chaining.
    pub fn stroke(mut self, stroke_style: &str) -> Self {
        self.style.stroke = String::from(stroke_style);
        self
    }

    /// Sets the stroke line width.
    ///
    /// # Parameters
    /// - `stroke_width`: the line width in pixels.
    ///
    /// # Returns
    /// `Self`, for method chaining.
    pub fn line_width(mut self, stroke_width: f64) -> Self {
        self.style.line_width = stroke_width;
        self
    }
}

impl<S: Shape> Draw for Styled<S> {
    /// Applies this wrapper's [`Style`] to the canvas context, draws the wrapped shape,
    /// then restores the previous context state.
    ///
    /// # Side Effects
    /// Calls `save`/`restore` on `canvas`'s 2D rendering context and temporarily overrides
    /// its stroke style and line width while the wrapped shape is drawn.
    fn draw(&self, canvas: &Canvas) {
        canvas.ctx().save();
        canvas.ctx().set_stroke_style_str(&self.style.stroke);
        canvas.ctx().set_line_width(self.style.line_width);
        canvas.draw(&self.shape);
        canvas.ctx().restore();
    }
}
