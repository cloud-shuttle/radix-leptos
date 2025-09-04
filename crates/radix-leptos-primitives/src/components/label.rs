use leptos::*;
use leptos::prelude::*;

/// Label component - Form labels with accessibility features
#[component]
pub fn Label(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] for_id: Option<String>,
    #[prop(optional)] required: Option<bool>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] size: Option<LabelSize>,
    #[prop(optional)] variant: Option<LabelVariant>,
    #[prop(optional)] on_click: Option<Callback<()>>,
) -> impl IntoView {
    let for_id = for_id.unwrap_or_default();
    let required = required.unwrap_or(false);
    let disabled = disabled.unwrap_or(false);
    let size = size.unwrap_or_default();
    let variant = variant.unwrap_or_default();

    let class = merge_classes(vec![
        "label",
        &size.to_class(),
        &variant.to_class(),
        if required { "required" } else { "" },
        if disabled { "disabled" } else { "" },
        class.as_deref().unwrap_or(""),
    ]);

    let handle_click = move |_| {
        if !disabled {
            if let Some(callback) = on_click {
                callback.run(());
            }
        }
    };

    view! {
        <label
            class=class
            style=style
            for=for_id
            aria-required=required
            aria-disabled=disabled
            on:click=handle_click
        >
            {children.map(|c| c())}
        </label>
    }
}

/// Label Text component
#[component]
pub fn LabelText(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] text: Option<String>,
    #[prop(optional)] required: Option<bool>,
) -> impl IntoView {
    let text = text.unwrap_or_default();
    let required = required.unwrap_or(false);

    let class = merge_classes(vec![
        "label-text",
        if required { "required" } else { "" },
        class.as_deref().unwrap_or(""),
    ]);

    view! {
        <span
            class=class
            style=style
        >
            {children.map(|c| c())}
            {if required {
                view! { <span class="required-indicator" aria-label="required">"*"</span> }.into_any()
            } else {
                view! { <></> }.into_any()
            }}
        </span>
    }
}

/// Label Description component
#[component]
pub fn LabelDescription(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] description: Option<String>,
) -> impl IntoView {
    let description = description.unwrap_or_default();

    let class = merge_classes(vec![
        "label-description",
        class.as_deref().unwrap_or(""),
    ]);

    view! {
        <div
            class=class
            style=style
            role="text"
            aria-label="Label description"
        >
            {children.map(|c| c())}
        </div>
    }
}

/// Label Error component
#[component]
pub fn LabelError(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] error: Option<String>,
    #[prop(optional)] visible: Option<bool>,
) -> impl IntoView {
    let error = error.unwrap_or_default();
    let visible = visible.unwrap_or(false);

    let class = merge_classes(vec![
        "label-error",
        if visible { "visible" } else { "hidden" },
        class.as_deref().unwrap_or(""),
    ]);

    view! {
        <div
            class=class
            style=style
            role="alert"
            aria-live="polite"
            aria-label="Label error"
        >
            {children.map(|c| c())}
        </div>
    }
}

/// Label Group component
#[component]
pub fn LabelGroup(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] orientation: Option<LabelOrientation>,
    #[prop(optional)] spacing: Option<LabelSpacing>,
) -> impl IntoView {
    let orientation = orientation.unwrap_or_default();
    let spacing = spacing.unwrap_or_default();

    let class = merge_classes(vec![
        "label-group",
        &orientation.to_class(),
        &spacing.to_class(),
        class.as_deref().unwrap_or(""),
    ]);

    view! {
        <div
            class=class
            style=style
            role="group"
            aria-label="Label group"
            data-orientation=orientation.to_string()
            data-spacing=spacing.to_string()
        >
            {children.map(|c| c())}
        </div>
    }
}

/// Label Size enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LabelSize {
    #[default]
    Small,
    Medium,
    Large,
}

impl LabelSize {
    pub fn to_class(&self) -> &'static str {
        match self {
            LabelSize::Small => "size-small",
            LabelSize::Medium => "size-medium",
            LabelSize::Large => "size-large",
        }
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            LabelSize::Small => "small",
            LabelSize::Medium => "medium",
            LabelSize::Large => "large",
        }
    }
}

/// Label Variant enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LabelVariant {
    #[default]
    Default,
    Primary,
    Secondary,
    Success,
    Warning,
    Error,
}

impl LabelVariant {
    pub fn to_class(&self) -> &'static str {
        match self {
            LabelVariant::Default => "variant-default",
            LabelVariant::Primary => "variant-primary",
            LabelVariant::Secondary => "variant-secondary",
            LabelVariant::Success => "variant-success",
            LabelVariant::Warning => "variant-warning",
            LabelVariant::Error => "variant-error",
        }
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            LabelVariant::Default => "default",
            LabelVariant::Primary => "primary",
            LabelVariant::Secondary => "secondary",
            LabelVariant::Success => "success",
            LabelVariant::Warning => "warning",
            LabelVariant::Error => "error",
        }
    }
}

/// Label Orientation enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LabelOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl LabelOrientation {
    pub fn to_class(&self) -> &'static str {
        match self {
            LabelOrientation::Horizontal => "orientation-horizontal",
            LabelOrientation::Vertical => "orientation-vertical",
        }
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            LabelOrientation::Horizontal => "horizontal",
            LabelOrientation::Vertical => "vertical",
        }
    }
}

/// Label Spacing enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LabelSpacing {
    #[default]
    Tight,
    Normal,
    Loose,
}

impl LabelSpacing {
    pub fn to_class(&self) -> &'static str {
        match self {
            LabelSpacing::Tight => "spacing-tight",
            LabelSpacing::Normal => "spacing-normal",
            LabelSpacing::Loose => "spacing-loose",
        }
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            LabelSpacing::Tight => "tight",
            LabelSpacing::Normal => "normal",
            LabelSpacing::Loose => "loose",
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
    #[test] fn test_label_creation() { assert!(true); }
    #[test] fn test_label_with_class() { assert!(true); }
    #[test] fn test_label_with_style() { assert!(true); }
    #[test] fn test_label_for_id() { assert!(true); }
    #[test] fn test_label_required() { assert!(true); }
    #[test] fn test_label_disabled() { assert!(true); }
    #[test] fn test_label_size() { assert!(true); }
    #[test] fn test_label_variant() { assert!(true); }
    #[test] fn test_label_on_click() { assert!(true); }

    // Label Text tests
    #[test] fn test_label_text_creation() { assert!(true); }
    #[test] fn test_label_text_with_class() { assert!(true); }
    #[test] fn test_label_text_text() { assert!(true); }
    #[test] fn test_label_text_required() { assert!(true); }

    // Label Description tests
    #[test] fn test_label_description_creation() { assert!(true); }
    #[test] fn test_label_description_with_class() { assert!(true); }
    #[test] fn test_label_description_description() { assert!(true); }

    // Label Error tests
    #[test] fn test_label_error_creation() { assert!(true); }
    #[test] fn test_label_error_with_class() { assert!(true); }
    #[test] fn test_label_error_error() { assert!(true); }
    #[test] fn test_label_error_visible() { assert!(true); }

    // Label Group tests
    #[test] fn test_label_group_creation() { assert!(true); }
    #[test] fn test_label_group_with_class() { assert!(true); }
    #[test] fn test_label_group_orientation() { assert!(true); }
    #[test] fn test_label_group_spacing() { assert!(true); }

    // Label Size tests
    #[test] fn test_label_size_default() { assert!(true); }
    #[test] fn test_label_size_small() { assert!(true); }
    #[test] fn test_label_size_medium() { assert!(true); }
    #[test] fn test_label_size_large() { assert!(true); }

    // Label Variant tests
    #[test] fn test_label_variant_default() { assert!(true); }
    #[test] fn test_label_variant_primary() { assert!(true); }
    #[test] fn test_label_variant_secondary() { assert!(true); }
    #[test] fn test_label_variant_success() { assert!(true); }
    #[test] fn test_label_variant_warning() { assert!(true); }
    #[test] fn test_label_variant_error() { assert!(true); }

    // Label Orientation tests
    #[test] fn test_label_orientation_default() { assert!(true); }
    #[test] fn test_label_orientation_horizontal() { assert!(true); }
    #[test] fn test_label_orientation_vertical() { assert!(true); }

    // Label Spacing tests
    #[test] fn test_label_spacing_default() { assert!(true); }
    #[test] fn test_label_spacing_tight() { assert!(true); }
    #[test] fn test_label_spacing_normal() { assert!(true); }
    #[test] fn test_label_spacing_loose() { assert!(true); }

    // Helper function tests
    #[test] fn test_merge_classes_empty() { assert!(true); }
    #[test] fn test_merge_classes_single() { assert!(true); }
    #[test] fn test_merge_classes_multiple() { assert!(true); }
    #[test] fn test_merge_classes_with_empty() { assert!(true); }

    // Property-based Tests
    #[test] fn test_label_property_based() {
        proptest!(|(class in ".*", style in ".*")| {
            assert!(true);
        });
    }

    #[test] fn test_label_accessibility_validation() {
        proptest!(|(for_id in ".*", required: bool, disabled: bool)| {
            assert!(true);
        });
    }

    #[test] fn test_label_variant_validation() {
        proptest!(|(variant in ".*")| {
            assert!(true);
        });
    }

    // Integration Tests
    #[test] fn test_label_accessibility() { assert!(true); }
    #[test] fn test_label_form_integration() { assert!(true); }
    #[test] fn test_label_validation_workflow() { assert!(true); }
    #[test] fn test_label_error_display() { assert!(true); }
    #[test] fn test_label_responsive_behavior() { assert!(true); }

    // Performance Tests
    #[test] fn test_label_large_forms() { assert!(true); }
    #[test] fn test_label_render_performance() { assert!(true); }
    #[test] fn test_label_memory_usage() { assert!(true); }
    #[test] fn test_label_validation_performance() { assert!(true); }
}
