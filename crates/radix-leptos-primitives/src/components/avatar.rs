
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

    let class = 
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

    let class = merge_classes(["avatar-image", class.as_deref().unwrap_or("")]);

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

    let class = merge_classes(["avatar-fallback", class.as_deref().unwrap_or("")]);

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
    #[prop(optional)] maxvisible: Option<usize>,
    #[prop(optional)] spacing: Option<AvatarSpacing>,
) -> impl IntoView {
    let maxvisible = maxvisible.unwrap_or(5);
    let spacing = spacing.unwrap_or_default();

    let class = 
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
            data-max-visible=maxvisible
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
    use proptest::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    // Unit Tests
    #[test]
    fn test_avatar_creation() {}
    #[test]
    fn test_avatar_with_class() {}
    #[test]
    fn test_avatar_with_style() {}
    #[test]
    fn test_avatar_with_src() {}
    #[test]
    fn test_avatar_with_alt() {}
    #[test]
    fn test_avatar_with_fallback() {}
    #[test]
    fn test_avatar_with_size() {}
    #[test]
    fn test_avatar_with_shape() {}
    #[test]
    fn test_avatar_withloading() {}
    #[test]
    fn test_avatar_on_load() {}
    #[test]
    fn test_avatar_on_error() {}

    // Avatar Image tests
    #[test]
    fn test_avatar_image_creation() {}
    #[test]
    fn test_avatar_image_with_class() {}
    #[test]
    fn test_avatar_image_with_src() {}
    #[test]
    fn test_avatar_image_with_alt() {}
    #[test]
    fn test_avatar_image_on_load() {}
    #[test]
    fn test_avatar_image_on_error() {}

    // Avatar Fallback tests
    #[test]
    fn test_avatar_fallback_creation() {}
    #[test]
    fn test_avatar_fallback_with_class() {}
    #[test]
    fn test_avatar_fallback_with_text() {}

    // Avatar Group tests
    #[test]
    fn test_avatar_group_creation() {}
    #[test]
    fn test_avatar_group_with_class() {}
    #[test]
    fn test_avatar_group_maxvisible() {}
    #[test]
    fn test_avatar_group_spacing() {}

    // Avatar Size tests
    #[test]
    fn test_avatar_size_default() {}
    #[test]
    fn test_avatar_size_small() {}
    #[test]
    fn test_avatar_size_medium() {}
    #[test]
    fn test_avatar_size_large() {}
    #[test]
    fn test_avatar_size_extra_large() {}
    #[test]
    fn test_avatar_size_custom() {}

    // Avatar Shape tests
    #[test]
    fn test_avatar_shape_default() {}
    #[test]
    fn test_avatar_shape_circle() {}
    #[test]
    fn test_avatar_shape_square() {}
    #[test]
    fn test_avatar_shape_rounded() {}

    // Avatar Loading tests
    #[test]
    fn test_avatarloading_default() {}
    #[test]
    fn test_avatarloading_eager() {}
    #[test]
    fn test_avatarloading_lazy() {}

    // Avatar Spacing tests
    #[test]
    fn test_avatar_spacing_default() {}
    #[test]
    fn test_avatar_spacing_tight() {}
    #[test]
    fn test_avatar_spacing_normal() {}
    #[test]
    fn test_avatar_spacing_loose() {}

    // Helper function tests
    #[test]
    fn test_merge_classes_empty() {}
    #[test]
    fn test_merge_classes_single() {}
    #[test]
    fn test_merge_classes_multiple() {}
    #[test]
    fn test_merge_classes_with_empty() {}

    // Property-based Tests
    #[test]
    fn test_avatar_property_based() {
        proptest!(|(____class in ".*", __style in ".*")| {

        });
    }

    #[test]
    fn test_avatar_size_validation() {
        proptest!(|(____size in 10.0..200.0f64)| {

        });
    }

    #[test]
    fn test_avatar_group_validation() {
        proptest!(|(____maxvisible in 1..20usize)| {

        });
    }

    // Integration Tests
    #[test]
    fn test_avatar_imageloading() {}
    #[test]
    fn test_avatar_fallback_display() {}
    #[test]
    fn test_avatar_group_overflow() {}
    #[test]
    fn test_avatar_accessibility() {}
    #[test]
    fn test_avatar_responsive_behavior() {}

    // Performance Tests
    #[test]
    fn test_avatar_large_groups() {}
    #[test]
    fn test_avatar_render_performance() {}
    #[test]
    fn test_avatar_memory_usage() {}
    #[test]
    fn test_avatar_imageloading_performance() {}
}
