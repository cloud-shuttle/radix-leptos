use leptos::*;
use leptos::prelude::*;
use wasm_bindgen::JsCast;

/// Label component with proper form control association
///
/// The Label component provides accessible form labeling with automatic
/// association to form controls, improving usability and accessibility.
///
/// # Features
/// - Automatic form control association via `for` attribute
/// - Click handling to focus associated controls
/// - Proper ARIA attributes
/// - Screen reader support
/// - Flexible styling options
///
/// # Example
///
/// ```rust
/// use leptos::*;
/// use radix_leptos_primitives::*;
///
/// #[component]
/// fn FormField() -> impl IntoView {
///     let input_id = "email-input";
///     
///     view! {
///         <div>
///             <Label for_control=input_id class="form-label">
///                 "Email Address"
///             </Label>
///             <input id=input_id type="email" class="form-input" />
///         </div>
///     }
/// }
/// ```

/// Generate a simple unique ID for components
fn generate_id(prefix: &str) -> String {
    static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let id = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    format!("{}-{}", prefix, id)
}

/// Merge CSS classes
fn merge_classes(existing: Option<&str>, additional: Option<&str>) -> Option<String> {
    match (existing, additional) {
        (Some(a), Some(b)) => Some(format!("{} {}", a, b)),
        (Some(a), None) => Some(a.to_string()),
        (None, Some(b)) => Some(b.to_string()),
        (None, None) => None,
    }
}

/// Label component for form control association
#[component]
pub fn Label(
    /// ID of the form control this label is associated with
    #[prop(optional)]
    for_control: Option<String>,
    /// Whether to render as child element instead of label
    #[prop(optional, default = false)]
    __as_child: bool,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Click event handler
    #[prop(optional)]
    on_click: Option<Callback<web_sys::MouseEvent>>,
    /// Child content
    children: Children,
) -> impl IntoView {
    let _label_id = generate_id("label");
    
    // Build base classes
    let base_classes = "radix-label";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    // Handle click events - focus associated control if available
    let handle_click = {
        let for_control = for_control.clone();
        move |e: web_sys::MouseEvent| {
            // Call custom click handler first
            if let Some(on_click) = on_click {
                on_click.run(e.clone());
            }
            
            // Auto-focus associated control
            if let Some(control_id) = &for_control {
                if let Some(window) = web_sys::window() {
                    if let Some(document) = window.document() {
                        if let Some(control) = document.get_element_by_id(control_id) {
                            if let Ok(html_element) = control.dyn_into::<web_sys::HtmlElement>() {
                                let _ = html_element.focus();
                            }
                        }
                    }
                }
            }
        }
    };
    
    view! {
        <label
            id=label_id
            class=combined_class
            style=style
            for=for_control.clone()
            on:click=handle_click
        >
            {children()}
        </label>
    }
}