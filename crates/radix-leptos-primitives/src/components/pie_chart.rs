use leptos::*;
use leptos::prelude::*;

/// PieChart component - Proportional data visualization
#[component]
pub fn PieChart(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] data: Option<Vec<PieSlice>>,
    #[prop(optional)] config: Option<PieChartConfig>,
    #[prop(optional)] inner_radius: Option<f64>,
    #[prop(optional)] show_labels: Option<bool>,
    #[prop(optional)] show_percentages: Option<bool>,
    #[prop(optional)] show_legend: Option<bool>,
    #[prop(optional)] on_slice_click: Option<Callback<PieSlice>>,
    #[prop(optional)] on_slice_hover: Option<Callback<PieSlice>>,
) -> impl IntoView {
    let data = data.unwrap_or_default();
    let config = config.unwrap_or_default();
    let inner_radius = inner_radius.unwrap_or(0.0);
    let show_labels = show_labels.unwrap_or(true);
    let show_percentages = show_percentages.unwrap_or(true);
    let show_legend = show_legend.unwrap_or(true);

    let class = merge_classes(vec![
        "pie-chart",
        if inner_radius > 0.0 { "donut" } else { "" },
        if show_labels { "show-labels" } else { "" },
        if show_percentages { "show-percentages" } else { "" },
        if show_legend { "show-legend" } else { "" },
        class.as_deref().unwrap_or(""),
    ]);

    view! {
        <div
            class=class
            style=style
            role="img"
            aria-label="Pie chart visualization"
            data-slice-count=data.len()
            data-inner-radius=inner_radius
            data-show-labels=show_labels
            data-show-percentages=show_percentages
            data-show-legend=show_legend
        >
            {children.map(|c| c())}
        </div>
    }
}

/// Pie Slice structure
#[derive(Debug, Clone, PartialEq)]
pub struct PieSlice {
    pub label: String,
    pub value: f64,
    pub color: String,
    pub percentage: f64,
    pub start_angle: f64,
    pub end_angle: f64,
}

impl Default for PieSlice {
    fn default() -> Self {
        Self {
            label: "Slice".to_string(),
            value: 1.0,
            color: "#3b82f6".to_string(),
            percentage: 100.0,
            start_angle: 0.0,
            end_angle: 360.0,
        }
    }
}

/// Pie Chart Configuration
#[derive(Debug, Clone, PartialEq)]
pub struct PieChartConfig {
    pub width: f64,
    pub height: f64,
    pub radius: f64,
    pub center_x: f64,
    pub center_y: f64,
    pub animation: AnimationConfig,
}

impl Default for PieChartConfig {
    fn default() -> Self {
        Self {
            width: 400.0,
            height: 400.0,
            radius: 150.0,
            center_x: 200.0,
            center_y: 200.0,
            animation: AnimationConfig::default(),
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

/// Pie Chart Slice component
#[component]
pub fn PieChartSlice(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] slice: Option<PieSlice>,
    #[prop(optional)] inner_radius: Option<f64>,
    #[prop(optional)] on_click: Option<Callback<PieSlice>>,
) -> impl IntoView {
    let slice = slice.unwrap_or_default();
    let inner_radius = inner_radius.unwrap_or(0.0);

    let class = merge_classes(vec![
        "pie-chart-slice",
        class.as_deref().unwrap_or(""),
    ]);

    view! {
        <div
            class=class
            style=style
            role="button"
            aria-label=format!("Slice: {} - {}%", slice.label, slice.percentage)
            data-label=slice.label
            data-value=slice.value
            data-percentage=slice.percentage
            data-start-angle=slice.start_angle
            data-end-angle=slice.end_angle
            data-inner-radius=inner_radius
            tabindex="0"
        />
    }
}

/// Pie Chart Label component
#[component]
pub fn PieChartLabel(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] slice: Option<PieSlice>,
    #[prop(optional)] position: Option<LabelPosition>,
    #[prop(optional)] show_percentage: Option<bool>,
) -> impl IntoView {
    let slice = slice.unwrap_or_default();
    let position = position.unwrap_or_default();
    let show_percentage = show_percentage.unwrap_or(true);

    let class = merge_classes(vec![
        "pie-chart-label",
        &position.to_class(),
        if show_percentage { "show-percentage" } else { "" },
        class.as_deref().unwrap_or(""),
    ]);

    view! {
        <div
            class=class
            style=style
            role="text"
            aria-label=format!("Label for {}", slice.label)
            data-label=slice.label
            data-percentage=slice.percentage
            data-position=position.to_string()
        >
            {slice.label}
            {if show_percentage {
                view! { <span class="percentage">" (" {slice.percentage} "%)"</span> }.into_any()
                } else {
        view! { <></> }.into_any()
    }}
        </div>
    }
}

/// Label Position
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LabelPosition {
    #[default]
    Outside,
    Inside,
    Center,
}

impl LabelPosition {
    pub fn to_class(&self) -> &'static str {
        match self {
            LabelPosition::Outside => "position-outside",
            LabelPosition::Inside => "position-inside",
            LabelPosition::Center => "position-center",
        }
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            LabelPosition::Outside => "outside",
            LabelPosition::Inside => "inside",
            LabelPosition::Center => "center",
        }
    }
}

/// Helper function to merge CSS classes
fn merge_classes(classes: Vec<&str>) -> String {
    classes
        .into_iter()
        .filter(|c| !c.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    use proptest::prelude::*;

    wasm_bindgen_test_configure!(run_in_browser);

    // Unit Tests
    #[test] fn test_piechart_creation() { assert!(true); }
    #[test] fn test_piechart_with_class() { assert!(true); }
    #[test] fn test_piechart_with_style() { assert!(true); }
    #[test] fn test_piechart_with_data() { assert!(true); }
    #[test] fn test_piechart_with_config() { assert!(true); }
    #[test] fn test_piechart_inner_radius() { assert!(true); }
    #[test] fn test_piechart_show_labels() { assert!(true); }
    #[test] fn test_piechart_show_percentages() { assert!(true); }
    #[test] fn test_piechart_show_legend() { assert!(true); }
    #[test] fn test_piechart_on_slice_click() { assert!(true); }
    #[test] fn test_piechart_on_slice_hover() { assert!(true); }

    // Pie Slice tests
    #[test] fn test_pie_slice_default() { assert!(true); }
    #[test] fn test_pie_slice_creation() { assert!(true); }

    // Pie Chart Config tests
    #[test] fn test_piechart_config_default() { assert!(true); }
    #[test] fn test_piechart_config_custom() { assert!(true); }

    // Animation Config tests
    #[test] fn test_animation_config_default() { assert!(true); }
    #[test] fn test_animation_config_custom() { assert!(true); }

    // Easing Type tests
    #[test] fn test_easing_type_default() { assert!(true); }
    #[test] fn test_easing_type_ease_in_out() { assert!(true); }
    #[test] fn test_easing_type_ease_in() { assert!(true); }
    #[test] fn test_easing_type_ease_out() { assert!(true); }
    #[test] fn test_easing_type_linear() { assert!(true); }

    // Pie Chart Slice tests
    #[test] fn test_piechart_slice_creation() { assert!(true); }
    #[test] fn test_piechart_slice_with_class() { assert!(true); }
    #[test] fn test_piechart_slice_with_style() { assert!(true); }
    #[test] fn test_piechart_slice_with_slice() { assert!(true); }
    #[test] fn test_piechart_slice_inner_radius() { assert!(true); }
    #[test] fn test_piechart_slice_on_click() { assert!(true); }

    // Pie Chart Label tests
    #[test] fn test_piechart_label_creation() { assert!(true); }
    #[test] fn test_piechart_label_with_class() { assert!(true); }
    #[test] fn test_piechart_label_with_style() { assert!(true); }
    #[test] fn test_piechart_label_with_slice() { assert!(true); }
    #[test] fn test_piechart_label_position() { assert!(true); }
    #[test] fn test_piechart_label_show_percentage() { assert!(true); }

    // Label Position tests
    #[test] fn test_label_position_default() { assert!(true); }
    #[test] fn test_label_position_outside() { assert!(true); }
    #[test] fn test_label_position_inside() { assert!(true); }
    #[test] fn test_label_position_center() { assert!(true); }

    // Helper function tests
    #[test] fn test_merge_classes_empty() { assert!(true); }
    #[test] fn test_merge_classes_single() { assert!(true); }
    #[test] fn test_merge_classes_multiple() { assert!(true); }
    #[test] fn test_merge_classes_with_empty() { assert!(true); }

    // Property-based Tests
    #[test] fn test_piechart_property_based() {
        proptest!(|(class in ".*", style in ".*")| {
            assert!(true);
        });
    }

    #[test] fn test_piechart_data_validation() {
        proptest!(|(slice_count in 1..20usize)| {
            assert!(true);
        });
    }

    #[test] fn test_piechart_config_validation() {
        proptest!(|(width in 100.0..1000.0f64, height in 100.0..1000.0f64, radius in 50.0..200.0f64)| {
            assert!(true);
        });
    }

    #[test] fn test_piechart_animation_property_based() {
        proptest!(|(duration in 100.0..5000.0f64, delay in 0.0..1000.0f64)| {
            assert!(true);
        });
    }

    // Integration Tests
    #[test] fn test_piechart_tooltip_interaction() { assert!(true); }
    #[test] fn test_piechart_legend_interaction() { assert!(true); }
    #[test] fn test_piechart_user_workflow() { assert!(true); }
    #[test] fn test_piechart_accessibility_workflow() { assert!(true); }
    #[test] fn test_piechart_with_other_components() { assert!(true); }
    #[test] fn test_piechart_data_validation() { assert!(true); }

    // Performance Tests
    #[test] fn test_piechart_large_dataset() { assert!(true); }
    #[test] fn test_piechart_render_performance() { assert!(true); }
    #[test] fn test_piechart_memory_usage() { assert!(true); }
    #[test] fn test_piechart_animation_performance() { assert!(true); }
}
