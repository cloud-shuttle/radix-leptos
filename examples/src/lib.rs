use leptos::*;
use radix_leptos_primitives::*;
use wasm_bindgen::prelude::*;
use leptos::prelude::*;

// Re-export the examples
pub mod test_components;
pub mod pagination_examples;
pub mod list_examples;
pub mod timeline_examples;
pub mod toast_examples;
pub mod alert_examples;
pub mod badge_examples;
pub mod avatar_examples;
pub mod image_examples;
pub mod video_examples;
pub mod audio_examples;
pub mod carousel_examples;
pub mod context_menu_examples;
pub mod dropdown_menu_examples;
pub mod menubar_examples;
pub mod scroll_area_examples;
pub mod tabs_examples;
pub mod component_showcase;
use test_components::ComponentTestSuite;
use pagination_examples::PaginationExamples;
use list_examples::ListExamples;
use timeline_examples::TimelineExamples;
use toast_examples::{ToastExamples, start_toast_examples as start_toast_examples_fn};
use alert_examples::{AlertExamples, start_alert_examples as start_alert_examples_fn};
use badge_examples::{BadgeExamples, start_badge_examples as start_badge_examples_fn};
use avatar_examples::{AvatarExamples, start_avatar_examples as start_avatar_examples_fn};
use image_examples::{ImageExamples, start_image_examples as start_image_examples_fn};
use video_examples::{VideoExamples, start_video_examples as start_video_examples_fn};
use audio_examples::{AudioExamples, start_audio_examples as start_audio_examples_fn};
use carousel_examples::{CarouselExamples, start_carousel_examples_fn};
use context_menu_examples::{ContextMenuExamples, start_context_menu_examples_fn};
use dropdown_menu_examples::{DropdownMenuExamples, start_dropdown_menu_examples_fn};
use menubar_examples::{MenubarExamples, start_menubar_examples_fn};
use scroll_area_examples::{ScrollAreaExamples, start_scroll_area_examples_fn};
use tabs_examples::{TabsExamples, start_tabs_examples_fn};


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

// Export the list examples mount function
#[wasm_bindgen]
pub fn start_list_examples() {
    // Add console logging to debug
    web_sys::console::log_1(&"Starting List Examples...".into());
    
    // Mount the ListExamples using Leptos 0.8
    mount_to_body(|| {
        view! {
            <ListExamples/>
        }
    });
    
    web_sys::console::log_1(&"List Examples mounted successfully!".into());
}

// Export the timeline examples mount function
#[wasm_bindgen]
pub fn start_timeline_examples() {
    // Add console logging to debug
    web_sys::console::log_1(&"Starting Timeline Examples...".into());
    
    // Mount the TimelineExamples using Leptos 0.8
    mount_to_body(|| {
        view! {
            <TimelineExamples/>
        }
    });
    
    web_sys::console::log_1(&"Timeline Examples mounted successfully!".into());
}

// Export the toast examples mount function
#[wasm_bindgen]
pub fn start_toast_examples() {
    start_toast_examples_fn();
}

// Export the alert examples mount function
#[wasm_bindgen]
pub fn start_alert_examples() {
    start_alert_examples_fn();
}

// Export the badge examples mount function
#[wasm_bindgen]
pub fn start_badge_examples() {
    start_badge_examples_fn();
}

// Export the avatar examples mount function
#[wasm_bindgen]
pub fn start_avatar_examples() {
    start_avatar_examples_fn();
}

// Export the image examples mount function
#[wasm_bindgen]
pub fn start_image_examples() {
    start_image_examples_fn();
}

// Export the video examples mount function
#[wasm_bindgen]
pub fn start_video_examples() {
    start_video_examples_fn();
}

// Export the audio examples mount function
#[wasm_bindgen]
pub fn start_audio_examples() {
    start_audio_examples_fn();
}

// Export the carousel examples mount function
#[wasm_bindgen]
pub fn start_carousel_examples() {
    start_carousel_examples_fn();
}

// Export the context menu examples mount function
#[wasm_bindgen]
pub fn start_context_menu_examples() {
    start_context_menu_examples_fn();
}

// Export the dropdown menu examples mount function
#[wasm_bindgen]
pub fn start_dropdown_menu_examples() {
    start_dropdown_menu_examples_fn();
}

// Export the menubar examples mount function
#[wasm_bindgen]
pub fn start_menubar_examples() {
    start_menubar_examples_fn();
}

// Export the scroll area examples mount function
#[wasm_bindgen]
pub fn start_scroll_area_examples() {
    start_scroll_area_examples_fn();
}

// Export the tabs examples mount function
#[wasm_bindgen]
pub fn start_tabs_examples() {
    start_tabs_examples_fn();
}

// Export the component showcase mount function
#[wasm_bindgen]
pub fn start_component_showcase() {
    component_showcase::start_component_showcase();
}
