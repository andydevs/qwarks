use crate::FrameCtx;

/// Internal state tracked between animation frames.
///
/// `Telemetry` is stored inside an `Rc<RefCell<Telemetry>>` shared between
/// the outer [`RAFLoop`](crate::RAFLoop) handle and the recurring frame
/// closure.  It is updated at the end of each frame and read at the start of
/// the next.
#[derive(Clone, Copy)]
pub struct Telemetry {
    /// Number of frames that have completed so far.
    ///
    /// Starts at `0` before the first frame and is incremented by
    /// [`update`](Self::update) at the end of each frame, so the value passed
    /// to a callback via [`get_frame_ctx`](Self::get_frame_ctx) reflects how
    /// many frames have already been rendered before the current one.
    pub frame_count: u32,
    /// The request ID returned by the most recent `requestAnimationFrame`
    /// call, or `None` before the first request has been made.
    ///
    /// Used by [`RAFLoop`](crate::RAFLoop)'s `Drop` implementation to cancel
    /// the in-flight frame request.
    pub pending_id: Option<i32>,
    /// The `DOMHighResTimeStamp` from the previous frame, or `None` on the
    /// first frame.
    ///
    /// Used to compute `delta` in [`get_frame_ctx`](Self::get_frame_ctx).
    pub last_timestamp: Option<f64>,
}

impl Telemetry {
    /// Builds a [`FrameCtx`] for the current frame.
    ///
    /// `delta` is computed as `timestamp - last_timestamp`, or `0.0` on the
    /// first frame when no previous timestamp exists.
    pub fn get_frame_ctx(&self, timestamp: f64) -> FrameCtx {
        let delta = self
            .last_timestamp
            .map(|last| timestamp - last)
            .unwrap_or(0.0);
        FrameCtx {
            frame_count: self.frame_count,
            timestamp,
            delta,
        }
    }

    /// Records that a frame has completed and a new one has been requested.
    ///
    /// Increments `frame_count`, stores `timestamp` as `last_timestamp`, and
    /// stores `frame_request_id` as `pending_id`.
    pub fn update(&mut self, timestamp: f64, frame_request_id: i32) {
        self.frame_count += 1;
        self.last_timestamp = Some(timestamp);
        self.pending_id = Some(frame_request_id);
    }
}
