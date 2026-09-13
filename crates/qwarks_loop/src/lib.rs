//! High-level `requestAnimationFrame` loop management for WebAssembly.
//!
//! This crate provides [`RAFLoop`], a safe wrapper around the browser's
//! `requestAnimationFrame` API. Each frame your callback receives a
//! [`FrameCtx`] containing the current frame count, the
//! `DOMHighResTimeStamp` provided by the browser, and the elapsed time
//! since the previous frame.
//!
//! The loop is cancelled automatically when the [`RAFLoop`] value is dropped.
//!
//! # Example
//!
//! ```no_run
//! use qwarks_loop::{RAFLoop, FrameCtx};
//!
//! let _loop = RAFLoop::new(|ctx: FrameCtx| {
//!     // Called once per animation frame.
//!     web_sys::console::log_1(&format!(
//!         "frame {} — delta {:.2} ms",
//!         ctx.frame_count, ctx.delta
//!     ).into());
//! }).expect("Failed to start animation loop");
//! ```

mod browser;
mod context;
mod rafloop;
mod telemetry;

pub use context::FrameCtx;
pub use rafloop::RAFLoop;
