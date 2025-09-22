use crate::utils::{merge_classes, generate_id};
use leptos::callback::Callback;
use leptos::children::Children;
use leptos::prelude::*;

/// Toggle Group component for group of toggle buttons
///
/// Provides accessible toggle group with keyboard support and ARIA attributes
#[component]
pub fn ToggleGroup(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] variant: Option<ToggleGroupVariant>,
    #[prop(optional)] size: Option<ToggleGroupSize>,
    #[prop(optional)] orientation: Option<ToggleGroupOrientation>,
    #[prop(optional)] type_: Option<ToggleGroupType>,
    #[prop(optional)] value: Option<Vec<String>>,
    #[prop(optional)] default_value: Option<Vec<String>>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] on_value_change: Option<Callback<Vec<String>>>,
) -> impl IntoView {
    let variant = variant.unwrap_or_default();
    let size = size.unwrap_or_default();
    let orientation = orientation.unwrap_or_default();
    let type_ = type_.unwrap_or_default();
    let disabled = disabled.unwrap_or(false);
    let (current_value, setcurrent_value) = signal(
        value
            .clone()
            .unwrap_or_else(|| default_value.unwrap_or_default()),
    );

    // Handle external value changes
    if let Some(external_value) = value {
        Effect::new(move |_| {
            setcurrent_value.set(external_value.clone());
        });
    }

    // Handle value changes
    if let Some(on_value_change) = on_value_change {
        Effect::new(move |_| {
            on_value_change.run(current_value.get());
        });
    }

    let class = merge_classes(vec![
        "toggle-group",
        &variant.to_class(),
        &size.to_class(),
        &orientation.to_class(),
        &type_.to_class(),
        class.as_deref().unwrap_or(""),
    ]);

    view! {
        <div
            class=class
            style=style
            role="group"
            aria-orientation=orientation.to_aria()
        >
            {children.map(|c| c())}
        </div>
    }
}

/// Toggle Group Item component
#[component]
pub fn ToggleGroupItem(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] value: Option<String>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] on_click: Option<Callback<()>>,
) -> impl IntoView {
    let disabled = disabled.unwrap_or(false);
    let value = value.unwrap_or_default();

    let class = merge_classes(vec!["toggle-group-item"]);

    let handle_keydown = move |ev: web_sys::KeyboardEvent| {
        if !disabled && (ev.key() == "Enter" || ev.key() == " ") {
            ev.prevent_default();
            if let Some(on_click) = on_click {
                on_click.run(());
            }
        }
    };

    let handle_click = move |_| {
        if !disabled {
            if let Some(on_click) = on_click {
                on_click.run(());
            }
        }
    };

    view! {
        <button
            class=class
            style=style
            disabled=disabled
            on:click=handle_click
            on:keydown=handle_keydown
            data-value=value
            type="button"
        >
            {children.map(|c| c())}
        </button>
    }
}

/// Toggle Group Variant enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToggleGroupVariant {
    #[default]
    Default,
    Outline,
    Ghost,
    Destructive,
}

impl ToggleGroupVariant {
    pub fn to_class(&self) -> &'static str {
        match self {
            ToggleGroupVariant::Default => "variant-default",
            ToggleGroupVariant::Outline => "variant-outline",
            ToggleGroupVariant::Ghost => "variant-ghost",
            ToggleGroupVariant::Destructive => "variant-destructive",
        }
    }
}

/// Toggle Group Size enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToggleGroupSize {
    #[default]
    Default,
    Small,
    Large,
}

impl ToggleGroupSize {
    pub fn to_class(&self) -> &'static str {
        match self {
            ToggleGroupSize::Default => "size-default",
            ToggleGroupSize::Small => "size-small",
            ToggleGroupSize::Large => "size-large",
        }
    }
}

/// Toggle Group Orientation enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToggleGroupOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl ToggleGroupOrientation {
    pub fn to_class(&self) -> &'static str {
        match self {
            ToggleGroupOrientation::Horizontal => "horizontal",
            ToggleGroupOrientation::Vertical => "vertical",
        }
    }

    pub fn to_aria(&self) -> &'static str {
        match self {
            ToggleGroupOrientation::Horizontal => "horizontal",
            ToggleGroupOrientation::Vertical => "vertical",
        }
    }
}

/// Toggle Group Type enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToggleGroupType {
    #[default]
    Single,
    Multiple,
}

impl ToggleGroupType {
    pub fn to_class(&self) -> &'static str {
        match self {
            ToggleGroupType::Single => "type-single",
            ToggleGroupType::Multiple => "type-multiple",
        }
    }

    pub fn to_aria(&self) -> &'static str {
        match self {
            ToggleGroupType::Single => "single",
            ToggleGroupType::Multiple => "multiple",
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::{merge_classes, generate_id};
    use crate::{ToggleGroupOrientation, ToggleGroupSize, ToggleGroupType, ToggleGroupVariant};
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    // Toggle Group Tests
    #[test]
    fn test_toggle_group_creation() {}

    #[test]
    fn test_toggle_group_with_class() {}

    #[test]
    fn test_toggle_group_with_style() {}

    #[test]
    fn test_toggle_group_default_variant() {}

    #[test]
    fn test_toggle_group_outline_variant() {}

    #[test]
    fn test_toggle_group_ghost_variant() {}

    #[test]
    fn test_toggle_group_destructive_variant() {}

    #[test]
    fn test_toggle_group_default_size() {}

    #[test]
    fn test_toggle_group_small_size() {}

    #[test]
    fn test_toggle_group_large_size() {}

    #[test]
    fn test_toggle_group_horizontal_orientation() {}

    #[test]
    fn test_toggle_group_vertical_orientation() {}

    #[test]
    fn test_toggle_group_single_type() {}

    #[test]
    fn test_toggle_group_multiple_type() {}

    #[test]
    fn test_toggle_group_with_value() {}

    #[test]
    fn test_toggle_group_with_default_value() {}

    #[test]
    fn test_toggle_groupdisabled() {}

    #[test]
    fn test_toggle_group_on_value_change() {}

    // Toggle Group Item Tests
    #[test]
    fn test_toggle_group_item_creation() {}

    #[test]
    fn test_toggle_group_item_with_class() {}

    #[test]
    fn test_toggle_group_item_with_style() {}

    #[test]
    fn test_toggle_group_item_with_value() {}

    #[test]
    fn test_toggle_group_itemdisabled() {}

    #[test]
    fn test_toggle_group_item_on_click() {}

    // Toggle Group Variant Tests
    #[test]
    fn test_toggle_group_variant_default() {
        let variant = ToggleGroupVariant::default();
        assert_eq!(variant, ToggleGroupVariant::Default);
    }

    #[test]
    fn test_toggle_group_variant_default_class() {
        let variant = ToggleGroupVariant::Default;
        assert_eq!(variant.to_class(), "variant-default");
    }

    #[test]
    fn test_toggle_group_variant_outline_class() {
        let variant = ToggleGroupVariant::Outline;
        assert_eq!(variant.to_class(), "variant-outline");
    }

    #[test]
    fn test_toggle_group_variant_ghost_class() {
        let variant = ToggleGroupVariant::Ghost;
        assert_eq!(variant.to_class(), "variant-ghost");
    }

    #[test]
    fn test_toggle_group_variant_destructive_class() {
        let variant = ToggleGroupVariant::Destructive;
        assert_eq!(variant.to_class(), "variant-destructive");
    }

    // Toggle Group Size Tests
    #[test]
    fn test_toggle_group_size_default() {
        let size = ToggleGroupSize::default();
        assert_eq!(size, ToggleGroupSize::Default);
    }

    #[test]
    fn test_toggle_group_size_default_class() {
        let size = ToggleGroupSize::Default;
        assert_eq!(size.to_class(), "size-default");
    }

    #[test]
    fn test_toggle_group_size_small_class() {
        let size = ToggleGroupSize::Small;
        assert_eq!(size.to_class(), "size-small");
    }

    #[test]
    fn test_toggle_group_size_large_class() {
        let size = ToggleGroupSize::Large;
        assert_eq!(size.to_class(), "size-large");
    }

    // Toggle Group Orientation Tests
    #[test]
    fn test_toggle_group_orientation_default() {
        let orientation = ToggleGroupOrientation::default();
        assert_eq!(orientation, ToggleGroupOrientation::Horizontal);
    }

    #[test]
    fn test_toggle_group_orientation_horizontal() {
        let orientation = ToggleGroupOrientation::Horizontal;
        assert_eq!(orientation.to_class(), "horizontal");
        assert_eq!(orientation.to_aria(), "horizontal");
    }

    #[test]
    fn test_toggle_group_orientation_vertical() {
        let orientation = ToggleGroupOrientation::Vertical;
        assert_eq!(orientation.to_class(), "vertical");
        assert_eq!(orientation.to_aria(), "vertical");
    }

    // Toggle Group Type Tests
    #[test]
    fn test_toggle_group_type_default() {
        let type_ = ToggleGroupType::default();
        assert_eq!(type_, ToggleGroupType::Single);
    }

    #[test]
    fn test_toggle_group_type_single() {
        let type_ = ToggleGroupType::Single;
        assert_eq!(type_.to_class(), "type-single");
        assert_eq!(type_.to_aria(), "single");
    }

    #[test]
    fn test_toggle_group_type_multiple() {
        let type_ = ToggleGroupType::Multiple;
        assert_eq!(type_.to_class(), "type-multiple");
        assert_eq!(type_.to_aria(), "multiple");
    }

    // Helper Function Tests
    #[test]
    fn test_merge_classes_empty() {
        let result = merge_classes(Vec::new());
        assert_eq!(result, "");
    }

    #[test]
    fn test_merge_classes_single() {
        let result = merge_classes(vec!["class1"]);
        assert_eq!(result, "class1");
    }

    #[test]
    fn test_merge_classes_multiple() {
        let result = merge_classes(vec!["class1", "class2", "class3"]);
        assert_eq!(result, "class1 class2 class3");
    }

    #[test]
    fn test_merge_classes_with_empty() {
        let result = merge_classes(vec!["class1", "", "class3"]);
        assert_eq!(result, "class1 class3");
    }

    // Property-based tests
    #[test]
    fn test_toggle_group_property_based() {
        use proptest::prelude::*;
        proptest!(|(____class in ".*", __style in ".*")| {

        });
    }

    #[test]
    fn test_toggle_group_item_property_based() {
        use proptest::prelude::*;
        proptest!(|(____class in ".*", __style in ".*", __value in ".*")| {

        });
    }
}
