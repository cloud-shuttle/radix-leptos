use leptos::*;
use web_sys::Element;
use wasm_bindgen::JsCast;
use std::collections::HashMap;

/// Slot component for merging props with child elements (equivalent to Radix's `asChild`)
/// 
/// The Slot component allows you to merge props from a parent component with its child element,
/// enabling flexible component composition while maintaining proper prop forwarding and event handling.
/// This is essential for the `as_child` pattern used throughout Radix components.
/// 
/// # Example
/// 
/// ```rust
/// use leptos::*;
/// use radix_leptos_core::Slot;
/// 
/// #[component]
/// fn DialogTrigger(
///     #[prop(optional)] _as_child: bool,
///     #[prop(optional)] class: Option<String>,
///     children: Children,
/// ) -> impl IntoView {
///     if as_child {
///         view! {
///             <Slot class=class>
///                 {children()}
///             </Slot>
///         }
///     } else {
///         view! {
///             <button class=class>
///                 {children()}
///             </button>
///         }
///     }
/// }
/// ```
/// Properties that can be merged with slotted elements
#[derive(Clone)]
pub struct SlotMergeProps {
    pub class: Option<String>,
    pub style: Option<String>,
    pub on_click: Option<Box<dyn Fn(web_sys::MouseEvent)>>,
    pub on_key_down: Option<Box<dyn Fn(web_sys::KeyboardEvent)>>,
    pub on_focus: Option<Box<dyn Fn(web_sys::FocusEvent)>>,
    pub on_blur: Option<Box<dyn Fn(web_sys::FocusEvent)>>,
    pub attrs: HashMap<String, String>,
}

#[component]
pub fn Slot(
    /// Additional CSS classes to merge
    #[prop(optional, into)]
    class: Option<String>,
    /// Additional styles to merge  
    #[prop(optional, into)]
    style: Option<String>,
    /// Click event handler to merge
    #[prop(optional)]
    on_click: Option<Box<dyn Fn(web_sys::MouseEvent)>>,
    /// Key down event handler to merge
    #[prop(optional)]
    on_key_down: Option<Box<dyn Fn(web_sys::KeyboardEvent)>>,
    /// Focus event handler to merge
    #[prop(optional)]
    on_focus: Option<Box<dyn Fn(web_sys::FocusEvent)>>,
    /// Blur event handler to merge
    #[prop(optional)]
    on_blur: Option<Box<dyn Fn(web_sys::FocusEvent)>>,
    /// Additional attributes to merge
    #[prop(optional)]
    attrs: Option<HashMap<String, String>>,
    /// Child elements to merge props with
    children: Children,
) -> impl IntoView {
    // Create props to merge with child
    let slot_props = SlotMergeProps {
        class,
        style,
        on_click,
        on_key_down,
        on_focus,
        on_blur,
        attrs: attrs.unwrap_or_default(),
    };
    
    // Store props in context for child components to access
    provide_context(slot_props);
    
    children()
}

/// Trait for components that can be slotted
pub trait Slottable {
    /// Apply slot props to this component
    fn with_slot_props(self, props: SlotMergeProps) -> Self;
}

/// Hook to get slot props from context
pub fn use_slot_props() -> Option<SlotMergeProps> {
    use_context::<SlotMergeProps>()
}

/// Utility to merge CSS classes
pub fn merge_classes(existing: Option<&str>, additional: Option<&str>) -> Option<String> {
    match (existing, additional) {
        (Some(existing), Some(additional)) => {
            Some(format!("{} {}", existing, additional))
        }
        (Some(existing), None) => Some(existing.to_string()),
        (None, Some(additional)) => Some(additional.to_string()),
        (None, None) => None,
    }
}

/// Utility to merge styles
pub fn merge_styles(existing: Option<&str>, additional: Option<&str>) -> Option<String> {
    match (existing, additional) {
        (Some(existing), Some(additional)) => {
            // Ensure both styles end with semicolons for proper merging
            let existing = if existing.ends_with(';') { existing } else { &format!("{};", existing) };
            let additional = if additional.ends_with(';') { additional } else { &format!("{};", additional) };
            Some(format!("{} {}", existing, additional))
        }
        (Some(existing), None) => Some(existing.to_string()),
        (None, Some(additional)) => Some(additional.to_string()),
        (None, None) => None,
    }
}

/// Utility to merge event handlers
pub fn compose_event_handlers<T, F1, F2>(
    handler1: Option<F1>,
    handler2: Option<F2>,
) -> Option<Box<dyn Fn(T)>>
where
    T: Clone + 'static,
    F1: Fn(T) + 'static,
    F2: Fn(T) + 'static,
{
    match (handler1, handler2) {
        (Some(h1), Some(h2)) => {
            Some(Box::new(move |event: T| {
                h1(event.clone());
                h2(event);
            }))
        }
        (Some(h1), None) => Some(Box::new(h1)),
        (None, Some(h2)) => Some(Box::new(h2)),
        (None, None) => None,
    }
}

/// Component that can receive and merge slot props
#[component]
pub fn Slottable<F>(
    render: F,
) -> impl IntoView
where
    F: Fn(Option<SlotProps>) -> View + 'static,
{
    let slot_props = use_slot_props();
    render(slot_props)
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    #[test]
    fn test_merge_classes() {
        assert_eq!(
            merge_classes(Some("btn"), Some("btn-primary")),
            Some("btn btn-primary".to_string())
        );
        
        assert_eq!(
            merge_classes(None, Some("btn-primary")),
            Some("btn-primary".to_string())
        );
        
        assert_eq!(
            merge_classes(Some("btn"), None),
            Some("btn".to_string())
        );
        
        assert_eq!(merge_classes(None, None), None);
    }
    
    #[test]
    fn test_merge_styles() {
        assert_eq!(
            merge_styles(Some("color: red"), Some("font-size: 14px")),
            Some("color: red; font-size: 14px;".to_string())
        );
        
        assert_eq!(
            merge_styles(Some("color: red;"), Some("font-size: 14px;")),
            Some("color: red; font-size: 14px;".to_string())
        );
        
        assert_eq!(
            merge_styles(None, Some("color: blue")),
            Some("color: blue".to_string())
        );
    }
    
    #[wasm_bindgen_test]
    fn test_slot_context() {
        run_test(|cx| {
            let mut found_props = false;
            
            let _view = view! { cx,
                <Slot class="test-class">
                    <Slottable render=move |props| {
                        if let Some(props) = props {
                            found_props = props.class == Some("test-class".to_string());
                        }
                        view! { cx, <div>"Test"</div> }
                    } />
                </Slot>
            };
            
            assert!(found_props);
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