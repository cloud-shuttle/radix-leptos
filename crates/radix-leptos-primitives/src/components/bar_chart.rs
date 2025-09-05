use leptos::*;
use leptos::prelude::*;

/// BarChart component - Categorical data display
#[component]
pub fn BarChart(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] data: Option<Vec<BarSeries>>,
    #[prop(optional)] config: Option<BarChartConfig>,
    #[prop(optional)] orientation: Option<BarOrientation>,
    #[prop(optional)] stacked: Option<bool>,
    #[prop(optional)] show_values: Option<bool>,
    #[prop(optional)] show_grid: Option<bool>,
    #[prop(optional)] on_bar_click: Option<Callback<BarData>>,
    #[prop(optional)] on_bar_hover: Option<Callback<BarData>>,
) -> impl IntoView {
    let data = data.unwrap_or_default();
    let config = config.unwrap_or_default();
    let orientation = orientation.unwrap_or_default();
    let stacked = stacked.unwrap_or(false);
    let show_values = show_values.unwrap_or(false);
    let show_grid = show_grid.unwrap_or(true);

    let class = merge_classes([
        "bar-chart",
        &orientation.to_class(),
        if stacked { "stacked" } else { "" },
        if show_values { "show-values" } else { "" },
        if show_grid { "show-grid" } else { "" },
        class.as_deref().unwrap_or(""),
    ]);

    view! {
        <div
            class=class
            style=style
            role="img"
            aria-label="Bar chart visualization"
            data-series-count=data.len()
            data-orientation=orientation.to_string()
            data-stacked=stacked
            data-show-values=show_values
            data-show-grid=show_grid
        >
            {children.map(|c| c())}
        </div>
    }
}

/// Bar Series structure
#[derive(Debug, Clone, PartialEq)]
pub struct BarSeries {
    pub name: String,
    pub data: Vec<BarData>,
    pub color: String,
    pub opacity: f64,
}

impl Default for BarSeries {
    fn default() -> Self {
        Self {
            name: "Series".to_string(),
            data: [],
            color: "#3b82f6".to_string(),
            opacity: 1.0,
        }
    }
}

/// Bar Data structure
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BarData {
    pub category: String,
    pub value: f64,
    pub label: Option<String>,
    pub color: Option<String>,
}

/// Bar Chart Configuration
#[derive(Debug, Clone, PartialEq)]
pub struct BarChartConfig {
    pub width: f64,
    pub height: f64,
    pub margin: ChartMargin,
    pub x_axis: AxisConfig,
    pub y_axis: AxisConfig,
    pub bar_spacing: f64,
    pub group_spacing: f64,
}

impl Default for BarChartConfig {
    fn default() -> Self {
        Self {
            width: 800.0,
            height: 400.0,
            margin: ChartMargin::default(),
            x_axis: AxisConfig::default(),
            y_axis: AxisConfig::default(),
            bar_spacing: 0.1,
            group_spacing: 0.2,
        }
    }
}

/// Chart Margin
#[derive(Debug, Clone, PartialEq)]
pub struct ChartMargin {
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub left: f64,
}

impl Default for ChartMargin {
    fn default() -> Self {
        Self {
            top: 20.0,
            right: 20.0,
            bottom: 40.0,
            left: 40.0,
        }
    }
}

/// Axis Configuration
#[derive(Debug, Clone, PartialEq)]
pub struct AxisConfig {
    pub label: Option<String>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub ticks: Option<usize>,
    pub format: Option<String>,
}

impl Default for AxisConfig {
    fn default() -> Self {
        Self {
            label: None,
            min: None,
            max: None,
            ticks: None,
            format: None,
        }
    }
}

/// Bar Orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BarOrientation {
    #[default]
    Vertical,
    Horizontal,
}

impl BarOrientation {
    pub fn to_class(&self) -> &'static str {
        match self {
            BarOrientation::Vertical => "orientation-vertical",
            BarOrientation::Horizontal => "orientation-horizontal",
        }
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            BarOrientation::Vertical => "vertical",
            BarOrientation::Horizontal => "horizontal",
        }
    }
}

/// Bar Chart Bar component
#[component]
pub fn BarChartBar(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] data: Option<BarData>,
    #[prop(optional)] width: Option<f64>,
    #[prop(optional)] height: Option<f64>,
    #[prop(optional)] on_click: Option<Callback<BarData>>,
) -> impl IntoView {
    let data = data.unwrap_or_default();
    let width = width.unwrap_or(20.0);
    let height = height.unwrap_or(100.0);

    let class = merge_classes([
        "bar-chart-bar",
        class.as_deref().unwrap_or(""),
    ]);

    view! {
        <div
            class=class
            style=style
            role="button"
            aria-label=format!("Bar: {} - {}", data.category, data.value)
            data-category=data.category
            data-value=data.value
            data-width=width
            data-height=height
            tabindex="0"
        />
    }
}

/// Bar Chart Group component
#[component]
pub fn BarChartGroup(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] category: Option<String>,
    #[prop(optional)] bars: Option<Vec<BarData>>,
) -> impl IntoView {
    let category = category.unwrap_or_default();
    let bars = bars.unwrap_or_default();

    let class = merge_classes([
        "bar-chart-group",
        class.as_deref().unwrap_or(""),
    ]);

    view! {
        <div
            class=class
            style=style
            role="group"
            aria-label=format!("Bar group: {}", category)
            data-category=category
            data-bar-count=bars.len()
        >
            {children.map(|c| c())}
        </div>
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
    #[test] fn test_barchart_creation() { 
    #[test] fn test_barchart_with_class() { 
    #[test] fn test_barchart_with_style() { 
    #[test] fn test_barchart_with_data() { 
    #[test] fn test_barchart_with_config() { 
    #[test] fn test_barchart_orientation() { 
    #[test] fn test_barchart_stacked() { 
    #[test] fn test_barchart_show_values() { 
    #[test] fn test_barchart_show_grid() { 
    #[test] fn test_barchart_on_bar_click() { 
    #[test] fn test_barchart_on_bar_hover() { 

    // Bar Series tests
    #[test] fn test_bar_series_default() { 
    #[test] fn test_bar_series_creation() { 

    // Bar Data tests
    #[test] fn test_bar_data_creation() { 

    // Bar Chart Config tests
    #[test] fn test_barchart_config_default() { 
    #[test] fn test_barchart_config_custom() { 

    // Chart Margin tests
    #[test] fn test_chart_margin_default() { 

    // Axis Config tests
    #[test] fn test_axis_config_default() { 
    #[test] fn test_axis_config_custom() { 

    // Bar Orientation tests
    #[test] fn test_bar_orientation_default() { 
    #[test] fn test_bar_orientation_vertical() { 
    #[test] fn test_bar_orientation_horizontal() { 

    // Bar Chart Bar tests
    #[test] fn test_barchart_bar_creation() { 
    #[test] fn test_barchart_bar_with_class() { 
    #[test] fn test_barchart_bar_with_style() { 
    #[test] fn test_barchart_bar_with_data() { 
    #[test] fn test_barchart_bar_width() { 
    #[test] fn test_barchart_bar_height() { 
    #[test] fn test_barchart_bar_on_click() { 

    // Bar Chart Group tests
    #[test] fn test_barchart_group_creation() { 
    #[test] fn test_barchart_group_with_class() { 
    #[test] fn test_barchart_group_with_style() { 
    #[test] fn test_barchart_group_category() { 
    #[test] fn test_barchart_group_bars() { 

    // Helper function tests
    #[test] fn test_merge_classes_empty() { 
    #[test] fn test_merge_classes_single() { 
    #[test] fn test_merge_classes_multiple() { 
    #[test] fn test_merge_classes_with_empty() { 

    // Property-based Tests
    #[test] fn test_barchart_property_based() {
        proptest!(|(__class in ".*", _style in ".*")| {
            
        });
    }

    #[test] fn test_barchart_data_validation() {
        proptest!(|(___series_count in 0..10usize, _bars_per_series in 0..50usize)| {
            
        });
    }

    #[test] fn test_barchart_config_validation() {
        proptest!(|(__width in 100.0..2000.0f64, _height in 100.0..2000.0f64)| {
            
        });
    }

    #[test] fn test_barchart_orientation_property_based() {
        proptest!(|(__orientation_index in 0..2usize)| {
            
        });
    }

    // Integration Tests
    #[test] fn test_barchart_tooltip_interaction() { 
    #[test] fn test_barchart_legend_interaction() { 
    #[test] fn test_barchart_user_workflow() { 
    #[test] fn test_barchart_accessibility_workflow() { 
    #[test] fn test_barchart_with_other_components() { 

    // Performance Tests
    #[test] fn test_barchart_large_dataset() { 
    #[test] fn test_barchart_render_performance() { 
    #[test] fn test_barchart_memory_usage() { 
    #[test] fn test_barchart_animation_performance() { 
}
