use leptos::*;
use leptos_use::{use_event_listener, UseEventListenerReturn};
use wasm_bindgen::JsCast;
use web_sys::{Element, Event, MouseEvent, TouchEvent, EventTarget};

/// Hook for detecting clicks outside a target element
/// 
/// This hook is essential for closing overlays, dropdowns, and other components
/// when the user clicks outside of them.
/// 
/// # Arguments
/// 
/// * `target` - The element to detect outside clicks for
/// * `handler` - Callback function to execute when outside click is detected
/// * `events` - Optional list of events to listen for (defaults to mousedown and touchstart)
/// 
/// # Example
/// 
/// ```rust
/// use leptos::*;
/// use radix_leptos_core::use_outside_click;
/// 
/// #[component]
/// pub fn Dropdown() -> impl IntoView {
///     let (open, set_open) = create_signal(false);
///     let dropdown_ref = create_node_ref::<web_sys::Element>();
///     
///     // Close dropdown when clicking outside
///     use_outside_click(
///         dropdown_ref,
///         move || set_open.set(false),
///         None,
///     );
///     
///     view! {
///         <div>
///             <button on:click=move |_| set_open.set(!open.get())>
///                 "Toggle Dropdown"
///             </button>
///             <Show when=move || open.get()>
///                 <div ref=dropdown_ref class="dropdown-content">
///                     "Dropdown content"
///                 </div>
///             </Show>
///         </div>
///     }
/// }
/// ```
pub fn use_outside_click<F>(
    target: NodeRef<Element>,
    handler: F,
    events: Option<Vec<&'static str>>,
) -> Vec<UseEventListenerReturn>
where
    F: Fn() + Clone + 'static,
{
    let events = events.unwrap_or_else(|| vec!["mousedown", "touchstart"]);
    let document = web_sys::window()
        .unwrap()
        .document()
        .unwrap();

    events
        .into_iter()
        .map(|event_type| {
            let handler_clone = handler.clone();
            
            use_event_listener(
                document.clone().unchecked_into::<EventTarget>(),
                event_type,
                move |event: Event| {
                    // Get the target element
                    if let Some(target_element) = target.get() {
                        // Check if click target is outside our element
                        if let Some(event_target) = event.target() {
                            if let Ok(clicked_element) = event_target.dyn_into::<Element>() {
                                // Check if clicked element is contained within our target
                                if !target_element.contains(Some(&clicked_element)) {
                                    handler_clone();
                                }
                            }
                        }
                    }
                },
            )
        })
        .collect()
}

/// Hook for detecting outside clicks with conditional execution
/// 
/// This variant only executes the handler when a condition is met,
/// useful for components that should only respond to outside clicks when active.
/// 
/// # Arguments
/// 
/// * `target` - The element to detect outside clicks for
/// * `handler` - Callback function to execute when outside click is detected
/// * `should_handle` - Signal that determines whether to handle the event
/// * `events` - Optional list of events to listen for
pub fn use_outside_click_conditional<F>(
    target: NodeRef<Element>,
    handler: F,
    should_handle: Signal<bool>,
    events: Option<Vec<&'static str>>,
) -> Vec<UseEventListenerReturn>
where
    F: Fn() + Clone + 'static,
{
    use_outside_click(
        target,
        move || {
            if should_handle.get() {
                handler();
            }
        },
        events,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    use web_sys::HtmlElement;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    #[wasm_bindgen_test]
    fn test_outside_click_detection() {
        run_test(|cx| {
            let (clicked_outside, set_clicked_outside) = create_signal(cx, false);
            let target_ref = create_node_ref::<web_sys::Element>();
            
            // Create target element
            let target = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .create_element("div")
                .unwrap();
            target.set_id("target");
            
            web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .body()
                .unwrap()
                .append_child(&target)
                .unwrap();
            
            // Set up outside click handler
            let _listeners = use_outside_click(
                target_ref,
                move || set_clicked_outside.set(true),
                None,
            );
            
            // Simulate click outside target
            let outside_element = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .create_element("button")
                .unwrap();
            
            web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .body()
                .unwrap()
                .append_child(&outside_element)
                .unwrap();
            
            // Simulate mouse click
            let event = web_sys::MouseEvent::new("mousedown").unwrap();
            outside_element.dispatch_event(&event).unwrap();
            
            assert!(clicked_outside.get());
        });
    }
    
    #[wasm_bindgen_test]
    fn test_inside_click_ignored() {
        run_test(|cx| {
            let (clicked_outside, set_clicked_outside) = create_signal(cx, false);
            let target_ref = create_node_ref::<web_sys::Element>();
            
            // Create target element with child
            let target = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .create_element("div")
                .unwrap();
            
            let child = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .create_element("button")
                .unwrap();
            
            target.append_child(&child).unwrap();
            
            web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .body()
                .unwrap()
                .append_child(&target)
                .unwrap();
            
            // Set up outside click handler
            let _listeners = use_outside_click(
                target_ref,
                move || set_clicked_outside.set(true),
                None,
            );
            
            // Simulate click on child (should not trigger)
            let event = web_sys::MouseEvent::new("mousedown").unwrap();
            child.dispatch_event(&event).unwrap();
            
            assert!(!clicked_outside.get());
        });
    }
    
    fn run_test<F>(f: F) where F: FnOnce(Scope) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let _ = create_runtime();
            run_scope(create_runtime(), f);
        });
    }
}