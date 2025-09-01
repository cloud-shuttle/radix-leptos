use leptos::*;
use leptos::prelude::*;

/// Tooltip side enum
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TooltipSide {
    Top,
    Right,
    Bottom,
    Left,
}

impl TooltipSide {
    pub fn as_str(&self) -> &'static str {
        match self {
            TooltipSide::Top => "top",
            TooltipSide::Right => "right",
            TooltipSide::Bottom => "bottom",
            TooltipSide::Left => "left",
        }
    }
}

/// Tooltip alignment enum
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TooltipAlignment {
    Start,
    Center,
    End,
}

impl TooltipAlignment {
    pub fn as_str(&self) -> &'static str {
        match self {
            TooltipAlignment::Start => "start",
            TooltipAlignment::Center => "center",
            TooltipAlignment::End => "end",
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

/// Tooltip component with proper accessibility and positioning
#[component]
pub fn Tooltip(
    /// Whether the tooltip is open
    #[prop(optional, default = false)]
    open: bool,
    /// Whether the tooltip is disabled
    #[prop(optional, default = false)]
    disabled: bool,
    /// Delay before showing tooltip (in milliseconds)
    #[prop(optional, default = 700)]
    delay_duration: u32,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content (trigger and content)
    children: Children,
) -> impl IntoView {
    let tooltip_id = generate_id("tooltip");
    
    // Build base classes
    let base_classes = "radix-tooltip";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <div
            id=tooltip_id
            class=combined_class
            style=style.unwrap_or_default()
            data-state=if open { "open" } else { "closed" }
            data-delay-duration=delay_duration
            aria-disabled=disabled
        >
            {children()}
        </div>
    }
}

/// TooltipTrigger component for triggering the tooltip
#[component]
pub fn TooltipTrigger(
    /// Whether the trigger is disabled
    #[prop(optional, default = false)]
    disabled: bool,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content (trigger content)
    children: Children,
) -> impl IntoView {
    let trigger_id = generate_id("tooltip-trigger");
    
    // Build base classes
    let base_classes = "radix-tooltip-trigger";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <div
            id=trigger_id
            class=combined_class
            style=style.unwrap_or_default()
            aria-disabled=disabled
            tabindex="0"
        >
            {children()}
        </div>
    }
}

/// TooltipContent component for tooltip content
#[component]
pub fn TooltipContent(
    /// Tooltip side
    #[prop(optional, default = TooltipSide::Top)]
    side: TooltipSide,
    /// Tooltip alignment
    #[prop(optional, default = TooltipAlignment::Center)]
    align: TooltipAlignment,
    /// Whether the tooltip is disabled
    #[prop(optional, default = false)]
    disabled: bool,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content (tooltip content)
    children: Children,
) -> impl IntoView {
    let content_id = generate_id("tooltip-content");
    
    // Build base classes
    let base_classes = "radix-tooltip-content";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <div
            id=content_id
            class=combined_class
            style=style.unwrap_or_default()
            role="tooltip"
            aria-disabled=disabled
            data-side=side.as_str()
            data-align=align.as_str()
            data-state="closed"
        >
            {children()}
        </div>
    }
}

/// TooltipArrow component for visual indicator
#[component]
pub fn TooltipArrow(
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
) -> impl IntoView {
    let arrow_id = generate_id("tooltip-arrow");
    
    // Build base classes
    let base_classes = "radix-tooltip-arrow";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <svg
            id=arrow_id
            class=combined_class
            style=style.unwrap_or_default()
            width="10"
            height="5"
            viewBox="0 0 10 5"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
        >
            <path
                d="M0 0L5 5L10 0"
                fill="currentColor"
            />
        </svg>
    }
}

/// TooltipProvider component for global tooltip configuration
#[component]
pub fn TooltipProvider(
    /// Delay before showing tooltip (in milliseconds)
    #[prop(optional, default = 700)]
    delay_duration: u32,
    /// Whether to skip delay when moving between triggers
    #[prop(optional, default = true)]
    skip_delay_duration: bool,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content
    children: Children,
) -> impl IntoView {
    let provider_id = generate_id("tooltip-provider");
    
    // Build base classes
    let base_classes = "radix-tooltip-provider";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <div
            id=provider_id
            class=combined_class
            style=style.unwrap_or_default()
            data-delay-duration=delay_duration
            data-skip-delay-duration=skip_delay_duration
        >
            {children()}
        </div>
    }
}
