use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use wasm_bindgen::{JsCast, JsValue, closure::Closure};
use web_sys::window;

use super::context::Ctx;
use super::state::State;

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