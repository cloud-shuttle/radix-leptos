use leptos::*;
// Only import what we actually use in the examples
use leptos::mount::mount_to_body;
use wasm_bindgen::prelude::*;

// Re-export only the core examples (available with core feature)
pub mod alert_examples;
pub mod badge_examples;
pub mod list_examples;
pub mod pagination_examples;
pub mod timeline_examples;
pub mod toast_examples;
pub mod real_demo;
pub mod simple_test;
// Note: test_components, avatar, image, video, audio, carousel, context_menu, menubar, scroll_area are not in core feature

use pagination_examples::PaginationExamples;

// Test function to see if wasm_bindgen is working
#[wasm_bindgen]
pub fn test_function() -> String {
    "Hello from Rust!".to_string()
}

// Export the pagination examples mount function
#[wasm_bindgen]
pub fn start_pagination_examples() {
    // Add console logging to debug
    web_sys::console::log_1(&"Starting Pagination Examples...".into());

    // Mount the PaginationExamples using Leptos 0.8
    mount_to_body(|| {
        view! {
            <PaginationExamples/>
        }
    });

    web_sys::console::log_1(&"Pagination Examples mounted successfully!".into());
}
