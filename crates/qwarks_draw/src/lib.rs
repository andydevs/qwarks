//! Jarsdraw — a Rust/WebAssembly canvas drawing library for the browser.
//!
//! Compiled to WASM via `wasm-bindgen`, this crate exposes drawing primitives
//! that operate on the HTML5 Canvas API through `web-sys`.

mod canvas;
mod draw;
mod polyline;
mod styled;

pub use canvas::Canvas;
pub use draw::Draw;
pub use polyline::Polyline;
pub use styled::{Shape, Style, Styled};
