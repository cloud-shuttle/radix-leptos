use leptos::*;
use leptos::prelude::*;
use wasm_bindgen::JsCast;

/// Accordion type enum
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AccordionType {
    Single,
    Multiple,
}

impl AccordionType {
    pub fn as_str(&self) -> &'static str {
        match self {
            AccordionType::Single => "single",
            AccordionType::Multiple => "multiple",
        }
    }
}

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

/// Accordion component with proper accessibility and state management
#[component]
pub fn Accordion(
    /// Accordion type (single or multiple)
    #[prop(optional, default = AccordionType::Single)]
    accordion_type: AccordionType,
    /// Whether the accordion is disabled
    #[prop(optional, default = false)]
    disabled: bool,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content (accordion items)
    children: Children,
) -> impl IntoView {
    let accordion_id = generate_id("accordion");
    
    // Build base classes
    let base_classes = "radix-accordion";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <div
            id=accordion_id
            class=combined_class
            style=style.unwrap_or_default()
            data-type=accordion_type.as_str()
            role="region"
            aria-disabled=disabled
        >
            {children()}
        </div>
    }
}

/// AccordionItem component for individual accordion sections
#[component]
pub fn AccordionItem(
    /// Unique value for this accordion item
    value: String,
    /// Whether this item is disabled
    #[prop(optional, default = false)]
    disabled: bool,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content (trigger and content)
    children: Children,
) -> impl IntoView {
    let item_id = generate_id("accordion-item");
    
    // Build base classes
    let base_classes = "radix-accordion-item";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <div
            id=item_id
            class=combined_class
            style=style.unwrap_or_default()
            data-state="closed"
            data-disabled=disabled
        >
            {children()}
        </div>
    }
}

/// AccordionTrigger component for expandable/collapsible headers
#[component]
pub fn AccordionTrigger(
    /// Whether this trigger is disabled
    #[prop(optional, default = false)]
    disabled: bool,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Click event handler
    #[prop(optional)]
    on_click: Option<Callback<web_sys::MouseEvent>>,
    /// Child content (trigger content)
    children: Children,
) -> impl IntoView {
    let trigger_id = generate_id("accordion-trigger");
    
    // Build base classes
    let base_classes = "radix-accordion-trigger";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    // Handle keyboard events
    let handle_keydown = {
        let disabled = disabled;
        let on_click = on_click.clone();
        
        move |e: web_sys::KeyboardEvent| {
            if !disabled && (e.key() == "Enter" || e.key() == " ") {
                e.prevent_default();
                if let Some(on_click) = on_click {
                    // Create a synthetic mouse event
                    if let Ok(mouse_event) = web_sys::MouseEvent::new("click") {
                        on_click.run(mouse_event);
                    }
                }
            }
        }
    };
    
    view! {
        <button
            id=trigger_id
            class=combined_class
            style=style.unwrap_or_default()
            disabled=disabled
            type="button"
            aria-expanded="false"
            aria-disabled=disabled

            on:keydown=handle_keydown
        >
            {children()}
            <svg 
                class="radix-accordion-chevron"
                width="16" 
                height="16" 
                viewBox="0 0 16 16" 
                fill="none" 
                xmlns="http://www.w3.org/2000/svg"
            >
                <path 
                    d="M6 4L10 8L6 12" 
                    stroke="currentColor" 
                    stroke-width="1.5" 
                    stroke-linecap="round" 
                    stroke-linejoin="round"
                />
            </svg>
        </button>
    }
}

/// AccordionContent component for expandable content
#[component]
pub fn AccordionContent(
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content (accordion content)
    children: Children,
) -> impl IntoView {
    let content_id = generate_id("accordion-content");
    
    // Build base classes
    let base_classes = "radix-accordion-content";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <div
            id=content_id
            class=combined_class
            style=style.unwrap_or_default()
            data-state="closed"
            aria-hidden="true"
        >
            <div class="radix-accordion-content-inner">
                {children()}
            </div>
        </div>
    }
}

/// AccordionHeader component for semantic structure
#[component]
pub fn AccordionHeader(
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content (header content)
    children: Children,
) -> impl IntoView {
    let header_id = generate_id("accordion-header");
    
    // Build base classes
    let base_classes = "radix-accordion-header";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <h3
            id=header_id
            class=combined_class
            style=style.unwrap_or_default()
        >
            {children()}
        </h3>
    }
}
