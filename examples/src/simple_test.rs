use wasm_bindgen::prelude::*;

/// Simple test function to verify WASM is working
#[wasm_bindgen]
pub fn simple_test_function() -> String {
    "WASM is working!".to_string()
}

/// Simple function that just logs to console
#[wasm_bindgen]
pub fn log_test() {
    web_sys::console::log_1(&"WASM log test successful!".into());
}
