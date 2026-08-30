//! Jarsdraw — a Rust/WebAssembly canvas drawing library for the browser.
//!
//! Compiled to WASM via `wasm-bindgen`, this crate exposes drawing primitives
//! that operate on the HTML5 Canvas API through `web-sys`.
//!
//! - [`Canvas`] wraps a `<canvas>` element and its 2D rendering context.
//! - [`Draw`] is implemented by anything that can render itself onto a [`Canvas`].
//! - [`Polyline`] is a connected-line-segment [`Draw`] primitive.
//! - [`Styled`] add chainable stroke styling on top of a [`Draw`] primitive.

mod canvas;
mod draw;
mod polyline;
mod styled;

pub use canvas::Canvas;
pub use draw::Draw;
pub use polyline::Polyline;
pub use styled::{Style, Styled};
