//! The [`RAFLoop`] type: a self-rescheduling `requestAnimationFrame` loop.
//!
//! Each frame, the loop builds a [`FrameCtx`] from its [`Telemetry`], invokes
//! the user callback, requests the next frame through a [`FrameAdapter`], and
//! records the new request ID. Dropping the loop cancels the pending request.

use crate::adapter::{BrowserAdapter, FrameAdapter};
use crate::telemetry::{FrameCtx, Telemetry};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use wasm_bindgen::closure::ScopedClosure;

/// Internal frame callback handle type
type JsCallbackHandle = Option<ScopedClosure<'static, dyn FnMut(f64)>>;

/// An active `requestAnimationFrame` loop.
///
/// While this value is alive the browser will invoke your callback once per
/// animation frame.  When the value is dropped the pending frame request is
/// cancelled via `cancelAnimationFrame`, stopping the loop.
///
/// # Example
///
/// ```no_run
/// use qwarks_loop::{RAFLoop, FrameCtx};
///
/// let _loop = RAFLoop::new(|ctx: FrameCtx| {
///     // update your scene here
/// }).expect("Failed to start RAF loop");
/// // Loop runs until `_loop` is dropped.
/// ```
pub struct RAFLoop {
    /// The recurring closure passed to `requestAnimationFrame`.
    /// Kept alive for the lifetime of the loop
    #[expect(unused, reason = "Exists to prevent closure from being dropped")]
    handle: Rc<RefCell<JsCallbackHandle>>,
    /// Shared loop state (frame count, last timestamp, pending request ID).
    telemetry: Rc<RefCell<Telemetry>>,
    /// Adapter used to call `requestAnimationFrame` / `cancelAnimationFrame`.
    adapter: Rc<dyn FrameAdapter>,
}

impl RAFLoop {
    /// Creates and starts a new animation loop using the real browser APIs.
    ///
    /// # Parameters
    /// - `callback`: Called once per animation frame with a [`FrameCtx`]
    ///   containing timing information for that frame.
    ///
    /// # Returns
    /// The running loop, or `Err` if the initial `requestAnimationFrame` call
    /// fails (e.g. when `window` is not available).
    ///
    /// # Side Effects
    /// Installs `console_error_panic_hook` (once) and schedules the first
    /// animation frame.
    ///
    /// # Panics
    /// The frame closure panics if requesting a subsequent frame fails.
    pub fn new(callback: impl FnMut(FrameCtx) + 'static) -> Result<Self, JsValue> {
        let adapter = Rc::new(BrowserAdapter);
        Self::with_adapter(callback, adapter)
    }

    /// Creates and starts a new animation loop driven by a custom
    /// [`FrameAdapter`].
    ///
    /// This is the internal constructor used by [`new`](Self::new) and exposed
    /// within the crate for testing with mock adapters.
    ///
    /// # Parameters
    /// - `callback`: Called once per animation frame with a [`FrameCtx`].
    /// - `adapter`: Scheduler used to request and cancel frames.
    ///
    /// # Returns
    /// The running loop, or `Err` if the initial frame request fails.
    ///
    /// # Side Effects
    /// Installs `console_error_panic_hook` (once) and schedules the first
    /// frame before returning.
    ///
    /// # Panics
    /// The frame closure panics if requesting a subsequent frame fails.
    pub(crate) fn with_adapter(
        callback: impl FnMut(FrameCtx) + 'static,
        adapter: Rc<dyn FrameAdapter>,
    ) -> Result<Self, JsValue> {
        // Enable panic hook in case anything fails here
        console_error_panic_hook::set_once();

        // Setup callback and telemetry. Generate frame callback
        let handle: Rc<RefCell<JsCallbackHandle>> = Rc::new(RefCell::new(None));
        let telemetry = Telemetry::create_new_handle();
        let new_closure = Self::js_closure(&adapter, &telemetry, &handle, callback);
        *handle.borrow_mut() = new_closure;

        // Call first frame. Initialize telemetry. Create RAF Loop
        let id = Self::request_next_frame_with_adapter(&adapter, &handle)?;
        telemetry.borrow_mut().initialize_state(id);
        Ok(Self { handle, telemetry, adapter })
    }

    /// Builds the recurring frame closure.
    ///
    /// On each invocation the closure builds a [`FrameCtx`], runs
    /// `user_callback`, requests the next frame through `adapter` using the
    /// closure stored in `handle`, and updates `telemetry`.
    ///
    /// # Parameters
    /// - `adapter`: Scheduler used to request the next frame.
    /// - `telemetry`: Shared loop state read and updated each frame.
    /// - `handle`: Shared slot that will hold the returned closure, so it can
    ///   reschedule itself.
    /// - `user_callback`: The user's per-frame callback.
    ///
    /// # Returns
    /// The closure wrapped in `Some`, ready to be stored in `handle`.
    ///
    /// # Panics
    /// The returned closure panics if requesting the next frame fails.
    fn js_closure(
        adapter: &Rc<dyn FrameAdapter>,
        telemetry: &Rc<RefCell<Telemetry>>,
        handle: &Rc<RefCell<JsCallbackHandle>>,
        mut user_callback: impl FnMut(FrameCtx) + 'static,
    ) -> JsCallbackHandle {
        // Make RC clones of handle, adapter, and telemetry
        let inner_handle = Rc::clone(handle);
        let inner_adapter = Rc::clone(adapter);
        let inner_telemetry = Rc::clone(telemetry);

        // Create callback closure
        Some(ScopedClosure::own(move |timestamp| {
            // Create ctx and do callback
            let inner_telemetry_ref = inner_telemetry.borrow();
            let ctx = inner_telemetry_ref.make_frame_ctx(timestamp);
            user_callback(ctx);
            drop(inner_telemetry_ref);

            // Request new frame and update telemetry
            let id = Self::request_next_frame_with_adapter(&inner_adapter, &inner_handle)
                .expect("Failed to create next frame request");
            inner_telemetry.borrow_mut().update_state(timestamp, id);
        }))
    }

    /// Requests the next animation frame using the closure stored in `handle`.
    ///
    /// # Parameters
    /// - `adapter`: Scheduler used to request the frame.
    /// - `handle`: Slot holding the frame closure.
    ///
    /// # Returns
    /// The new request ID, or `Err` if `handle` is empty or the adapter call
    /// fails.
    ///
    /// # Side Effects
    /// Registers a pending animation frame request with `adapter`.
    fn request_next_frame_with_adapter(
        adapter: &Rc<dyn FrameAdapter>,
        handle: &RefCell<JsCallbackHandle>,
    ) -> Result<i32, JsValue> {
        let err = JsValue::from("Loop closure was None during frame request");
        let handle_ref = handle.borrow();
        let next_handle = handle_ref.as_ref().ok_or(err)?;
        adapter.request_animation_frame(next_handle)
    }
}

impl Drop for RAFLoop {
    /// Cancels the pending animation frame request when the loop is dropped.
    ///
    /// If no request is pending (which should not happen in normal use) this
    /// is a no-op.
    ///
    /// # Side Effects
    /// Calls `cancelAnimationFrame` on the pending request, stopping the loop.
    ///
    /// # Panics
    /// Panics if `cancelAnimationFrame` returns an error.
    fn drop(&mut self) {
        if let Some(id) = self.telemetry.borrow().pending_id {
            self.adapter
                .cancel_animation_frame(id)
                .expect("Failed to cancel animation frame on drop")
        }
    }
}
