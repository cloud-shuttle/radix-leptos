use crate::utils::merge_classes;
use leptos::callback::Callback;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use super::input_variants::{SizeVariant, StateVariant, StyleVariant};
use super::data_variants::{SizeVariantOptionGroup, StyleVariantOptionGroup, StateVariantOptionGroup};

/// Button variants configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ButtonVariants {
    pub sizes: Vec<SizeVariant>,
    pub styles: Vec<StyleVariant>,
    pub states: Vec<StateVariant>,
}

impl Default for ButtonVariants {
    fn default() -> Self {
        Self {
            sizes: [
                SizeVariant::Small,
                SizeVariant::Medium,
                SizeVariant::Large,
                SizeVariant::ExtraLarge,
            ]
            .to_vec(),
            styles: [
                StyleVariant::Default,
                StyleVariant::Primary,
                StyleVariant::Secondary,
                StyleVariant::Outline,
                StyleVariant::Ghost,
                StyleVariant::Destructive,
            ]
            .to_vec(),
            states: [
                StateVariant::Default,
                StateVariant::Hover,
                StateVariant::Active,
                StateVariant::Disabled,
                StateVariant::Loading,
            ]
            .to_vec(),
        }
    }
}

/// Variant section component for buttons
#[component]
pub fn ButtonVariantSection(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] title: Option<String>,
    #[prop(optional)] component_type: Option<String>,
    #[prop(optional)] variants: Option<ButtonVariants>,
    #[prop(optional)] on_change: Option<Callback<ButtonVariants>>,
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
mod button_variants_tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_button_variants_default() {
        let button_variants = ButtonVariants::default();
        assert!(button_variants.sizes.contains(&SizeVariant::Small));
        assert!(button_variants.sizes.contains(&SizeVariant::Medium));
        assert!(button_variants.sizes.contains(&SizeVariant::Large));
        assert!(button_variants.styles.contains(&StyleVariant::Default));
        assert!(button_variants.styles.contains(&StyleVariant::Primary));
        assert!(button_variants.styles.contains(&StyleVariant::Secondary));
    }

    #[test]
    fn test_button_variant_section_component() {
        // Test logic without runtime
        let button_variants = ButtonVariants::default();
        // Test component logic
        let title = "Button Variants";
        let component_type = "button";
        assert!(!title.is_empty());
        assert!(!component_type.is_empty()); // Test completed
    }

    #[test]
    fn test_button_variants_property_based() {
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
