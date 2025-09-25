
/// Gauge component - Metric display
#[component]
pub fn Gauge(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] value: Option<f64>,
    #[prop(optional)] min: Option<f64>,
    #[prop(optional)] max: Option<f64>,
    #[prop(optional)] config: Option<GaugeConfig>,
    #[prop(optional)] show_value: Option<bool>,
    #[prop(optional)] show_ticks: Option<bool>,
    #[prop(optional)] animated: Option<bool>,
    #[prop(optional)] on_value_change: Option<Callback<f64>>,
) -> impl IntoView {
    let value = value.unwrap_or(0.0);
    let min = min.unwrap_or(0.0);
    let _max = max.unwrap_or(100.0);
    let config = config.unwrap_or_default();
    let show_value = show_value.unwrap_or(true);
    let show_ticks = show_ticks.unwrap_or(true);
    let animated = animated.unwrap_or(true);

    let class = merge_classes(vec![
        "gauge",
        </div>
    }
}

/// Gauge Configuration
#[derive(Debug, Clone, PartialEq)]
pub struct GaugeConfig {
    pub width: f64,
    pub height: f64,
    pub start_angle: f64,
    pub end_angle: f64,
    pub stroke_width: f64,
    pub colors: GaugeColors,
    pub animation: AnimationConfig,
}

impl Default for GaugeConfig {
    fn default() -> Self {
        Self {
            width: 200.0,
            height: 200.0,
            start_angle: -135.0,
            end_angle: 135.0,
            stroke_width: 20.0,
            colors: GaugeColors::default(),
            animation: AnimationConfig::default(),
        }
    }
}

/// Gauge Colors
#[derive(Debug, Clone, PartialEq)]
pub struct GaugeColors {
    pub background: String,
    pub fill: String,
    pub text: String,
    pub tick: String,
}

impl Default for GaugeColors {
    fn default() -> Self {
        Self {
            background: "#e5e7eb".to_string(),
            fill: "#3b82f6".to_string(),
            text: "#374151".to_string(),
            tick: "#6b7280".to_string(),
        }
    }
}

/// Animation Configuration
#[derive(Debug, Clone, PartialEq)]
pub struct AnimationConfig {
    pub duration: f64,
    pub easing: EasingType,
    pub delay: f64,
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            duration: 1000.0,
            easing: EasingType::EaseInOut,
            delay: 0.0,
        }
    }
}

/// Easing Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EasingType {
    #[default]
    EaseInOut,
    EaseIn,
    EaseOut,
    Linear,
}

impl EasingType {
    pub fn to_class(&self) -> &'static str {
        match self {
            EasingType::EaseInOut => "ease-in-out",
            EasingType::EaseIn => "ease-in",
            EasingType::EaseOut => "ease-out",
            EasingType::Linear => "linear",
        }
    }
}

/// Gauge Arc component
#[component]
pub fn GaugeArc(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] start_angle: Option<f64>,
    #[prop(optional)] end_angle: Option<f64>,
    #[prop(optional)] stroke_width: Option<f64>,
    #[prop(optional)] color: Option<String>,
) -> impl IntoView {
    let start_angle = start_angle.unwrap_or(-135.0);
    let end_angle = end_angle.unwrap_or(135.0);
    let stroke_width = stroke_width.unwrap_or(20.0);
    let color = color.unwrap_or_default();

    let class = merge_classes(vec![
        "gauge-arc",
        class.as_deref().unwrap_or(""),
    ]);

    view! {
        <div
            class=class
            style=style
            role="presentation"
            aria-hidden="true"
            data-start-angle=start_angle
            data-end-angle=end_angle
            data-stroke-width=stroke_width
            data-color=color
        />
    }
}

/// Gauge Needle component
#[component]
pub fn GaugeNeedle(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] angle: Option<f64>,
    #[prop(optional)] length: Option<f64>,
    #[prop(optional)] color: Option<String>,
) -> impl IntoView {
    let angle = angle.unwrap_or(0.0);
    let length = length.unwrap_or(80.0);
    let color = color.unwrap_or_default();

    let class = merge_classes(vec![
        "gauge-needle",
        class.as_deref().unwrap_or(""),
    ]);

    view! {
        <div
            class=class
            style=style
            role="presentation"
            aria-hidden="true"
            data-angle=angle
            data-length=length
            data-color=color
        />
    }
}

/// Gauge Tick component
#[component]
pub fn GaugeTick(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] angle: Option<f64>,
    #[prop(optional)] length: Option<f64>,
    #[prop(optional)] color: Option<String>,
    #[prop(optional)] label: Option<String>,
) -> impl IntoView {
    let angle = angle.unwrap_or(0.0);
    let length = length.unwrap_or(10.0);
    let color = color.unwrap_or_default();
    let label = label.unwrap_or_default();

    let class = merge_classes(vec![
        "gauge-tick",
        class.as_deref().unwrap_or(""),
    ]);

    view! {
        <div
            class=class
            style=style
            role="presentation"
            aria-hidden="true"
            data-angle=angle
            data-length=length
            data-color=color
            data-label=label
        />
    }
}

/// Gauge Value component
#[component]
pub fn GaugeValue(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] value: Option<f64>,
    #[prop(optional)] unit: Option<String>,
    #[prop(optional)] format: Option<String>,
) -> impl IntoView {
    let value = value.unwrap_or(0.0);
    let unit = unit.unwrap_or_default();
    let format = format.unwrap_or_default();

    let class = merge_classes(vec![
        "gauge-value",
        class.as_deref().unwrap_or(""),
    ]);

    view! {
        <div
            class=class
            style=style
            role="text"
            aria-label="Gauge value"
            data-value=value
            data-unit=unit
            data-format=format
        >
            {children.map(|c| c())}
        </div>
    }
}

/// Helper function to merge CSS classes
fn merge_optional_classes(classes: Vec<&str>) -> String {
    classes
        .into_iter()
        .filter(|c| !c.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;
    use proptest::prelude::*;
use crate::utils::merge_classes;

    wasm_bindgen_test_configure!(run_in_browser);

    // Unit Tests
    #[test] fn test_gauge_creation() { 
    #[test] fn test_gauge_with_class() { 
    #[test] fn test_gauge_with_style() { 
    #[test] fn test_gauge_with_value() { 
    #[test] fn test_gauge_with_min() { 
    #[test] fn test_gauge_with_max() { 
    #[test] fn test_gauge_with_config() { 
    #[test] fn test_gauge_show_value() { 
    #[test] fn test_gauge_show_ticks() { 
    #[test] fn test_gauge_animated() { 
    #[test] fn test_gauge_on_value_change() { 

    // Gauge Config tests
    #[test] fn test_gauge_config_default() { 
    #[test] fn test_gauge_config_custom() { 

    // Gauge Colors tests
    #[test] fn test_gauge_colors_default() { 
    #[test] fn test_gauge_colors_custom() { 

    // Animation Config tests
    #[test] fn test_animation_config_default() { 
    #[test] fn test_animation_config_custom() { 

    // Easing Type tests
    #[test] fn test_easing_type_default() { 
    #[test] fn test_easing_type_ease_in_out() { 
    #[test] fn test_easing_type_ease_in() { 
    #[test] fn test_easing_type_ease_out() { 
    #[test] fn test_easing_type_linear() { 

    // Gauge Arc tests
    #[test] fn test_gauge_arc_creation() { 
    #[test] fn test_gauge_arc_with_class() { 
    #[test] fn test_gauge_arc_with_style() { 
    #[test] fn test_gauge_arc_start_angle() { 
    #[test] fn test_gauge_arc_end_angle() { 
    #[test] fn test_gauge_arc_stroke_width() { 
    #[test] fn test_gauge_arc_color() { 

    // Gauge Needle tests
    #[test] fn test_gauge_needle_creation() { 
    #[test] fn test_gauge_needle_with_class() { 
    #[test] fn test_gauge_needle_with_style() { 
    #[test] fn test_gauge_needle_angle() { 
    #[test] fn test_gauge_needle_length() { 
    #[test] fn test_gauge_needle_color() { 

    // Gauge Tick tests
    #[test] fn test_gauge_tick_creation() { 
    #[test] fn test_gauge_tick_with_class() { 
    #[test] fn test_gauge_tick_with_style() { 
    #[test] fn test_gauge_tick_angle() { 
    #[test] fn test_gauge_tick_length() { 
    #[test] fn test_gauge_tick_color() { 
    #[test] fn test_gauge_tick_label() { 

    // Gauge Value tests
    #[test] fn test_gauge_value_creation() { 
    #[test] fn test_gauge_value_with_class() { 
    #[test] fn test_gauge_value_with_style() { 
    #[test] fn test_gauge_value_value() { 
    #[test] fn test_gauge_value_unit() { 
    #[test] fn test_gauge_value_format() { 

    // Helper function tests
    #[test] fn test_merge_classes_empty() { 
    #[test] fn test_merge_classes_single() { 
    #[test] fn test_merge_classes_multiple() { 
    #[test] fn test_merge_classes_with_empty() { 

    // Property-based Tests
    #[test] fn test_gauge_property_based() {
        proptest!(|(____class in ".*", __style in ".*")| {
            
        });
    }

    #[test] fn test_gauge_value_validation() {
        proptest!(|(____value in 0.0..100.0f64, __min in 0.0..50.0f64, __max in 50.0..200.0f64)| {
            
        });
    }

    #[test] fn test_gauge_config_validation() {
        proptest!(|(____width in 100.0..500.0f64, __height in 100.0..500.0f64)| {
            
        });
    }

    #[test] fn test_gauge_angle_property_based() {
        proptest!(|(__start_angle in -180.0..0.0f64, __end_angle in 0.0..180.0f64)| {
            
        });
    }

    // Integration Tests
    #[test] fn test_gauge_user_interaction() { 
    #[test] fn test_gauge_accessibility() { 
    #[test] fn test_gauge_keyboard_navigation() { 
    #[test] fn test_gauge_value_formatting() { 
    #[test] fn test_gauge_animation_behavior() { 

    // Performance Tests
    #[test] fn test_gauge_render_performance() { 
    #[test] fn test_gauge_memory_usage() { 
    #[test] fn test_gauge_animation_performance() { 
    #[test] fn test_gauge_value_update_performance() { 
}
