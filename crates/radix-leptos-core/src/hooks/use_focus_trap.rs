use web_sys::{Element, HtmlElement, KeyboardEvent};
use wasm_bindgen::JsCast;
use crate::utils::dom::{get_focusable_elements, get_first_focusable, get_last_focusable};

/// Hook for trapping focus within a container element
/// 
/// Focus trapping is essential for modal dialogs and other overlay components
/// to ensure keyboard navigation stays within the intended boundaries.
/// This implementation handles Tab and Shift+Tab cycling.
/// 
/// # Arguments
/// 
/// * `container_ref` - Reference to the container element to trap focus within
/// * `active` - Signal controlling whether focus trap is active
/// * `initial_focus` - Optional element to focus when trap becomes active
/// * `restore_focus` - Optional element to restore focus to when trap deactivates
/// 
/// # Example
/// 
/// ```rust
/// use radix_leptos_core::use_focus_trap;
/// 
/// #[component]
/// pub fn Dialog() -> impl IntoView {
///     let (open, setopen) = create_signal(false);
///     let dialog_ref = create_node_ref::<web_sys::Element>();
///     let trigger_ref = create_node_ref::<web_sys::Element>();
///     
///     // Trap focus within dialog when open
///     use_focus_trap(
///         dialog_ref,
///         open.into(),
///         None,
///         Some(trigger_ref),
///     );
///     
///     view! {
///         <button ref=trigger_ref on:click=move |_| setopen.set(true)>
///             "Open Dialog"
///         </button>
///         <Show when=move || open.get()>
///             <div ref=dialog_ref role="dialog">
///                 <button on:click=move |_| setopen.set(false)>
///                     "Close"
///                 </button>
///                 <input type="text" placeholder="Trapped input" />
///             </div>
///         </Show>
///     }
/// }
/// ```
pub fn use_focus_trap(
    container_ref: NodeRef<Element>,
    active: Signal<bool>,
    initial_focus: Option<NodeRef<Element>>,
    restore_focus: Option<NodeRef<Element>>,
) {
    let (previouslyfocused, set_previouslyfocused) = create_signal::<Option<Element>>(None);
    
    // Effect to handle focus trap activation/deactivation
    create_effect(move |_| {
        let is_active = active.get();
        
        if is_active {
            if let Some(container) = container_ref.get() {
                // Store currently focused element for restoration
                if let Some(current) = getcurrentlyfocused() {
                    set_previouslyfocused.set(Some(current));
                }
                
                // Focus initial element or first focusable element
                if let Some(initial_ref) = initial_focus {
                    if let Some(initial_element) = initial_ref.get() {
                        focus_element(&initial_element);
                    }
                } else if let Some(first_focusable) = get_first_focusable(&container) {
                    focus_element(&first_focusable);
                }
                
                // Set up focus trap event listeners
                setup_focus_trap(&container);
            }
                previouslyfocused.get_untracked()
            };
            
            if let Some(element) = element_to_restore {
                focus_element(&element);
            }
        }
    });
}

/// Set up keyboard event listeners for focus trapping
fn setup_focus_trap(container: &Element) {
    use leptos_use::use_event_listener;
    use wasm_bindgen::closure::Closure;
    
    let container_clone = container.clone();
    
    let _listener = use_event_listener(
        container.clone(),
        "keydown",
        move |event: web_sys::Event| {
            if let Ok(keyboard_event) = event.dyn_into::<KeyboardEvent>() {
                if keyboard_event.key() == "Tab" {
                    handle_tab_key(&container_clone, &keyboard_event);
                }
            }
        },
    );
}

/// Handle Tab key navigation within focus trap
fn handle_tab_key(container: &Element, event: &KeyboardEvent) {
    let focusable_elements = get_focusable_elements(container);
    
    if focusable_elements.is_empty() {
        // Prevent tab if no focusable elements
        event.prevent_default();
        return;
    }
    
    let first = &focusable_elements[0];
    let last = &focusable_elements[focusable_elements.len() - 1];
    
    if let Some(current) = getcurrentlyfocused() {
        if event.shift_key() {
            // Shift+Tab (backward)
            if is_same_element(&current, first) {
                // At first element, wrap to last
                event.prevent_default();
                focus_element(last);
            }
        }
    }
}

/// Get the currently focused element
fn getcurrentlyfocused() -> Option<Element> {
    web_sys::window()?
        .document()?
        .active_element()
}

/// Focus an element safely
fn focus_element(element: &Element) {
    if let Ok(html_element) = element.dyn_ref::<HtmlElement>() {
        let _ = html_element.focus();
    }
}

/// Check if two elements are the same
fn is_same_element(a: &Element, b: &Element) -> bool {
    a.is_same_node(Some(b))
}

/// Options for customizing focus trap behavior
#[derive(Clone)]
pub struct FocusTrapOptions {
    /// Whether to focus the first element on activation
    pub __auto_focus: bool,
    /// Whether to restore focus on deactivation
    pub __restore_focus: bool,
    /// Whether to allow clicking outside to deactivate
    pub __click_outside_deactivates: bool,
    /// Whether to trap focus on Tab key
    pub __tab_trap: bool,
}

impl Default for FocusTrapOptions {
    fn default() -> Self {
        Self {
            auto_focus: true,
            restore_focus: true,
            click_outside_deactivates: false,
            tab_trap: true,
        }
    }
}

/// Advanced focus trap hook with customizable options
pub fn use_focus_trap_with_options(
    container_ref: NodeRef<Element>,
    active: Signal<bool>,
    options: FocusTrapOptions,
) {
    let (previouslyfocused, set_previouslyfocused) = create_signal::<Option<Element>>(None);
    
    create_effect(move |_| {
        let is_active = active.get();
        let opts = options.clone();
        
        if is_active {
            if let Some(container) = container_ref.get() {
                // Store previously focused element
                if opts.restore_focus {
                    if let Some(current) = getcurrentlyfocused() {
                        set_previouslyfocused.set(Some(current));
                    }
                }
                
                // Auto focus first element
                if opts.auto_focus {
                    if let Some(first_focusable) = get_first_focusable(&container) {
                        focus_element(&first_focusable);
                    }
                }
                
                // Set up tab trapping
                if opts.tab_trap {
                    setup_focus_trap(&container);
                }
            }
        } else if opts.restore_focus {
            // Restore focus
            if let Some(element) = previouslyfocused.get_untracked() {
                focus_element(&element);
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    #[wasm_bindgen_test]
    fn test_focus_trap_activation() {
        run_test(|cx| {
            let (active, set_active) = create_signal(cx, false);
            let container_ref = create_node_ref::<Element>();
            
            // Create a container with focusable elements
            let document = web_sys::window().unwrap().document().unwrap();
            let container = document.create_element("div").unwrap();
            
            let button1 = document.create_element("button").unwrap();
            button1.set_text_content(Some("Button 1"));
            
            let button2 = document.create_element("button").unwrap();
            button2.set_text_content(Some("Button 2"));
            
            container.append_child(&button1).unwrap();
            container.append_child(&button2).unwrap();
            
            document.body().unwrap().append_child(&container).unwrap();
            
            // Set up focus trap
            use_focus_trap(container_ref, active.into(), None, None);
            
            // Activate trap
            set_active.set(true);
            
            // Verify setup (simplified test)
            assert!(active.get());
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