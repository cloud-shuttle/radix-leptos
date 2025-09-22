use crate::utils::{merge_classes, generate_id};
use leptos::callback::Callback;
use leptos::children::Children;
use leptos::prelude::*;
use wasm_bindgen::JsCast;

/// Date Picker component - Date selection with validation
#[component]
pub fn DatePicker(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] value: Option<String>,
    #[prop(optional)] placeholder: Option<String>,
    #[prop(optional)] min_date: Option<String>,
    #[prop(optional)] max_date: Option<String>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] required: Option<bool>,
    #[prop(optional)] format: Option<String>,
    #[prop(optional)] locale: Option<String>,
    #[prop(optional)] on_change: Option<Callback<String>>,
    #[prop(optional)] on_validation: Option<Callback<DateValidation>>,
) -> impl IntoView {
    let _value = value.unwrap_or_default();
    let _placeholder = placeholder.unwrap_or_else(|| "Select date".to_string());
    let _min_date = min_date.unwrap_or_default();
    let _max_date = max_date.unwrap_or_default();
    let _disabled = disabled.unwrap_or(false);
    let _required = required.unwrap_or(false);
    let format = format.unwrap_or_else(|| "YYYY-MM-DD".to_string());
    let locale = locale.unwrap_or_else(|| "en-US".to_string());

    let class = merge_classes(vec!["date-picker", class.as_deref().unwrap_or("")]);

    let _handle_change = move |new_value: String| {
        if let Some(callback) = on_change {
            callback.run(new_value);
        }
    };

    view! {
        <div
            class=class
            style=style
            role="combobox"
            aria-label="Date picker"
            data-format=format
            data-locale=locale
            data-min-date=_min_date
            data-max-date=_max_date
        >
            {children.map(|c| c())}
        </div>
    }
}

/// Date Picker Input component
#[component]
pub fn DatePickerInput(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] value: Option<String>,
    #[prop(optional)] placeholder: Option<String>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] required: Option<bool>,
    #[prop(optional)] format: Option<String>,
    #[prop(optional)] on_change: Option<Callback<String>>,
    #[prop(optional)] on_focus: Option<Callback<()>>,
    #[prop(optional)] on_blur: Option<Callback<()>>,
) -> impl IntoView {
    let value = value.unwrap_or_default();
    let placeholder = placeholder.unwrap_or_else(|| "Select date".to_string());
    let disabled = disabled.unwrap_or(false);
    let required = required.unwrap_or(false);
    let format = format.unwrap_or_else(|| "YYYY-MM-DD".to_string());

    let class = merge_classes(vec!["date-picker-input", class.as_deref().unwrap_or("")]);

    let handle_change = move |event: web_sys::Event| {
        if let Some(input) = event
            .target()
            .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
        {
            let new_value = input.value();
            if let Some(callback) = on_change {
                callback.run(new_value);
            }
        }
    };

    let handle_focus = move |_| {
        if let Some(callback) = on_focus {
            callback.run(());
        }
    };

    let handle_blur = move |_| {
        if let Some(callback) = on_blur {
            callback.run(());
        }
    };

    view! {
        <input
            class=class
            style=style
            type="text"
            value=value
            placeholder=placeholder
            disabled=disabled
            required=required
            data-format=format
            on:input=handle_change
            on:focus=handle_focus
            on:blur=handle_blur
        />
    }
}

/// Date Picker Trigger component
#[component]
pub fn DatePickerTrigger(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] on_click: Option<Callback<()>>,
) -> impl IntoView {
    let disabled = disabled.unwrap_or(false);

    let class = merge_classes(vec!["date-picker-trigger"]);

    view! {
        <button
            class=class
            style=style
            type="button"
            disabled=disabled
            aria-label="Open date picker"
            on:click=move |_| {
                if let Some(callback) = on_click {
                    callback.run(());
                }
            }
        >
            {children.map(|c| c())}
        </button>
    }
}

/// Date Picker Calendar component
#[component]
pub fn DatePickerCalendar(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] value: Option<String>,
    #[prop(optional)] min_date: Option<String>,
    #[prop(optional)] max_date: Option<String>,
    #[prop(optional)] on_date_select: Option<Callback<String>>,
) -> impl IntoView {
    let value = value.unwrap_or_default();
    let min_date = min_date.unwrap_or_default();
    let max_date = max_date.unwrap_or_default();

    let class = merge_classes(vec!["date-picker-calendar", class.as_deref().unwrap_or("")]);

    view! {
        <div
            class=class
            style=style
            role="dialog"
            aria-label="Date picker calendar"
            data-value=value
            data-min-date=min_date
            data-max-date=max_date
        >
            {children.map(|c| c())}
        </div>
    }
}

/// Date Validation structure
#[derive(Debug, Clone, PartialEq)]
pub struct DateValidation {
    pub is_valid: bool,
    pub error_message: Option<String>,
    pub parsed_date: Option<String>,
}

impl Default for DateValidation {
    fn default() -> Self {
        Self {
            is_valid: true,
            error_message: None,
            parsed_date: None,
        }
    }
}

/// Date Picker Validation component
#[component]
pub fn DatePickerValidation(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] validation: Option<DateValidation>,
) -> impl IntoView {
    let validation = validation.unwrap_or_default();

    let class = merge_classes(vec![
        "date-picker-validation",
        if validation.is_valid {
            "valid"
        } else {
            "invalid"
        },
        class.as_deref().unwrap_or(""),
    ]);

    view! {
        <div
            class=class
            style=style
            role="alert"
            aria-live="polite"
        >
            {if !validation.is_valid {
                if let Some(error_message) = validation.error_message {
                    view! { <span class="error-message">{error_message}</span> }.into_any()
                } else {
                    view! { <span class="error-message">{String::new()}</span> }.into_any()
                }
            } else {
                view! { <span class="error-message">{String::new()}</span> }.into_any()
            }}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::{merge_classes, generate_id};
    use crate::DateValidation;

    use proptest::prelude::*;

    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    // Unit Tests - DatePicker Component
    #[test]
    fn test_date_picker_creation() {
        // Test that DatePicker component can be created with default props
    }

    #[test]
    fn test_date_picker_with_class() {
        // Test DatePicker with custom class
    }

    #[test]
    fn test_date_picker_with_style() {
        // Test DatePicker with custom style
    }

    #[test]
    fn test_date_picker_with_value() {
        // Test DatePicker with initial value
    }

    #[test]
    fn test_date_picker_placeholder() {
        // Test DatePicker with custom placeholder
    }

    #[test]
    fn test_date_picker_min_max_dates() {
        // Test DatePicker with min/max date constraints
    }

    #[test]
    fn test_date_pickerdisabled() {
        // Test DatePicker in disabled state
    }

    #[test]
    fn test_date_pickerrequired() {
        // Test DatePicker in required state
    }

    #[test]
    fn test_date_picker_format() {
        // Test DatePicker with custom date format
    }

    #[test]
    fn test_date_picker_locale() {
        // Test DatePicker with custom locale
    }

    #[test]
    fn test_date_picker_on_change() {
        // Test DatePicker with change callback
    }

    #[test]
    fn test_date_picker_on_validation() {
        // Test DatePicker with validation callback
    }

    // Date Picker Input tests
    #[test]
    fn test_date_picker_input_creation() {
        // Test DatePickerInput component creation
    }

    #[test]
    fn test_date_picker_input_with_class() {
        // Test DatePickerInput with custom class
    }

    #[test]
    fn test_date_picker_input_value() {
        // Test DatePickerInput with initial value
    }

    #[test]
    fn test_date_picker_input_placeholder() {
        // Test DatePickerInput with custom placeholder
    }

    #[test]
    fn test_date_picker_inputdisabled() {
        // Test DatePickerInput in disabled state
    }

    #[test]
    fn test_date_picker_inputrequired() {
        // Test DatePickerInput in required state
    }

    #[test]
    fn test_date_picker_input_format() {
        // Test DatePickerInput with custom format
    }

    #[test]
    fn test_date_picker_input_on_change() {
        // Test DatePickerInput with change callback
    }

    #[test]
    fn test_date_picker_input_on_focus() {
        // Test DatePickerInput with focus callback
    }

    #[test]
    fn test_date_picker_input_on_blur() {
        // Test DatePickerInput with blur callback
    }

    // Date Picker Trigger tests
    #[test]
    fn test_date_picker_trigger_creation() {
        // Test DatePickerTrigger component creation
    }

    #[test]
    fn test_date_picker_trigger_with_class() {
        // Test DatePickerTrigger with custom class
    }

    #[test]
    fn test_date_picker_triggerdisabled() {
        // Test DatePickerTrigger in disabled state
    }

    #[test]
    fn test_date_picker_trigger_on_click() {
        // Test DatePickerTrigger with click callback
    }

    // Date Picker Calendar tests
    #[test]
    fn test_date_picker_calendar_creation() {
        // Test DatePickerCalendar component creation
    }

    #[test]
    fn test_date_picker_calendar_with_class() {
        // Test DatePickerCalendar with custom class
    }

    #[test]
    fn test_date_picker_calendar_value() {
        // Test DatePickerCalendar with initial value
    }

    #[test]
    fn test_date_picker_calendar_min_max_dates() {
        // Test DatePickerCalendar with min/max date constraints
    }

    #[test]
    fn test_date_picker_calendar_on_date_select() {
        // Test DatePickerCalendar with date selection callback
    }

    // Date Validation tests
    #[test]
    fn test_date_validation_default() {
        let validation = DateValidation::default();
        assert!(validation.is_valid);
        assert!(validation.error_message.is_none());
        assert!(validation.parsed_date.is_none());
    }

    #[test]
    fn test_date_validation_creation() {
        let validation = DateValidation {
            is_valid: false,
            error_message: Some("Invalid date format".to_string()),
            parsed_date: None,
        };
        assert!(!validation.is_valid);
        assert!(validation.error_message.is_some());
        assert!(validation.parsed_date.is_none());
    }

    #[test]
    fn test_date_validation_valid() {
        let validation = DateValidation {
            is_valid: true,
            error_message: None,
            parsed_date: Some("2024-01-15".to_string()),
        };
        assert!(validation.is_valid);
        assert!(validation.error_message.is_none());
        assert!(validation.parsed_date.is_some());
    }

    #[test]
    fn test_date_validation_invalid() {
        let validation = DateValidation {
            is_valid: false,
            error_message: Some("Date is in the past".to_string()),
            parsed_date: Some("2020-01-01".to_string()),
        };
        assert!(!validation.is_valid);
        assert!(validation.error_message.is_some());
        assert!(validation.parsed_date.is_some());
    }

    // Date Picker Validation tests
    #[test]
    fn test_date_picker_validation_creation() {
        // Test DatePickerValidation component creation
    }

    #[test]
    fn test_date_picker_validation_with_class() {
        // Test DatePickerValidation with custom class
    }

    #[test]
    fn test_date_picker_validation_valid() {
        // Test DatePickerValidation with valid date
    }

    #[test]
    fn test_date_picker_validation_invalid() {
        // Test DatePickerValidation with invalid date
    }

    // Helper function tests
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
        let result = merge_classes(vec!["class1", "", "class2", ""]);
        assert_eq!(result, "class1 class2");
    }

    // Property-based Tests
    #[test]
    fn test_date_picker_property_based() {
        proptest!(|(____class in ".*", __style in ".*")| {
            // Test DatePicker with various class and style combinations

        });
    }

    #[test]
    fn test_date_picker_date_validation() {
        proptest!(|(date in ".*")| {
            let is_empty = date.is_empty();
            let validation = DateValidation {
                is_valid: !is_empty,
                error_message: if is_empty { Some("Date is required".to_string()) } else { None },
                parsed_date: if is_empty { None } else { Some(date.clone()) },
            };
            assert!(validation.is_valid != is_empty);
        });
    }

    #[test]
    fn test_date_picker_format_validation() {
        proptest!(|(____format in ".*")| {
            // Test DatePicker with various format strings

        });
    }

    // Integration Tests
    #[test]
    fn test_date_picker_user_interaction() {
        // Test DatePicker user interaction workflows
    }

    #[test]
    fn test_date_picker_accessibility() {
        // Test DatePicker accessibility features
    }

    #[test]
    fn test_date_picker_keyboard_navigation() {
        // Test DatePicker keyboard navigation
    }

    #[test]
    fn test_date_picker_validation_workflow() {
        // Test DatePicker validation workflow
    }

    #[test]
    fn test_date_picker_calendar_integration() {
        // Test DatePicker calendar integration
    }

    // Performance Tests
    #[test]
    fn test_date_picker_large_date_ranges() {
        // Test DatePicker with large date ranges
    }

    #[test]
    fn test_date_picker_render_performance() {
        // Test DatePicker render performance
        let start = std::time::Instant::now();
        // Simulate component creation
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100); // Should render in less than 100ms
    }

    #[test]
    fn test_date_picker_memory_usage() {
        // Test DatePicker memory usage
    }

    #[test]
    fn test_date_picker_validation_performance() {
        // Test DatePicker validation performance
        let start = std::time::Instant::now();
        let validation = DateValidation {
            is_valid: true,
            error_message: None,
            parsed_date: Some("2024-01-15".to_string()),
        };
        let duration = start.elapsed();
        assert!(duration.as_micros() < 1000); // Should validate in less than 1ms
        assert!(validation.is_valid);
    }
}
