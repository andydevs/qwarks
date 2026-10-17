/// Per-frame context passed to the animation loop callback.
#[derive(Debug)]
pub struct FrameCtx {
    /// Number of frames that have been rendered so far, starting at 1.
    pub frame_count: u32,
    /// The `DOMHighResTimeStamp` provided by `requestAnimationFrame` for the current frame,
    /// in milliseconds.
    pub timestamp: f64,
    /// Time elapsed since the previous frame in milliseconds.
    /// On the first frame this is `0.0`.
    pub delta: f64,
}
