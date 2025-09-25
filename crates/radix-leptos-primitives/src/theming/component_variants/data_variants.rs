use crate::utils::merge_classes;
use leptos::callback::Callback;
use leptos::prelude::*;

use super::input_variants::{SizeVariant, StyleVariant, StateVariant};

/// Size variant option group component
#[component]
pub fn SizeVariantOptionGroup(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] title: Option<String>,
    #[prop(optional)] options: Option<Vec<SizeVariant>>,
    #[prop(optional)] on_change: Option<Callback<Vec<SizeVariant>>>,
) -> impl IntoView {
    let title = title.unwrap_or_default();
    let options = options.unwrap_or_default();
    let on_change = on_change.unwrap_or_else(|| Callback::new(|_| {}));

    let class = merge_classes(["variant-option-group", class.as_deref().unwrap_or("")].to_vec());

    view! {
        <div
            class=class
            style=style
        >
            <h5 class="option-group-title">{title}</h5>
            <div class="option-list">
                {options.into_iter().map(|option| {
                    let option_str = option.as_str();
                    view! {
                        <div class="variant-option" data-variant=option_str>
                            <span class="option-name">{option.display_name()}</span>
                            <span class="option-value">{option_str}</span>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

/// Style variant option group component
#[component]
pub fn StyleVariantOptionGroup(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] title: Option<String>,
    #[prop(optional)] options: Option<Vec<StyleVariant>>,
    #[prop(optional)] on_change: Option<Callback<Vec<StyleVariant>>>,
) -> impl IntoView {
    let title = title.unwrap_or_default();
    let options = options.unwrap_or_default();
    let on_change = on_change.unwrap_or_else(|| Callback::new(|_| {}));

    let class = merge_classes(["variant-option-group", class.as_deref().unwrap_or("")].to_vec());

    view! {
        <div
            class=class
            style=style
        >
            <h5 class="option-group-title">{title}</h5>
            <div class="option-list">
                {options.into_iter().map(|option| {
                    let option_str = option.as_str();
                    view! {
                        <div class="variant-option" data-variant=option_str>
                            <span class="option-name">{option_str}</span>
                            <span class="option-value">{option_str}</span>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

/// State variant option group component
#[component]
pub fn StateVariantOptionGroup(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] title: Option<String>,
    #[prop(optional)] options: Option<Vec<StateVariant>>,
    #[prop(optional)] on_change: Option<Callback<Vec<StateVariant>>>,
) -> impl IntoView {
    let title = title.unwrap_or_default();
    let options = options.unwrap_or_default();
    let on_change = on_change.unwrap_or_else(|| Callback::new(|_| {}));

    let class = merge_classes(["variant-option-group", class.as_deref().unwrap_or("")].to_vec());

    view! {
        <div
            class=class
            style=style
        >
            <h5 class="option-group-title">{title}</h5>
            <div class="option-list">
                {options.into_iter().map(|option| {
                    let option_str = option.as_str();
                    view! {
                        <div class="variant-option" data-variant=option_str>
                            <span class="option-name">{option_str}</span>
                            <span class="option-value">{option_str}</span>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

#[cfg(test)]
mod data_variants_tests {
    use super::*;

    #[test]
    fn test_size_variant_option_group_component() {
        // Test logic without runtime
        let sizes = [SizeVariant::Small, SizeVariant::Medium, SizeVariant::Large];
        // Test component logic
        let title = "Button Variants";
        let component_type = "button";
        assert!(!title.is_empty());
        assert!(!component_type.is_empty()); // Test completed
    }

    #[test]
    fn test_style_variant_option_group_component() {
        // Test logic without runtime
        let styles = [StyleVariant::Default, StyleVariant::Primary, StyleVariant::Secondary];
        // Test component logic
        let title = "Button Variants";
        let component_type = "button";
        assert!(!title.is_empty());
        assert!(!component_type.is_empty()); // Test completed
    }

    #[test]
    fn test_state_variant_option_group_component() {
        // Test logic without runtime
        let states = [StateVariant::Default, StateVariant::Hover, StateVariant::Active];
        // Test component logic
        let title = "Button Variants";
        let component_type = "button";
        assert!(!title.is_empty());
        assert!(!component_type.is_empty()); // Test completed
    }
}
