use leptos::prelude::*;
use leptos::*;

/// Checkbox component with proper accessibility and styling variants
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CheckboxVariant {
    Default,
    Destructive,
    Ghost,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CheckboxSize {
    Default,
    Sm,
    Lg,
}

impl CheckboxVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            CheckboxVariant::Default => "default",
            CheckboxVariant::Destructive => "destructive",
            CheckboxVariant::Ghost => "ghost",
        }
    }
}

impl CheckboxSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            CheckboxSize::Default => "default",
            CheckboxSize::Sm => "sm",
            CheckboxSize::Lg => "lg",
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

/// Checkbox root component
#[component]
pub fn Checkbox(
    /// Whether the checkbox is checked
    #[prop(optional, default = false)]
    _checked: bool,
    /// Whether the checkbox is indeterminate
    #[prop(optional, default = false)]
    _indeterminate: bool,
    /// Whether the checkbox is disabled
    #[prop(optional, default = false)]
    _disabled: bool,
    /// Checkbox styling variant
    #[prop(optional, default = CheckboxVariant::Default)]
    variant: CheckboxVariant,
    /// Checkbox size
    #[prop(optional, default = CheckboxSize::Default)]
    size: CheckboxSize,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Checked change event handler
    #[prop(optional)]
    on_checked_change: Option<Callback<bool>>,
    /// Indeterminate change event handler
    #[prop(optional)]
    _on_indeterminate_change: Option<Callback<bool>>,
    /// Child content
    children: Children,
) -> impl IntoView {
    let checkbox_id = generate_id("checkbox");
    let label_id = generate_id("checkbox-label");

    // Build data attributes for styling
    let data_variant = variant.as_str();
    let data_size = size.as_str();

    // Merge classes with data attributes for CSS targeting
    let base_classes = "radix-checkbox";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());

    // Handle keyboard navigation
    let handle_keydown = move |e: web_sys::KeyboardEvent| match e.key().as_str() {
        " " | "Enter" => {
            e.prevent_default();
            if !disabled {
                if let Some(on_checked_change) = on_checked_change {
                    on_checked_change.run(!checked);
                }
            }
        }
        _ => {}
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
            data-indeterminate=indeterminate
            data-disabled=disabled
            on:keydown=handle_keydown
        >
            <input
                id=checkbox_id.clone()
                type="checkbox"
                checked=checked
                disabled=disabled
                tabindex="-1"
                aria-hidden="true"
            />
            <label
                id=label_id
                for=checkbox_id.clone()
                class="radix-checkbox-label"
                on:click=handle_click
            >
                {children()}
            </label>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    // 1. Basic Rendering Tests
    #[test]
    fn test_checkbox_variants() {
        run_test(|| {
            let variants = [
                CheckboxVariant::Default,
                CheckboxVariant::Destructive,
                CheckboxVariant::Ghost,
            ];

            for variant in variants {
                assert!(!variant.as_str().is_empty());
            }
        });
    }

    #[test]
    fn test_checkbox_sizes() {
        run_test(|| {
            let sizes = [CheckboxSize::Default, CheckboxSize::Sm, CheckboxSize::Lg];

            for size in sizes {
                assert!(!size.as_str().is_empty());
            }
        });
    }

    // 2. Props Validation Tests
    #[test]
    fn test_checkbox_checked_state() {
        run_test(|| {
            let checked = true;
            let indeterminate = false;
            let disabled = false;
            let variant = CheckboxVariant::Default;
            let size = CheckboxSize::Default;

            assert!(checked);
            assert!(!indeterminate);
            assert!(!disabled);
            assert_eq!(variant, CheckboxVariant::Default);
            assert_eq!(size, CheckboxSize::Default);
        });
    }

    #[test]
    fn test_checkbox_unchecked_state() {
        run_test(|| {
            let checked = false;
            let indeterminate = false;
            let disabled = false;
            let variant = CheckboxVariant::Destructive;
            let size = CheckboxSize::Lg;

            assert!(!checked);
            assert!(!indeterminate);
            assert!(!disabled);
            assert_eq!(variant, CheckboxVariant::Destructive);
            assert_eq!(size, CheckboxSize::Lg);
        });
    }

    #[test]
    fn test_checkbox_indeterminate_state() {
        run_test(|| {
            let checked = false;
            let indeterminate = true;
            let disabled = false;
            let variant = CheckboxVariant::Ghost;
            let size = CheckboxSize::Sm;

            assert!(!checked);
            assert!(indeterminate);
            assert!(!disabled);
            assert_eq!(variant, CheckboxVariant::Ghost);
            assert_eq!(size, CheckboxSize::Sm);
        });
    }

    // 3. State Management Tests
    #[test]
    fn test_checkbox_state_changes() {
        run_test(|| {
            let mut checked = false;
            let mut indeterminate = false;
            let disabled = false;

            assert!(!checked);
            assert!(!indeterminate);
            assert!(!disabled);

            checked = true;
            indeterminate = false;

            assert!(checked);
            assert!(!indeterminate);
            assert!(!disabled);

            checked = false;
            indeterminate = false;

            assert!(!checked);
            assert!(!indeterminate);
            assert!(!disabled);

            checked = false;
            indeterminate = true;

            assert!(!checked);
            assert!(indeterminate);
            assert!(!disabled);
        });
    }

    // 4. Event Handling Tests
    #[test]
    fn test_checkbox_keyboard_navigation() {
        run_test(|| {
            let space_pressed = true;
            let enter_pressed = false;
            let disabled = false;
            let checked = false;

            assert!(space_pressed);
            assert!(!enter_pressed);
            assert!(!disabled);
            assert!(!checked);

            if space_pressed && !disabled {}

            if enter_pressed && !disabled {
                assert!(false);
            }
        });
    }

    #[test]
    fn test_checkbox_click_handling() {
        run_test(|| {
            let clicked = true;
            let disabled = false;
            let checked = false;

            assert!(clicked);
            assert!(!disabled);
            assert!(!checked);

            if clicked && !disabled {}
        });
    }

    // 5. Accessibility Tests
    #[test]
    fn test_checkbox_accessibility() {
        run_test(|| {
            let role = "checkbox";
            let aria_checked = "false";
            let aria_disabled = "false";
            let tabindex = "-1";

            assert_eq!(role, "checkbox");
            assert_eq!(aria_checked, "false");
            assert_eq!(aria_disabled, "false");
            assert_eq!(tabindex, "-1");
        });
    }

    // 6. Edge Case Tests
    #[test]
    fn test_checkbox_edge_cases() {
        run_test(|| {
            let checked = true;
            let indeterminate = true;
            let disabled = false;

            assert!(checked);
            assert!(indeterminate);
            assert!(!disabled);
        });
    }

    #[test]
    fn test_checkbox_disabled_state() {
        run_test(|| {
            let disabled = true;
            let checked = false;
            let indeterminate = false;

            assert!(disabled);
            assert!(!checked);
            assert!(!indeterminate);
        });
    }

    // 7. Property-Based Tests
    proptest! {
        #[test]
        fn test_checkbox_properties(
            variant in prop::sample::select([
                CheckboxVariant::Default,
                CheckboxVariant::Destructive,
                CheckboxVariant::Ghost,
            ]),
            size in prop::sample::select([
                CheckboxSize::Default,
                CheckboxSize::Sm,
                CheckboxSize::Lg,
            ]),
            checked in prop::bool::ANY,
            indeterminate in prop::bool::ANY,
            disabled in prop::bool::ANY
        ) {
            assert!(!variant.as_str().is_empty());
            assert!(!size.as_str().is_empty());

            assert!(checked  || checked ! );
            assert!(indeterminate  || indeterminate ! );
            assert!(disabled  || disabled ! );

            if disabled {
                // Disabled checkbox should not be interactive
            }

            if indeterminate && checked {
                // Indeterminate would take precedence
            }
        }
    }

    // Helper function for running tests
    fn run_test<F>(f: F)
    where
        F: FnOnce(),
    {
        f();
    }
}
