use crate::{
    FrameCtx,
    browser::{self, BrowserAdapter},
    telemetry::Telemetry,
};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{JsValue, prelude::Closure};

/// An active `requestAnimationFrame` loop.
///
/// While this value is alive the browser will invoke your callback once per
/// animation frame.  When the value is dropped the pending frame request is
/// cancelled via `cancelAnimationFrame`, stopping the loop.
///
/// # Example
///
/// ```no_run
/// use wasm_raf_handler::{RAFLoop, FrameCtx};
///
/// let _loop = RAFLoop::new(|ctx: FrameCtx| {
///     // update your scene here
/// }).expect("Failed to start RAF loop");
/// // Loop runs until `_loop` is dropped.
/// ```
pub struct RAFLoop {
    /// The recurring closure passed to `requestAnimationFrame`.
    ///
    /// Kept alive for the lifetime of the loop; the `#[allow(unused)]`
    /// suppresses the dead-code lint because the value is never read — it
    /// exists solely to prevent the closure from being dropped.
    #[allow(unused)]
    frame_callback: Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>>,
    /// Shared loop state (frame count, last timestamp, pending request ID).
    telemetry: Rc<RefCell<Telemetry>>,
    /// Adapter used to call `requestAnimationFrame` / `cancelAnimationFrame`.
    adapter: Rc<dyn browser::Adapter>,
}

impl RAFLoop {
    /// Creates a new animation loop using the real browser APIs.
    ///
    /// `callback` is called once per animation frame with a [`FrameCtx`]
    /// containing timing information for that frame.
    ///
    /// Returns `Err` if the initial `requestAnimationFrame` call fails (e.g.
    /// when `window` is not available).
    pub fn new<F: FnMut(FrameCtx) + 'static>(callback: F) -> Result<Self, JsValue> {
        let adapter = Rc::new(BrowserAdapter);
        Self::with_adapter(callback, adapter)
    }

    /// Creates a new animation loop driven by a custom [`browser::Adapter`].
    ///
    /// This is the internal constructor used by [`new`](Self::new) and exposed
    /// for testing with mock adapters.  The loop starts immediately: the first
    /// frame is requested before this function returns.
    fn with_adapter<F: FnMut(FrameCtx) + 'static>(
        mut callback: F,
        adapter: Rc<dyn browser::Adapter>,
    ) -> Result<Self, JsValue> {
        // Enable panic hook in case anything fails here
        console_error_panic_hook::set_once();

        // Setup callback and telemetry
        let frame_callback: Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>> =
            Rc::new(RefCell::new(None));
        let telemetry = Rc::new(RefCell::new(Telemetry {
            frame_count: 0,
            pending_id: None,
            last_timestamp: None,
        }));

        // Create copies and move to inner callback
        let inner_callback = Rc::clone(&frame_callback);
        let inner_adapter = Rc::clone(&adapter);
        let inner_telemetry = Rc::clone(&telemetry);
        *frame_callback.borrow_mut() = Some(Closure::new(move |timestamp| {
            // Create ctx and do callback
            let ctx = inner_telemetry.borrow().get_frame_ctx(timestamp);
            callback(ctx);

            // Request new frame
            let frame_request_id = {
                let new_frame = inner_callback.borrow();
                let new_frame = new_frame.as_ref().expect("Loop closure is Sone");
                inner_adapter
                    .request_animation_frame(new_frame)
                    .expect("Request animation frame successful")
            };

            // Update telemetry
            inner_telemetry
                .borrow_mut()
                .update(timestamp, frame_request_id);
        }));

        // Call first frame. Initialize telemetry
        let id = {
            let callback = frame_callback.borrow();
            let callback = callback.as_ref().ok_or(JsValue::from(
                "Loop closure was None during first frame request",
            ))?;
            adapter.request_animation_frame(callback)
        }?;
        telemetry.borrow_mut().pending_id = Some(id);

        // Return RAF Loop
        Ok(Self {
            frame_callback,
            telemetry,
            adapter,
        })
    }
}

impl Drop for RAFLoop {
    /// Cancels the pending animation frame request when the loop is dropped.
    ///
    /// If no request is pending (which should not happen in normal use) this
    /// is a no-op.  Panics if `cancelAnimationFrame` returns an error.
    fn drop(&mut self) {
        if let Some(id) = self.telemetry.borrow().pending_id {
            self.adapter
                .cancel_animation_frame(id)
                .expect("Cancel animation frame")
        }
    }
}
