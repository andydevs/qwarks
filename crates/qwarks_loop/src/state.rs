use std::{cell::Cell, rc::Rc};

use super::context::Ctx;

pub(crate) struct State {
    stop: Rc<Cell<bool>>,
    frame_count: u32,
    last_timestamp: Option<f64>,
}

impl State {
    pub(crate) fn new() -> Self {
        Self {
            stop: Rc::new(Cell::new(false)),
            frame_count: 0,
            last_timestamp: None,
        }
    }

    pub(crate) fn update(&mut self, timestamp: f64) {
        self.frame_count += 1;
        self.last_timestamp = Some(timestamp);
    }

    pub(crate) fn get_ctx(&self, timestamp: f64) -> Ctx {
        let last = self.last_timestamp.unwrap_or(timestamp);
        Ctx {
            frame_count: self.frame_count,
            timestamp,
            delta: timestamp - last,
        }
    }

    pub(crate) fn create_stop_signal(&self) -> Rc<Cell<bool>> {
        Rc::clone(&self.stop)
    }

    pub(crate) fn stopped(&self) -> bool {
        self.stop.get()
    }
}
