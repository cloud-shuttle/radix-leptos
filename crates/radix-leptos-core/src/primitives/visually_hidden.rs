use leptos::*;
use leptos::prelude::*;

/// VisuallyHidden component for content that should only be available to screen readers
/// 
/// This component hides content visually while keeping it accessible to assistive technologies.
/// It's commonly used for:
/// - Additional context for screen readers
/// - Skip links
/// - Form labels that would be redundant visually
/// - Status announcements
/// 
/// # Example
/// 
/// ```rust
/// use leptos::*;
/// use radix_leptos_core::VisuallyHidden;
/// 
/// #[component]
/// fn SearchButton() -> impl IntoView {
///     view! {
///         <button>
///             <svg aria-hidden="true">
///                 // Search icon
///             </svg>
///             <VisuallyHidden>"Search"</VisuallyHidden>
///         </button>
///     }
/// }
/// ```
#[component]
pub fn VisuallyHidden(
    /// Additional CSS classes to apply
    #[prop(optional, into)]
    class: Option<String>,
    /// HTML element to render (defaults to span)
    #[prop(optional, default = "span".to_string(), into)]
    _as_: String,
    /// Content that should be hidden visually
    children: Children,
) -> impl IntoView {
    let visually_hidden_style = "position: absolute; border: 0px; width: 1px; height: 1px; padding: 0px; margin: -1px; overflow: hidden; clip: rect(0px, 0px, 0px, 0px); white-space: nowrap; overflow-wrap: normal;";
    
    let combined_class = match class {
        Some(user_class) => format!("radix-visually-hidden {}", user_class),
        None => "radix-visually-hidden".to_string(),
    };
    
    // Use a single view type to avoid compatibility issues
    view! {
        <span 
            class=combined_class
            style=visually_hidden_style
        >
            {children()}
        </span>
    }
}

/// Hook to get visually hidden styles as a string
pub fn use_visually_hidden_style() -> &'static str {
    "position: absolute; border: 0px; width: 1px; height: 1px; padding: 0px; margin: -1px; overflow: hidden; clip: rect(0px, 0px, 0px, 0px); white-space: nowrap; overflow-wrap: normal;"
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    #[wasm_bindgen_test]
    fn test_visually_hidden_rendering() {
        run_test(|cx| {
            let rendered = leptos::ssr::render_to_string(cx, move || {
                view! {
                    <VisuallyHidden>
                        "Hidden content"
                    </VisuallyHidden>
                }
            });
            
            // Should contain the hidden content
            assert!(rendered.contains("Hidden content"));
            // Should have visually hidden class
            assert!(rendered.contains("radix-visually-hidden"));
            // Should have proper styling
            assert!(rendered.contains("position: absolute"));
            assert!(rendered.contains("width: 1px"));
            assert!(rendered.contains("height: 1px"));
        });
    }
    
    #[wasm_bindgen_test]
    fn test_custom_element_type() {
        run_test(|cx| {
            let rendered = leptos::ssr::render_to_string(cx, move || {
                view! {
                    <VisuallyHidden as_="div">
                        "Hidden div content"
                    </VisuallyHidden>
                }
            });
            
            assert!(rendered.contains("<div"));
            assert!(rendered.contains("Hidden div content"));
        });
    }
    
    #[wasm_bindgen_test]
    fn test_custom_class() {
        run_test(|cx| {
            let rendered = leptos::ssr::render_to_string(cx, move || {
                view! {
                    <VisuallyHidden class="custom-hidden">
                        "Custom class content"
                    </VisuallyHidden>
                }
            });
            
            assert!(rendered.contains("radix-visually-hidden custom-hidden"));
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