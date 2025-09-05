// use leptos_use::{use_event_listener, UseEventListenerReturn};
use wasm_bindgen::JsCast;
use web_sys::{KeyboardEvent, EventTarget};

/// Hook for handling Escape key events
/// 
/// This hook provides a convenient way to handle Escape key presses,
/// commonly used for closing dialogs, dropdowns, and other overlay components.
/// 
/// # Arguments
/// 
/// * `handler` - Callback function to execute when Escape is pressed
/// * `target` - Optional target element (defaults to document)
/// 
/// # Example
/// 
/// ```rust
/// use radix_leptos_core::use_escape_keydown;
/// 
/// #[component]
/// pub fn Dialog() -> impl IntoView {
///     let (open, setopen) = create_signal(false);
///     
///     // Close dialog when Escape is pressed
///     use_escape_keydown(
///         move || setopen.set(false),
///         None,
///     );
///     
///     view! {
///         <Show when=move || open.get()>
///             <div role="dialog">
///                 "Dialog content"
///                 <button on:click=move |_| setopen.set(false)>
///                     "Close"
///                 </button>
///             </div>
///         </Show>
///     }
/// }
/// ```
pub fn use_escape_keydown<F>(
    handler: F,
    target: Option<EventTarget>,
) -> UseEventListenerReturn
where
    F: Fn() + 'static,
{
    let target = target.unwrap_or_else(|| {
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .unchecked_into()
    });

    use_event_listener(
        target,
        "keydown",
        move |event: web_sys::Event| {
            if let Ok(keyboard_event) = event.dyn_into::<KeyboardEvent>() {
                if keyboard_event.key() == "Escape" {
                    keyboard_event.prevent_default();
                    handler();
                }
            }
        },
    )
}

/// Hook for handling Escape key with conditional execution
/// 
/// This variant only executes the handler when a condition is met,
/// useful for components that should only respond to Escape when active.
/// 
/// # Arguments
/// 
/// * `handler` - Callback function to execute when Escape is pressed
/// * `should_handle` - Signal that determines whether to handle the event
/// * `target` - Optional target element (defaults to document)
pub fn use_escape_keydown_conditional<F>(
    handler: F,
    should_handle: Signal<bool>,
    target: Option<EventTarget>,
) -> UseEventListenerReturn
where
    F: Fn() + 'static,
{
    use_escape_keydown(
        move || {
            if should_handle.get() {
                handler();
            }
        },
        target,
    )
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    #[wasm_bindgen_test]
    fn test_escape_handler_called() {
        run_test(|cx| {
            let (called, set_called) = create_signal(cx, false);
            
            let _listener = use_escape_keydown(
                move || set_called.set(true),
                None,
            );
            
            // Simulate Escape key press
            let event = web_sys::KeyboardEvent::new_with_keyboard_event_init_dict(
                "keydown",
                &web_sys::KeyboardEventInit::new().key("Escape"),
            ).unwrap();
            
            web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .dispatch_event(&event)
                .unwrap();
            
            assert!(called.get());
        });
    }
    
    #[wasm_bindgen_test]
    fn test_conditional_escape_handler() {
        run_test(|cx| {
            let (called, set_called) = create_signal(cx, false);
            let (should_handle, set_should_handle) = create_signal(cx, false);
            
            let _listener = use_escape_keydown_conditional(
                move || set_called.set(true),
                should_handle.into(),
                None,
            );
            
            // Should not be called when condition is false
            simulate_escape_key();
            assert!(!called.get());
            
            // Should be called when condition is true
            set_should_handle.set(true);
            simulate_escape_key();
            assert!(called.get());
        });
    }
    
    fn simulate_escape_key() {
        let event = web_sys::KeyboardEvent::new_with_keyboard_event_init_dict(
            "keydown",
            &web_sys::KeyboardEventInit::new().key("Escape"),
        ).unwrap();
        
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .dispatch_event(&event)
            .unwrap();
    }
    
    fn run_test<F>(f: F) where F: FnOnce(Scope) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let _ = create_runtime();
            run_scope(create_runtime(), f);
        });
    }
}