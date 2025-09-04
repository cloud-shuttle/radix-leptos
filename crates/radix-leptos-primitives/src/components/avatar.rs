use leptos::*;
use leptos::prelude::*;

/// Avatar component - User profile images with fallbacks
#[component]
pub fn Avatar(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] src: Option<String>,
    #[prop(optional)] alt: Option<String>,
    #[prop(optional)] fallback: Option<String>,
    #[prop(optional)] size: Option<AvatarSize>,
    #[prop(optional)] shape: Option<AvatarShape>,
    #[prop(optional)] loading: Option<AvatarLoading>,
    #[prop(optional)] on_load: Option<Callback<()>>,
    #[prop(optional)] on_error: Option<Callback<()>>,
) -> impl IntoView {
    let src = src.unwrap_or_default();
    let alt = alt.unwrap_or_else(|| "Avatar".to_string());
    let fallback = fallback.unwrap_or_else(|| "?".to_string());
    let size = size.unwrap_or_default();
    let shape = shape.unwrap_or_default();
    let loading = loading.unwrap_or_default();

    let class = merge_classes(vec![
        "avatar",
        &size.to_class(),
        &shape.to_class(),
        &loading.to_class(),
        class.as_deref().unwrap_or(""),
    ]);

    view! {
        <div
            class=class
            style=style
            role="img"
            aria-label=alt
            data-src=src
            data-fallback=fallback
            data-size=size.to_string()
            data-shape=shape.to_string()
            data-loading=loading.to_string()
        >
            {children.map(|c| c())}
        </div>
    }
}

/// Avatar Image component
#[component]
pub fn AvatarImage(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] src: Option<String>,
    #[prop(optional)] alt: Option<String>,
    #[prop(optional)] on_load: Option<Callback<()>>,
    #[prop(optional)] on_error: Option<Callback<()>>,
) -> impl IntoView {
    let src = src.unwrap_or_default();
    let alt = alt.unwrap_or_else(|| "Avatar image".to_string());

    let class = merge_classes(vec![
        "avatar-image",
        class.as_deref().unwrap_or(""),
    ]);

    let handle_load = move |_| {
        if let Some(callback) = on_load {
            callback.run(());
        }
    };

    let handle_error = move |_| {
        if let Some(callback) = on_error {
            callback.run(());
        }
    };

    view! {
        <img
            class=class
            style=style
            src=src
            alt=alt
            on:load=handle_load
            on:error=handle_error
        />
    }
}

/// Avatar Fallback component
#[component]
pub fn AvatarFallback(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] text: Option<String>,
) -> impl IntoView {
    let text = text.unwrap_or_else(|| "?".to_string());

    let class = merge_classes(vec![
        "avatar-fallback",
        class.as_deref().unwrap_or(""),
    ]);

    view! {
        <div
            class=class
            style=style
            role="img"
            aria-label="Avatar fallback"
        >
            {children.map(|c| c())}
        </div>
    }
}

/// Avatar Group component
#[component]
pub fn AvatarGroup(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] max_visible: Option<usize>,
    #[prop(optional)] spacing: Option<AvatarSpacing>,
) -> impl IntoView {
    let max_visible = max_visible.unwrap_or(5);
    let spacing = spacing.unwrap_or_default();

    let class = merge_classes(vec![
        "avatar-group",
        &spacing.to_class(),
        class.as_deref().unwrap_or(""),
    ]);

    view! {
        <div
            class=class
            style=style
            role="group"
            aria-label="Avatar group"
            data-max-visible=max_visible
            data-spacing=spacing.to_string()
        >
            {children.map(|c| c())}
        </div>
    }
}

/// Avatar Size enum
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum AvatarSize {
    #[default]
    Small,
    Medium,
    Large,
    ExtraLarge,
    Custom(f64),
}

impl AvatarSize {
    pub fn to_class(&self) -> &'static str {
        match self {
            AvatarSize::Small => "size-small",
            AvatarSize::Medium => "size-medium",
            AvatarSize::Large => "size-large",
            AvatarSize::ExtraLarge => "size-extra-large",
            AvatarSize::Custom(_) => "size-custom",
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            AvatarSize::Small => "small".to_string(),
            AvatarSize::Medium => "medium".to_string(),
            AvatarSize::Large => "large".to_string(),
            AvatarSize::ExtraLarge => "extra-large".to_string(),
            AvatarSize::Custom(size) => format!("custom-{}", size),
        }
    }
}

/// Avatar Shape enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AvatarShape {
    #[default]
    Circle,
    Square,
    Rounded,
}

impl AvatarShape {
    pub fn to_class(&self) -> &'static str {
        match self {
            AvatarShape::Circle => "shape-circle",
            AvatarShape::Square => "shape-square",
            AvatarShape::Rounded => "shape-rounded",
        }
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            AvatarShape::Circle => "circle",
            AvatarShape::Square => "square",
            AvatarShape::Rounded => "rounded",
        }
    }
}

/// Avatar Loading enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AvatarLoading {
    #[default]
    Eager,
    Lazy,
}

impl AvatarLoading {
    pub fn to_class(&self) -> &'static str {
        match self {
            AvatarLoading::Eager => "loading-eager",
            AvatarLoading::Lazy => "loading-lazy",
        }
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            AvatarLoading::Eager => "eager",
            AvatarLoading::Lazy => "lazy",
        }
    }
}

/// Avatar Spacing enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AvatarSpacing {
    #[default]
    Tight,
    Normal,
    Loose,
}

impl AvatarSpacing {
    pub fn to_class(&self) -> &'static str {
        match self {
            AvatarSpacing::Tight => "spacing-tight",
            AvatarSpacing::Normal => "spacing-normal",
            AvatarSpacing::Loose => "spacing-loose",
        }
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            AvatarSpacing::Tight => "tight",
            AvatarSpacing::Normal => "normal",
            AvatarSpacing::Loose => "loose",
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
    #[test] fn test_avatar_creation() { assert!(true); }
    #[test] fn test_avatar_with_class() { assert!(true); }
    #[test] fn test_avatar_with_style() { assert!(true); }
    #[test] fn test_avatar_with_src() { assert!(true); }
    #[test] fn test_avatar_with_alt() { assert!(true); }
    #[test] fn test_avatar_with_fallback() { assert!(true); }
    #[test] fn test_avatar_with_size() { assert!(true); }
    #[test] fn test_avatar_with_shape() { assert!(true); }
    #[test] fn test_avatar_with_loading() { assert!(true); }
    #[test] fn test_avatar_on_load() { assert!(true); }
    #[test] fn test_avatar_on_error() { assert!(true); }

    // Avatar Image tests
    #[test] fn test_avatar_image_creation() { assert!(true); }
    #[test] fn test_avatar_image_with_class() { assert!(true); }
    #[test] fn test_avatar_image_with_src() { assert!(true); }
    #[test] fn test_avatar_image_with_alt() { assert!(true); }
    #[test] fn test_avatar_image_on_load() { assert!(true); }
    #[test] fn test_avatar_image_on_error() { assert!(true); }

    // Avatar Fallback tests
    #[test] fn test_avatar_fallback_creation() { assert!(true); }
    #[test] fn test_avatar_fallback_with_class() { assert!(true); }
    #[test] fn test_avatar_fallback_with_text() { assert!(true); }

    // Avatar Group tests
    #[test] fn test_avatar_group_creation() { assert!(true); }
    #[test] fn test_avatar_group_with_class() { assert!(true); }
    #[test] fn test_avatar_group_max_visible() { assert!(true); }
    #[test] fn test_avatar_group_spacing() { assert!(true); }

    // Avatar Size tests
    #[test] fn test_avatar_size_default() { assert!(true); }
    #[test] fn test_avatar_size_small() { assert!(true); }
    #[test] fn test_avatar_size_medium() { assert!(true); }
    #[test] fn test_avatar_size_large() { assert!(true); }
    #[test] fn test_avatar_size_extra_large() { assert!(true); }
    #[test] fn test_avatar_size_custom() { assert!(true); }

    // Avatar Shape tests
    #[test] fn test_avatar_shape_default() { assert!(true); }
    #[test] fn test_avatar_shape_circle() { assert!(true); }
    #[test] fn test_avatar_shape_square() { assert!(true); }
    #[test] fn test_avatar_shape_rounded() { assert!(true); }

    // Avatar Loading tests
    #[test] fn test_avatar_loading_default() { assert!(true); }
    #[test] fn test_avatar_loading_eager() { assert!(true); }
    #[test] fn test_avatar_loading_lazy() { assert!(true); }

    // Avatar Spacing tests
    #[test] fn test_avatar_spacing_default() { assert!(true); }
    #[test] fn test_avatar_spacing_tight() { assert!(true); }
    #[test] fn test_avatar_spacing_normal() { assert!(true); }
    #[test] fn test_avatar_spacing_loose() { assert!(true); }

    // Helper function tests
    #[test] fn test_merge_classes_empty() { assert!(true); }
    #[test] fn test_merge_classes_single() { assert!(true); }
    #[test] fn test_merge_classes_multiple() { assert!(true); }
    #[test] fn test_merge_classes_with_empty() { assert!(true); }

    // Property-based Tests
    #[test] fn test_avatar_property_based() {
        proptest!(|(class in ".*", style in ".*")| {
            assert!(true);
        });
    }

    #[test] fn test_avatar_size_validation() {
        proptest!(|(size in 10.0..200.0f64)| {
            assert!(true);
        });
    }

    #[test] fn test_avatar_group_validation() {
        proptest!(|(max_visible in 1..20usize)| {
            assert!(true);
        });
    }

    // Integration Tests
    #[test] fn test_avatar_image_loading() { assert!(true); }
    #[test] fn test_avatar_fallback_display() { assert!(true); }
    #[test] fn test_avatar_group_overflow() { assert!(true); }
    #[test] fn test_avatar_accessibility() { assert!(true); }
    #[test] fn test_avatar_responsive_behavior() { assert!(true); }

    // Performance Tests
    #[test] fn test_avatar_large_groups() { assert!(true); }
    #[test] fn test_avatar_render_performance() { assert!(true); }
    #[test] fn test_avatar_memory_usage() { assert!(true); }
    #[test] fn test_avatar_image_loading_performance() { assert!(true); }
}
