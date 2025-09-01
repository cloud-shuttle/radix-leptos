use leptos::*;
use leptos::prelude::*;

/// Popover side enum
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PopoverSide {
    Top,
    Right,
    Bottom,
    Left,
}

impl PopoverSide {
    pub fn as_str(&self) -> &'static str {
        match self {
            PopoverSide::Top => "top",
            PopoverSide::Right => "right",
            PopoverSide::Bottom => "bottom",
            PopoverSide::Left => "left",
        }
    }
}

/// Popover alignment enum
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PopoverAlignment {
    Start,
    Center,
    End,
}

impl PopoverAlignment {
    pub fn as_str(&self) -> &'static str {
        match self {
            PopoverAlignment::Start => "start",
            PopoverAlignment::Center => "center",
            PopoverAlignment::End => "end",
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

/// Popover component with proper accessibility and positioning
#[component]
pub fn Popover(
    /// Whether the popover is open
    #[prop(optional, default = false)]
    open: bool,
    /// Whether the popover is disabled
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
    let popover_id = generate_id("popover");
    
    // Build base classes
    let base_classes = "radix-popover";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <div
            id=popover_id
            class=combined_class
            style=style.unwrap_or_default()
            data-state=if open { "open" } else { "closed" }
            aria-disabled=disabled
        >
            {children()}
        </div>
    }
}

/// PopoverTrigger component for triggering the popover
#[component]
pub fn PopoverTrigger(
    /// Whether the trigger is disabled
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
    let trigger_id = generate_id("popover-trigger");
    
    // Build base classes
    let base_classes = "radix-popover-trigger";
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
            aria-haspopup="dialog"
            aria-expanded="false"
            aria-disabled=disabled
            on:keydown=handle_keydown
        >
            {children()}
        </button>
    }
}

/// PopoverContent component for popover content
#[component]
pub fn PopoverContent(
    /// Popover side
    #[prop(optional, default = PopoverSide::Bottom)]
    side: PopoverSide,
    /// Popover alignment
    #[prop(optional, default = PopoverAlignment::Center)]
    align: PopoverAlignment,
    /// Whether the popover is disabled
    #[prop(optional, default = false)]
    disabled: bool,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content (popover content)
    children: Children,
) -> impl IntoView {
    let content_id = generate_id("popover-content");
    
    // Build base classes
    let base_classes = "radix-popover-content";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <div
            id=content_id
            class=combined_class
            style=style.unwrap_or_default()
            role="dialog"
            aria-modal="true"
            aria-disabled=disabled
            data-side=side.as_str()
            data-align=align.as_str()
            data-state="closed"
            tabindex="-1"
        >
            {children()}
        </div>
    }
}

/// PopoverClose component for closing the popover
#[component]
pub fn PopoverClose(
    /// Whether the close button is disabled
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
    /// Child content (close button content)
    children: Children,
) -> impl IntoView {
    let close_id = generate_id("popover-close");
    
    // Build base classes
    let base_classes = "radix-popover-close";
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
            id=close_id
            class=combined_class
            style=style.unwrap_or_default()
            disabled=disabled
            type="button"
            aria-label="Close popover"
            aria-disabled=disabled
            on:keydown=handle_keydown
        >
            {children()}
        </button>
    }
}

/// PopoverArrow component for visual indicator
#[component]
pub fn PopoverArrow(
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
) -> impl IntoView {
    let arrow_id = generate_id("popover-arrow");
    
    // Build base classes
    let base_classes = "radix-popover-arrow";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <svg
            id=arrow_id
            class=combined_class
            style=style.unwrap_or_default()
            width="10"
            height="5"
            viewBox="0 0 10 5"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
        >
            <path
                d="M0 0L5 5L10 0"
                fill="currentColor"
            />
        </svg>
    }
}
