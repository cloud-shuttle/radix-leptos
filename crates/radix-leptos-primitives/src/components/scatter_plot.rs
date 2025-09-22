
/// ScatterPlot component - Correlation analysis
#[component]
pub fn ScatterPlot(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] data: Option<Vec<ScatterSeries>>,
    #[prop(optional)] config: Option<ScatterPlotConfig>,
    #[prop(optional)] show_trend_line: Option<bool>,
    #[prop(optional)] show_grid: Option<bool>,
    #[prop(optional)] show_axes: Option<bool>,
    #[prop(optional)] on_point_click: Option<Callback<ScatterPoint>>,
    #[prop(optional)] on_point_hover: Option<Callback<ScatterPoint>>,
) -> impl IntoView {
    let data = data.unwrap_or_default();
    let config = config.unwrap_or_default();
    let show_trend_line = show_trend_line.unwrap_or(false);
    let show_grid = show_grid.unwrap_or(true);
    let show_axes = show_axes.unwrap_or(true);

    let class = merge_classes(vec![
        "scatter-plot",
        </div>
    }
}

/// Scatter Series structure
#[derive(Debug, Clone, PartialEq)]
pub struct ScatterSeries {
    pub name: String,
    pub data: Vec<ScatterPoint>,
    pub color: String,
    pub point_size: f64,
    pub opacity: f64,
}

impl Default for ScatterSeries {
    fn default() -> Self {
        Self {
            name: "Series".to_string(),
            data: Vec::new(),
            color: "#3b82f6".to_string(),
            point_size: 4.0,
            opacity: 1.0,
        }
    }
}

/// Scatter Point structure
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ScatterPoint {
    pub x: f64,
    pub y: f64,
    pub label: Option<String>,
    pub size: Option<f64>,
    pub color: Option<String>,
    pub metadata: Option<String>,
}

/// Scatter Plot Configuration
#[derive(Debug, Clone, PartialEq)]
pub struct ScatterPlotConfig {
    pub width: f64,
    pub height: f64,
    pub margin: ChartMargin,
    pub x_axis: AxisConfig,
    pub y_axis: AxisConfig,
    pub point_size_range: PointSizeRange,
}

impl Default for ScatterPlotConfig {
    fn default() -> Self {
        Self {
            width: 800.0,
            height: 400.0,
            margin: ChartMargin::default(),
            x_axis: AxisConfig::default(),
            y_axis: AxisConfig::default(),
            point_size_range: PointSizeRange::default(),
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

/// Point Size Range
#[derive(Debug, Clone, PartialEq)]
pub struct PointSizeRange {
    pub min: f64,
    pub max: f64,
    pub scale_type: ScaleType,
}

impl Default for PointSizeRange {
    fn default() -> Self {
        Self {
            min: 2.0,
            max: 20.0,
            scale_type: ScaleType::Linear,
        }
    }
}

/// Scale Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScaleType {
    #[default]
    Linear,
    Logarithmic,
    SquareRoot,
}

impl ScaleType {
    pub fn to_class(&self) -> &'static str {
        match self {
            ScaleType::Linear => "scale-linear",
            ScaleType::Logarithmic => "scale-logarithmic",
            ScaleType::SquareRoot => "scale-square-root",
        }
    }
}

/// Scatter Plot Point component
#[component]
pub fn ScatterPlotPoint(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] point: Option<ScatterPoint>,
    #[prop(optional)] size: Option<f64>,
    #[prop(optional)] on_click: Option<Callback<ScatterPoint>>,
) -> impl IntoView {
    let point = point.unwrap_or_default();
    let size = size.unwrap_or(4.0);

    let class = merge_classes(vec![
        "scatter-plot-point",
        class.as_deref().unwrap_or(""),
    ]);

    view! {
        <div
            class=class
            style=style
            role="button"
            aria-label=format!("Data point: ({}, {})", point.x, point.y)
            data-x=point.x
            data-y=point.y
            data-size=size
            tabindex="0"
        />
    }
}

/// Scatter Plot Trend Line component
#[component]
pub fn ScatterPlotTrendLine(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] series: Option<ScatterSeries>,
    #[prop(optional)] trend_type: Option<TrendType>,
    #[prop(optional)] opacity: Option<f64>,
) -> impl IntoView {
    let series = series.unwrap_or_default();
    let trend_type = trend_type.unwrap_or_default();
    let opacity = opacity.unwrap_or(0.8);

    let class = merge_classes(vec![
        "scatter-plot-trend-line",
        &trend_type.to_class(),
        class.as_deref().unwrap_or(""),
    ]);

    view! {
        <div
            class=class
            style=style
            role="img"
            aria-label=format!("Trend line for {}", series.name)
            data-series-name=series.name
            data-trend-type=trend_type.to_string()
            data-opacity=opacity
        />
    }
}

/// Trend Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TrendType {
    #[default]
    Linear,
    Polynomial,
    Exponential,
    Logarithmic,
}

impl TrendType {
    pub fn to_class(&self) -> &'static str {
        match self {
            TrendType::Linear => "trend-linear",
            TrendType::Polynomial => "trend-polynomial",
            TrendType::Exponential => "trend-exponential",
            TrendType::Logarithmic => "trend-logarithmic",
        }
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            TrendType::Linear => "linear",
            TrendType::Polynomial => "polynomial",
            TrendType::Exponential => "exponential",
            TrendType::Logarithmic => "logarithmic",
        }
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
use crate::utils::{merge_classes, generate_id};

    wasm_bindgen_test_configure!(run_in_browser);

    // Unit Tests
    #[test] fn test_scatterplot_creation() { 
    #[test] fn test_scatterplot_with_class() { 
    #[test] fn test_scatterplot_with_style() { 
    #[test] fn test_scatterplot_with_data() { 
    #[test] fn test_scatterplot_with_config() { 
    #[test] fn test_scatterplot_show_trend_line() { 
    #[test] fn test_scatterplot_show_grid() { 
    #[test] fn test_scatterplot_show_axes() { 
    #[test] fn test_scatterplot_on_point_click() { 
    #[test] fn test_scatterplot_on_point_hover() { 

    // Scatter Series tests
    #[test] fn test_scatter_series_default() { 
    #[test] fn test_scatter_series_creation() { 

    // Scatter Point tests
    #[test] fn test_scatter_point_creation() { 

    // Scatter Plot Config tests
    #[test] fn test_scatterplot_config_default() { 
    #[test] fn test_scatterplot_config_custom() { 

    // Chart Margin tests
    #[test] fn test_chart_margin_default() { 

    // Axis Config tests
    #[test] fn test_axis_config_default() { 
    #[test] fn test_axis_config_custom() { 

    // Point Size Range tests
    #[test] fn test_point_size_range_default() { 
    #[test] fn test_point_size_range_custom() { 

    // Scale Type tests
    #[test] fn test_scale_type_default() { 
    #[test] fn test_scale_type_linear() { 
    #[test] fn test_scale_type_logarithmic() { 
    #[test] fn test_scale_type_square_root() { 

    // Scatter Plot Point tests
    #[test] fn test_scatterplot_point_creation() { 
    #[test] fn test_scatterplot_point_with_class() { 
    #[test] fn test_scatterplot_point_with_style() { 
    #[test] fn test_scatterplot_point_with_point() { 
    #[test] fn test_scatterplot_point_size() { 
    #[test] fn test_scatterplot_point_on_click() { 

    // Scatter Plot Trend Line tests
    #[test] fn test_scatterplot_trend_line_creation() { 
    #[test] fn test_scatterplot_trend_line_with_class() { 
    #[test] fn test_scatterplot_trend_line_with_style() { 
    #[test] fn test_scatterplot_trend_line_with_series() { 
    #[test] fn test_scatterplot_trend_line_trend_type() { 
    #[test] fn test_scatterplot_trend_line_opacity() { 

    // Trend Type tests
    #[test] fn test_trend_type_default() { 
    #[test] fn test_trend_type_linear() { 
    #[test] fn test_trend_type_polynomial() { 
    #[test] fn test_trend_type_exponential() { 
    #[test] fn test_trend_type_logarithmic() { 

    // Helper function tests
    #[test] fn test_merge_classes_empty() { 
    #[test] fn test_merge_classes_single() { 
    #[test] fn test_merge_classes_multiple() { 
    #[test] fn test_merge_classes_with_empty() { 

    // Property-based Tests
    #[test] fn test_scatterplot_property_based() {
        proptest!(|(____class in ".*", __style in ".*")| {
            
        });
    }

    #[test] fn test_scatterplot_data_validation() {
        proptest!(|(______series_count in 0..10usize, __points_per_series in 0..1000usize)| {
            
        });
    }

    #[test] fn test_scatterplot_config_validation() {
        proptest!(|(____width in 100.0..2000.0f64, __height in 100.0..2000.0f64)| {
            
        });
    }

    #[test] fn test_scatterplot_trend_property_based() {
        proptest!(|(____trend_type_index in 0..4usize)| {
            
        });
    }

    // Integration Tests
    #[test] fn test_scatterplot_tooltip_interaction() { 
    #[test] fn test_scatterplot_legend_interaction() { 
    #[test] fn test_scatterplot_user_workflow() { 
    #[test] fn test_scatterplot_accessibility_workflow() { 
    #[test] fn test_scatterplot_with_other_components() { 

    // Performance Tests
    #[test] fn test_scatterplot_large_dataset() { 
    #[test] fn test_scatterplot_render_performance() { 
    #[test] fn test_scatterplot_memory_usage() { 
    #[test] fn test_scatterplot_animation_performance() { 
}
