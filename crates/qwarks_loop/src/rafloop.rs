use crate::{
    FrameCtx,
    browser::{self, BrowserAdapter},
    telemetry::Telemetry,
};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{JsValue, prelude::Closure};

pub struct RAFLoop {
    #[allow(unused)]
    frame_callback: Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>>,
    telemetry: Rc<RefCell<Telemetry>>,
    adapter: Rc<dyn browser::Adapter>,
}

impl RAFLoop {
    pub fn new<F: FnMut(FrameCtx) + 'static>(callback: F) -> Result<Self, JsValue> {
        let adapter = Rc::new(BrowserAdapter);
        Self::with_adapter(callback, adapter)
    }

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
            let state = inner_telemetry.borrow();
            let delta = state
                .last_timestamp
                .map(|last| timestamp - last)
                .unwrap_or(0.0);
            let ctx = FrameCtx {
                frame_count: state.frame_count,
                timestamp,
                delta,
            };
            callback(ctx);

            // Request new frame
            let id = {
                let new_frame = inner_callback.borrow();
                let new_frame = new_frame.as_ref().expect("Loop closure is Sone");
                inner_adapter
                    .request_animation_frame(new_frame)
                    .expect("Request animation frame successful")
            };

            // Update telemetry
            let mut tm = inner_telemetry.borrow_mut();
            tm.frame_count += 1;
            tm.last_timestamp = Some(timestamp);
            tm.pending_id = Some(id);
        }));

        // Call first frame. Update telemetry
        let id = {
            let callback = frame_callback.borrow();
            let callback = callback
                .as_ref()
                .ok_or(JsValue::from("Loop closure was None during frame request"))?;
            adapter.request_animation_frame(callback)
        }?;
        telemetry.borrow_mut().pending_id = Some(id);

        Ok(Self {
            frame_callback,
            telemetry,
            adapter,
        })
    }
}

impl Drop for RAFLoop {
    fn drop(&mut self) {
        if let Some(id) = self.telemetry.borrow().pending_id {
            self.adapter
                .cancel_animation_frame(id)
                .expect("Cancel animation frame")
        }
    }
}
