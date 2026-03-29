use wasm_bindgen::{JsCast, JsValue, prelude::ScopedClosure};
use web_sys::window;

/// Abstraction over the browser's animation-frame scheduling API.
///
/// The primary implementation is [`BrowserAdapter`], which delegates to the
/// real `window.requestAnimationFrame` / `window.cancelAnimationFrame` Web
/// APIs. Alternative implementations can be supplied (e.g. in tests) via
/// [`RAFLoop::with_adapter`](crate::rafloop::RAFLoop).
pub trait Adapter {
    /// Schedules `callback` to be invoked before the next browser repaint.
    ///
    /// Returns the request ID that can later be passed to
    /// [`cancel_animation_frame`](Self::cancel_animation_frame).
    fn request_animation_frame(
        &self,
        callback: &ScopedClosure<'_, dyn FnMut(f64)>,
    ) -> Result<i32, JsValue>;

    /// Cancels a previously scheduled animation frame identified by `id`.
    ///
    /// If `id` does not correspond to a pending request this is a no-op.
    fn cancel_animation_frame(&self, id: i32) -> Result<(), JsValue>;
}

/// Production [`Adapter`] that calls the real browser APIs.
///
/// Resolves `window` on every call, so it is safe to construct once and reuse
/// across frames even if the global object changes between calls.
pub struct BrowserAdapter;

impl Adapter for BrowserAdapter {
    /// Calls `window.cancelAnimationFrame(id)`.
    ///
    /// Returns `Err` if the global `window` object is not available.
    fn cancel_animation_frame(&self, id: i32) -> Result<(), JsValue> {
        window()
            .ok_or(JsValue::from("Window was not defined"))?
            .cancel_animation_frame(id)
    }

    /// Calls `window.requestAnimationFrame(callback)`.
    ///
    /// Returns `Err` if the global `window` object is not available.
    fn request_animation_frame(
        &self,
        callback: &ScopedClosure<'_, dyn FnMut(f64)>,
    ) -> Result<i32, JsValue> {
        window()
            .ok_or(JsValue::from("Window was not defined"))?
            .request_animation_frame(callback.as_ref().unchecked_ref())
    }
}
