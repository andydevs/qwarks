//! WebAssembly `requestAnimationFrame` loop handler.
//!
//! This crate provides a simple, RAII-based wrapper around the browser's
//! `requestAnimationFrame` API for use in WebAssembly projects.
//!
//! # Usage
//!
//! Create a [`Loop`] with a callback that receives a [`Ctx`] on every frame.
//! The loop runs until the [`Loop`] handle is dropped.
//!
//! ```ignore
//! let handle = Loop::new(|ctx: Ctx| {
//!     // runs every animation frame
//! })?;
//! // drop `handle` to stop the loop
//! ```

mod context;
mod rafloop;
mod state;

pub use context::Ctx;
pub use rafloop::Loop;
