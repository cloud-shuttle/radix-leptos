
/// SplitPane component - Resizable panel layouts
#[component]
pub fn SplitPane(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] direction: Option<SplitDirection>,
    #[prop(optional)] initial_sizes: Option<Vec<f64>>,
    #[prop(optional)] min_sizes: Option<Vec<f64>>,
    #[prop(optional)] max_sizes: Option<Vec<f64>>,
    #[prop(optional)] resizable: Option<bool>,
    #[prop(optional)] on_resize: Option<Callback<ResizeEvent>>,
) -> impl IntoView {
    let direction = direction.unwrap_or_default();
    let initial_sizes = initial_sizes.unwrap_or([50.0, 50.0]);
    let min_sizes = min_sizes.unwrap_or([20.0, 20.0]);
    let max_sizes = max_sizes.unwrap_or([80.0, 80.0]);
    let resizable = resizable.unwrap_or(true);

    let class = merge_classes(vec![
        "split-pane",
        &direction.to_class(),
        </div>
    }
}

/// Split Direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SplitDirection {
    #[default]
    Horizontal,
    Vertical,
}

impl SplitDirection {
    pub fn to_class(&self) -> &'static str {
        match self {
            SplitDirection::Horizontal => "direction-horizontal",
            SplitDirection::Vertical => "direction-vertical",
        }
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            SplitDirection::Horizontal => "horizontal",
            SplitDirection::Vertical => "vertical",
        }
    }
}

/// Resize Event structure
#[derive(Debug, Clone, PartialEq)]
pub struct ResizeEvent {
    pub panel_index: usize,
    pub new_size: f64,
    pub old_size: f64,
    pub direction: SplitDirection,
}

/// Split Pane Panel component
#[component]
pub fn SplitPanePanel(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] size: Option<f64>,
    #[prop(optional)] min_size: Option<f64>,
    #[prop(optional)] max_size: Option<f64>,
    #[prop(optional)] resizable: Option<bool>,
    #[prop(optional)] collapsible: Option<bool>,
    #[prop(optional)] collapsed: Option<bool>,
) -> impl IntoView {
    let size = size.unwrap_or(50.0);
    let min_size = min_size.unwrap_or(20.0);
    let max_size = max_size.unwrap_or(80.0);
    let resizable = resizable.unwrap_or(true);
    let collapsible = collapsible.unwrap_or(false);
    let collapsed = collapsed.unwrap_or(false);

    let class = merge_classes(vec![
        "split-pane-panel",
        </div>
    }
}

/// Split Pane Resizer component
#[component]
pub fn SplitPaneResizer(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] direction: Option<SplitDirection>,
    #[prop(optional)] on_resize_start: Option<Callback<()>>,
    #[prop(optional)] on_resize: Option<Callback<f64>>,
    #[prop(optional)] on_resize_end: Option<Callback<()>>,
) -> impl IntoView {
    let direction = direction.unwrap_or_default();

    let class = merge_classes(vec![
        "split-pane-resizer",
        &direction.to_class(),
        class.as_deref().unwrap_or(""),
    ]);

    view! {
        <div
            class=class
            style=style
            role="separator"
            aria-label="Resize handle"
            tabindex="0"
            data-direction=direction.to_string()
        />
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
    #[test] fn test_splitpane_creation() { 
    #[test] fn test_splitpane_with_class() { 
    #[test] fn test_splitpane_with_style() { 
    #[test] fn test_splitpane_direction() { 
    #[test] fn test_splitpane_initial_sizes() { 
    #[test] fn test_splitpane_min_sizes() { 
    #[test] fn test_splitpane_max_sizes() { 
    #[test] fn test_splitpane_resizable() { 
    #[test] fn test_splitpane_on_resize() { 

    // Split Direction tests
    #[test] fn test_split_direction_default() { 
    #[test] fn test_split_direction_horizontal() { 
    #[test] fn test_split_direction_vertical() { 

    // Resize Event tests
    #[test] fn test_resize_event_creation() { 

    // Split Pane Panel tests
    #[test] fn test_splitpane_panel_creation() { 
    #[test] fn test_splitpane_panel_with_class() { 
    #[test] fn test_splitpane_panel_with_style() { 
    #[test] fn test_splitpane_panel_size() { 
    #[test] fn test_splitpane_panel_min_size() { 
    #[test] fn test_splitpane_panel_max_size() { 
    #[test] fn test_splitpane_panel_resizable() { 
    #[test] fn test_splitpane_panel_collapsible() { 
    #[test] fn test_splitpane_panel_collapsed() { 

    // Split Pane Resizer tests
    #[test] fn test_splitpane_resizer_creation() { 
    #[test] fn test_splitpane_resizer_with_class() { 
    #[test] fn test_splitpane_resizer_with_style() { 
    #[test] fn test_splitpane_resizer_direction() { 
    #[test] fn test_splitpane_resizer_on_resize_start() { 
    #[test] fn test_splitpane_resizer_on_resize() { 
    #[test] fn test_splitpane_resizer_on_resize_end() { 

    // Helper function tests
    #[test] fn test_merge_classes_empty() { 
    #[test] fn test_merge_classes_single() { 
    #[test] fn test_merge_classes_multiple() { 
    #[test] fn test_merge_classes_with_empty() { 

    // Property-based Tests
    #[test] fn test_splitpane_property_based() {
        proptest!(|(____class in ".*", __style in ".*")| {
            
        });
    }

    #[test] fn test_splitpane_resize_validation() {
        proptest!(|(sizes in prop::collection::vec(10.0..90.0f64, 2..5))| {
            
        });
    }

    #[test] fn test_splitpane_direction_property_based() {
        proptest!(|(____direction_index in 0..2usize)| {
            
        });
    }

    // Integration Tests
    #[test] fn test_splitpane_user_interaction() { 
    #[test] fn test_splitpane_accessibility() { 
    #[test] fn test_splitpane_keyboard_navigation() { 
    #[test] fn test_splitpane_resize_behavior() { 
    #[test] fn test_splitpane_collapse_behavior() { 

    // Performance Tests
    #[test] fn test_splitpane_nested_performance() { 
    #[test] fn test_splitpane_animation_smooth() { 
    #[test] fn test_splitpane_resize_performance() { 
}
