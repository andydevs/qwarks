use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};
use wasm_bindgen::{JsCast, JsValue, closure::Closure};
use web_sys::window;

#[derive(Debug)]
pub struct Ctx {
    pub frame_count: u32,
    pub timestamp: f64,
    pub delta: f64,
}

struct State {
    stop: Rc<Cell<bool>>,
    frame_count: u32,
    last_timestamp: Option<f64>,
}

impl State {
    fn new() -> Self {
        Self {
            stop: Rc::new(Cell::new(false)),
            frame_count: 0,
            last_timestamp: None,
        }
    }

    fn update(&mut self, timestamp: f64) {
        self.frame_count += 1;
        self.last_timestamp = Some(timestamp);
    }

    fn get_ctx(&self, timestamp: f64) -> Ctx {
        let last = self.last_timestamp.unwrap_or(timestamp);
        Ctx {
            frame_count: self.frame_count,
            timestamp,
            delta: timestamp - last,
        }
    }

    fn create_stop_signal(&self) -> Rc<Cell<bool>> {
        Rc::clone(&self.stop)
    }

    fn stopped(&self) -> bool {
        self.stop.get()
    }
}

type CbPtr = Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>>;

fn call_request_frame(callback: &CbPtr) -> Result<(), JsValue> {
    window()
        .ok_or(JsValue::from("Window was not defined"))?
        .request_animation_frame(
            callback
                .borrow()
                .as_ref()
                .ok_or(JsValue::from("CbPtr was None during frame request"))?
                .as_ref()
                .unchecked_ref(),
        )
        .map_err(|_| JsValue::from("Error requesting new animation frame"))?;
    Ok(())
}

pub struct Loop(Rc<Cell<bool>>);

impl Loop {
    pub fn new<F: FnMut(Ctx) + 'static>(mut cb: F) -> Result<Self, JsValue> {
        // Enable panic hook in case request frame fails internally
        console_error_panic_hook::set_once();

        // Initialize state
        let mut state = State::new();
        let self_ptr = Self(state.create_stop_signal());

        // Setup callback
        let outer: CbPtr = Rc::new(RefCell::new(None));
        let inner = Rc::clone(&outer);
        *outer.borrow_mut() = Some(Closure::new(move |timestamp| {
            let ctx = state.get_ctx(timestamp);
            cb(ctx);
            state.update(timestamp);

            // Drop callback if stop is called. Else request new frame
            if state.stopped() {
                inner.take();
            } else {
                call_request_frame(&inner).unwrap();
            }
        }));

        // Call the first animation frame consuming first ptr
        call_request_frame(&outer)?;

        // Output with ptr
        Ok(self_ptr)
    }
}

impl Drop for Loop {
    fn drop(&mut self) {
        self.0.set(true);
    }
}
