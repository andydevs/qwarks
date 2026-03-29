use wasm_bindgen::{JsCast, JsValue, prelude::ScopedClosure};
use web_sys::window;

pub trait Adapter {
    fn request_animation_frame(
        &self,
        callback: &ScopedClosure<'_, dyn FnMut(f64)>,
    ) -> Result<i32, JsValue>;

    fn cancel_animation_frame(&self, id: i32) -> Result<(), JsValue>;
}

pub struct BrowserAdapter;

impl Adapter for BrowserAdapter {
    fn cancel_animation_frame(&self, id: i32) -> Result<(), JsValue> {
        window()
            .ok_or(JsValue::from("Window was not defined"))?
            .cancel_animation_frame(id)
    }

    fn request_animation_frame(
        &self,
        callback: &ScopedClosure<'_, dyn FnMut(f64)>,
    ) -> Result<i32, JsValue> {
        window()
            .ok_or(JsValue::from("Window was not defined"))?
            .request_animation_frame(callback.as_ref().unchecked_ref())
    }
}
