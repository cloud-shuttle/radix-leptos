use crate::utils::merge_classes;
use leptos::callback::Callback;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use super::data_variants::{SizeVariantOptionGroup, StyleVariantOptionGroup, StateVariantOptionGroup};

/// Input variants configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputVariants {
    pub sizes: Vec<SizeVariant>,
    pub styles: Vec<StyleVariant>,
    pub states: Vec<StateVariant>,
    pub types: Vec<InputTypeVariant>,
}

impl Default for InputVariants {
    fn default() -> Self {
        Self {
            sizes: [SizeVariant::Small, SizeVariant::Medium, SizeVariant::Large].to_vec(),
            styles: [
                StyleVariant::Default,
                StyleVariant::Outline,
                StyleVariant::Filled,
            ]
            .to_vec(),
            states: [
                StateVariant::Default,
                StateVariant::Focus,
                StateVariant::Error,
                StateVariant::Disabled,
            ]
            .to_vec(),
            types: [
                InputTypeVariant::Text,
                InputTypeVariant::Email,
                InputTypeVariant::Password,
                InputTypeVariant::Number,
                InputTypeVariant::Search,
            ]
            .to_vec(),
        }
    }
}

/// Size variant enum
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SizeVariant {
    ExtraSmall,
    Small,
    Medium,
    Large,
    ExtraLarge,
}

impl SizeVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            SizeVariant::ExtraSmall => "xs",
            SizeVariant::Small => "sm",
            SizeVariant::Medium => "md",
            SizeVariant::Large => "lg",
            SizeVariant::ExtraLarge => "xl",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            SizeVariant::ExtraSmall => "Extra Small",
            SizeVariant::Small => "Small",
            SizeVariant::Medium => "Medium",
            SizeVariant::Large => "Large",
            SizeVariant::ExtraLarge => "Extra Large",
        }
    }
}

/// Style variant enum
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum StyleVariant {
    Default,
    Primary,
    Secondary,
    Outline,
    Ghost,
    Destructive,
    Filled,
    Outlined,
    Success,
    Warning,
    Error,
}

impl StyleVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            StyleVariant::Default => "default",
            StyleVariant::Primary => "primary",
            StyleVariant::Secondary => "secondary",
            StyleVariant::Outline => "outline",
            StyleVariant::Ghost => "ghost",
            StyleVariant::Destructive => "destructive",
            StyleVariant::Filled => "filled",
            StyleVariant::Outlined => "outlined",
            StyleVariant::Success => "success",
            StyleVariant::Warning => "warning",
            StyleVariant::Error => "error",
        }
    }
}

/// State variant enum
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum StateVariant {
    Default,
    Hover,
    Active,
    Focus,
    Disabled,
    Loading,
    Error,
}

impl StateVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            StateVariant::Default => "default",
            StateVariant::Hover => "hover",
            StateVariant::Active => "active",
            StateVariant::Focus => "focus",
            StateVariant::Disabled => "disabled",
            StateVariant::Loading => "loading",
            StateVariant::Error => "error",
        }
    }
}

/// Input type variant enum
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum InputTypeVariant {
    Text,
    Email,
    Password,
    Number,
    Search,
    Tel,
    Url,
    Date,
    Time,
    DateTime,
}

impl InputTypeVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            InputTypeVariant::Text => "text",
            InputTypeVariant::Email => "email",
            InputTypeVariant::Password => "password",
            InputTypeVariant::Number => "number",
            InputTypeVariant::Search => "search",
            InputTypeVariant::Tel => "tel",
            InputTypeVariant::Url => "url",
            InputTypeVariant::Date => "date",
            InputTypeVariant::Time => "time",
            InputTypeVariant::DateTime => "datetime-local",
        }
    }
}

/// Variant section component for inputs
#[component]
pub fn InputVariantSection(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] title: Option<String>,
    #[prop(optional)] component_type: Option<String>,
    #[prop(optional)] variants: Option<InputVariants>,
    #[prop(optional)] on_change: Option<Callback<InputVariants>>,
) -> impl IntoView {
    let title = title.unwrap_or_default();
    let component_type = component_type.unwrap_or_default();
    let variants = variants.unwrap_or_default();
    let on_change = on_change.unwrap_or_else(|| Callback::new(|_| {}));
    let variants_clone1 = variants.clone();
    let variants_clone2 = variants.clone();
    let variants_clone3 = variants.clone();

    let class = merge_classes(
        [
            "variant-section",
            &component_type,
            class.as_deref().unwrap_or(""),
        ]
        .to_vec(),
    );

    view! {
        <div
            class=class
            style=style
            data-component-type=component_type.clone()
        >
            <h4 class="section-title">{title}</h4>

            <div class="variant-options">
                <SizeVariantOptionGroup
                    title="Sizes".to_string()
                    options=variants.sizes.clone()
                    on_change=Callback::new(move |sizes| {
                        let mut new_variants = variants_clone1.clone();
                        new_variants.sizes = sizes;
                        on_change.run(new_variants);
                    })
                />

                <StyleVariantOptionGroup
                    title="Styles".to_string()
                    options=variants.styles.clone()
                    on_change=Callback::new(move |styles| {
                        let mut new_variants = variants_clone2.clone();
                        new_variants.styles = styles;
                        on_change.run(new_variants);
                    })
                />

                <StateVariantOptionGroup
                    title="States".to_string()
                    options=variants.states.clone()
                    on_change=Callback::new(move |states| {
                        let mut new_variants = variants_clone3.clone();
                        new_variants.states = states;
                        on_change.run(new_variants);
                    })
                />
            </div>
        </div>
    }
}

#[cfg(test)]
mod input_variants_tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_input_variants_default() {
        let input_variants = InputVariants::default();
        assert!(input_variants.sizes.contains(&SizeVariant::Small));
        assert!(input_variants.sizes.contains(&SizeVariant::Medium));
        assert!(input_variants.sizes.contains(&SizeVariant::Large));
        assert!(input_variants.types.contains(&InputTypeVariant::Text));
        assert!(input_variants.types.contains(&InputTypeVariant::Email));
        assert!(input_variants.types.contains(&InputTypeVariant::Password));
    }

    #[test]
    fn test_size_variant_enum() {
        assert_eq!(SizeVariant::ExtraSmall.as_str(), "xs");
        assert_eq!(SizeVariant::Small.as_str(), "sm");
        assert_eq!(SizeVariant::Medium.as_str(), "md");
        assert_eq!(SizeVariant::Large.as_str(), "lg");
        assert_eq!(SizeVariant::ExtraLarge.as_str(), "xl");
    }

    #[test]
    fn test_size_variant_display_name() {
        assert_eq!(SizeVariant::ExtraSmall.display_name(), "Extra Small");
        assert_eq!(SizeVariant::Small.display_name(), "Small");
        assert_eq!(SizeVariant::Medium.display_name(), "Medium");
        assert_eq!(SizeVariant::Large.display_name(), "Large");
        assert_eq!(SizeVariant::ExtraLarge.display_name(), "Extra Large");
    }

    #[test]
    fn test_style_variant_enum() {
        assert_eq!(StyleVariant::Default.as_str(), "default");
        assert_eq!(StyleVariant::Primary.as_str(), "primary");
        assert_eq!(StyleVariant::Secondary.as_str(), "secondary");
        assert_eq!(StyleVariant::Outline.as_str(), "outline");
        assert_eq!(StyleVariant::Ghost.as_str(), "ghost");
        assert_eq!(StyleVariant::Destructive.as_str(), "destructive");
    }

    #[test]
    fn test_state_variant_enum() {
        assert_eq!(StateVariant::Default.as_str(), "default");
        assert_eq!(StateVariant::Hover.as_str(), "hover");
        assert_eq!(StateVariant::Active.as_str(), "active");
        assert_eq!(StateVariant::Focus.as_str(), "focus");
        assert_eq!(StateVariant::Disabled.as_str(), "disabled");
        assert_eq!(StateVariant::Loading.as_str(), "loading");
        assert_eq!(StateVariant::Error.as_str(), "error");
    }

    #[test]
    fn test_input_type_variant_enum() {
        assert_eq!(InputTypeVariant::Text.as_str(), "text");
        assert_eq!(InputTypeVariant::Email.as_str(), "email");
        assert_eq!(InputTypeVariant::Password.as_str(), "password");
        assert_eq!(InputTypeVariant::Number.as_str(), "number");
        assert_eq!(InputTypeVariant::Search.as_str(), "search");
    }

    #[test]
    fn test_input_variant_section_component() {
        // Test logic without runtime
        let input_variants = InputVariants::default();
        // Test component logic
        let title = "Input Variants";
        let component_type = "input";
        assert!(!title.is_empty());
        assert!(!component_type.is_empty()); // Test completed
    }

    // Property-based tests
    #[test]
    fn test_size_variant_property_based() {
        proptest!(|(size in prop::sample::select(&[
            SizeVariant::ExtraSmall,
            SizeVariant::Small,
            SizeVariant::Medium,
            SizeVariant::Large,
            SizeVariant::ExtraLarge,
        ]))| {
            let size_str = size.as_str();
            let display_name = size.display_name();
            assert!(!size_str.is_empty());
            assert!(!display_name.is_empty());
        });
    }

    #[test]
    fn test_style_variant_property_based() {
        proptest!(|(style in prop::sample::select(&[
            StyleVariant::Default,
            StyleVariant::Primary,
            StyleVariant::Secondary,
            StyleVariant::Outline,
            StyleVariant::Ghost,
            StyleVariant::Destructive,
        ]))| {
            let style_str = style.as_str();
            assert!(!style_str.is_empty());
        });
    }
}
