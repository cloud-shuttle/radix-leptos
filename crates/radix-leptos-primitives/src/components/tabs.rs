use leptos::*;
use leptos::prelude::*;

/// Tabs orientation enum
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TabsOrientation {
    Horizontal,
    Vertical,
}

impl TabsOrientation {
    pub fn as_str(&self) -> &'static str {
        match self {
            TabsOrientation::Horizontal => "horizontal",
            TabsOrientation::Vertical => "vertical",
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

/// Tabs component with proper accessibility and state management
#[component]
pub fn Tabs(
    /// Current selected tab value
    #[prop(optional)]
    value: Option<String>,
    /// Default selected tab value
    #[prop(optional)]
    default_value: Option<String>,
    /// Whether the tabs are disabled
    #[prop(optional, default = false)]
    disabled: bool,
    /// Tabs orientation
    #[prop(optional, default = TabsOrientation::Horizontal)]
    orientation: TabsOrientation,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Change event handler
    #[prop(optional)]
    on_change: Option<Callback<String>>,
    /// Child content (tab list and tab panels)
    children: Children,
) -> impl IntoView {
    let tabs_id = generate_id("tabs");
    
    // Build base classes
    let base_classes = "radix-tabs";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <div
            id=tabs_id
            class=combined_class
            style=style.unwrap_or_default()
            data-orientation=orientation.as_str()
            data-value=value.unwrap_or_default()
            role="tablist"
            aria-orientation=orientation.as_str()
            aria-disabled=disabled
        >
            {children()}
        </div>
    }
}

/// TabsList component for tab navigation
#[component]
pub fn TabsList(
    /// Whether the tabs list is disabled
    #[prop(optional, default = false)]
    disabled: bool,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content (tab triggers)
    children: Children,
) -> impl IntoView {
    let list_id = generate_id("tabs-list");
    
    // Build base classes
    let base_classes = "radix-tabs-list";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <div
            id=list_id
            class=combined_class
            style=style.unwrap_or_default()
            role="tablist"
            aria-disabled=disabled
        >
            {children()}
        </div>
    }
}

/// TabsTrigger component for individual tab buttons
#[component]
pub fn TabsTrigger(
    /// Unique value for this tab
    value: String,
    /// Whether this tab is disabled
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
    /// Child content (tab label)
    children: Children,
) -> impl IntoView {
    let trigger_id = generate_id("tabs-trigger");
    
    // Build base classes
    let base_classes = "radix-tabs-trigger";
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
            role="tab"
            aria-selected="false"
            aria-disabled=disabled
            data-value=value
            data-state="inactive"
            on:keydown=handle_keydown
        >
            {children()}
        </button>
    }
}

/// TabsContent component for tab panel content
#[component]
pub fn TabsContent(
    /// Unique value for this tab panel
    value: String,
    /// Whether this content is disabled
    #[prop(optional, default = false)]
    disabled: bool,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content (tab panel content)
    children: Children,
) -> impl IntoView {
    let content_id = generate_id("tabs-content");
    
    // Build base classes
    let base_classes = "radix-tabs-content";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <div
            id=content_id
            class=combined_class
            style=style.unwrap_or_default()
            role="tabpanel"
            aria-disabled=disabled
            data-value=value
            data-state="inactive"
            tabindex="0"
        >
            {children()}
        </div>
    }
}
