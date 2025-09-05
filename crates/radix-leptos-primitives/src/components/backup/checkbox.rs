use wasm_bindgen::JsCast;

/// Checkbox state enum
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CheckboxState {
    Unchecked,
    Checked,
    Indeterminate,
}

impl CheckboxState {
    pub fn as_str(&self) -> &'static str {
        match self {
            CheckboxState::Unchecked => "unchecked",
            CheckboxState::Checked => "checked",
            CheckboxState::Indeterminate => "indeterminate",
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

/// Checkbox component with proper accessibility and state management
#[component]
pub fn Checkbox(
    /// Current state of the checkbox
    #[prop(optional, default = CheckboxState::Unchecked)]
    state: CheckboxState,
    /// Whether the checkbox is disabled
    #[prop(optional, default = false)]
    _disabled: bool,
    /// Whether the checkbox is required
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
    on_change: Option<Callback<CheckboxState>>,
    /// Click event handler
    #[prop(optional)]
    on_click: Option<Callback<web_sys::MouseEvent>>,
    /// Child content (label)
    children: Children,
) -> impl IntoView {
    let __checkbox_id = generate_id("checkbox");
    
    // Build base classes
    let base_classes = "radix-checkbox";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    // Handle checkbox click
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
                    CheckboxState::Unchecked => CheckboxState::Checked,
                    CheckboxState::Checked => CheckboxState::Unchecked,
                    CheckboxState::Indeterminate => CheckboxState::Checked,
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
    let ariachecked = match state {
        CheckboxState::Unchecked => Some("false"),
        CheckboxState::Checked => Some("true"),
        CheckboxState::Indeterminate => Some("mixed"),
    };
    
    view! {
        <div
            id=checkbox_id.clone()
            class=combined_class
            style=style.unwrap_or_default()
            role="checkbox"
            aria-checked=ariachecked
            aria-required=required
            aria-disabled=disabled
                name=name.unwrap_or_default()
                value=value.unwrap_or_default()
                checked=state == CheckboxState::Checked
                disabled=disabled
                required=required
                style="position: absolute; opacity: 0; pointer-events: none;"
            />
            <div class="radix-checkbox-indicator">
                {move || match state {
                    CheckboxState::Checked => view! {
                        <svg width="15" height="15" viewBox="0 0 15 15" fill="none" xmlns="http://www.w3.org/2000/svg">
                            <path d="M11.4669 3.72684C11.7558 3.91574 11.8369 4.30308 11.648 4.59198L7.39799 11.092C7.29783 11.2452 7.13556 11.3467 6.95402 11.3699C6.77247 11.3931 6.58989 11.3355 6.45446 11.2124L3.70446 8.71241C3.44905 8.48022 3.43023 8.08494 3.66242 7.82953C3.89461 7.57412 4.28989 7.55529 4.5453 7.78749L6.75292 9.79441L10.6018 3.90792C10.7907 3.61902 11.178 3.53795 11.4669 3.72684Z" fill="currentColor" fill-rule="evenodd" clip-rule="evenodd"/>
                        </svg>
                    },
                    CheckboxState::Indeterminate => view! {
                        <svg width="15" height="15" viewBox="0 0 15 15" fill="none" xmlns="http://www.w3.org/2000/svg">
                            <path d="M3.5 7.5C3.5 7.22386 3.72386 7 4 7H11C11.2761 7 11.5 7.22386 11.5 7.5C11.5 7.77614 11.2761 8 11 8H4C3.72386 8 3.5 7.77614 3.5 7.5Z" fill="currentColor" fill-rule="evenodd" clip-rule="evenodd"/>
                        </svg>
                    },
                    CheckboxState::Unchecked => view! {
                        <svg width="15" height="15" viewBox="0 0 15 15" fill="none" xmlns="http://www.w3.org/2000/svg">
                            <rect width="15" height="15" rx="2" fill="none" stroke="currentColor" stroke-width="1.5"/>
                        </svg>
                    }
                }}
            </div>
            {children()}
        </div>
    }
}

/// Checkbox with label component for easier usage
#[component]
pub fn CheckboxWithLabel(
    /// Current state of the checkbox
    #[prop(optional, default = CheckboxState::Unchecked)]
    state: CheckboxState,
    /// Whether the checkbox is disabled
    #[prop(optional, default = false)]
    _disabled: bool,
    /// Whether the checkbox is required
    #[prop(optional, default = false)]
    _required: bool,
    /// Name attribute for form submission
    #[prop(optional)]
    name: Option<String>,
    /// Value attribute for form submission
    #[prop(optional)]
    value: Option<String>,
    /// CSS classes for the checkbox
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles for the checkbox
    #[prop(optional)]
    style: Option<String>,
    /// Child content (label)
    children: Children,
) -> impl IntoView {
    let __checkbox_id = generate_id("checkbox");
    
    view! {
        <div style="display: flex; align-items: center; gap: 8px;">
            <Checkbox
                state=state
                disabled=disabled
                required=required
                name=name.unwrap_or_default()
                value=value.unwrap_or_default()
                class=class.unwrap_or_default()
                style=style.unwrap_or_default()
            >
                <label for=format!("{}-input", checkbox_id) class="radix-checkbox-label".to_string()>
                    {children()}
                </label>
            </Checkbox>
        </div>
    }
}
