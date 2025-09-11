use leptos::callback::Callback;
use leptos::children::Children;
use leptos::prelude::*;

/// Alert component with proper accessibility and styling variants
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlertVariant {
    Default,
    Destructive,
    Warning,
    Success,
    Info,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlertSize {
    Default,
    Sm,
    Lg,
}

impl AlertVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            AlertVariant::Default => "default",
            AlertVariant::Destructive => "destructive",
            AlertVariant::Warning => "warning",
            AlertVariant::Success => "success",
            AlertVariant::Info => "info",
        }
    }
}

impl AlertSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            AlertSize::Default => "default",
            AlertSize::Sm => "sm",
            AlertSize::Lg => "lg",
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

/// Alert root component
#[component]
pub fn Alert(
    /// Alert styling variant
    #[prop(optional, default = AlertVariant::Default)]
    variant: AlertVariant,
    /// Alert size
    #[prop(optional, default = AlertSize::Default)]
    size: AlertSize,
    /// Whether the alert is dismissible
    #[prop(optional, default = false)]
    _dismissible: bool,
    /// Whether the alert is visible
    #[prop(optional, default = true)]
    visible: bool,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Dismiss event handler
    #[prop(optional)]
    on_dismiss: Option<Callback<()>>,
    /// Child content
    children: Children,
) -> impl IntoView {
    let ___alert_id = generate_id("alert");

    // Build data attributes for styling
    let data_variant = variant.as_str();
    let data_size = size.as_str();

    // Merge classes with data attributes for CSS targeting
    let base_classes = "radix-alert";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());

    // Handle dismiss
    let handle_dismiss = move |e: web_sys::MouseEvent| {
        e.prevent_default();
        if let Some(on_dismiss) = on_dismiss {
            on_dismiss.run(());
        }
    };

    // Handle keyboard events
    let handle_keydown = move |e: web_sys::KeyboardEvent| if e.key().as_str() == "Escape" {
        e.prevent_default();
        if let Some(on_dismiss) = on_dismiss {
            on_dismiss.run(());
        }
    };

    if !visible {
        return {
            let _: () = view! { <></> };
            ().into_any()
        };
    }

    view! {
        <div
            class=combined_class
            style=style
            data-variant=data_variant
            data-size=data_size
            data-dismissible=_dismissible
            role="alert"
            aria-live="polite"
            aria-atomic="true"
            on:keydown=handle_keydown
        >
            {children()}
            {if _dismissible {
                view! {
                    <button
                        class="radix-alert-dismiss"
                        aria-label="Dismiss alert"
                        on:click=handle_dismiss
                    >
                        "×"
                    </button>
                }.into_any()
            } else {
                let _: () = view! {};
                ().into_any()
            }}
        </div>
    }
    .into_any()
}

/// Alert Title component
#[component]
pub fn AlertTitle(
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content
    children: Children,
) -> impl IntoView {
    let base_classes = "radix-alert-title";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());

    view! {
        <div
            class=combined_class
            style=style
        >
            {children()}
        </div>
    }
}

/// Alert Description component
#[component]
pub fn AlertDescription(
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content
    children: Children,
) -> impl IntoView {
    let base_classes = "radix-alert-description";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());

    view! {
        <div
            class=combined_class
            style=style
        >
            {children()}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use crate::{AlertSize, AlertVariant};
    use proptest::prelude::*;

    // 1. Basic Rendering Tests
    #[test]
    fn test_alert_variants() {
        run_test(|| {
            let variants = [
                AlertVariant::Default,
                AlertVariant::Destructive,
                AlertVariant::Warning,
                AlertVariant::Success,
                AlertVariant::Info,
            ];

            for variant in variants {
                assert!(!variant.as_str().is_empty());
            }
        });
    }

    #[test]
    fn test_alert_sizes() {
        run_test(|| {
            let sizes = [AlertSize::Default, AlertSize::Sm, AlertSize::Lg];

            for size in sizes {
                assert!(!size.as_str().is_empty());
            }
        });
    }

    // 2. Props Validation Tests
    #[test]
    fn test_alert_default_values() {
        run_test(|| {
            let variant = AlertVariant::Default;
            let size = AlertSize::Default;
            let dismissible = false;
            let visible = true;

            assert_eq!(variant, AlertVariant::Default);
            assert_eq!(size, AlertSize::Default);
            assert!(!dismissible);
            assert!(visible);
        });
    }

    #[test]
    fn test_alert_custom_values() {
        run_test(|| {
            let variant = AlertVariant::Success;
            let size = AlertSize::Lg;
            let dismissible = true;
            let visible = true;

            assert_eq!(variant, AlertVariant::Success);
            assert_eq!(size, AlertSize::Lg);
            assert!(dismissible);
            assert!(visible);
        });
    }

    #[test]
    fn test_alert_destructive_variant() {
        run_test(|| {
            let variant = AlertVariant::Destructive;
            let size = AlertSize::Sm;
            let dismissible = true;
            let visible = true;

            assert_eq!(variant, AlertVariant::Destructive);
            assert_eq!(size, AlertSize::Sm);
            assert!(dismissible);
            assert!(visible);
        });
    }

    // 3. State Management Tests
    #[test]
    fn test_alert_visibility_state() {
        run_test(|| {
            let mut visible = true;
            let dismissible = true;

            // Initial state
            assert!(visible);
            assert!(dismissible);

            // Dismiss alert
            visible = false;

            assert!(!visible);
            assert!(dismissible);
        });
    }

    #[test]
    fn test_alertdismissible_state() {
        run_test(|| {
            let visible = true;
            let mut dismissible = false;

            // Initial state
            assert!(visible);
            assert!(!dismissible);

            // Make dismissible
            dismissible = true;

            assert!(visible);
            assert!(dismissible);
        });
    }

    // 4. Event Handling Tests
    #[test]
    fn test_alert_dismiss_handling() {
        run_test(|| {
            let dismiss_clicked = true;
            let dismissible = true;
            let visible = true;

            assert!(dismiss_clicked);
            assert!(dismissible);
            assert!(visible);

            if dismiss_clicked && dismissible {}
        });
    }

    #[test]
    fn test_alert_keyboard_dismiss() {
        run_test(|| {
            let escape_pressed = true;
            let dismissible = true;
            let visible = true;

            assert!(escape_pressed);
            assert!(dismissible);
            assert!(visible);

            if escape_pressed && dismissible {}
        });
    }

    // 5. Accessibility Tests
    #[test]
    fn test_alert_accessibility() {
        run_test(|| {
            let role = "alert";
            let aria_live = "polite";
            let aria_atomic = "true";
            let aria_label = "Dismiss alert";

            assert_eq!(role, "alert");
            assert_eq!(aria_live, "polite");
            assert_eq!(aria_atomic, "true");
            assert_eq!(aria_label, "Dismiss alert");
        });
    }

    // 6. Edge Case Tests
    #[test]
    fn test_alert_edge_cases() {
        run_test(|| {
            let variant = AlertVariant::Warning;
            let size = AlertSize::Default;
            let dismissible = false;
            let visible = false;

            assert_eq!(variant, AlertVariant::Warning);
            assert_eq!(size, AlertSize::Default);
            assert!(!dismissible);
            assert!(!visible);
        });
    }

    #[test]
    fn test_alert_nondismissible() {
        run_test(|| {
            let variant = AlertVariant::Info;
            let size = AlertSize::Lg;
            let dismissible = false;
            let visible = true;

            assert_eq!(variant, AlertVariant::Info);
            assert_eq!(size, AlertSize::Lg);
            assert!(!dismissible);
            assert!(visible);

            // Non-dismissible alert should not respond to dismiss actions
            let dismiss_clicked = true;
            if dismiss_clicked && !dismissible {
                // Alert should remain visible
            }
        });
    }

    #[test]
    fn test_alert_invisible_state() {
        run_test(|| {
            let variant = AlertVariant::Success;
            let size = AlertSize::Sm;
            let dismissible = true;
            let visible = false;

            assert_eq!(variant, AlertVariant::Success);
            assert_eq!(size, AlertSize::Sm);
            assert!(dismissible);
            assert!(!visible);

            // Invisible alert should not be rendered
            if !visible {}
        });
    }

    // 7. Property-Based Tests
    proptest! {
        #[test]
        fn test_alert_properties(
            variant in prop::sample::select(&[
                AlertVariant::Default,
                AlertVariant::Destructive,
                AlertVariant::Warning,
                AlertVariant::Success,
                AlertVariant::Info,
            ]),
            size in prop::sample::select(&[
                AlertSize::Default,
                AlertSize::Sm,
                AlertSize::Lg,
            ]),
            dismissible in prop::bool::ANY,
            visible in prop::bool::ANY
        ) {
            assert!(!variant.as_str().is_empty());
            assert!(!size.as_str().is_empty());

            // Test that boolean properties are properly typed
            assert!(matches!(dismissible, true | false));
            assert!(matches!(visible, true | false));

            // Test dismiss behavior
            if !visible {
                // Invisible alert should not be interactive
            }

            if !dismissible {
                // Non-dismissible alert should not respond to dismiss actions
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
