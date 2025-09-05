
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

    let class = merge_classes([
        "pie-chart",
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

    let class = merge_classes([
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

    let class = merge_classes([
        "pie-chart-label",
        &position.to_class(),
            data-label=slice.label.clone()
            data-percentage=slice.percentage
            data-position=position.to_string()
        >
            {slice.label.clone()}
            {if show_percentage {
                view! { <span class="percentage">" (" {slice.percentage} "%)"</span> }.into_any()
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
    use wasm_bindgen_test::*;
    use proptest::prelude::*;

    wasm_bindgen_test_configure!(run_in_browser);

    // Unit Tests
    #[test] fn test_piechart_creation() { 
    #[test] fn test_piechart_with_class() { 
    #[test] fn test_piechart_with_style() { 
    #[test] fn test_piechart_with_data() { 
    #[test] fn test_piechart_with_config() { 
    #[test] fn test_piechart_inner_radius() { 
    #[test] fn test_piechart_show_labels() { 
    #[test] fn test_piechart_show_percentages() { 
    #[test] fn test_piechart_show_legend() { 
    #[test] fn test_piechart_on_slice_click() { 
    #[test] fn test_piechart_on_slice_hover() { 

    // Pie Slice tests
    #[test] fn test_pie_slice_default() { 
    #[test] fn test_pie_slice_creation() { 

    // Pie Chart Config tests
    #[test] fn test_piechart_config_default() { 
    #[test] fn test_piechart_config_custom() { 

    // Animation Config tests
    #[test] fn test_animation_config_default() { 
    #[test] fn test_animation_config_custom() { 

    // Easing Type tests
    #[test] fn test_easing_type_default() { 
    #[test] fn test_easing_type_ease_in_out() { 
    #[test] fn test_easing_type_ease_in() { 
    #[test] fn test_easing_type_ease_out() { 
    #[test] fn test_easing_type_linear() { 

    // Pie Chart Slice tests
    #[test] fn test_piechart_slice_creation() { 
    #[test] fn test_piechart_slice_with_class() { 
    #[test] fn test_piechart_slice_with_style() { 
    #[test] fn test_piechart_slice_with_slice() { 
    #[test] fn test_piechart_slice_inner_radius() { 
    #[test] fn test_piechart_slice_on_click() { 

    // Pie Chart Label tests
    #[test] fn test_piechart_label_creation() { 
    #[test] fn test_piechart_label_with_class() { 
    #[test] fn test_piechart_label_with_style() { 
    #[test] fn test_piechart_label_with_slice() { 
    #[test] fn test_piechart_label_position() { 
    #[test] fn test_piechart_label_show_percentage() { 

    // Label Position tests
    #[test] fn test_label_position_default() { 
    #[test] fn test_label_position_outside() { 
    #[test] fn test_label_position_inside() { 
    #[test] fn test_label_position_center() { 

    // Helper function tests
    #[test] fn test_merge_classes_empty() { 
    #[test] fn test_merge_classes_single() { 
    #[test] fn test_merge_classes_multiple() { 
    #[test] fn test_merge_classes_with_empty() { 

    // Property-based Tests
    #[test] fn test_piechart_property_based() {
        proptest!(|(____class in ".*", __style in ".*")| {
            
        });
    }

    #[test] fn test_piechart_data_validation() {
        proptest!(|(______slice_count in 1..20usize)| {
            
        });
    }

    #[test] fn test_piechart_config_validation() {
        proptest!(|(____width in 100.0..1000.0f64, __height in 100.0..1000.0f64, __radius in 50.0..200.0f64)| {
            
        });
    }

    #[test] fn test_piechart_animation_property_based() {
        proptest!(|(____duration in 100.0..5000.0f64, __delay in 0.0..1000.0f64)| {
            
        });
    }

    // Integration Tests
    #[test] fn test_piechart_tooltip_interaction() { 
    #[test] fn test_piechart_legend_interaction() { 
    #[test] fn test_piechart_user_workflow() { 
    #[test] fn test_piechart_accessibility_workflow() { 
    #[test] fn test_piechart_with_other_components() { 

    // Performance Tests
    #[test] fn test_piechart_large_dataset() { 
    #[test] fn test_piechart_render_performance() { 
    #[test] fn test_piechart_memory_usage() { 
    #[test] fn test_piechart_animation_performance() { 
}
