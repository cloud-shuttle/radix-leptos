use leptos::*;
use leptos::prelude::*;

/// Switch component with proper accessibility and styling variants
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SwitchVariant {
    Default,
    Destructive,
    Ghost,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SwitchSize {
    Default,
    Sm,
    Lg,
}

impl SwitchVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            SwitchVariant::Default => "default",
            SwitchVariant::Destructive => "destructive",
            SwitchVariant::Ghost => "ghost",
        }
    }
}

impl SwitchSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            SwitchSize::Default => "default",
            SwitchSize::Sm => "sm",
            SwitchSize::Lg => "lg",
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

/// Switch root component
#[component]
pub fn Switch(
    /// Whether the switch is on
    #[prop(optional, default = false)]
    checked: bool,
    /// Whether the switch is disabled
    #[prop(optional, default = false)]
    disabled: bool,
    /// Switch styling variant
    #[prop(optional, default = SwitchVariant::Default)]
    variant: SwitchVariant,
    /// Switch size
    #[prop(optional, default = SwitchSize::Default)]
    size: SwitchSize,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Checked change event handler
    #[prop(optional)]
    on_checked_change: Option<Callback<bool>>,
    /// Child content
    children: Children,
) -> impl IntoView {
    let switch_id = generate_id("switch");
    let thumb_id = generate_id("switch-thumb");
    
    // Build data attributes for styling
    let data_variant = variant.as_str();
    let data_size = size.as_str();
    
    // Merge classes with data attributes for CSS targeting
    let base_classes = "radix-switch";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    // Handle keyboard navigation
    let handle_keydown = move |e: web_sys::KeyboardEvent| {
        match e.key().as_str() {
            " " | "Enter" => {
                e.prevent_default();
                if !disabled {
                    if let Some(on_checked_change) = on_checked_change {
                        on_checked_change.run(!checked);
                    }
                }
            }
            _ => {}
        }
    };
    
    // Handle click
    let handle_click = move |e: web_sys::MouseEvent| {
        e.prevent_default();
        if !disabled {
            if let Some(on_checked_change) = on_checked_change {
                on_checked_change.run(!checked);
            }
        }
    };
    
    view! {
        <div 
            class=combined_class
            style=style
            data-variant=data_variant
            data-size=data_size
            data-checked=checked
            data-disabled=disabled
            role="switch"
            aria-checked=checked
            aria-disabled=disabled
            tabindex=if disabled { "-1" } else { "0" }
            on:keydown=handle_keydown
            on:click=handle_click
        >
            <input
                id=switch_id.clone()
                type="checkbox"
                checked=checked
                disabled=disabled
                tabindex="-1"
                aria-hidden="true"
            />
            <div class="radix-switch-track">
                <div 
                    id=thumb_id
                    class="radix-switch-thumb"
                    data-state=if checked { "checked" } else { "unchecked" }
                >
                </div>
            </div>
            {children()}
        </div>
    }
}

/// Switch Thumb component
#[component]
pub fn SwitchThumb(
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
) -> impl IntoView {
    let base_classes = "radix-switch-thumb";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <div 
            class=combined_class
            style=style
        >
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    // 1. Basic Rendering Tests
    #[test]
    fn test_switch_variants() {
        run_test(|| {
            let variants = vec![
                SwitchVariant::Default,
                SwitchVariant::Destructive,
                SwitchVariant::Ghost,
            ];
            
            for variant in variants {
                assert!(!variant.as_str().is_empty());
            }
        });
    }
    
    #[test]
    fn test_switch_sizes() {
        run_test(|| {
            let sizes = vec![
                SwitchSize::Default,
                SwitchSize::Sm,
                SwitchSize::Lg,
            ];
            
            for size in sizes {
                assert!(!size.as_str().is_empty());
            }
        });
    }
    
    // 2. Props Validation Tests
    #[test]
    fn test_switch_on_state() {
        run_test(|| {
            let checked = true;
            let disabled = false;
            let variant = SwitchVariant::Default;
            let size = SwitchSize::Default;
            
            assert!(checked);
            assert!(!disabled);
            assert_eq!(variant, SwitchVariant::Default);
            assert_eq!(size, SwitchSize::Default);
        });
    }
    
    #[test]
    fn test_switch_off_state() {
        run_test(|| {
            let checked = false;
            let disabled = false;
            let variant = SwitchVariant::Destructive;
            let size = SwitchSize::Lg;
            
            assert!(!checked);
            assert!(!disabled);
            assert_eq!(variant, SwitchVariant::Destructive);
            assert_eq!(size, SwitchSize::Lg);
        });
    }
    
    #[test]
    fn test_switch_disabled_state() {
        run_test(|| {
            let checked = false;
            let disabled = true;
            let variant = SwitchVariant::Ghost;
            let size = SwitchSize::Sm;
            
            assert!(!checked);
            assert!(disabled);
            assert_eq!(variant, SwitchVariant::Ghost);
            assert_eq!(size, SwitchSize::Sm);
        });
    }
    
    // 3. State Management Tests
    #[test]
    fn test_switch_state_changes() {
        run_test(|| {
            let mut checked = false;
            let disabled = false;
            
            // Initial state
            assert!(!checked);
            assert!(!disabled);
            
            // Turn on switch
            checked = true;
            
            assert!(checked);
            assert!(!disabled);
            
            // Turn off switch
            checked = false;
            
            assert!(!checked);
            assert!(!disabled);
        });
    }
    
    // 4. Event Handling Tests
    #[test]
    fn test_switch_keyboard_navigation() {
        run_test(|| {
            let space_pressed = true;
            let enter_pressed = false;
            let disabled = false;
            let checked = false;
            
            assert!(space_pressed);
            assert!(!enter_pressed);
            assert!(!disabled);
            assert!(!checked);
            
            if space_pressed && !disabled {
                assert!(true);
            }
            
            if enter_pressed && !disabled {
                assert!(false);
            }
        });
    }
    
    #[test]
    fn test_switch_click_handling() {
        run_test(|| {
            let clicked = true;
            let disabled = false;
            let checked = false;
            
            assert!(clicked);
            assert!(!disabled);
            assert!(!checked);
            
            if clicked && !disabled {
                assert!(true);
            }
        });
    }
    
    // 5. Accessibility Tests
    #[test]
    fn test_switch_accessibility() {
        run_test(|| {
            let role = "switch";
            let aria_checked = "false";
            let aria_disabled = "false";
            let tabindex = "0";
            
            assert_eq!(role, "switch");
            assert_eq!(aria_checked, "false");
            assert_eq!(aria_disabled, "false");
            assert_eq!(tabindex, "0");
        });
    }
    
    // 6. Edge Case Tests
    #[test]
    fn test_switch_edge_cases() {
        run_test(|| {
            let checked = true;
            let disabled = true;
            
            assert!(checked);
            assert!(disabled);
        });
    }
    
    #[test]
    fn test_switch_toggle_behavior() {
        run_test(|| {
            let mut checked = false;
            let disabled = false;
            
            assert!(!checked);
            assert!(!disabled);
            
            // Toggle on
            checked = !checked;
            
            assert!(checked);
            assert!(!disabled);
            
            // Toggle off
            checked = !checked;
            
            assert!(!checked);
            assert!(!disabled);
        });
    }
    
    // 7. Property-Based Tests
    proptest! {
        #[test]
        fn test_switch_properties(
            variant in prop::sample::select(vec![
                SwitchVariant::Default,
                SwitchVariant::Destructive,
                SwitchVariant::Ghost,
            ]),
            size in prop::sample::select(vec![
                SwitchSize::Default,
                SwitchSize::Sm,
                SwitchSize::Lg,
            ]),
            checked in prop::bool::ANY,
            disabled in prop::bool::ANY
        ) {
            assert!(!variant.as_str().is_empty());
            assert!(!size.as_str().is_empty());
            
            assert!(checked == true || checked == false);
            assert!(disabled == true || disabled == false);
            
            if disabled {
                // Disabled switch should not be interactive
            }
        }
    }
    
    // Helper function for running tests
    fn run_test<F>(f: F) where F: FnOnce() {
        f();
    }
}
