//! Animation-frame scheduling adapters.
//!
//! Defines the [`FrameAdapter`] trait, which abstracts over the browser's
//! `requestAnimationFrame` / `cancelAnimationFrame` APIs, and
//! [`BrowserAdapter`], the production implementation that calls them on the
//! global `window` object.

use wasm_bindgen::closure::ScopedClosure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Window, window};

/// Closure type passed to `requestAnimationFrame`.
///
/// Receives the frame's `DOMHighResTimeStamp` in milliseconds.
type JsCallback = ScopedClosure<'static, dyn FnMut(f64)>;

/// Request ID returned by `requestAnimationFrame` and accepted by
/// `cancelAnimationFrame`.
pub(crate) type FrameReqId = i32;

/// Abstraction over the browser's animation-frame scheduling API.
///
/// The primary implementation is [`BrowserAdapter`], which delegates to the
/// real `window.requestAnimationFrame` / `window.cancelAnimationFrame` Web
/// APIs. Alternative implementations can be supplied (e.g. in tests) via
/// [`RAFLoop::with_adapter`](crate::RAFLoop::with_adapter).
pub(crate) trait FrameAdapter {
    /// Schedules `callback` to be invoked before the next browser repaint.
    ///
    /// # Parameters
    /// - `callback`: Closure to invoke with the frame timestamp.
    ///
    /// # Returns
    /// The request ID that can later be passed to
    /// [`cancel_animation_frame`](Self::cancel_animation_frame), or `Err` if
    /// the frame could not be scheduled.
    ///
    /// # Side Effects
    /// Registers a pending animation frame request with the scheduler.
    fn request_animation_frame(&self, callback: &JsCallback) -> Result<i32, JsValue>;

    /// Cancels a previously scheduled animation frame.
    ///
    /// If `id` does not correspond to a pending request this is a no-op.
    ///
    /// # Parameters
    /// - `id`: Request ID returned by
    ///   [`request_animation_frame`](Self::request_animation_frame).
    ///
    /// # Returns
    /// `Ok(())` on success, or `Err` if the cancellation could not be made.
    ///
    /// # Side Effects
    /// Removes the pending animation frame request from the scheduler.
    fn cancel_animation_frame(&self, id: i32) -> Result<(), JsValue>;
}

/// Production [`FrameAdapter`] that calls the real browser APIs.
///
/// Resolves `window` on every call, so it is safe to construct once and reuse
/// across frames even if the global object changes between calls.
pub(crate) struct BrowserAdapter;

impl BrowserAdapter {
    /// Looks up the global `window` object.
    ///
    /// # Returns
    /// The current [`Window`], or `Err` if no global `window` is defined
    /// (e.g. in a worker or non-browser environment).
    fn get_js_window_obj(&self) -> Result<Window, JsValue> {
        window().ok_or(JsValue::from("Window was not defined"))
    }
}

impl FrameAdapter for BrowserAdapter {
    /// Calls `window.cancelAnimationFrame(id)`.
    ///
    /// # Parameters
    /// - `id`: Request ID returned by a previous `requestAnimationFrame` call.
    ///
    /// # Returns
    /// `Ok(())` on success, or `Err` if the global `window` object is not
    /// available or the browser call throws.
    ///
    /// # Side Effects
    /// Cancels the pending browser animation frame request.
    fn cancel_animation_frame(&self, id: FrameReqId) -> Result<(), JsValue> {
        let window = self.get_js_window_obj()?;
        window.cancel_animation_frame(id)
    }

    /// Calls `window.requestAnimationFrame(callback)`.
    ///
    /// # Parameters
    /// - `handle_ref`: Closure to invoke before the next repaint.
    ///
    /// # Returns
    /// The browser's request ID, or `Err` if the global `window` object is
    /// not available or the browser call throws.
    ///
    /// # Side Effects
    /// Registers a pending browser animation frame request.
    fn request_animation_frame(&self, handle_ref: &JsCallback) -> Result<FrameReqId, JsValue> {
        let window = self.get_js_window_obj()?;
        let handle_unchecked = handle_ref.as_ref().unchecked_ref();
        window.request_animation_frame(handle_unchecked)
    }
}
