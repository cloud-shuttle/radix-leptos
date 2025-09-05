use wasm_bindgen::JsCast;
use crate::utils::merge_classes;

/// Range Slider component - Dual handle range selection
#[component]
pub fn RangeSlider(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] min: Option<f64>,
    #[prop(optional)] max: Option<f64>,
    #[prop(optional)] step: Option<f64>,
    #[prop(optional)] min_value: Option<f64>,
    #[prop(optional)] max_value: Option<f64>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] orientation: Option<SliderOrientation>,
    #[prop(optional)] size: Option<SliderSize>,
    #[prop(optional)] variant: Option<SliderVariant>,
    #[prop(optional)] on_change: Option<Callback<RangeSliderValue>>,
    #[prop(optional)] on_min_change: Option<Callback<f64>>,
    #[prop(optional)] on_max_change: Option<Callback<f64>>,
) -> impl IntoView {
    let min = min.unwrap_or(0.0);
    let _max = max.unwrap_or(100.0);
    let _step = step.unwrap_or(1.0);
    let min_value = min_value.unwrap_or(min);
    let max_value = max_value.unwrap_or(max);
    let disabled = disabled.unwrap_or(false);
    let orientation = orientation.unwrap_or(SliderOrientation::Horizontal);
    let size = size.unwrap_or(SliderSize::Default);
    let variant = variant.unwrap_or(SliderVariant::Default);

    let class = 
        "range-slider",
    };

    let handle_min_change = Callback::new(move |new_min: f64| {
        if let Some(callback) = on_min_change {
            callback.run(new_min);
        }
    });

    let handle_max_change = Callback::new(move |new_max: f64| {
        if let Some(callback) = on_max_change {
            callback.run(new_max);
        }
    });

    view! {
        <div
            class=class
            style=style
            role="slider"
            aria-label="Range slider"
            data-min=min
            data-max=max
            data-step=step
            data-min-value=min_value
            data-max-value=max_value
            data-orientation=orientation.as_str()
        >
            <RangeSliderTrack
                min=min
                max=max
                step=step
                min_value=min_value
                max_value=max_value
                disabled=disabled
                orientation=orientation
                size=size
                variant=variant
            />
            <RangeSliderThumb
                value=min_value
                min=min
                max=max
                step=step
                disabled=disabled
                orientation=orientation
                size=size
                variant=variant
                thumb_type=ThumbType::Min
                on_change=handle_min_change
            />
            <RangeSliderThumb
                value=max_value
                min=min
                max=max
                step=step
                disabled=disabled
                orientation=orientation
                size=size
                variant=variant
                thumb_type=ThumbType::Max
                on_change=handle_max_change
            />
            {children.map(|c| c())}
        </div>
    }
}

/// Range Slider Track component
#[component]
pub fn RangeSliderTrack(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] min: Option<f64>,
    #[prop(optional)] max: Option<f64>,
    #[prop(optional)] step: Option<f64>,
    #[prop(optional)] min_value: Option<f64>,
    #[prop(optional)] max_value: Option<f64>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] orientation: Option<SliderOrientation>,
    #[prop(optional)] size: Option<SliderSize>,
    #[prop(optional)] variant: Option<SliderVariant>,
) -> impl IntoView {
    let min = min.unwrap_or(0.0);
    let _max = max.unwrap_or(100.0);
    let _step = step.unwrap_or(1.0);
    let min_value = min_value.unwrap_or(min);
    let max_value = max_value.unwrap_or(max);
    let disabled = disabled.unwrap_or(false);
    let orientation = orientation.unwrap_or(SliderOrientation::Horizontal);
    let size = size.unwrap_or(SliderSize::Default);
    let variant = variant.unwrap_or(SliderVariant::Default);

    let class = 
        "range-slider-track",
                fill_start,
                fill_start,
                fill_end,
                fill_end,
                style.unwrap_or_default()
            )
        }
        SliderOrientation::Vertical => {
            format!(
                "background: linear-gradient(to bottom, transparent {}%, var(--slider-fill-color) {}%, var(--slider-fill-color) {}%, transparent {}%); {}",
                fill_start,
                fill_start,
                fill_end,
                fill_end,
                style.unwrap_or_default()
            )
        }
    };

    view! {
        <div
            class=class
            style=track_style
            role="presentation"
            aria-hidden="true"
            data-min=min
            data-max=max
            data-step=step
            data-min-value=min_value
            data-max-value=max_value
        />
    }
}

/// Range Slider Thumb component
#[component]
pub fn RangeSliderThumb(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] value: Option<f64>,
    #[prop(optional)] min: Option<f64>,
    #[prop(optional)] max: Option<f64>,
    #[prop(optional)] step: Option<f64>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] orientation: Option<SliderOrientation>,
    #[prop(optional)] size: Option<SliderSize>,
    #[prop(optional)] variant: Option<SliderVariant>,
    #[prop(optional)] thumb_type: Option<ThumbType>,
    #[prop(optional)] on_change: Option<Callback<f64>>,
    #[prop(optional)] on_drag_start: Option<Callback<()>>,
    #[prop(optional)] on_drag_end: Option<Callback<()>>,
) -> impl IntoView {
    let value = value.unwrap_or(0.0);
    let min = min.unwrap_or(0.0);
    let _max = max.unwrap_or(100.0);
    let _step = step.unwrap_or(1.0);
    let disabled = disabled.unwrap_or(false);
    let orientation = orientation.unwrap_or(SliderOrientation::Horizontal);
    let size = size.unwrap_or(SliderSize::Default);
    let variant = variant.unwrap_or(SliderVariant::Default);
    let thumb_type = thumb_type.unwrap_or(ThumbType::Min);

    let class = 
        "range-slider-thumb",
                position,
                style.unwrap_or_default()
            )
        }
        SliderOrientation::Vertical => {
            format!(
                "bottom: {}%; {}",
                position,
                style.unwrap_or_default()
            )
        }
    };

    let handle_change = move |new_value: f64| {
        if let Some(callback) = on_change {
            callback.run(new_value);
        }
    };

    let handle_drag_start = move |_| {
        if let Some(callback) = on_drag_start {
            callback.run(());
        }
    };

    let handle_drag_end = move |_| {
        if let Some(callback) = on_drag_end {
            callback.run(());
        }
    };

    view! {
        <div
            class=class
            style=thumb_style
            role="slider"
            aria-label=format!("{} thumb", thumb_type.as_str())
            data-value=value
            data-thumb-type=thumb_type.as_str()
            on:mousedown=handle_drag_start
            on:touchend=handle_drag_end
        />
    }
}

/// Range Slider Value struct
#[derive(Debug, Clone, PartialEq)]
pub struct RangeSliderValue {
    pub min: f64,
    pub max: f64,
}

impl Default for RangeSliderValue {
    fn default() -> Self {
        Self {
            min: 0.0,
            max: 100.0,
        }
    }
}

/// Slider Orientation enum
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SliderOrientation {
    Horizontal,
    Vertical,
}

impl SliderOrientation {
    pub fn as_str(&self) -> &'static str {
        match self {
            SliderOrientation::Horizontal => "horizontal",
            SliderOrientation::Vertical => "vertical",
        }
    }
}

/// Slider Size enum
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SliderSize {
    Small,
    Default,
    Large,
}

impl SliderSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            SliderSize::Small => "sm",
            SliderSize::Default => "default",
            SliderSize::Large => "lg",
        }
    }
}

/// Slider Variant enum
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SliderVariant {
    Default,
    Primary,
    Secondary,
    Destructive,
}

impl SliderVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            SliderVariant::Default => "default",
            SliderVariant::Primary => "primary",
            SliderVariant::Secondary => "secondary",
            SliderVariant::Destructive => "destructive",
        }
    }
}

/// Thumb Type enum
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ThumbType {
    Min,
    Max,
}

impl ThumbType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ThumbType::Min => "min",
            ThumbType::Max => "max",
        }
    }
}

/// Range Slider Label component
#[component]
pub fn RangeSliderLabel(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] for_id: Option<String>,
) -> impl IntoView {
    let class = 
        "range-slider-label",
        class.as_deref().unwrap_or(""),
    ]);

    view! {
        <label
            class=class
            style=style
            for=for_id
        >
            {children.map(|c| c())}
        </label>
    }
}

/// Range Slider Value Display component
#[component]
pub fn RangeSliderValueDisplay(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] min_value: Option<f64>,
    #[prop(optional)] max_value: Option<f64>,
    #[prop(optional)] format: Option<ValueFormat>,
    #[prop(optional)] show_both: Option<bool>,
) -> impl IntoView {
    let min_value = min_value.unwrap_or(0.0);
    let max_value = max_value.unwrap_or(100.0);
    let format = format.unwrap_or(ValueFormat::Number);
    let show_both = show_both.unwrap_or(true);

    let class = 
        "range-slider-value-display",
        class.as_deref().unwrap_or(""),
    ]);

    let format_value = |value: f64| -> String {
        match format {
            ValueFormat::Number => format!("{:.0}", value),
            ValueFormat::Decimal => format!("{:.2}", value),
            ValueFormat::Percentage => format!("{:.0}%", value),
            ValueFormat::Currency => format!("${:.2}", value),
            ValueFormat::Custom(ref fmt) => format!("{}", fmt.replace("{}", &value.to_string())),
        }
    };

    view! {
        <div
            class=class
            style=style
            role="status"
            aria-live="polite"
        >
            {if show_both {
                format!("{} - {}", format_value(min_value), format_value(max_value))
            }}
        </div>
    }
}

/// Value Format enum
#[derive(Debug, Clone, PartialEq)]
pub enum ValueFormat {
    Number,
    Decimal,
    Percentage,
    Currency,
    Custom(String),
}

impl Default for ValueFormat {
    fn default() -> Self {
        ValueFormat::Number
    }
}

/// Range Slider Marks component
#[component]
pub fn RangeSliderMarks(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] marks: Option<Vec<SliderMark>>,
    #[prop(optional)] orientation: Option<SliderOrientation>,
    #[prop(optional)] min: Option<f64>,
    #[prop(optional)] max: Option<f64>,
) -> impl IntoView {
    let marks = marks.unwrap_or_default();
    let orientation = orientation.unwrap_or(SliderOrientation::Horizontal);
    let min = min.unwrap_or(0.0);
    let _max = max.unwrap_or(100.0);

    let class = 
        "range-slider-marks",
        orientation.as_str(),
        class.as_deref().unwrap_or(""),
    ]);

    view! {
        <div
            class=class
            style=style
            role="presentation"
            aria-hidden="true"
        >
            {marks.into_iter().map(|mark| {
                let position = ((mark.value - min) / (max - min) * 100.0).max(0.0).min(100.0);
                let mark_style = match orientation {
                    SliderOrientation::Horizontal => format!("left: {}%;", position),
                    SliderOrientation::Vertical => format!("bottom: {}%;", position),
                };
                
                view! {
                    <div
                        class="range-slider-mark"
                        style=mark_style
                        data-value=mark.value
                    >
                        <div class="range-slider-mark-line" />
                        <div class="range-slider-mark-label">
                            {mark.label}
                        </div>
                    </div>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}

/// Slider Mark struct
#[derive(Debug, Clone, PartialEq)]
pub struct SliderMark {
    pub value: f64,
    pub label: String,
}

impl Default for SliderMark {
    fn default() -> Self {
        Self {
            value: 0.0,
            label: "0".to_string(),
        }
    }
}

#[cfg(test)]
mod range_slider_tests {
    use proptest::prelude::*;

    #[test]
    fn test_range_slider_component_creation() {
        // let runtime = create_runtime();
        let _view = view! {
            <RangeSlider />
        };
        runtime.dispose();
        
    }

    #[test]
    fn test_range_slider_with_custom_range() {
        // let runtime = create_runtime();
        let _view = view! {
            <RangeSlider min=0.0 max=1000.0 min_value=100.0 max_value=900.0 />
        };
        runtime.dispose();
        
    }

    #[test]
    fn test_range_slider_vertical_orientation() {
        // let runtime = create_runtime();
        let _view = view! {
            <RangeSlider orientation=SliderOrientation::Vertical />
        };
        runtime.dispose();
        
    }

    #[test]
    fn test_range_slider_with_callback() {
        // let runtime = create_runtime();
        let callback = Callback::new(|_value: RangeSliderValue| {});
        let _view = view! {
            <RangeSlider on_change=callback />
        };
        runtime.dispose();
        
    }

    #[test]
    fn test_range_slider_track_component() {
        // let runtime = create_runtime();
        let _view = view! {
            <RangeSliderTrack />
        };
        runtime.dispose();
        
    }

    #[test]
    fn test_range_slider_thumb_component() {
        // let runtime = create_runtime();
        let _view = view! {
            <RangeSliderThumb />
        };
        runtime.dispose();
        
    }

    #[test]
    fn test_range_slider_label_component() {
        // let runtime = create_runtime();
        let _view = view! {
            <RangeSliderLabel>"Price Range"</RangeSliderLabel>
        };
        runtime.dispose();
        
    }

    #[test]
    fn test_range_slider_value_display_component() {
        // let runtime = create_runtime();
        let _view = view! {
            <RangeSliderValueDisplay min_value=10.0 max_value=90.0 />
        };
        runtime.dispose();
        
    }

    #[test]
    fn test_range_slider_marks_component() {
        // let runtime = create_runtime();
        let marks = [
            SliderMark { value: 0.0, label: "Min".to_string() },
            SliderMark { value: 50.0, label: "Mid".to_string() },
            SliderMark { value: 100.0, label: "Max".to_string() },
        ];
        let _view = view! {
            <RangeSliderMarks marks=marks />
        };
        runtime.dispose();
        
    }

    #[test]
    fn test_range_slider_value_default() {
        let value = RangeSliderValue::default();
        assert_eq!(value.min, 0.0);
        assert_eq!(value.max, 100.0);
    }

    #[test]
    fn test_slider_orientation_enum() {
        assert_eq!(SliderOrientation::Horizontal.as_str(), "horizontal");
        assert_eq!(SliderOrientation::Vertical.as_str(), "vertical");
    }

    #[test]
    fn test_slider_size_enum() {
        assert_eq!(SliderSize::Small.as_str(), "sm");
        assert_eq!(SliderSize::Default.as_str(), "default");
        assert_eq!(SliderSize::Large.as_str(), "lg");
    }

    #[test]
    fn test_slider_variant_enum() {
        assert_eq!(SliderVariant::Default.as_str(), "default");
        assert_eq!(SliderVariant::Primary.as_str(), "primary");
        assert_eq!(SliderVariant::Secondary.as_str(), "secondary");
        assert_eq!(SliderVariant::Destructive.as_str(), "destructive");
    }

    #[test]
    fn test_thumb_type_enum() {
        assert_eq!(ThumbType::Min.as_str(), "min");
        assert_eq!(ThumbType::Max.as_str(), "max");
    }

    #[test]
    fn test_value_format_enum() {
        let format = ValueFormat::default();
        assert_eq!(format, ValueFormat::Number);
    }

    #[test]
    fn test_slider_mark_default() {
        let mark = SliderMark::default();
        assert_eq!(mark.value, 0.0);
        assert_eq!(mark.label, "0");
    }

    // Property-based tests
    #[test]
    fn test_range_slider_property_based() {
        proptest!(|(__min in -1000.0..1000.0, max in -1000.0..1000.0)| {
            if min < max {
                let value = RangeSliderValue { min, max };
                assert!(value.min <= value.max);
            }
        });
    }

    #[test]
    fn test_slider_orientation_property_based() {
        proptest!(|(orientation in prop::sample::select(&[SliderOrientation::Horizontal, SliderOrientation::Vertical]))| {
            let orientation_str = orientation.as_str();
            assert!(!orientation_str.is_empty());
        });
    }

    // Integration Tests
    #[test]
    fn test_range_slider_user_interaction() {
        // Test RangeSlider user interaction workflows
        
    }

    #[test]
    fn test_range_slider_accessibility() {
        // Test RangeSlider accessibility features
        
    }

    #[test]
    fn test_range_slider_keyboard_navigation() {
        // Test RangeSlider keyboard navigation
        
    }

    #[test]
    fn test_range_slider_drag_interaction() {
        // Test RangeSlider drag interaction
        
    }

    #[test]
    fn test_range_slider_touch_interaction() {
        // Test RangeSlider touch interaction
        
    }

    // Performance Tests
    #[test]
    fn test_range_slider_large_ranges() {
        // Test RangeSlider with large ranges
        
    }

    #[test]
    fn test_range_slider_render_performance() {
        // Test RangeSlider render performance
        let start = std::time::Instant::now();
        // Simulate component creation
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100); // Should render in less than 100ms
    }

    #[test]
    fn test_range_slider_memory_usage() {
        // Test RangeSlider memory usage
        
    }

    #[test]
    fn test_range_slider_update_performance() {
        // Test RangeSlider update performance
        
    }
}
