use leptos::*;
use leptos::prelude::*;
use wasm_bindgen::JsCast;

/// Switch state enum
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SwitchState {
    Off,
    On,
}

impl SwitchState {
    pub fn as_str(&self) -> &'static str {
        match self {
            SwitchState::Off => "off",
            SwitchState::On => "on",
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

/// Switch component with proper accessibility and state management
#[component]
pub fn Switch(
    /// Current state of the switch
    #[prop(optional, default = SwitchState::Off)]
    state: SwitchState,
    /// Whether the switch is disabled
    #[prop(optional, default = false)]
    _disabled: bool,
    /// Whether the switch is required
    #[prop(optional, default = false)]
    _required: bool,
    /// Name attribute for form submission
    #[prop(optional)]
    name: Option<String>,
    /// Value attribute for form submission
    #[prop(optional)]
    value: Option<String>,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Change event handler
    #[prop(optional)]
    on_change: Option<Callback<SwitchState>>,
    /// Click event handler
    #[prop(optional)]
    on_click: Option<Callback<web_sys::MouseEvent>>,
    /// Child content (label)
    children: Children,
) -> impl IntoView {
    let _switch_id = generate_id("switch");
    
    // Build base classes
    let base_classes = "radix-switch";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    // Handle switch click
    let handle_click = {
        let state = state;
        let disabled = disabled;
        let on_change = on_change.clone();
        let on_click = on_click.clone();
        
        move |e: web_sys::MouseEvent| {
            if !disabled {
                // Call custom click handler first
                if let Some(on_click) = on_click {
                    on_click.run(e.clone());
                }
                
                // Toggle state and call change handler
                let new_state = match state {
                    SwitchState::Off => SwitchState::On,
                    SwitchState::On => SwitchState::Off,
                };
                
                if let Some(on_change) = on_change {
                    on_change.run(new_state);
                }
            }
        }
    };
    
    // Handle keyboard events
    let handle_keydown = {
        let disabled = disabled;
        
        move |e: web_sys::KeyboardEvent| {
            if !disabled && (e.key() == "Enter" || e.key() == " ") {
                e.prevent_default();
                // Simulate click event
                if let Some(target) = e.target() {
                    if let Ok(element) = target.dyn_into::<web_sys::Element>() {
                        if let Ok(mouse_event) = web_sys::MouseEvent::new("click") {
                            element.dispatch_event(&mouse_event).ok();
                        }
                    }
                }
            }
        }
    };
    
    // Determine ARIA attributes
    let aria_checked = match state {
        SwitchState::Off => Some("false"),
        SwitchState::On => Some("true"),
    };
    
    view! {
        <div
            id=switch_id.clone()
            class=combined_class
            style=style.unwrap_or_default()
            role="switch"
            aria-checked=aria_checked
            aria-required=required
            aria-disabled=disabled
            tabindex=if disabled { None } else { Some(0) }
            data-state=state.as_str()
            on:click=handle_click
            on:keydown=handle_keydown
        >
            <input
                type="checkbox"
                id=format!("{}-input", switch_id)
                name=name.unwrap_or_default()
                value=value.unwrap_or_default()
                checked=state == SwitchState::On
                disabled=disabled
                required=required
                style="position: absolute; opacity: 0; pointer-events: none;"
            />
            <div class="radix-switch-track">
                <div class="radix-switch-thumb">
                    {move || match state {
                        SwitchState::On => view! {
                            <svg width="12" height="12" viewBox="0 0 12 12" fill="none" xmlns="http://www.w3.org/2000/svg">
                                <path d="M9.5 3.5L4.5 8.5L2.5 6.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
                            </svg>
                        },
                        SwitchState::Off => view! {
                            <svg width="12" height="12" viewBox="0 0 12 12" fill="none" xmlns="http://www.w3.org/2000/svg">
                                <rect width="12" height="12" rx="6" fill="none" stroke="currentColor" stroke-width="1.5"/>
                            </svg>
                        }
                    }}
                </div>
            </div>
            {children()}
        </div>
    }
}

/// Switch with label component for easier usage
#[component]
pub fn SwitchWithLabel(
    /// Current state of the switch
    #[prop(optional, default = SwitchState::Off)]
    state: SwitchState,
    /// Whether the switch is disabled
    #[prop(optional, default = false)]
    _disabled: bool,
    /// Whether the switch is required
    #[prop(optional, default = false)]
    _required: bool,
    /// Name attribute for form submission
    #[prop(optional)]
    name: Option<String>,
    /// Value attribute for form submission
    #[prop(optional)]
    value: Option<String>,
    /// CSS classes for the switch
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles for the switch
    #[prop(optional)]
    style: Option<String>,
    /// Child content (label)
    children: Children,
) -> impl IntoView {
    let _switch_id = generate_id("switch");
    
    view! {
        <div style="display: flex; align-items: center; gap: 8px;">
            <Switch
                state=state
                disabled=disabled
                required=required
                name=name.unwrap_or_default()
                value=value.unwrap_or_default()
                class=class.unwrap_or_default()
                style=style.unwrap_or_default()
            >
                <label for=format!("{}-input", switch_id) class="radix-switch-label".to_string()>
                    {children()}
                </label>
            </Switch>
        </div>
    }
}
