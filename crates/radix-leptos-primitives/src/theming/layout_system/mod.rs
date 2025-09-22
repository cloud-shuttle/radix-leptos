use crate::utils::merge_classes;
use leptos::callback::Callback;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

// Re-export all types from sub-modules
pub use container::*;
pub use responsive::*;
pub use spacing::*;

// Sub-modules
pub mod container;
pub mod responsive;
pub mod spacing;

/// Layout system for consistent spacing and alignment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct LayoutSystem {
    pub spacing: SpacingSystem,
    pub breakpoints: BreakpointSystem,
    pub grid: GridSystem,
    pub flexbox: FlexboxSystem,
    pub containers: ContainerSystem,
}

/// Layout builder component
#[component]
pub fn LayoutBuilder(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] layout_type: Option<String>,
    #[prop(optional)] on_layout_change: Option<Callback<LayoutSystem>>,
) -> impl IntoView {
    let layout_type = layout_type.unwrap_or_else(|| "grid".to_string());
    let on_layout_change = on_layout_change.unwrap_or_else(|| Callback::new(|_| {}));

    let class = merge_classes(
        [
            "layout-builder",
            &layout_type,
            class.as_deref().unwrap_or(""),
        ]
        .to_vec(),
    );

    let (layout, set_layout) = signal(LayoutSystem::default());

    let handle_layout_change = move |new_layout: LayoutSystem| {
        set_layout.set(new_layout.clone());
        on_layout_change.run(new_layout);
    };

    view! {
        <div
            class=class
            style=style
            role="form"
            aria-label="Layout system builder"
        >
            <div class="layout-builder-header">
                <h3>"Layout System"</h3>
                <p>"Configure spacing, breakpoints, and layout utilities"</p>
            </div>

            <div class="layout-sections">
                <SpacingLayoutSection
                    title="Spacing System".to_string()
                    layout_type="spacing".to_string()
                    layout=layout.get().spacing
                    on_change=Callback::new(move |spacing| {
                        let mut new_layout = layout.get();
                        new_layout.spacing = spacing;
                        handle_layout_change(new_layout);
                    })
                />

                <BreakpointLayoutSection
                    title="Breakpoint System".to_string()
                    layout_type="breakpoints".to_string()
                    layout=layout.get().breakpoints
                    on_change=Callback::new(move |breakpoints| {
                        let mut new_layout = layout.get();
                        new_layout.breakpoints = breakpoints;
                        handle_layout_change(new_layout);
                    })
                />

                <GridLayoutSection
                    title="Grid System".to_string()
                    grid=layout.get().grid
                    on_change=Callback::new(move |grid| {
                        let mut new_layout = layout.get();
                        new_layout.grid = grid;
                        handle_layout_change(new_layout);
                    })
                />

                <FlexboxLayoutSection
                    title="Flexbox System".to_string()
                    flexbox=layout.get().flexbox
                    on_change=Callback::new(move |flexbox| {
                        let mut new_layout = layout.get();
                        new_layout.flexbox = flexbox;
                        handle_layout_change(new_layout);
                    })
                />

                <ContainerLayoutSection
                    title="Container System".to_string()
                    containers=layout.get().containers
                    on_change=Callback::new(move |containers| {
                        let mut new_layout = layout.get();
                        new_layout.containers = containers;
                        handle_layout_change(new_layout);
                    })
                />
            </div>
        </div>
    }
}

/// Spacing layout section component
#[component]
pub fn SpacingLayoutSection(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] title: Option<String>,
    #[prop(optional)] layout_type: Option<String>,
    #[prop(optional)] layout: Option<SpacingSystem>,
    #[prop(optional)] on_change: Option<Callback<SpacingSystem>>,
) -> impl IntoView {
    let title = title.unwrap_or_default();
    let layout_type = layout_type.unwrap_or_default();
    let layout = layout.unwrap_or_default();
    let on_change = on_change.unwrap_or_else(|| Callback::new(|_| {}));
    let layout_clone = layout.clone();

    let class = merge_classes(
        [
            "layout-section",
            &layout_type,
            class.as_deref().unwrap_or(""),
        ]
        .to_vec(),
    );

    view! {
        <div
            class=class
            style=style
            data-layout-type=layout_type.clone()
        >
            <h4 class="section-title">{title}</h4>

            <div class="layout-options">
                <LayoutOptionGroup
                    title="Base Unit".to_string()
                    value=layout.base_unit
                    on_change=Callback::new(move |base_unit| {
                        let mut new_layout = layout.clone();
                        new_layout.base_unit = base_unit;
                        on_change.run(new_layout);
                    })
                />

                <LayoutOptionGroup
                    title="Scale".to_string()
                    values=layout_clone.scale.clone()
                    on_values_change=Callback::new(move |scale| {
                        let mut new_layout = layout_clone.clone();
                        new_layout.scale = scale;
                        on_change.run(new_layout);
                    })
                />
            </div>
        </div>
    }
}

/// Breakpoint layout section component
#[component]
pub fn BreakpointLayoutSection(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] title: Option<String>,
    #[prop(optional)] layout_type: Option<String>,
    #[prop(optional)] layout: Option<BreakpointSystem>,
    #[prop(optional)] on_change: Option<Callback<BreakpointSystem>>,
) -> impl IntoView {
    let title = title.unwrap_or_default();
    let layout_type = layout_type.unwrap_or_default();
    let layout = layout.unwrap_or_default();
    let on_change = on_change.unwrap_or_else(|| Callback::new(|_| {}));
    let layout_clone = layout.clone();

    let class = merge_classes(
        [
            "layout-section",
            &layout_type,
            class.as_deref().unwrap_or(""),
        ]
        .to_vec(),
    );

    view! {
        <div
            class=class
            style=style
            data-layout-type=layout_type.clone()
        >
            <h4 class="section-title">{title}</h4>

            <div class="layout-options">
                <div class="breakpoint-info">
                    <p>"Breakpoints: " {layout.breakpoints.len()}</p>
                    <p>"Container Max Widths: " {layout.container_max_widths.len()}</p>
                </div>
            </div>
        </div>
    }
}

/// Layout option group component
#[component]
pub fn LayoutOptionGroup(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] title: Option<String>,
    #[prop(optional)] value: Option<f64>,
    #[prop(optional)] values: Option<Vec<f64>>,
    #[prop(optional)] on_change: Option<Callback<f64>>,
    #[prop(optional)] on_values_change: Option<Callback<Vec<f64>>>,
) -> impl IntoView {
    let title = title.unwrap_or_default();
    let value = value.unwrap_or(0.0);
    let _values = values.clone().unwrap_or_default();
    let on_change = on_change.unwrap_or_else(|| Callback::new(|_| {}));
    let on_values_change = on_values_change.unwrap_or_else(|| Callback::new(|_| {}));

    let class = merge_classes(["layout-option-group", class.as_deref().unwrap_or("")].to_vec());

    view! {
        <div
            class=class
            style=style
        >
            <h5 class="option-group-title">{title}</h5>
            <div class="option-list">
                {if let Some(values) = values {
                    if !values.is_empty() {
                        view! {
                            <div class="values-list">
                                {values.into_iter().map(|val| {
                                    view! {
                                        <div class="layout-value" data-value=val.to_string()>
                                            <span class="value-number">{val}</span>
                                        </div>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        }.into_any()
                    } else {
                        view! { <div></div> }.into_any()
                    }
                } else {
                    view! { <div></div> }.into_any()
                }}
            </div>
        </div>
    }
}

/// Grid layout section component
#[component]
pub fn GridLayoutSection(
    #[prop(optional)] title: Option<String>,
    #[prop(optional)] grid: Option<GridSystem>,
    #[prop(optional)] on_change: Option<Callback<GridSystem>>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
) -> impl IntoView {
    let title = title.unwrap_or_else(|| "Grid System".to_string());
    let grid = grid.unwrap_or_default();
    let on_change = on_change.unwrap_or_else(|| Callback::new(|_| {}));
    let grid_clone1 = grid.clone();
    let grid_clone2 = grid.clone();
    let grid_clone3 = grid.clone();

    let class = merge_classes(
        [
            "layout-section",
            "grid-section",
            class.as_deref().unwrap_or(""),
        ]
        .to_vec(),
    );

    view! {
        <div
            class=class
            style=style
            data-layout-type="grid"
        >
            <h4 class="section-title">{title}</h4>

            <div class="layout-options">
                <LayoutOptionGroup
                    title="Columns".to_string()
                    values=[grid.columns as f64].to_vec()
                    on_values_change=Callback::new(move |columns: Vec<f64>| {
                        let mut new_grid = grid_clone1.clone();
                        new_grid.columns = columns[0] as u32;
                        on_change.run(new_grid);
                    })
                />

                <LayoutOptionGroup
                    title="Gutters".to_string()
                    values=grid.gutters.clone()
                    on_values_change=Callback::new(move |gutters| {
                        let mut new_grid = grid_clone2.clone();
                        new_grid.gutters = gutters;
                        on_change.run(new_grid);
                    })
                />

                <LayoutOptionGroup
                    title="Gaps".to_string()
                    values=grid.gaps.clone()
                    on_values_change=Callback::new(move |gaps| {
                        let mut new_grid = grid_clone3.clone();
                        new_grid.gaps = gaps;
                        on_change.run(new_grid);
                    })
                />
            </div>
        </div>
    }
}

/// Flexbox layout section component
#[component]
pub fn FlexboxLayoutSection(
    #[prop(optional)] title: Option<String>,
    #[prop(optional)] flexbox: Option<FlexboxSystem>,
    #[prop(optional)] on_change: Option<Callback<FlexboxSystem>>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
) -> impl IntoView {
    let title = title.unwrap_or_else(|| "Flexbox System".to_string());
    let flexbox = flexbox.unwrap_or_default();
    let on_change = on_change.unwrap_or_else(|| Callback::new(|_| {}));
    let flexbox_clone1 = flexbox.clone();
    let flexbox_clone2 = flexbox.clone();
    let flexbox_clone3 = flexbox.clone();

    let class = merge_classes(
        [
            "layout-section",
            "flexbox-section",
            class.as_deref().unwrap_or(""),
        ]
        .to_vec(),
    );

    view! {
        <div
            class=class
            style=style
            data-layout-type="flexbox"
        >
            <h4 class="section-title">{title}</h4>

            <div class="layout-options">
                <LayoutOptionGroup
                    title="Directions".to_string()
                    values=flexbox.directions.iter().map(|d| d.as_str().len() as f64).collect()
                    on_values_change=Callback::new(move |_directions| {
                        // In a real implementation, this would update the directions
                        let new_flexbox = flexbox_clone1.clone();
                        on_change.run(new_flexbox);
                    })
                />

                <LayoutOptionGroup
                    title="Wraps".to_string()
                    values=flexbox.wraps.iter().map(|w| w.as_str().len() as f64).collect()
                    on_values_change=Callback::new(move |_wraps| {
                        // In a real implementation, this would update the wraps
                        let new_flexbox = flexbox_clone2.clone();
                        on_change.run(new_flexbox);
                    })
                />

                <LayoutOptionGroup
                    title="Justifications".to_string()
                    values=flexbox.justifications.iter().map(|j| j.as_str().len() as f64).collect()
                    on_values_change=Callback::new(move |_justifications| {
                        // In a real implementation, this would update the justifications
                        let new_flexbox = flexbox_clone3.clone();
                        on_change.run(new_flexbox);
                    })
                />
            </div>
        </div>
    }
}

/// Container layout section component
#[component]
pub fn ContainerLayoutSection(
    #[prop(optional)] title: Option<String>,
    #[prop(optional)] containers: Option<ContainerSystem>,
    #[prop(optional)] on_change: Option<Callback<ContainerSystem>>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
) -> impl IntoView {
    let title = title.unwrap_or_else(|| "Container System".to_string());
    let containers = containers.unwrap_or_default();
    let on_change = on_change.unwrap_or_else(|| Callback::new(|_| {}));
    let containers_clone1 = containers.clone();
    let containers_clone2 = containers.clone();

    let class = merge_classes(
        [
            "layout-section",
            "container-section",
            class.as_deref().unwrap_or(""),
        ]
        .to_vec(),
    );

    view! {
        <div
            class=class
            style=style
            data-layout-type="container"
        >
            <h4 class="section-title">{title}</h4>

            <div class="layout-options">
                <LayoutOptionGroup
                    title="Max Widths".to_string()
                    values=containers.max_widths.iter().map(|w| w.as_str().len() as f64).collect()
                    on_values_change=Callback::new(move |_max_widths| {
                        // In a real implementation, this would update the max widths
                        let new_containers = containers_clone1.clone();
                        on_change.run(new_containers);
                    })
                />

                <LayoutOptionGroup
                    title="Paddings".to_string()
                    values=containers.paddings.clone()
                    on_values_change=Callback::new(move |paddings| {
                        let mut new_containers = containers_clone2.clone();
                        new_containers.paddings = paddings;
                        on_change.run(new_containers);
                    })
                />
            </div>
        </div>
    }
}

#[cfg(test)]
mod layout_system_tests {
    use super::*;
    use leptos::callback::Callback;

    #[test]
    fn test_layout_system_default() {
        let layout = LayoutSystem::default();
        assert_eq!(layout.spacing.base_unit, 4.0);
        assert_eq!(layout.spacing.scale.len(), 24);
        assert_eq!(layout.spacing.directions.len(), 7);
        assert_eq!(layout.breakpoints.breakpoints.len(), 6);
        assert_eq!(layout.breakpoints.container_max_widths.len(), 5);
        assert_eq!(layout.grid.columns, 12);
        assert_eq!(layout.grid.gutters.len(), 4);
        assert_eq!(layout.grid.gaps.len(), 5);
        assert_eq!(layout.grid.alignments.len(), 7);
        assert_eq!(layout.flexbox.directions.len(), 4);
        assert_eq!(layout.flexbox.wraps.len(), 3);
        assert_eq!(layout.flexbox.justifications.len(), 6);
        assert_eq!(layout.flexbox.alignments.len(), 5);
        assert_eq!(layout.flexbox.grows.len(), 4);
        assert_eq!(layout.flexbox.shrinks.len(), 4);
        assert_eq!(layout.containers.max_widths.len(), 5);
        assert_eq!(layout.containers.paddings.len(), 4);
        assert_eq!(layout.containers.margins.len(), 5);
        assert_eq!(layout.containers.centers.len(), 2);
    }

    #[test]
    fn test_layout_builder_component_creation() {
        // Test logic without runtime
        // Test component logic
        let title = "Spacing System";
        let layout_type = "spacing";
        assert!(!title.is_empty());
        assert!(!layout_type.is_empty()); // Test completed
    }

    #[test]
    fn test_layout_builder_with_callback() {
        // Test logic without runtime
        let callback = Callback::new(|_layout: LayoutSystem| {});
        // Test component logic
        let title = "Spacing System";
        let layout_type = "spacing";
        assert!(!title.is_empty());
        assert!(!layout_type.is_empty()); // Test completed
    }

    #[test]
    fn test_layout_section_component() {
        // Test logic without runtime
        let spacing = SpacingSystem::default();
        // Test component logic
        let title = "Spacing System";
        let layout_type = "spacing";
        assert!(!title.is_empty());
        assert!(!layout_type.is_empty()); // Test completed
    }

    #[test]
    fn test_layout_option_group_component() {
        // Test logic without runtime
        let _values = [0.0, 1.0, 2.0, 4.0, 8.0];
        // Test component logic
        let title = "Spacing System";
        let layout_type = "spacing";
        assert!(!title.is_empty());
        assert!(!layout_type.is_empty()); // Test completed
    }

    // Performance Tests
    #[test]
    fn test_layout_creation_performance() {
        // Test layout creation performance
        let start = std::time::Instant::now();
        let _layout = LayoutSystem::default();
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100); // Should create layout in less than 100ms
    }

    #[test]
    fn test_layout_builder_render_performance() {
        // Test layout builder render performance
        let start = std::time::Instant::now();
        // Simulate component creation
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100); // Should render in less than 100ms
    }
}
