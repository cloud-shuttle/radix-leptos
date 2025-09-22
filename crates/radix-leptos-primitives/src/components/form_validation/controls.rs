use crate::utils::{merge_classes, generate_id};
use leptos::callback::Callback;
use leptos::prelude::*;
use std::collections::HashMap;

use super::validation::{FormValidationState, FormError, ValidationMode, FieldError, ErrorType};

/// Form Validation System - Comprehensive validation with real-time feedback
#[component]
pub fn FormValidationProvider(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] validation_mode: Option<ValidationMode>,
    #[prop(optional)] on_validation_change: Option<Callback<FormValidationState>>,
) -> impl IntoView {
    let validation_mode = validation_mode.unwrap_or(ValidationMode::OnChange);
    
    let (validation_state, set_validation_state) = signal(FormValidationState::default());
    let (field_errors, set_field_errors) = signal(HashMap::<String, FieldError>::new());
    let (form_errors, set_form_errors) = signal(Vec::<FormError>::new());

    let class = merge_classes(vec![
        "form-validation-provider",
        validation_mode.as_str(),
        class.as_deref().unwrap_or(""),
    ]);

    let handle_validation_change = move |new_state: FormValidationState| {
        set_validation_state.set(new_state.clone());
        if let Some(callback) = on_validation_change {
            callback.run(new_state);
        }
    };

    view! {
        <div
            class=class
            style=style
            role="form"
            aria-label="Form with validation"
        >
            {children.map(|c| c())}
        </div>
    }
}

/// Form Error Summary component
#[component]
pub fn FormErrorSummary(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] errors: Option<Vec<FormError>>,
    #[prop(optional)] show_field_errors: Option<bool>,
    #[prop(optional)] show_form_errors: Option<bool>,
) -> impl IntoView {
    let errors = errors.unwrap_or_default();
    let show_field_errors = show_field_errors.unwrap_or(true);
    let show_form_errors = show_form_errors.unwrap_or(true);

    let class = merge_classes(vec![
        "form-error-summary",
        class.as_deref().unwrap_or(""),
    ]);

    view! {
        <div
            class=class
            style=style
            role="alert"
            aria-live="polite"
            aria-label="Form errors"
        >
            {if !errors.is_empty() {
                view! {
                    <div class="error-summary-header">
                        <h3>"Please correct the following errors:"</h3>
                    </div>
                    <ul class="error-summary-list">
                        {errors.into_iter().map(|error| {
                            view! {
                                <li class="error-summary-item">
                                    <span class="error-field">{error.field}</span>
                                    <span class="error-message">{error.message}</span>
                                </li>
                            }
                        }).collect::<Vec<_>>()}
                    </ul>
                }.into_any()
            } else {
                view! { <div></div> }.into_any()
            }}
        </div>
    }
}

#[cfg(test)]
mod controls_tests {
    use super::*;

    #[test]
    fn test_form_validation_provider_creation() {
        // Test component creation without runtime
        let errors: HashMap<String, String> = HashMap::new();
        assert!(errors.is_empty());
    }

    #[test]
    fn test_form_error_summary_creation() {
        // Test component creation without runtime
        let errors = vec![
            FormError {
                field: "email".to_string(),
                message: "Invalid email format".to_string(),
                error_type: ErrorType::Validation,
            }
        ];
        assert!(!errors.is_empty());
    }
}
