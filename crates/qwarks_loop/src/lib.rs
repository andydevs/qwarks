use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn my_func(a: i32, b: i32, c: i32) -> i32 {
    a * b + c
}
