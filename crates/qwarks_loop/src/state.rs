use std::{cell::Cell, rc::Rc};

use super::context::Ctx;

/// Internal mutable state shared across animation frames.
pub(crate) struct State {
    /// Shared stop flag; set to `true` to halt the loop after the current frame.
    stop: Rc<Cell<bool>>,
    /// Running count of frames rendered.
    frame_count: u32,
    /// Timestamp of the most recently completed frame, or `None` before the first frame.
    last_timestamp: Option<f64>,
}

impl State {
    /// Creates a new [`State`] with zeroed counters and the stop flag unset.
    pub(crate) fn new() -> Self {
        Self {
            stop: Rc::new(Cell::new(false)),
            frame_count: 0,
            last_timestamp: None,
        }
    }

    /// Advances the frame counter and records `timestamp` as the last seen timestamp.
    pub(crate) fn update(&mut self, timestamp: f64) {
        self.frame_count += 1;
        self.last_timestamp = Some(timestamp);
    }

    /// Builds a [`Ctx`] for the current frame using `timestamp`.
    ///
    /// `delta` is computed as the difference between `timestamp` and the last recorded
    /// timestamp, falling back to `0.0` on the first frame.
    pub(crate) fn get_ctx(&self, timestamp: f64) -> Ctx {
        let last = self.last_timestamp.unwrap_or(timestamp);
        Ctx {
            frame_count: self.frame_count,
            timestamp,
            delta: timestamp - last,
        }
    }

    /// Returns a clone of the shared stop-flag handle.
    ///
    /// Setting the flag to `true` via this handle will cause the loop to stop
    /// after the current frame completes.
    pub(crate) fn create_stop_signal(&self) -> Rc<Cell<bool>> {
        Rc::clone(&self.stop)
    }

    /// Returns `true` if the stop flag has been set.
    pub(crate) fn stopped(&self) -> bool {
        self.stop.get()
    }
}
