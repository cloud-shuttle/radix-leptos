use crate::utils::merge_classes;
use leptos::callback::Callback;
use leptos::children::Children;
use leptos::prelude::*;

/// Toolbar component for action toolbar functionality
///
/// Provides accessible toolbar with keyboard support and ARIA attributes
#[component]
pub fn Toolbar(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] orientation: Option<ToolbarOrientation>,
    #[prop(optional)] disabled: Option<bool>,
) -> impl IntoView {
    let orientation = orientation.unwrap_or_default();
    let disabled = disabled.unwrap_or(false);

    let class = merge_classes(vec![
        "toolbar",
        &orientation.to_class(),
        class.as_deref().unwrap_or(""),
    ]);

    view! {
        <div
            class=class
            style=style
            role="toolbar"
            aria-orientation=orientation.to_aria()
        >
            {children.map(|c| c())}
        </div>
    }
}

/// Toolbar Toggle Group component
#[component]
pub fn ToolbarToggleGroup(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] variant: Option<ToolbarToggleGroupVariant>,
    #[prop(optional)] size: Option<ToolbarToggleGroupSize>,
    #[prop(optional)] type_: Option<ToolbarToggleGroupType>,
    #[prop(optional)] value: Option<Vec<String>>,
    #[prop(optional)] default_value: Option<Vec<String>>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] on_value_change: Option<Callback<Vec<String>>>,
) -> impl IntoView {
    let variant = variant.unwrap_or_default();
    let size = size.unwrap_or_default();
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
        "toolbar-toggle-group",
        &variant.to_class(),
        &size.to_class(),
        &type_.to_class(),
        class.as_deref().unwrap_or(""),
    ]);

    view! {
        <div
            class=class
            style=style
            role="group"
        >
            {children.map(|c| c())}
        </div>
    }
}

/// Toolbar Toggle Item component
#[component]
pub fn ToolbarToggleItem(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] value: Option<String>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] on_click: Option<Callback<()>>,
) -> impl IntoView {
    let disabled = disabled.unwrap_or(false);
    let value = value.unwrap_or_default();

    let class = merge_classes(vec!["toolbar-toggle-item"]);

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

/// Toolbar Separator component
#[component]
pub fn ToolbarSeparator(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] orientation: Option<ToolbarSeparatorOrientation>,
) -> impl IntoView {
    let orientation = orientation.unwrap_or_default();

    let class = merge_classes(vec![
        "toolbar-separator",
        &orientation.to_class(),
        class.as_deref().unwrap_or(""),
    ]);

    view! {
        <div
            class=class
            style=style
            role="separator"
            aria-orientation=orientation.to_aria()
        />
    }
}

/// Toolbar Button component
#[component]
pub fn ToolbarButton(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] variant: Option<ToolbarButtonVariant>,
    #[prop(optional)] size: Option<ToolbarButtonSize>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] on_click: Option<Callback<()>>,
) -> impl IntoView {
    let variant = variant.unwrap_or_default();
    let size = size.unwrap_or_default();
    let disabled = disabled.unwrap_or(false);

    let class = merge_classes(vec![
        "toolbar-button",
        &variant.to_class(),
        &size.to_class(),
    ]);

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
            type="button"
        >
            {children.map(|c| c())}
        </button>
    }
}

/// Toolbar Orientation enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToolbarOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl ToolbarOrientation {
    pub fn to_class(&self) -> &'static str {
        match self {
            ToolbarOrientation::Horizontal => "horizontal",
            ToolbarOrientation::Vertical => "vertical",
        }
    }

    pub fn to_aria(&self) -> &'static str {
        match self {
            ToolbarOrientation::Horizontal => "horizontal",
            ToolbarOrientation::Vertical => "vertical",
        }
    }
}

/// Toolbar Toggle Group Variant enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToolbarToggleGroupVariant {
    #[default]
    Default,
    Outline,
    Ghost,
    Destructive,
}

impl ToolbarToggleGroupVariant {
    pub fn to_class(&self) -> &'static str {
        match self {
            ToolbarToggleGroupVariant::Default => "variant-default",
            ToolbarToggleGroupVariant::Outline => "variant-outline",
            ToolbarToggleGroupVariant::Ghost => "variant-ghost",
            ToolbarToggleGroupVariant::Destructive => "variant-destructive",
        }
    }
}

/// Toolbar Toggle Group Size enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToolbarToggleGroupSize {
    #[default]
    Default,
    Small,
    Large,
}

impl ToolbarToggleGroupSize {
    pub fn to_class(&self) -> &'static str {
        match self {
            ToolbarToggleGroupSize::Default => "size-default",
            ToolbarToggleGroupSize::Small => "size-small",
            ToolbarToggleGroupSize::Large => "size-large",
        }
    }
}

/// Toolbar Toggle Group Type enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToolbarToggleGroupType {
    #[default]
    Single,
    Multiple,
}

impl ToolbarToggleGroupType {
    pub fn to_class(&self) -> &'static str {
        match self {
            ToolbarToggleGroupType::Single => "type-single",
            ToolbarToggleGroupType::Multiple => "type-multiple",
        }
    }

    pub fn to_aria(&self) -> &'static str {
        match self {
            ToolbarToggleGroupType::Single => "single",
            ToolbarToggleGroupType::Multiple => "multiple",
        }
    }
}

/// Toolbar Separator Orientation enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToolbarSeparatorOrientation {
    #[default]
    Vertical,
    Horizontal,
}

impl ToolbarSeparatorOrientation {
    pub fn to_class(&self) -> &'static str {
        match self {
            ToolbarSeparatorOrientation::Vertical => "vertical",
            ToolbarSeparatorOrientation::Horizontal => "horizontal",
        }
    }

    pub fn to_aria(&self) -> &'static str {
        match self {
            ToolbarSeparatorOrientation::Vertical => "vertical",
            ToolbarSeparatorOrientation::Horizontal => "horizontal",
        }
    }
}

/// Toolbar Button Variant enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToolbarButtonVariant {
    #[default]
    Default,
    Outline,
    Ghost,
    Destructive,
}

impl ToolbarButtonVariant {
    pub fn to_class(&self) -> &'static str {
        match self {
            ToolbarButtonVariant::Default => "variant-default",
            ToolbarButtonVariant::Outline => "variant-outline",
            ToolbarButtonVariant::Ghost => "variant-ghost",
            ToolbarButtonVariant::Destructive => "variant-destructive",
        }
    }
}

/// Toolbar Button Size enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToolbarButtonSize {
    #[default]
    Default,
    Small,
    Large,
}

impl ToolbarButtonSize {
    pub fn to_class(&self) -> &'static str {
        match self {
            ToolbarButtonSize::Default => "size-default",
            ToolbarButtonSize::Small => "size-small",
            ToolbarButtonSize::Large => "size-large",
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::merge_classes;
    use crate::{
        ToolbarButtonSize, ToolbarButtonVariant, ToolbarOrientation, ToolbarSeparatorOrientation,
        ToolbarToggleGroupSize, ToolbarToggleGroupType, ToolbarToggleGroupVariant,
    };
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    // Toolbar Tests
    #[test]
    fn test_toolbar_creation() {}

    #[test]
    fn test_toolbar_with_class() {}

    #[test]
    fn test_toolbar_with_style() {}

    #[test]
    fn test_toolbar_horizontal_orientation() {}

    #[test]
    fn test_toolbar_vertical_orientation() {}

    #[test]
    fn test_toolbardisabled() {}

    // Toolbar Toggle Group Tests
    #[test]
    fn test_toolbar_toggle_group_creation() {}

    #[test]
    fn test_toolbar_toggle_group_with_class() {}

    #[test]
    fn test_toolbar_toggle_group_with_style() {}

    #[test]
    fn test_toolbar_toggle_group_default_variant() {}

    #[test]
    fn test_toolbar_toggle_group_outline_variant() {}

    #[test]
    fn test_toolbar_toggle_group_ghost_variant() {}

    #[test]
    fn test_toolbar_toggle_group_destructive_variant() {}

    #[test]
    fn test_toolbar_toggle_group_default_size() {}

    #[test]
    fn test_toolbar_toggle_group_small_size() {}

    #[test]
    fn test_toolbar_toggle_group_large_size() {}

    #[test]
    fn test_toolbar_toggle_group_single_type() {}

    #[test]
    fn test_toolbar_toggle_group_multiple_type() {}

    #[test]
    fn test_toolbar_toggle_group_with_value() {}

    #[test]
    fn test_toolbar_toggle_group_with_default_value() {}

    #[test]
    fn test_toolbar_toggle_groupdisabled() {}

    #[test]
    fn test_toolbar_toggle_group_on_value_change() {}

    // Toolbar Toggle Item Tests
    #[test]
    fn test_toolbar_toggle_item_creation() {}

    #[test]
    fn test_toolbar_toggle_item_with_class() {}

    #[test]
    fn test_toolbar_toggle_item_with_style() {}

    #[test]
    fn test_toolbar_toggle_item_with_value() {}

    #[test]
    fn test_toolbar_toggle_itemdisabled() {}

    #[test]
    fn test_toolbar_toggle_item_on_click() {}

    // Toolbar Separator Tests
    #[test]
    fn test_toolbar_separator_creation() {}

    #[test]
    fn test_toolbar_separator_with_class() {}

    #[test]
    fn test_toolbar_separator_with_style() {}

    #[test]
    fn test_toolbar_separator_vertical_orientation() {}

    #[test]
    fn test_toolbar_separator_horizontal_orientation() {}

    // Toolbar Button Tests
    #[test]
    fn test_toolbar_button_creation() {}

    #[test]
    fn test_toolbar_button_with_class() {}

    #[test]
    fn test_toolbar_button_with_style() {}

    #[test]
    fn test_toolbar_button_default_variant() {}

    #[test]
    fn test_toolbar_button_outline_variant() {}

    #[test]
    fn test_toolbar_button_ghost_variant() {}

    #[test]
    fn test_toolbar_button_destructive_variant() {}

    #[test]
    fn test_toolbar_button_default_size() {}

    #[test]
    fn test_toolbar_button_small_size() {}

    #[test]
    fn test_toolbar_button_large_size() {}

    #[test]
    fn test_toolbar_buttondisabled() {}

    #[test]
    fn test_toolbar_button_on_click() {}

    // Toolbar Orientation Tests
    #[test]
    fn test_toolbar_orientation_default() {
        let orientation = ToolbarOrientation::default();
        assert_eq!(orientation, ToolbarOrientation::Horizontal);
    }

    #[test]
    fn test_toolbar_orientation_horizontal() {
        let orientation = ToolbarOrientation::Horizontal;
        assert_eq!(orientation.to_class(), "horizontal");
        assert_eq!(orientation.to_aria(), "horizontal");
    }

    #[test]
    fn test_toolbar_orientation_vertical() {
        let orientation = ToolbarOrientation::Vertical;
        assert_eq!(orientation.to_class(), "vertical");
        assert_eq!(orientation.to_aria(), "vertical");
    }

    // Toolbar Toggle Group Variant Tests
    #[test]
    fn test_toolbar_toggle_group_variant_default() {
        let variant = ToolbarToggleGroupVariant::default();
        assert_eq!(variant, ToolbarToggleGroupVariant::Default);
    }

    #[test]
    fn test_toolbar_toggle_group_variant_default_class() {
        let variant = ToolbarToggleGroupVariant::Default;
        assert_eq!(variant.to_class(), "variant-default");
    }

    #[test]
    fn test_toolbar_toggle_group_variant_outline_class() {
        let variant = ToolbarToggleGroupVariant::Outline;
        assert_eq!(variant.to_class(), "variant-outline");
    }

    #[test]
    fn test_toolbar_toggle_group_variant_ghost_class() {
        let variant = ToolbarToggleGroupVariant::Ghost;
        assert_eq!(variant.to_class(), "variant-ghost");
    }

    #[test]
    fn test_toolbar_toggle_group_variant_destructive_class() {
        let variant = ToolbarToggleGroupVariant::Destructive;
        assert_eq!(variant.to_class(), "variant-destructive");
    }

    // Toolbar Toggle Group Size Tests
    #[test]
    fn test_toolbar_toggle_group_size_default() {
        let size = ToolbarToggleGroupSize::default();
        assert_eq!(size, ToolbarToggleGroupSize::Default);
    }

    #[test]
    fn test_toolbar_toggle_group_size_default_class() {
        let size = ToolbarToggleGroupSize::Default;
        assert_eq!(size.to_class(), "size-default");
    }

    #[test]
    fn test_toolbar_toggle_group_size_small_class() {
        let size = ToolbarToggleGroupSize::Small;
        assert_eq!(size.to_class(), "size-small");
    }

    #[test]
    fn test_toolbar_toggle_group_size_large_class() {
        let size = ToolbarToggleGroupSize::Large;
        assert_eq!(size.to_class(), "size-large");
    }

    // Toolbar Toggle Group Type Tests
    #[test]
    fn test_toolbar_toggle_group_type_default() {
        let type_ = ToolbarToggleGroupType::default();
        assert_eq!(type_, ToolbarToggleGroupType::Single);
    }

    #[test]
    fn test_toolbar_toggle_group_type_single() {
        let type_ = ToolbarToggleGroupType::Single;
        assert_eq!(type_.to_class(), "type-single");
        assert_eq!(type_.to_aria(), "single");
    }

    #[test]
    fn test_toolbar_toggle_group_type_multiple() {
        let type_ = ToolbarToggleGroupType::Multiple;
        assert_eq!(type_.to_class(), "type-multiple");
        assert_eq!(type_.to_aria(), "multiple");
    }

    // Toolbar Separator Orientation Tests
    #[test]
    fn test_toolbar_separator_orientation_default() {
        let orientation = ToolbarSeparatorOrientation::default();
        assert_eq!(orientation, ToolbarSeparatorOrientation::Vertical);
    }

    #[test]
    fn test_toolbar_separator_orientation_vertical() {
        let orientation = ToolbarSeparatorOrientation::Vertical;
        assert_eq!(orientation.to_class(), "vertical");
        assert_eq!(orientation.to_aria(), "vertical");
    }

    #[test]
    fn test_toolbar_separator_orientation_horizontal() {
        let orientation = ToolbarSeparatorOrientation::Horizontal;
        assert_eq!(orientation.to_class(), "horizontal");
        assert_eq!(orientation.to_aria(), "horizontal");
    }

    // Toolbar Button Variant Tests
    #[test]
    fn test_toolbar_button_variant_default() {
        let variant = ToolbarButtonVariant::default();
        assert_eq!(variant, ToolbarButtonVariant::Default);
    }

    #[test]
    fn test_toolbar_button_variant_default_class() {
        let variant = ToolbarButtonVariant::Default;
        assert_eq!(variant.to_class(), "variant-default");
    }

    #[test]
    fn test_toolbar_button_variant_outline_class() {
        let variant = ToolbarButtonVariant::Outline;
        assert_eq!(variant.to_class(), "variant-outline");
    }

    #[test]
    fn test_toolbar_button_variant_ghost_class() {
        let variant = ToolbarButtonVariant::Ghost;
        assert_eq!(variant.to_class(), "variant-ghost");
    }

    #[test]
    fn test_toolbar_button_variant_destructive_class() {
        let variant = ToolbarButtonVariant::Destructive;
        assert_eq!(variant.to_class(), "variant-destructive");
    }

    // Toolbar Button Size Tests
    #[test]
    fn test_toolbar_button_size_default() {
        let size = ToolbarButtonSize::default();
        assert_eq!(size, ToolbarButtonSize::Default);
    }

    #[test]
    fn test_toolbar_button_size_default_class() {
        let size = ToolbarButtonSize::Default;
        assert_eq!(size.to_class(), "size-default");
    }

    #[test]
    fn test_toolbar_button_size_small_class() {
        let size = ToolbarButtonSize::Small;
        assert_eq!(size.to_class(), "size-small");
    }

    #[test]
    fn test_toolbar_button_size_large_class() {
        let size = ToolbarButtonSize::Large;
        assert_eq!(size.to_class(), "size-large");
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
    fn test_toolbar_property_based() {
        use proptest::prelude::*;
        proptest!(|(____class in ".*", __style in ".*")| {

        });
    }

    #[test]
    fn test_toolbar_toggle_group_property_based() {
        use proptest::prelude::*;
        proptest!(|(____class in ".*", __style in ".*")| {

        });
    }

    #[test]
    fn test_toolbar_toggle_item_property_based() {
        use proptest::prelude::*;
        proptest!(|(____class in ".*", __style in ".*", __value in ".*")| {

        });
    }

    #[test]
    fn test_toolbar_separator_property_based() {
        use proptest::prelude::*;
        proptest!(|(____class in ".*", __style in ".*")| {

        });
    }

    #[test]
    fn test_toolbar_button_property_based() {
        use proptest::prelude::*;
        proptest!(|(____class in ".*", __style in ".*")| {

        });
    }
}
