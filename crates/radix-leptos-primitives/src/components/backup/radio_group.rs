use leptos::*;
use leptos::prelude::*;

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

/// RadioGroup component with proper accessibility and state management
#[component]
pub fn RadioGroup(
    /// Current selected value
    #[prop(optional)]
    value: Option<String>,
    /// Whether the radio group is disabled
    #[prop(optional, default = false)]
    disabled: bool,
    /// Whether the radio group is required
    #[prop(optional, default = false)]
    required: bool,

    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Change event handler
    #[prop(optional)]
    on_change: Option<Callback<String>>,
    /// Child content (radio options)
    children: Children,
) -> impl IntoView {
    let radio_group_id = generate_id("radio-group");
    
    // Build base classes
    let base_classes = "radix-radio-group";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <div
            id=radio_group_id.clone()
            class=combined_class
            style=style.unwrap_or_default()
            role="radiogroup"
            aria-required=required
            aria-disabled=disabled
            data-value=value.unwrap_or_default()
        >
            {children()}
        </div>
    }
}

/// RadioGroupItem component for individual radio options
#[component]
pub fn RadioGroupItem(
    /// Value of this radio option
    value: String,
    /// Whether this radio option is disabled
    #[prop(optional, default = false)]
    disabled: bool,
    /// Whether this radio option is required
    #[prop(optional, default = false)]
    required: bool,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Change event handler
    #[prop(optional)]
    on_change: Option<Callback<String>>,
    /// Child content (label)
    children: Children,
) -> impl IntoView {
    let radio_id = generate_id("radio");
    
    // Build base classes
    let base_classes = "radix-radio-group-item";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    // Handle radio click
    let handle_click = {
        let value = value.clone();
        let disabled = disabled;
        let on_change = on_change.clone();
        
        move |_e: web_sys::MouseEvent| {
            if !disabled {
                if let Some(on_change) = on_change {
                    on_change.run(value.clone());
                }
            }
        }
    };
    
    // Handle keyboard events
    let handle_keydown = {
        let disabled = disabled;
        let value = value.clone();
        let on_change = on_change.clone();
        
        move |e: web_sys::KeyboardEvent| {
            if !disabled && (e.key() == "Enter" || e.key() == " ") {
                e.prevent_default();
                if let Some(on_change) = on_change {
                    on_change.run(value.clone());
                }
            }
        }
    };
    
    view! {
        <div
            id=radio_id.clone()
            class=combined_class
            style=style.unwrap_or_default()
            role="radio"
            aria-checked="false"
            aria-required=required
            aria-disabled=disabled
            tabindex=if disabled { None } else { Some(0) }
            data-value=value.clone()
            on:click=handle_click
            on:keydown=handle_keydown
        >
            <input
                type="radio"
                id=format!("{}-input", radio_id)
                value=value.clone()
                disabled=disabled
                required=required
                style="position: absolute; opacity: 0; pointer-events: none;"
            />
            <div class="radix-radio-group-indicator">
                <div class="radix-radio-group-dot"></div>
            </div>
            {children()}
        </div>
    }
}

/// RadioGroupItem with label component for easier usage
#[component]
pub fn RadioGroupItemWithLabel(
    /// Value of this radio option
    value: String,
    /// Whether this radio option is disabled
    #[prop(optional, default = false)]
    disabled: bool,
    /// Whether this radio option is required
    #[prop(optional, default = false)]
    required: bool,
    /// CSS classes for the radio item
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles for the radio item
    #[prop(optional)]
    style: Option<String>,
    /// Child content (label)
    children: Children,
) -> impl IntoView {
    let radio_id = generate_id("radio");
    
    view! {
        <div style="display: flex; align-items: center; gap: 8px;">
            <RadioGroupItem
                value=value
                disabled=disabled
                required=required
                class=class.unwrap_or_default()
                style=style.unwrap_or_default()
            >
                <label for=format!("{}-input", radio_id) class="radix-radio-group-label".to_string()>
                    {children()}
                </label>
            </RadioGroupItem>
        </div>
    }
}
