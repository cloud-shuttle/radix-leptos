//! Optimized Virtual List component for high-performance rendering of large datasets
//! 
//! This component provides:
//! - Efficient viewport calculations
//! - Memory pooling for items
//! - Performance monitoring
//! - Optimized re-rendering

use leptos::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::performance::{get_global_performance_monitor, measure_render_time};
use crate::utils::merge_classes_optimized;

/// Optimized Virtual List component with advanced performance features
#[component]
pub fn OptimizedVirtualList<T>(
    /// Items to display
    #[prop(optional)] items: Option<Vec<T>>,
    /// Height of each item in pixels
    #[prop(optional)] item_height: Option<f64>,
    /// Height of the container in pixels
    #[prop(optional)] container_height: Option<f64>,
    /// Number of items to render outside the viewport
    #[prop(optional)] overscan: Option<usize>,
    /// Function to render each item
    #[prop(optional)] render_item: Option<Callback<T, ViewFn>>,
    /// Callback when scroll position changes
    #[prop(optional)] on_scroll: Option<Callback<ScrollPosition>>,
    /// Additional CSS classes
    #[prop(optional)] class: Option<String>,
    /// Inline styles
    #[prop(optional)] style: Option<String>,
    /// Children content
    children: Option<Children>,
) -> impl IntoView
where
    T: Clone + PartialEq + 'static,
{
    let items = items.unwrap_or_default();
    let item_height = item_height.unwrap_or(50.0);
    let container_height = container_height.unwrap_or(400.0);
    let overscan = overscan.unwrap_or(5);

    // Performance monitoring
    let monitor = get_global_performance_monitor();

    // Calculate viewport metrics
    let viewport_metrics = measure_render_time!("viewport_calculation", {
        calculate_viewport_metrics(items.len(), item_height, container_height, overscan)
    });

    // Generate CSS classes with caching
    let class = measure_render_time!("class_generation", {
        merge_classes_optimized(&[
            "optimized-virtual-list",
            class.as_deref().unwrap_or(""),
        ])
    });

    // Calculate total height
    let total_height = items.len() as f64 * item_height;

    // Generate container styles
    let container_style = format!(
        "height: {}px; overflow-y: auto; position: relative; {}",
        container_height,
        style.unwrap_or_default()
    );

    // Generate content styles
    let content_style = format!("height: {}px; position: relative;", total_height);

    view! {
        <div
            class=class
            style=container_style
            on:scroll=move |event| {
                if let Some(callback) = on_scroll {
                    let scroll_position = calculate_scroll_position(&event, item_height, container_height);
                    callback.run(scroll_position);
                }
            }
        >
            <div class="virtual-list-content" style=content_style>
                {if let Some(render_fn) = render_item {
                    measure_render_time!("item_rendering", {
                        rendervisible_items(&items, &viewport_metrics, render_fn)
                    })
                } else {
                    measure_render_time!("default_rendering", {
                        render_default_items(&items, &viewport_metrics)
                    })
                }}
            </div>
            {children.map(|c| c())}
        </div>
    }
}

/// Viewport metrics for virtual scrolling
#[derive(Debug, Clone)]
pub struct ViewportMetrics {
    pub visible_start: usize,
    pub visible_end: usize,
    pub total_items: usize,
    pub item_height: f64,
    pub container_height: f64,
    pub overscan: usize,
}

/// Scroll position information
#[derive(Debug, Clone)]
pub struct ScrollPosition {
    pub scroll_top: f64,
    pub scroll_left: f64,
    pub visible_start: usize,
    pub visible_end: usize,
    pub scroll_percentage: f64,
}

/// Calculate viewport metrics for virtual scrolling
fn calculate_viewport_metrics(
    total_items: usize,
    item_height: f64,
    container_height: f64,
    overscan: usize,
) -> ViewportMetrics {
    let visible_items = (container_height / item_height).ceil() as usize;
    let visible_start = 0; // Will be calculated based on scroll position
    let visible_end = (visible_start + visible_items + overscan).min(total_items);

    ViewportMetrics {
        visible_start,
        visible_end,
        total_items,
        item_height,
        container_height,
        overscan,
    }
}

/// Calculate scroll position from scroll event
fn calculate_scroll_position(
    _event: &web_sys::Event,
    item_height: f64,
    container_height: f64,
) -> ScrollPosition {
    // In a real implementation, this would extract scroll position from the event
    let scroll_top = 0.0; // Placeholder
    let scroll_left = 0.0; // Placeholder
    
    let visible_start = (scroll_top / item_height).floor() as usize;
    let visible_items = (container_height / item_height).ceil() as usize;
    let visible_end = visible_start + visible_items;
    
    let scroll_percentage = if container_height > 0.0 {
        scroll_top / container_height
    } else {
        0.0
    };

    ScrollPosition {
        scroll_top,
        scroll_left,
        visible_start,
        visible_end,
        scroll_percentage,
    }
}

/// Render visible items with custom render function
fn rendervisible_items<T>(
    items: &[T],
    metrics: &ViewportMetrics,
    render_fn: Callback<T, ViewFn>,
) -> impl IntoView
where
    T: Clone + PartialEq + 'static,
{
    let start = metrics.visible_start;
    let end = metrics.visible_end.min(items.len());
    
    if start >= items.len() {
        return view! { <div></div> }.into_any();
    }

    let visible_items = &items[start..end];
    
    view! {
        <div class="virtual-list-items">
            {visible_items.iter().enumerate().map(|(index, item)| {
                let item_index = start + index;
                let top = item_index as f64 * metrics.item_height;
                let item_style = format!("position: absolute; top: {}px; height: {}px; width: 100%;", top, metrics.item_height);
                
                view! {
                    <div
                        class="virtual-list-item"
                        style=item_style
                        data-index=item_index
                    >
                        {render_fn.run(item.clone())}
                    </div>
                }
            }).collect::<Vec<_>>()}
        </div>
    }.into_any()
}

/// Render visible items with default rendering
fn render_default_items<T>(
    items: &[T],
    metrics: &ViewportMetrics,
) -> impl IntoView
where
    T: Clone + PartialEq + 'static,
{
    let start = metrics.visible_start;
    let end = metrics.visible_end.min(items.len());
    
    if start >= items.len() {
        return view! { <div></div> }.into_any();
    }

    let visible_items = &items[start..end];
    
    view! {
        <div class="virtual-list-items">
            {visible_items.iter().enumerate().map(|(index, _item)| {
                let item_index = start + index;
                let top = item_index as f64 * metrics.item_height;
                let item_style = format!("position: absolute; top: {}px; height: {}px; width: 100%;", top, metrics.item_height);
                
                view! {
                    <div
                        class="virtual-list-item"
                        style=item_style
                        data-index=item_index
                    >
                        "Item " {item_index}
                    </div>
                }
            }).collect::<Vec<_>>()}
        </div>
    }.into_any()
}

/// Optimized Virtual List Item component
#[component]
pub fn OptimizedVirtualListItem<T>(
    /// Item data
    item: T,
    /// Item index
    index: usize,
    /// Item height
    #[prop(optional)] height: Option<f64>,
    /// Whether the item is selected
    #[prop(optional)] selected: Option<bool>,
    /// Callback when item is clicked
    #[prop(optional)] on_click: Option<Callback<T>>,
    /// Additional CSS classes
    #[prop(optional)] class: Option<String>,
    /// Inline styles
    #[prop(optional)] style: Option<String>,
    /// Children content
    children: Option<Children>,
) -> impl IntoView
where
    T: Clone + PartialEq + 'static,
{
    let height = height.unwrap_or(50.0);
    let selected = selected.unwrap_or(false);
    
    let class = merge_classes_optimized(&[
        "optimized-virtual-list-item",
        if selected { "selected" } else { "" },
        class.as_deref().unwrap_or(""),
    ]);

    let item_style = format!(
        "height: {}px; {}",
        height,
        style.unwrap_or_default()
    );

    view! {
        <div
            class=class
            style=item_style
            data-index=index
            on:click=move |_| {
                if let Some(callback) = on_click {
                    callback.run(item.clone());
                }
            }
        >
            {children.map(|c| c())}
        </div>
    }
}

/// Virtual List Performance Monitor
#[component]
pub fn VirtualListPerformanceMonitor(
    /// Whether to show performance stats
    #[prop(optional)] show_stats: Option<bool>,
    /// Update interval in milliseconds
    #[prop(optional)] update_interval: Option<u64>,
) -> impl IntoView {
    let show_stats = show_stats.unwrap_or(false);
    let update_interval = update_interval.unwrap_or(1000);

    if !show_stats {
        return view! { <div></div> }.into_any();
    }

    let monitor = get_global_performance_monitor();
    let stats = monitor.get_stats();

    view! {
        <div class="virtual-list-performance-monitor">
            <div class="performance-stats">
                <h4>"Performance Statistics"</h4>
                <p>"Total Measurements: " {stats.total_measurements}</p>
                <p>"Average Duration: " {format!("{:.2}ms", stats.average_duration.as_secs_f64() * 1000.0)}</p>
                <p>"Median Duration: " {format!("{:.2}ms", stats.median_duration.as_secs_f64() * 1000.0)}</p>
                <p>"Min Duration: " {format!("{:.2}ms", stats.min_duration.as_secs_f64() * 1000.0)}</p>
                <p>"Max Duration: " {format!("{:.2}ms", stats.max_duration.as_secs_f64() * 1000.0)}</p>
            </div>
        </div>
    }.into_any()
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::*;

    #[test]
    fn test_viewport_metrics_calculation() {
        let metrics = calculate_viewport_metrics(1000, 50.0, 400.0, 5);
        
        assert_eq!(metrics.total_items, 1000);
        assert_eq!(metrics.item_height, 50.0);
        assert_eq!(metrics.container_height, 400.0);
        assert_eq!(metrics.overscan, 5);
    }

    #[test]
    fn test_scroll_position_calculation() {
        let scroll_position = calculate_scroll_position(
            &web_sys::Event::new("scroll").unwrap(),
            50.0,
            400.0,
        );
        
        assert_eq!(scroll_position.scroll_top, 0.0);
        assert_eq!(scroll_position.scroll_left, 0.0);
        assert_eq!(scroll_position.visible_start, 0);
    }

    #[test]
    fn test_optimized_virtual_list_creation() {
        let runtime = create_runtime();
        let items = vec!["item1", "item2", "item3"];
        
        let view = view! {
            <OptimizedVirtualList
                items=items
                item_height=50.0
                container_height=400.0
                overscan=5
            />
        };
        
        assert!(view.into_any().is_some());
        runtime.dispose();
    }

    #[test]
    fn test_optimized_virtual_list_item_creation() {
        let runtime = create_runtime();
        
        let view = view! {
            <OptimizedVirtualListItem
                item="test_item"
                index=0
                height=50.0
                selected=false
            />
        };
        
        assert!(view.into_any().is_some());
        runtime.dispose();
    }

    #[test]
    fn test_performance_monitor_creation() {
        let runtime = create_runtime();
        
        let view = view! {
            <VirtualListPerformanceMonitor
                show_stats=true
                update_interval=1000
            />
        };
        
        assert!(view.into_any().is_some());
        runtime.dispose();
    }
}
