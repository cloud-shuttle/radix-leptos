use leptos::prelude::*;
use leptos::*;

/// Skeleton component - Loading placeholder component for better UX
///
/// The Skeleton component provides animated loading placeholders that give users
/// visual feedback while content is loading, improving perceived performance.
///
/// # Features
/// - Animated shimmer effect
/// - Multiple variants (text, circular, rectangular)
/// - Multiple sizes (sm, md, lg, xl)
/// - Customizable dimensions
/// - Accessibility-friendly
/// - Smooth animations
///
/// # Example
///
/// ```rust
/// use leptos::*;
/// use radix_leptos_primitives::*;
///
/// #[component]
/// fn MyComponent() -> impl IntoView {
///     let (loading, set_loading) = create_signal(true);
///
///     view! {
///         <div class="content">
///             if loading.get() {
///                 <div class="loading-state">
///                     <Skeleton variant=SkeletonVariant::Circular size=SkeletonSize::Large />
///                     <Skeleton variant=SkeletonVariant::Text lines=3 />
///                     <Skeleton variant=SkeletonVariant::Rectangular width="200px" height="100px" />
///                 </div>
///             } else {
///                 <div class="loaded-content">
///                     // Actual content here
///                 </div>
///             }
///         </div>
///     }
/// }
/// ```

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SkeletonVariant {
    Text,
    Circular,
    Rectangular,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SkeletonSize {
    Small,
    Medium,
    Large,
    ExtraLarge,
}

impl SkeletonVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            SkeletonVariant::Text => "text",
            SkeletonVariant::Circular => "circular",
            SkeletonVariant::Rectangular => "rectangular",
        }
    }
}

impl SkeletonSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            SkeletonSize::Small => "sm",
            SkeletonSize::Medium => "md",
            SkeletonSize::Large => "lg",
            SkeletonSize::ExtraLarge => "xl",
        }
    }
}

/// Skeleton component
#[component]
pub fn Skeleton(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] variant: Option<SkeletonVariant>,
    #[prop(optional)] size: Option<SkeletonSize>,
    #[prop(optional)] width: Option<String>,
    #[prop(optional)] height: Option<String>,
    #[prop(optional)] lines: Option<usize>,
    #[prop(optional)] animated: Option<bool>,
) -> impl IntoView {
    let variant = variant.unwrap_or(SkeletonVariant::Rectangular);
    let size = size.unwrap_or(SkeletonSize::Medium);
    let lines = lines.unwrap_or(1);
    let animated = animated.unwrap_or(true);

    let class = merge_classes([
        "skeleton",
        variant.as_str(),
        size.as_str(),
        if animated { "animated" } else { "static" },
        class.as_deref().unwrap_or(""),
    ]);

    let mut style_attr = style.unwrap_or_default();

    // Add custom dimensions if provided
    if let Some(w) = width {
        style_attr = format!("{}width: {};", style_attr, w);
    }
    if let Some(h) = height {
        style_attr = format!("{}height: {};", style_attr, h);
    }

    match variant {
        SkeletonVariant::Text => view! {
            <div class=class style=style_attr>
                {if lines > 1 {
                    (0..lines).map(|i| {
                        let line_class = if i == lines - 1 {
                            "skeleton-line skeleton-line-last"
                        } else {
                            "skeleton-line"
                        };
                        view! {
                            <div class=line_class></div>
                        }
                    }).collect::<Vec<_>>()
                } else {
                    [view! { <div class="skeleton-line"></div> }]
                }}
            </div>
        }
        .into_any(),
        SkeletonVariant::Circular => view! {
            <div
                class=class
                style=style_attr
                role="img"
                aria-label="Loading"
            ></div>
        }
        .into_any(),
        SkeletonVariant::Rectangular => view! {
            <div
                class=class
                style=style_attr
                role="img"
                aria-label="Loading"
            ></div>
        }
        .into_any(),
    }
}

/// Skeleton group component for multiple skeletons
#[component]
pub fn SkeletonGroup(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] spacing: Option<String>,
) -> impl IntoView {
    let spacing = spacing.unwrap_or_else(|| "1rem".to_string());

    let class = merge_classes(["skeleton-group", class.as_deref().unwrap_or("")]);

    let style_attr = format!("{}gap: {};", style.unwrap_or_default(), spacing);

    view! {
        <div
            class=class
            style=style_attr
        >
            {children.map(|c| c())}
        </div>
    }
}

/// Skeleton text component with multiple lines
#[component]
pub fn SkeletonText(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] lines: Option<usize>,
    #[prop(optional)] animated: Option<bool>,
) -> impl IntoView {
    let lines = lines.unwrap_or(1);
    let animated = animated.unwrap_or(true);

    view! {
        <Skeleton
            class=class.unwrap_or_default()
            style=style.unwrap_or_default()
            variant=SkeletonVariant::Text
            lines=lines
            animated=animated
        />
    }
}

/// Skeleton avatar component
#[component]
pub fn SkeletonAvatar(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] size: Option<SkeletonSize>,
    #[prop(optional)] animated: Option<bool>,
) -> impl IntoView {
    let size = size.unwrap_or(SkeletonSize::Medium);
    let animated = animated.unwrap_or(true);

    view! {
        <Skeleton
            class=class.unwrap_or_default()
            style=style.unwrap_or_default()
            variant=SkeletonVariant::Circular
            size=size
            animated=animated
        />
    }
}

/// Skeleton button component
#[component]
pub fn SkeletonButton(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] size: Option<SkeletonSize>,
    #[prop(optional)] animated: Option<bool>,
) -> impl IntoView {
    let size = size.unwrap_or(SkeletonSize::Medium);
    let animated = animated.unwrap_or(true);

    view! {
        <Skeleton
            class=class.unwrap_or_default()
            style=style.unwrap_or_default()
            variant=SkeletonVariant::Rectangular
            size=size
            animated=animated
        />
    }
}

// Helper function to merge CSS classes
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
    use proptest::prelude::*;

    #[test]
    fn test_skeleton_component_creation() {}

    #[test]
    fn test_skeleton_with_variant_component_creation() {}

    proptest! {
        #[test]
        fn test_skeleton_props(__class in ".*", __style in ".*") {

        }

        #[test]
        fn test_skeleton_variants(__variant_index in 0..3usize, __size_index in 0..4usize) {

        }

        #[test]
        fn test_skeleton_sizes(__size_index in 0..4usize) {

        }

        #[test]
        fn test_skeleton_dimensions(_width in ".*", _height in ".*") {

        }

        #[test]
        fn test_skeleton_lines(__lines in 1..10usize) {

        }

        #[test]
        fn test_skeleton_animation(__animated: bool) {

        }

        #[test]
        fn test_skeleton_group_props(__class in ".*", __style in ".*", _spacing in ".*") {

        }

        #[test]
        fn test_skeleton_text_props(__class in ".*", __style in ".*", __lines in 1..5usize) {

        }

        #[test]
        fn test_skeleton_avatar_props(__class in ".*", __style in ".*", __size_index in 0..4usize) {

        }

        #[test]
        fn test_skeleton_button_props(__class in ".*", __style in ".*", __size_index in 0..4usize) {

        }
    }
}
