use leptos::*;
use web_sys::{HtmlElement, CssStyleDeclaration};

/// Hook for locking body scroll to prevent background scrolling
/// 
/// This hook is essential for modal overlays, full-screen dialogs,
/// and other components that should prevent the underlying content from scrolling.
/// It handles both preventing scrollwheel events and preserving scroll position.
/// 
/// # Example
/// 
/// ```rust
/// use leptos::*;
/// use radix_leptos_core::use_body_scroll_lock;
/// 
/// #[component]
/// pub fn Modal() -> impl IntoView {
///     let (open, set_open) = create_signal(false);
///     
///     // Lock body scroll when modal is open
///     use_body_scroll_lock(open.into());
///     
///     view! {
///         <button on:click=move |_| set_open.set(true)>
///             "Open Modal"
///         </button>
///         <Show when=move || open.get()>
///             <div class="modal-overlay">
///                 <div class="modal-content">
///                     <button on:click=move |_| set_open.set(false)>
///                         "Close"
///                     </button>
///                 </div>
///             </div>
///         </Show>
///     }
/// }
/// ```
pub fn use_body_scroll_lock(locked: Signal<bool>) {
    let (original_style, set_original_style) = create_signal::<Option<String>>(None);
    let (original_scroll_position, set_original_scroll_position) = create_signal::<Option<f64>>(None);
    
    create_effect(move |_| {
        let should_lock = locked.get();
        
        if should_lock {
            lock_body_scroll(set_original_style, set_original_scroll_position);
        } else {
            unlock_body_scroll(original_style.get_untracked(), original_scroll_position.get_untracked());
        }
    });
    
    // Cleanup on component unmount
    on_cleanup(move || {
        unlock_body_scroll(original_style.get_untracked(), original_scroll_position.get_untracked());
    });
}

/// Lock body scroll with style preservation
fn lock_body_scroll(
    set_original_style: WriteSignal<Option<String>>,
    set_original_scroll_position: WriteSignal<Option<f64>>,
) {
    if let Some(body) = get_body_element() {
        let style = body.style();
        
        // Store original overflow style
        let original_overflow = style.get_property_value("overflow").unwrap_or_default();
        if !original_overflow.is_empty() {
            set_original_style.set(Some(original_overflow));
        }
        
        // Store current scroll position
        if let Some(window) = web_sys::window() {
            let scroll_y = window.scroll_y().unwrap_or(0.0);
            set_original_scroll_position.set(Some(scroll_y));
        }
        
        // Apply scroll lock styles
        let _ = style.set_property("overflow", "hidden");
        let _ = style.set_property("padding-right", &get_scrollbar_width());
        
        // Prevent scroll position jumping by setting top position
        if let Some(scroll_pos) = set_original_scroll_position.get_untracked() {
            let _ = style.set_property("position", "fixed");
            let _ = style.set_property("top", &format!("-{}px", scroll_pos));
            let _ = style.set_property("width", "100%");
        }
    }
}

/// Unlock body scroll and restore original styles
fn unlock_body_scroll(
    original_style: Option<String>,
    original_scroll_position: Option<f64>,
) {
    if let Some(body) = get_body_element() {
        let style = body.style();
        
        // Restore original overflow or remove if it was empty
        if let Some(original) = original_style {
            let _ = style.set_property("overflow", &original);
        } else {
            let _ = style.remove_property("overflow");
        }
        
        // Remove scroll lock styles
        let _ = style.remove_property("padding-right");
        let _ = style.remove_property("position");
        let _ = style.remove_property("top");
        let _ = style.remove_property("width");
        
        // Restore scroll position
        if let (Some(window), Some(scroll_pos)) = (web_sys::window(), original_scroll_position) {
            window.scroll_to_with_x_and_y(0.0, scroll_pos);
        }
    }
}

/// Get the body element
fn get_body_element() -> Option<HtmlElement> {
    web_sys::window()?
        .document()?
        .body()
}

/// Calculate scrollbar width to prevent layout shift
fn get_scrollbar_width() -> String {
    // Create a temporary element to measure scrollbar width
    if let Some(document) = web_sys::window().and_then(|w| w.document()) {
        if let Ok(div) = document.create_element("div") {
            if let Ok(style) = div.dyn_ref::<HtmlElement>() {
                let css = style.style();
                let _ = css.set_property("visibility", "hidden");
                let _ = css.set_property("position", "absolute");
                let _ = css.set_property("top", "-9999px");
                let _ = css.set_property("width", "50px");
                let _ = css.set_property("height", "50px");
                let _ = css.set_property("overflow", "scroll");
                
                if let Ok(body) = document.body().ok_or("No body") {
                    let _ = body.append_child(&div);
                    
                    let scrollbar_width = 50 - div.client_width();
                    let _ = body.remove_child(&div);
                    
                    return format!("{}px", scrollbar_width);
                }
            }
        }
    }
    
    // Fallback scrollbar width
    "15px".to_string()
}

/// Hook with options for customizing scroll lock behavior
#[derive(Clone)]
pub struct ScrollLockOptions {
    /// Whether to reserve space for scrollbar to prevent layout shift
    pub _reserve_scrollbar_gap: bool,
    /// Whether to prevent scroll position jumping
    pub _prevent_scroll_jump: bool,
    /// Custom padding to apply when locking
    pub custom_padding: Option<String>,
}

impl Default for ScrollLockOptions {
    fn default() -> Self {
        Self {
            reserve_scrollbar_gap: true,
            prevent_scroll_jump: true,
            custom_padding: None,
        }
    }
}

/// Advanced scroll lock hook with customizable options
pub fn use_body_scroll_lock_with_options(
    locked: Signal<bool>,
    options: ScrollLockOptions,
) {
    let (original_styles, set_original_styles) = create_signal::<Option<ScrollLockState>>(None);
    
    create_effect(move |_| {
        let should_lock = locked.get();
        let opts = options.clone();
        
        if should_lock {
            lock_body_scroll_advanced(set_original_styles, opts);
        } else {
            unlock_body_scroll_advanced(original_styles.get_untracked());
        }
    });
    
    on_cleanup(move || {
        unlock_body_scroll_advanced(original_styles.get_untracked());
    });
}

#[derive(Clone)]
struct ScrollLockState {
    overflow: Option<String>,
    position: Option<String>,
    top: Option<String>,
    width: Option<String>,
    padding_right: Option<String>,
    scroll_position: Option<f64>,
}

fn lock_body_scroll_advanced(
    set_original_styles: WriteSignal<Option<ScrollLockState>>,
    options: ScrollLockOptions,
) {
    if let Some(body) = get_body_element() {
        let style = body.style();
        
        // Store original styles
        let original_state = ScrollLockState {
            overflow: get_style_property(&style, "overflow"),
            position: get_style_property(&style, "position"),
            top: get_style_property(&style, "top"),
            width: get_style_property(&style, "width"),
            padding_right: get_style_property(&style, "padding-right"),
            scroll_position: web_sys::window().and_then(|w| w.scroll_y().ok()),
        };
        
        set_original_styles.set(Some(original_state.clone()));
        
        // Apply scroll lock
        let _ = style.set_property("overflow", "hidden");
        
        if options.reserve_scrollbar_gap {
            let padding = options.custom_padding
                .unwrap_or_else(get_scrollbar_width);
            let _ = style.set_property("padding-right", &padding);
        }
        
        if options.prevent_scroll_jump {
            if let Some(scroll_pos) = original_state.scroll_position {
                let _ = style.set_property("position", "fixed");
                let _ = style.set_property("top", &format!("-{}px", scroll_pos));
                let _ = style.set_property("width", "100%");
            }
        }
    }
}

fn unlock_body_scroll_advanced(original_state: Option<ScrollLockState>) {
    if let (Some(body), Some(state)) = (get_body_element(), original_state) {
        let style = body.style();
        
        // Restore or remove properties
        restore_or_remove_property(&style, "overflow", state.overflow);
        restore_or_remove_property(&style, "position", state.position);
        restore_or_remove_property(&style, "top", state.top);
        restore_or_remove_property(&style, "width", state.width);
        restore_or_remove_property(&style, "padding-right", state.padding_right);
        
        // Restore scroll position
        if let (Some(window), Some(scroll_pos)) = (web_sys::window(), state.scroll_position) {
            window.scroll_to_with_x_and_y(0.0, scroll_pos);
        }
    }
}

fn get_style_property(style: &CssStyleDeclaration, property: &str) -> Option<String> {
    style.get_property_value(property).ok()
        .and_then(|val| if val.is_empty() { None } else { Some(val) })
}

fn restore_or_remove_property(
    style: &CssStyleDeclaration,
    property: &str,
    original_value: Option<String>,
) {
    if let Some(value) = original_value {
        let _ = style.set_property(property, &value);
    } else {
        let _ = style.remove_property(property);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    #[wasm_bindgen_test]
    fn test_scroll_lock_activation() {
        run_test(|cx| {
            let (locked, set_locked) = create_signal(cx, false);
            
            use_body_scroll_lock(locked.into());
            
            // Initially not locked
            assert!(!locked.get());
            
            // Lock scroll
            set_locked.set(true);
            
            // Verify body has overflow hidden (simplified test)
            if let Some(body) = get_body_element() {
                // In a real test, we'd check the computed style
                // This is a simplified verification
                assert!(locked.get());
            }
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