#[derive(Clone, Copy)]
pub struct Telemetry {
    pub frame_count: u32,
    pub pending_id: Option<i32>,
    pub last_timestamp: Option<f64>,
}
