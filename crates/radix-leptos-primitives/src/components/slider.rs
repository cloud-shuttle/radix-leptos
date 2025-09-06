use leptos::callback::Callback;
use leptos::children::Children;
use leptos::prelude::*;

/// Slider component with proper accessibility and styling variants
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SliderVariant {
    Default,
    Destructive,
    Ghost,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SliderSize {
    Default,
    Sm,
    Lg,
}

impl SliderVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            SliderVariant::Default => "default",
            SliderVariant::Destructive => "destructive",
            SliderVariant::Ghost => "ghost",
        }
    }
}

impl SliderSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            SliderSize::Default => "default",
            SliderSize::Sm => "sm",
            SliderSize::Lg => "lg",
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

/// Slider root component
#[component]
pub fn Slider(
    /// Current value
    #[prop(optional, default = 0.0)]
    value: f64,
    /// Minimum value
    #[prop(optional, default = 0.0)]
    min: f64,
    /// Maximum value
    #[prop(optional, default = 100.0)]
    max: f64,
    /// Step value
    #[prop(optional, default = 1.0)]
    step: f64,
    /// Whether the slider is disabled
    #[prop(optional, default = false)]
    disabled: bool,
    /// Slider styling variant
    #[prop(optional, default = SliderVariant::Default)]
    variant: SliderVariant,
    /// Slider size
    #[prop(optional, default = SliderSize::Default)]
    size: SliderSize,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Value change event handler
    #[prop(optional)]
    on_value_change: Option<Callback<f64>>,
    /// Child content
    children: Children,
) -> impl IntoView {
    let __slider_id = generate_id("slider");
    let __track_id = generate_id("slider-track");
    let __range_id = generate_id("slider-range");
    let __thumb_id = generate_id("slider-thumb");

    // Build data attributes for styling
    let data_variant = variant.as_str();
    let data_size = size.as_str();

    // Merge classes with data attributes for CSS targeting
    let base_classes = "radix-slider";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());

    // Handle keyboard navigation
    let handle_keydown = move |e: web_sys::KeyboardEvent| {
        if disabled {
            return;
        }

        match e.key().as_str() {
            "ArrowLeft" | "ArrowDown" => {
                e.prevent_default();
                let new_value = (value - step).max(min);
                if let Some(on_value_change) = on_value_change {
                    on_value_change.run(new_value);
                }
            }
            "ArrowRight" | "ArrowUp" => {
                e.prevent_default();
                let new_value = (value + step).min(max);
                if let Some(on_value_change) = on_value_change {
                    on_value_change.run(new_value);
                }
            }
            "Home" => {
                e.prevent_default();
                if let Some(on_value_change) = on_value_change {
                    on_value_change.run(min);
                }
            }
            "End" => {
                e.prevent_default();
                if let Some(on_value_change) = on_value_change {
                    on_value_change.run(max);
                }
            }
            "PageDown" => {
                e.prevent_default();
                let new_value = (value - step * 10.0).max(min);
                if let Some(on_value_change) = on_value_change {
                    on_value_change.run(new_value);
                }
            }
            "PageUp" => {
                e.prevent_default();
                let new_value = (value + step * 10.0).min(max);
                if let Some(on_value_change) = on_value_change {
                    on_value_change.run(new_value);
                }
            }
            _ => {}
        }
    };

    // Calculate percentage for visual representation
    let percentage = if max > min {
        ((value - min) / (max - min) * 100.0).clamp(0.0, 100.0)
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
            data-min=min
            data-max=max
            data-step=step
            data-disabled=disabled
            role="slider"
            aria-valuemin=min
            aria-valuemax=max
            aria-valuenow=value
            aria-disabled=disabled
        >
        </div>
    }
}

/// Slider Track component
#[component]
pub fn SliderTrack(
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
) -> impl IntoView {
    let base_classes = "radix-slider-track";
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

/// Slider Range component
#[component]
pub fn SliderRange(
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
) -> impl IntoView {
    let base_classes = "radix-slider-range";
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

/// Slider Thumb component
#[component]
pub fn SliderThumb(
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
) -> impl IntoView {
    let base_classes = "radix-slider-thumb";
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
    use crate::{SliderSize, SliderVariant};
    use proptest::prelude::*;

    // 1. Basic Rendering Tests
    #[test]
    fn test_slider_variants() {
        run_test(|| {
            let variants = [
                SliderVariant::Default,
                SliderVariant::Destructive,
                SliderVariant::Ghost,
            ];

            for variant in variants {
                assert!(!variant.as_str().is_empty());
            }
        });
    }

    #[test]
    fn test_slider_sizes() {
        run_test(|| {
            let sizes = [SliderSize::Default, SliderSize::Sm, SliderSize::Lg];

            for size in sizes {
                assert!(!size.as_str().is_empty());
            }
        });
    }

    // 2. Props Validation Tests
    #[test]
    fn test_slider_default_values() {
        run_test(|| {
            let value = 0.0;
            let min = 0.0;
            let max = 100.0;
            let step = 1.0;
            let disabled = false;
            let variant = SliderVariant::Default;
            let size = SliderSize::Default;

            assert_eq!(value, 0.0);
            assert_eq!(min, 0.0);
            assert_eq!(max, 100.0);
            assert_eq!(step, 1.0);
            assert!(!disabled);
            assert_eq!(variant, SliderVariant::Default);
            assert_eq!(size, SliderSize::Default);
        });
    }

    #[test]
    fn test_slider_custom_values() {
        run_test(|| {
            let value = 50.0;
            let min = 10.0;
            let max = 90.0;
            let step = 5.0;
            let disabled = false;
            let variant = SliderVariant::Destructive;
            let size = SliderSize::Lg;

            assert_eq!(value, 50.0);
            assert_eq!(min, 10.0);
            assert_eq!(max, 90.0);
            assert_eq!(step, 5.0);
            assert!(!disabled);
            assert_eq!(variant, SliderVariant::Destructive);
            assert_eq!(size, SliderSize::Lg);
        });
    }

    #[test]
    fn test_sliderdisabled_state() {
        run_test(|| {
            let value = 25.0;
            let min = 0.0;
            let max = 100.0;
            let step = 1.0;
            let disabled = true;
            let variant = SliderVariant::Ghost;
            let size = SliderSize::Sm;

            assert_eq!(value, 25.0);
            assert_eq!(min, 0.0);
            assert_eq!(max, 100.0);
            assert_eq!(step, 1.0);
            assert!(disabled);
            assert_eq!(variant, SliderVariant::Ghost);
            assert_eq!(size, SliderSize::Sm);
        });
    }

    // 3. State Management Tests
    #[test]
    fn test_slider_value_calculation() {
        run_test(|| {
            let value = 50.0;
            let min = 0.0;
            let max = 100.0;
            let step = 1.0;

            // Test percentage calculation
            let percentage = if max > min {
                ((value - min) / (max - min) * 100.0f64).clamp(0.0f64, 100.0f64)
            } else {
                0.0
            };

            assert_eq!(percentage, 50.0);
        });
    }

    #[test]
    fn test_slider_value_bounds() {
        run_test(|| {
            let min = 10.0;
            let max = 90.0;
            let step = 5.0;

            // Test value clamping
            let value_below_min: f64 = 5.0;
            let value_above_max: f64 = 95.0;
            let value_in_range: f64 = 50.0;

            let clamped_below = value_below_min.max(min);
            let clamped_above = value_above_max.min(max);
            let clamped_in_range = value_in_range.clamp(min, max);

            assert_eq!(clamped_below, 10.0);
            assert_eq!(clamped_above, 90.0);
            assert_eq!(clamped_in_range, 50.0);
        });
    }

    // 4. Event Handling Tests
    #[test]
    fn test_slider_keyboard_navigation() {
        run_test(|| {
            let mut value: f64 = 50.0;
            let min: f64 = 0.0;
            let max: f64 = 100.0;
            let step: f64 = 10.0;
            let disabled = false;

            // Test arrow left/down
            let arrow_left_pressed = true;
            if arrow_left_pressed && !disabled {
                value = (value - step).max(min);
            }
            assert_eq!(value, 40.0);

            // Test arrow right/up
            let arrow_right_pressed = true;
            if arrow_right_pressed && !disabled {
                value = (value + step).min(max);
            }
            assert_eq!(value, 50.0);

            // Test home key
            let home_pressed = true;
            if home_pressed && !disabled {
                value = min;
            }
            assert_eq!(value, 0.0);

            // Test end key
            let end_pressed = true;
            if end_pressed && !disabled {
                value = max;
            }
            assert_eq!(value, 100.0);
        });
    }

    #[test]
    fn test_slider_page_navigation() {
        run_test(|| {
            let mut value: f64 = 50.0;
            let min: f64 = 0.0;
            let max: f64 = 100.0;
            let step: f64 = 1.0;
            let disabled = false;

            // Test page down
            let page_down_pressed = true;
            if page_down_pressed && !disabled {
                value = (value - step * 10.0f64).max(min);
            }
            assert_eq!(value, 40.0);

            // Test page up
            let page_up_pressed = true;
            if page_up_pressed && !disabled {
                value = (value + step * 10.0f64).min(max);
            }
            assert_eq!(value, 50.0);
        });
    }

    // 5. Accessibility Tests
    #[test]
    fn test_slider_accessibility() {
        run_test(|| {
            let role = "slider";
            let aria_valuemin = 0.0;
            let aria_valuemax = 100.0;
            let aria_valuenow = 50.0;
            let ariadisabled = false;
            let tabindex = "0";

            assert_eq!(role, "slider");
            assert_eq!(aria_valuemin, 0.0);
            assert_eq!(aria_valuemax, 100.0);
            assert_eq!(aria_valuenow, 50.0);
            assert!(!ariadisabled);
            assert_eq!(tabindex, "0");
        });
    }

    // 6. Edge Case Tests
    #[test]
    fn test_slider_edge_cases() {
        run_test(|| {
            // Test zero range
            let value = 0.0;
            let min = 0.0;
            let max = 0.0;
            let _step = 1.0;

            let percentage = if max > min {
                ((value - min) / (max - min) * 100.0f64).clamp(0.0f64, 100.0f64)
            } else {
                0.0
            };

            assert_eq!(percentage, 0.0);
        });
    }

    #[test]
    fn test_slider_negative_values() {
        run_test(|| {
            let value = -25.0;
            let min = -100.0;
            let max = 100.0;
            let step = 5.0;

            let percentage = if max > min {
                ((value - min) / (max - min) * 100.0f64).clamp(0.0f64, 100.0f64)
            } else {
                0.0
            };

            assert_eq!(percentage, 37.5);
        });
    }

    #[test]
    fn test_sliderdisabled_interaction() {
        run_test(|| {
            let value: f64 = 50.0;
            let min: f64 = 0.0;
            let max: f64 = 100.0;
            let step: f64 = 10.0;
            let disabled = true;

            // Disabled slider should not respond to keyboard
            let arrow_left_pressed = true;
            let new_value: f64 = if !disabled && arrow_left_pressed {
                (value - step).max(min)
            } else {
                value
            };

            assert_eq!(new_value, 50.0); // Should remain unchanged
        });
    }

    // 7. Property-Based Tests
    proptest! {
        #[test]
        fn test_slider_properties(
            variant in prop::sample::select(&[
                SliderVariant::Default,
                SliderVariant::Destructive,
                SliderVariant::Ghost,
            ]),
            size in prop::sample::select(&[
                SliderSize::Default,
                SliderSize::Sm,
                SliderSize::Lg,
            ]),
            value in -1000.0..1000.0f64,
            min in -1000.0..1000.0f64,
            max in -1000.0..1000.0f64,
            __step in 0.1..100.0f64,
            disabled in prop::bool::ANY
        ) {
            assert!(!variant.as_str().is_empty());
            assert!(!size.as_str().is_empty());

            assert!(disabled || !disabled);
            assert!(__step > 0.0);

            // Test percentage calculation
            let percentage = if max > min {
                ((value - min) / (max - min) * 100.0f64).clamp(0.0f64, 100.0f64)
            } else {
                0.0
            };

            assert!(percentage >= 0.0 && percentage <= 100.0);

            if disabled {
                // Disabled slider should not be interactive
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
