use crate::FrameCtx;

#[derive(Clone, Copy)]
pub struct Telemetry {
    pub frame_count: u32,
    pub pending_id: Option<i32>,
    pub last_timestamp: Option<f64>,
}

impl Telemetry {
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

    pub fn update(&mut self, timestamp: f64, frame_request_id: i32) {
        self.frame_count += 1;
        self.last_timestamp = Some(timestamp);
        self.pending_id = Some(frame_request_id);
    }
}
