//! WebAssembly entry point for the qwarks demo site.

use wasm_bindgen::prelude::*;

/// Returns the qwarks library version.
#[wasm_bindgen]
pub fn version() -> String {
    qwarks::version().to_string()
}
