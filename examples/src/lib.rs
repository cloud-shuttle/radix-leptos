use leptos::*;
use radix_leptos_primitives::*;
use wasm_bindgen::prelude::*;
use leptos::prelude::*;

// Re-export the examples
pub mod test_components;
use test_components::ComponentTestSuite;

// Test function to see if wasm_bindgen is working
#[wasm_bindgen]
pub fn test_function() -> String {
    "Hello from Rust!".to_string()
}

// Export the mount function for web
#[wasm_bindgen]
pub fn start_app() {
    // Add console logging to debug
    web_sys::console::log_1(&"Starting Leptos app...".into());
    
    // Mount the ComponentTestSuite using Leptos 0.8
    mount_to_body(|| {
        view! {
            <ComponentTestSuite/>
        }
    });
    
    web_sys::console::log_1(&"Leptos app mounted successfully!".into());
}
