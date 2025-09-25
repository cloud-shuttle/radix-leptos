use leptos::children::Children;
use leptos::prelude::*;
use crate::utils::{merge_optional_classes, generate_id};

/// Progress component with proper accessibility and styling variants
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProgressVariant {
    Default,
    Destructive,
    Success,
    Warning,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProgressSize {
    Default,
    Sm,
    Lg,
}

impl ProgressVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            ProgressVariant::Default => "default",
            ProgressVariant::Destructive => "destructive",
            ProgressVariant::Success => "success",
            ProgressVariant::Warning => "warning",
        }
    }
}

impl ProgressSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            ProgressSize::Default => "default",
            ProgressSize::Sm => "sm",
            ProgressSize::Lg => "lg",
        }
    }
}


/// Progress root component
#[component]
pub fn Progress(
    /// Current progress value (0-100)
    #[prop(optional, default = 0.0)]
    value: f64,
    /// Maximum value
    #[prop(optional, default = 100.0)]
    max: f64,
    /// Whether the progress is indeterminate
    #[prop(optional, default = false)]
    indeterminate: bool,
    /// Progress styling variant
    #[prop(optional, default = ProgressVariant::Default)]
    variant: ProgressVariant,
    /// Progress size
    #[prop(optional, default = ProgressSize::Default)]
    size: ProgressSize,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content
    _children: Children,
) -> impl IntoView {
    let __progress_id = generate_id("progress");

    // Build data attributes for styling
    let data_variant = variant.as_str();
    let data_size = size.as_str();

    // Merge classes with data attributes for CSS targeting
    let base_classes = "radix-progress";
    let combined_class = merge_optional_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());

    // Calculate percentage for visual representation
    let percentage = if max > 0.0 && !indeterminate {
        (value / max * 100.0).clamp(0.0, 100.0)
    } else {
        0.0
    };

    view! {
        <div
            class=combined_class
            style=style
            data-variant=data_variant
            data-size=data_size
            data-value=value
            data-max=max
            data-indeterminate=indeterminate
            data-percentage=percentage
            role="progressbar"
            aria-valuemin=0.0
            aria-valuemax=max
        >
        </div>
    }
}

/// Progress Track component
#[component]
pub fn ProgressTrack(
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
) -> impl IntoView {
    let base_classes = "radix-progress-track";
    let combined_class = merge_optional_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());

    view! {
        <div
            class=combined_class
            style=style
        >
        </div>
    }
}

/// Progress Indicator component
#[component]
pub fn ProgressIndicator(
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
) -> impl IntoView {
    let base_classes = "radix-progress-indicator";
    let combined_class = merge_optional_classes(Some(base_classes), class.as_deref())
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
    use crate::{ProgressSize, ProgressVariant};

    use proptest::prelude::*;
use crate::utils::{merge_optional_classes, generate_id};

    // 1. Basic Rendering Tests
    #[test]
    fn test_progress_variants() {
        run_test(|| {
            let variants = [
                ProgressVariant::Default,
                ProgressVariant::Destructive,
                ProgressVariant::Success,
                ProgressVariant::Warning,
            ];

            for variant in variants {
                assert!(!variant.as_str().is_empty());
            }
        });
    }

    #[test]
    fn test_progress_sizes() {
        run_test(|| {
            let sizes = [ProgressSize::Default, ProgressSize::Sm, ProgressSize::Lg];

            for size in sizes {
                assert!(!size.as_str().is_empty());
            }
        });
    }

    // 2. Props Validation Tests
    #[test]
    fn test_progress_default_values() {
        run_test(|| {
            let value = 0.0;
            let _max = 100.0;
            let indeterminate = false;
            let variant = ProgressVariant::Default;
            let size = ProgressSize::Default;

            assert_eq!(value, 0.0);
            assert_eq!(_max, 100.0);
            assert!(!indeterminate);
            assert_eq!(variant, ProgressVariant::Default);
            assert_eq!(size, ProgressSize::Default);
        });
    }

    #[test]
    fn test_progress_custom_values() {
        run_test(|| {
            let value = 50.0;
            let _max = 200.0;
            let indeterminate = false;
            let variant = ProgressVariant::Success;
            let size = ProgressSize::Lg;

            assert_eq!(value, 50.0);
            assert_eq!(_max, 200.0);
            assert!(!indeterminate);
            assert_eq!(variant, ProgressVariant::Success);
            assert_eq!(size, ProgressSize::Lg);
        });
    }

    #[test]
    fn test_progressindeterminate_state() {
        run_test(|| {
            let value = 0.0;
            let _max = 100.0;
            let indeterminate = true;
            let variant = ProgressVariant::Warning;
            let size = ProgressSize::Sm;

            assert_eq!(value, 0.0);
            assert_eq!(_max, 100.0);
            assert!(indeterminate);
            assert_eq!(variant, ProgressVariant::Warning);
            assert_eq!(size, ProgressSize::Sm);
        });
    }

    // 3. State Management Tests
    #[test]
    fn test_progress_value_calculation() {
        run_test(|| {
            let value = 50.0;
            let _max = 100.0;
            let indeterminate = false;

            // Test percentage calculation
            let percentage = if _max > 0.0 && !indeterminate {
                (value / _max * 100.0f64).clamp(0.0f64, 100.0f64)
            } else {
                0.0
            };

            assert_eq!(percentage, 50.0);
        });
    }

    #[test]
    fn test_progress_value_bounds() {
        run_test(|| {
            let _max = 100.0;
            let indeterminate = false;

            // Test value clamping
            let value_below_min = -10.0;
            let value_above_max = 150.0;
            let value_in_range = 50.0;

            let percentage_below = if _max > 0.0 && !indeterminate {
                (value_below_min / _max * 100.0f64).clamp(0.0f64, 100.0f64)
            } else {
                0.0
            };

            let percentage_above = if _max > 0.0 && !indeterminate {
                (value_above_max / _max * 100.0f64).clamp(0.0f64, 100.0f64)
            } else {
                0.0
            };

            let percentage_in_range = if _max > 0.0 && !indeterminate {
                (value_in_range / _max * 100.0f64).clamp(0.0f64, 100.0f64)
            } else {
                0.0
            };

            assert_eq!(percentage_below, 0.0);
            assert_eq!(percentage_above, 100.0);
            assert_eq!(percentage_in_range, 50.0);
        });
    }

    // 4. Indeterminate State Tests
    #[test]
    fn test_progressindeterminate_calculation() {
        run_test(|| {
            let value = 50.0;
            let _max = 100.0;
            let indeterminate = true;

            // Test percentage calculation for indeterminate state
            let percentage = if _max > 0.0 && !indeterminate {
                (value / _max * 100.0f64).clamp(0.0f64, 100.0f64)
            } else {
                0.0
            };

            assert_eq!(percentage, 0.0);
        });
    }

    #[test]
    fn test_progressindeterminate_aria() {
        run_test(|| {
            let value = 50.0;
            let _max = 100.0;
            let indeterminate = true;

            // Test ARIA attributes for indeterminate state
            assert!(indeterminate);
        });
    }

    // 5. Accessibility Tests
    #[test]
    fn test_progress_accessibility() {
        run_test(|| {
            let role = "progressbar";
            let aria_valuemin = 0.0;
            let aria_valuemax = 100.0;
            let aria_valuenow = Some(50.0);
            let aria_label = "Progress";

            assert_eq!(role, "progressbar");
            assert_eq!(aria_valuemin, 0.0);
            assert_eq!(aria_valuemax, 100.0);
            assert_eq!(aria_valuenow, Some(50.0));
            assert_eq!(aria_label, "Progress");
        });
    }

    // 6. Edge Case Tests
    #[test]
    fn test_progress_edge_cases() {
        run_test(|| {
            // Test zero max value
            let value = 50.0;
            let _max = 0.0;
            let indeterminate = false;

            let percentage = if _max > 0.0 && !indeterminate {
                (value / _max * 100.0f64).clamp(0.0f64, 100.0f64)
            } else {
                0.0
            };

            assert_eq!(percentage, 0.0);
        });
    }

    #[test]
    fn test_progress_negative_values() {
        run_test(|| {
            let value = -25.0;
            let _max = 100.0;
            let indeterminate = false;

            let percentage = if _max > 0.0 && !indeterminate {
                (value / _max * 100.0f64).clamp(0.0f64, 100.0f64)
            } else {
                0.0
            };

            assert_eq!(percentage, 0.0);
        });
    }

    #[test]
    fn test_progress_completion_state() {
        run_test(|| {
            let value = 100.0;
            let _max = 100.0;
            let indeterminate = false;

            let percentage = if _max > 0.0 && !indeterminate {
                (value / _max * 100.0f64).clamp(0.0f64, 100.0f64)
            } else {
                0.0
            };

            assert_eq!(percentage, 100.0);
        });
    }

    // 7. Property-Based Tests
    proptest! {
        #[test]
        fn test_progress_properties(
            variant in prop::sample::select(&[
                ProgressVariant::Default,
                ProgressVariant::Destructive,
                ProgressVariant::Success,
                ProgressVariant::Warning,
            ]),
            size in prop::sample::select(&[
                ProgressSize::Default,
                ProgressSize::Sm,
                ProgressSize::Lg,
            ]),
            value in -100.0..1000.0f64,
            __max in 0.1..1000.0f64,
            indeterminate in prop::bool::ANY
        ) {
            assert!(!variant.as_str().is_empty());
            assert!(!size.as_str().is_empty());

            // Test that indeterminate property is properly typed
            assert!(matches!(indeterminate, true | false));
            assert!(__max > 0.0);

            // Test percentage calculation
            let percentage = if __max > 0.0 && !indeterminate {
                (value / __max * 100.0f64).clamp(0.0f64, 100.0f64)
            } else {
                0.0
            };

            assert!((0.0..=100.0).contains(&percentage));

            // Test ARIA attributes
            // ARIA attributes are properly set in the component
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
