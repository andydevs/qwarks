//! Frame timing state.
//!
//! Defines [`FrameCtx`], the per-frame context handed to user callbacks, and
//! [`Telemetry`], the internal state shared between a [`RAFLoop`](crate::RAFLoop)
//! and its recurring frame closure.

use crate::adapter::FrameReqId;
use std::cell::RefCell;
use std::rc::Rc;

/// Per-frame context passed to the animation loop callback.
#[derive(Debug)]
pub struct FrameCtx {
    /// Number of frames that completed before this one, starting at `0` on
    /// the first frame.
    pub frame_count: u32,
    /// The `DOMHighResTimeStamp` provided by `requestAnimationFrame` for the current frame,
    /// in milliseconds.
    pub timestamp: f64,
    /// Time elapsed since the previous frame in milliseconds.
    /// On the first frame this is `0.0`.
    pub delta: f64,
}

/// Internal state tracked between animation frames.
///
/// `Telemetry` is stored inside an `Rc<RefCell<Telemetry>>` shared between
/// the outer [`RAFLoop`](crate::RAFLoop) handle and the recurring frame
/// closure.  It is updated at the end of each frame and read at the start of
/// the next.
#[derive(Clone, Copy)]
pub(crate) struct Telemetry {
    /// The request ID returned by the most recent `requestAnimationFrame`
    /// call, or `None` before the first request has been made.
    ///
    /// Used by [`RAFLoop`](crate::RAFLoop)'s `Drop` implementation to cancel
    /// the in-flight frame request.
    pub(crate) pending_id: Option<FrameReqId>,
    /// Number of frames that have completed so far.
    ///
    /// Starts at `0` before the first frame and is incremented by
    /// [`update_state`](Self::update_state) at the end of each frame, so the
    /// value passed to a callback via [`make_frame_ctx`](Self::make_frame_ctx)
    /// reflects how many frames have already been rendered before the current
    /// one.
    frame_count: u32,
    /// The `DOMHighResTimeStamp` from the previous frame, or `None` on the
    /// first frame.
    ///
    /// Used to compute `delta` in [`make_frame_ctx`](Self::make_frame_ctx).
    last_timestamp: Option<f64>,
}

impl Telemetry {
    /// Creates a fresh, shareable telemetry handle.
    ///
    /// # Returns
    /// An `Rc<RefCell<Telemetry>>` with a zero frame count, no previous
    /// timestamp, and no pending request.
    pub(crate) fn create_new_handle() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            frame_count: 0,
            pending_id: None,
            last_timestamp: None,
        }))
    }

    /// Builds a [`FrameCtx`] for the current frame.
    ///
    /// `delta` is computed as `timestamp - last_timestamp`, or `0.0` on the
    /// first frame when no previous timestamp exists.
    ///
    /// # Parameters
    /// - `timestamp`: The `DOMHighResTimeStamp` for the current frame, in
    ///   milliseconds.
    ///
    /// # Returns
    /// The context to pass to the user callback.
    pub(crate) fn make_frame_ctx(&self, timestamp: f64) -> FrameCtx {
        let delta = self.last_timestamp.map(|last| timestamp - last).unwrap_or(0.0);
        FrameCtx { frame_count: self.frame_count, timestamp, delta }
    }

    /// Resets the state for a newly started loop.
    ///
    /// # Parameters
    /// - `frame_request_id`: Request ID of the loop's first frame request.
    ///
    /// # Side Effects
    /// Sets `frame_count` to `0`, clears `last_timestamp`, and stores
    /// `frame_request_id` as `pending_id`.
    pub(crate) fn initialize_state(&mut self, frame_request_id: FrameReqId) {
        self.frame_count = 0;
        self.last_timestamp = None;
        self.pending_id = Some(frame_request_id);
    }

    /// Records that a frame has completed and a new one has been requested.
    ///
    /// # Parameters
    /// - `timestamp`: The `DOMHighResTimeStamp` of the frame that completed.
    /// - `frame_request_id`: Request ID of the newly scheduled frame.
    ///
    /// # Side Effects
    /// Increments `frame_count`, stores `timestamp` as `last_timestamp`, and
    /// stores `frame_request_id` as `pending_id`.
    pub(crate) fn update_state(&mut self, timestamp: f64, frame_request_id: FrameReqId) {
        self.frame_count += 1;
        self.last_timestamp = Some(timestamp);
        self.pending_id = Some(frame_request_id);
    }
}
