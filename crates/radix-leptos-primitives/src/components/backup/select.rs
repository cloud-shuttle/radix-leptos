use leptos::*;
use leptos::prelude::*;

/// Select position enum
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SelectPosition {
    Item,
    Popper,
}

impl SelectPosition {
    pub fn as_str(&self) -> &'static str {
        match self {
            SelectPosition::Item => "item",
            SelectPosition::Popper => "popper",
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

/// Select component with proper accessibility and dropdown functionality
#[component]
pub fn Select(
    /// Current selected value
    #[prop(optional)]
    value: Option<String>,
    /// Default selected value
    #[prop(optional)]
    default_value: Option<String>,
    /// Whether the select is disabled
    #[prop(optional, default = false)]
    _disabled: bool,
    /// Whether the select is required
    #[prop(optional, default = false)]
    _required: bool,
    /// Name attribute for form submission
    #[prop(optional)]
    name: Option<String>,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Change event handler
    #[prop(optional)]
    on_change: Option<Callback<String>>,
    /// Child content (trigger, content, and items)
    children: Children,
) -> impl IntoView {
    let _select_id = generate_id("select");
    
    // Build base classes
    let base_classes = "radix-select";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <div
            id=select_id
            class=combined_class
            style=style.unwrap_or_default()
            data-value=value.unwrap_or_default()
            data-disabled=disabled
            data-required=required
            aria-disabled=disabled
        >
            {children()}
        </div>
    }
}

/// SelectTrigger component for the select button
#[component]
pub fn SelectTrigger(
    /// Whether the trigger is disabled
    #[prop(optional, default = false)]
    _disabled: bool,
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
    let _trigger_id = generate_id("select-trigger");
    
    // Build base classes
    let base_classes = "radix-select-trigger";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    // Handle keyboard events
    let handle_keydown = {
        let disabled = disabled;
        let on_click = on_click.clone();
        
        move |e: web_sys::KeyboardEvent| {
            if !disabled && (e.key() == "Enter" || e.key() == " " || e.key() == "ArrowDown") {
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
            role="combobox"
            aria-expanded="false"
            aria-haspopup="listbox"
            aria-autocomplete="none"
            aria-disabled=disabled
            on:keydown=handle_keydown
        >
            {children()}
            <svg 
                class="radix-select-chevron"
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

/// SelectValue component for displaying the selected value
#[component]
pub fn SelectValue(
    /// Placeholder text when no value is selected
    #[prop(optional)]
    placeholder: Option<String>,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content (value display)
    children: Children,
) -> impl IntoView {
    let _value_id = generate_id("select-value");
    
    // Build base classes
    let base_classes = "radix-select-value";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <span
            id=value_id
            class=combined_class
            style=style.unwrap_or_default()
            data-placeholder=placeholder.unwrap_or_default()
        >
            {children()}
        </span>
    }
}

/// SelectContent component for the dropdown content
#[component]
pub fn SelectContent(
    /// Whether the content is disabled
    #[prop(optional, default = false)]
    _disabled: bool,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content (select items)
    children: Children,
) -> impl IntoView {
    let _content_id = generate_id("select-content");
    
    // Build base classes
    let base_classes = "radix-select-content";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <div
            id=content_id
            class=combined_class
            style=style.unwrap_or_default()
            role="listbox"
            aria-disabled=disabled
            data-state="closed"
            tabindex="-1"
        >
            {children()}
        </div>
    }
}

/// SelectItem component for individual select options
#[component]
pub fn SelectItem(
    /// Unique value for this item
    value: String,
    /// Whether this item is disabled
    #[prop(optional, default = false)]
    _disabled: bool,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Click event handler
    #[prop(optional)]
    on_click: Option<Callback<web_sys::MouseEvent>>,
    /// Child content (item content)
    children: Children,
) -> impl IntoView {
    let _item_id = generate_id("select-item");
    
    // Build base classes
    let base_classes = "radix-select-item";
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
        <div
            id=item_id
            class=combined_class
            style=style.unwrap_or_default()
            role="option"
            aria-selected="false"
            aria-disabled=disabled
            data-value=value
            data-state="inactive"
            tabindex="-1"
            on:keydown=handle_keydown
        >
            {children()}
        </div>
    }
}

/// SelectSeparator component for visual separation
#[component]
pub fn SelectSeparator(
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
) -> impl IntoView {
    let _separator_id = generate_id("select-separator");
    
    // Build base classes
    let base_classes = "radix-select-separator";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <div
            id=separator_id
            class=combined_class
            style=style.unwrap_or_default()
            role="separator"
        />
    }
}

/// SelectGroup component for grouping related items
#[component]
pub fn SelectGroup(
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content (group items)
    children: Children,
) -> impl IntoView {
    let _group_id = generate_id("select-group");
    
    // Build base classes
    let base_classes = "radix-select-group";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <div
            id=group_id
            class=combined_class
            style=style.unwrap_or_default()
            role="group"
        >
            {children()}
        </div>
    }
}

/// SelectLabel component for group labels
#[component]
pub fn SelectLabel(
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content (label content)
    children: Children,
) -> impl IntoView {
    let _label_id = generate_id("select-label");
    
    // Build base classes
    let base_classes = "radix-select-label";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <div
            id=label_id
            class=combined_class
            style=style.unwrap_or_default()
        >
            {children()}
        </div>
    }
}
