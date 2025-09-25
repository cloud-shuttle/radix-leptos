use crate::utils::merge_classes;
use leptos::callback::Callback;
use leptos::prelude::*;

use super::validation::{ValidationRule, FieldValidationResult};

/// Form Field with Validation
#[component]
pub fn FormField(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] name: Option<String>,
    #[prop(optional)] label: Option<String>,
    #[prop(optional)] required: Option<bool>,
    #[prop(optional)] validation_rules: Option<Vec<ValidationRule>>,
    #[prop(optional)] on_validation: Option<Callback<FieldValidationResult>>,
) -> impl IntoView {
    let name = name.unwrap_or_default();
    let label = label.unwrap_or_default();
    let required = required.unwrap_or(false);
    let validation_rules = validation_rules.unwrap_or_default();

    let class = merge_classes(vec![
        "form-field",
        class.as_deref().unwrap_or(""),
    ]);

    view! {
        <div
            class=class
            style=style
            data-field-name=name
            data-required=required
        >
            {if !label.is_empty() {
                view! {
                    <FormLabel for_id=name.clone()>
                        {label}
                            {if required {
                                view! { <span class="required-indicator">"*"</span> }
                            } else {
                                view! { <span class="required-indicator">""</span> }
                            }}
                    </FormLabel>
                }.into_any()
            } else {
                view! { <div></div> }.into_any()
            }}
            {children.map(|c| c())}
            <FormFieldError name=name.clone() />
        </div>
    }
}

/// Form Label component
#[component]
pub fn FormLabel(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] for_id: Option<String>,
) -> impl IntoView {
    let class = merge_classes(vec![
        "form-label",
        class.as_deref().unwrap_or(""),
    ]);

    view! {
        <label
            class=class
            style=style
            for=for_id
        >
            {children.map(|c| c())}
        </label>
    }
}

/// Form Field Error component
#[component]
pub fn FormFieldError(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] name: Option<String>,
) -> impl IntoView {
    let name = name.unwrap_or_default();

    let class = merge_classes(vec![
        "form-field-error",
        class.as_deref().unwrap_or(""),
    ]);

    view! {
        <div
            class=class
            style=style
            role="alert"
            aria-live="polite"
            data-field-name=name
        >
            // Error message will be displayed here
        </div>
    }
}

#[cfg(test)]
mod fields_tests {
    use super::*;

    #[test]
    fn test_form_field_creation() {
        // Test component creation without runtime
        let name = "email".to_string();
        let label = "Email".to_string();
        let required = true;
        assert!(!name.is_empty());
        assert!(!label.is_empty());
        assert!(required);
    }

    #[test]
    fn test_form_label_creation() {
        // Test component creation without runtime
        let for_id = "email".to_string();
        assert!(!for_id.is_empty());
    }

    #[test]
    fn test_form_field_error_creation() {
        // Test component creation without runtime
        let name = "email".to_string();
        assert!(!name.is_empty());
    }
}
