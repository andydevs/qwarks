//! Defines the [`Draw`] trait implemented by every drawable primitive.

use crate::Canvas;

/// A shape or primitive that can render itself onto a [`Canvas`].
pub trait Draw {
    /// Renders this value onto `canvas`.
    ///
    /// # Parameters
    /// - `canvas`: the [`Canvas`] to draw onto.
    ///
    /// # Side Effects
    /// Issues drawing calls against `canvas`'s 2D rendering context.
    fn draw(&self, canvas: &Canvas);
}
