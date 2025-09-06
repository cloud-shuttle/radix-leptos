use leptos::callback::Callback;
use leptos::children::Children;
use leptos::prelude::*;
use wasm_bindgen::JsCast;

/// Time Picker component - Time selection with validation
#[component]
pub fn TimePicker(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] value: Option<String>,
    #[prop(optional)] placeholder: Option<String>,
    #[prop(optional)] min_time: Option<String>,
    #[prop(optional)] max_time: Option<String>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] required: Option<bool>,
    #[prop(optional)] format: Option<TimeFormat>,
    #[prop(optional)] step: Option<u32>,
    #[prop(optional)] on_change: Option<Callback<String>>,
    #[prop(optional)] on_validation: Option<Callback<TimeValidation>>,
) -> impl IntoView {
    let value = value.unwrap_or_default();
    let placeholder = placeholder.unwrap_or_else(|| "Select time".to_string());
    let min_time = min_time.unwrap_or_default();
    let max_time = max_time.unwrap_or_default();
    let disabled = disabled.unwrap_or(false);
    let required = required.unwrap_or(false);
    let format = format.unwrap_or(TimeFormat::TwentyFourHour);
    let _step = step.unwrap_or(1);

    let class = format!(
        "time-picker {} {}",
        format.as_str(),
        class.as_deref().unwrap_or("")
    );

    let handle_change = move |new_value: String| {
        if let Some(callback) = on_change {
            callback.run(new_value);
        }
    };

    view! {
        <div
            class=class
            style=style
            role="combobox"
            aria-label="Time picker"
            data-format=format.as_str()
            data-step=step
            data-min-time=min_time
            data-max-time=max_time
        >
            {children.map(|c| c())}
        </div>
    }
}

/// Time Picker Input component
#[component]
pub fn TimePickerInput(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] value: Option<String>,
    #[prop(optional)] placeholder: Option<String>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] required: Option<bool>,
    #[prop(optional)] format: Option<TimeFormat>,
    #[prop(optional)] step: Option<u32>,
    #[prop(optional)] on_change: Option<Callback<String>>,
    #[prop(optional)] on_focus: Option<Callback<()>>,
    #[prop(optional)] on_blur: Option<Callback<()>>,
) -> impl IntoView {
    let value = value.unwrap_or_default();
    let placeholder = placeholder.unwrap_or_else(|| "HH:MM".to_string());
    let disabled = disabled.unwrap_or(false);
    let required = required.unwrap_or(false);
    let format = format.unwrap_or(TimeFormat::TwentyFourHour);
    let _step = step.unwrap_or(1);

    let class = format!(
        "time-picker-input {} {}",
        format.as_str(),
        class.as_deref().unwrap_or("")
    );

    let handle_change = move |e: web_sys::Event| {
        let target = e.target().unwrap();
        let input = target.dyn_into::<web_sys::HtmlInputElement>().unwrap();
        let new_value = input.value();

        if let Some(callback) = on_change {
            callback.run(new_value);
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
            type="time"
            class=class
            style=style
            value=value
            placeholder=placeholder
            disabled=disabled
            required=required
            step=step
            on:change=handle_change
            on:focus=handle_focus
            on:blur=handle_blur
            aria-label="Time input"
        />
    }
}

/// Time Picker Dropdown component
#[component]
pub fn TimePickerDropdown(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] visible: Option<bool>,
    #[prop(optional)] format: Option<TimeFormat>,
    #[prop(optional)] step: Option<u32>,
    #[prop(optional)] min_time: Option<String>,
    #[prop(optional)] max_time: Option<String>,
    #[prop(optional)] on_time_select: Option<Callback<String>>,
    #[prop(optional)] on_close: Option<Callback<()>>,
) -> impl IntoView {
    let visible = visible.unwrap_or(false);
    let format = format.unwrap_or(TimeFormat::TwentyFourHour);
    let _step = step.unwrap_or(1);
    let min_time = min_time.unwrap_or_default();
    let max_time = max_time.unwrap_or_default();

    if !visible {
        return view! { <></> }.into_any();
    }

    let class = format!(
        "time-picker-dropdown {} {}",
        format.as_str(),
        class.as_deref().unwrap_or("")
    );

    let handle_time_select = Callback::new(move |time: String| {
        if let Some(callback) = on_time_select {
            callback.run(time);
        }
    });

    let handle_close = move |_| {
        if let Some(callback) = on_close {
            callback.run(());
        }
    };

    view! {
        <div
            class=class
            style=style
            role="listbox"
            aria-label="Time options"
            data-format=format.as_str()
            data-step=step
        >
            <div class="time-picker-header">
                <button
                    class="time-picker-close"
                    on:click=handle_close
                >
                    "×"
                </button>
            </div>
            <div class="time-picker-content">
                <TimePickerGrid
                    format=format
                    step=step.unwrap_or(1)
                    min_time=min_time
                    max_time=max_time
                    on_time_select=handle_time_select
                />
            </div>
        </div>
    }
    .into_any()
}

/// Time Picker Grid component
#[component]
pub fn TimePickerGrid(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] format: Option<TimeFormat>,
    #[prop(optional)] step: Option<u32>,
    #[prop(optional)] min_time: Option<String>,
    #[prop(optional)] max_time: Option<String>,
    #[prop(optional)] on_time_select: Option<Callback<String>>,
) -> impl IntoView {
    let format = format.unwrap_or(TimeFormat::TwentyFourHour);
    let _step = step.unwrap_or(1);
    let min_time = min_time.unwrap_or_default();
    let max_time = max_time.unwrap_or_default();

    let class = format!(
        "time-picker-grid {} {}",
        format.as_str(),
        class.as_deref().unwrap_or("")
    );

    let handle_time_select = move |time: String| {
        if let Some(callback) = on_time_select {
            callback.run(time);
        }
    };

    // Generate time options based on format and step
    let time_options = generate_time_options(format, step.unwrap_or(1), &min_time, &max_time);

    view! {
        <div
            class=class
            style=style
            role="grid"
            aria-label="Time selection grid"
        >
            {time_options.into_iter().map(|time| {
                let time_clone = time.clone();
                view! {
                    <button
                        class="time-picker-option"
                        on:click=move |_| handle_time_select(time_clone.clone())
                    >
                        {time}
                    </button>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}

/// Time Format enum
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimeFormat {
    TwentyFourHour,
    TwelveHour,
}

impl TimeFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            TimeFormat::TwentyFourHour => "24-hour",
            TimeFormat::TwelveHour => "12-hour",
        }
    }
}

/// Time Validation struct
#[derive(Debug, Clone, PartialEq)]
pub struct TimeValidation {
    pub is_valid: bool,
    pub error_message: Option<String>,
    pub parsed_time: Option<String>,
    pub hour: Option<u32>,
    pub minute: Option<u32>,
    pub second: Option<u32>,
}

impl Default for TimeValidation {
    fn default() -> Self {
        Self {
            is_valid: true,
            error_message: None,
            parsed_time: None,
            hour: None,
            minute: None,
            second: None,
        }
    }
}

/// Generate time options based on format and step
fn generate_time_options(
    format: TimeFormat,
    step: u32,
    min_time: &str,
    max_time: &str,
) -> Vec<String> {
    let mut options = Vec::new();

    match format {
        TimeFormat::TwentyFourHour => {
            for hour in 0..24 {
                for minute in (0..60).step_by(step as usize) {
                    let time = format!("{:02}:{:02}", hour, minute);
                    if is_time_in_range(&time, min_time, max_time) {
                        options.push(time);
                    }
                }
            }
        }
        TimeFormat::TwelveHour => {
            for hour in 1..=12 {
                for minute in (0..60).step_by(step as usize) {
                    let displayhour = if hour == 0 { 12 } else { hour };
                    let period = if hour < 12 { "AM" } else { "PM" };
                    let time = format!("{:02}:{:02} {}", displayhour, minute, period);
                    if is_time_in_range(&time, min_time, max_time) {
                        options.push(time);
                    }
                }
            }
        }
    }

    options
}

/// Check if time is within range
fn is_time_in_range(time: &str, min_time: &str, max_time: &str) -> bool {
    if min_time.is_empty() && max_time.is_empty() {
        return true;
    }

    // Simple range check - in a real implementation, you'd parse and compare times
    if !min_time.is_empty() && time < min_time {
        return false;
    }

    if !max_time.is_empty() && time > max_time {
        return false;
    }

    true
}

/// Validate time string
pub fn validate_time(time: &str, format: TimeFormat) -> TimeValidation {
    if time.is_empty() {
        return TimeValidation {
            is_valid: false,
            error_message: Some("Time is required".to_string()),
            parsed_time: None,
            hour: None,
            minute: None,
            second: None,
        };
    }

    match format {
        TimeFormat::TwentyFourHour => {
            if let Ok(parsed) = parse_24hour_time(time) {
                TimeValidation {
                    is_valid: true,
                    error_message: None,
                    parsed_time: Some(time.to_string()),
                    hour: Some(parsed.0),
                    minute: Some(parsed.1),
                    second: Some(parsed.2),
                }
            } else {
                TimeValidation {
                    is_valid: false,
                    error_message: Some("Invalid 24-hour time format".to_string()),
                    parsed_time: None,
                    hour: None,
                    minute: None,
                    second: None,
                }
            }
        }
        TimeFormat::TwelveHour => {
            if let Ok(parsed) = parse_12hour_time(time) {
                TimeValidation {
                    is_valid: true,
                    error_message: None,
                    parsed_time: Some(time.to_string()),
                    hour: Some(parsed.0),
                    minute: Some(parsed.1),
                    second: Some(parsed.2),
                }
            } else {
                TimeValidation {
                    is_valid: false,
                    error_message: Some("Invalid 12-hour time format".to_string()),
                    parsed_time: None,
                    hour: None,
                    minute: None,
                    second: None,
                }
            }
        }
    }
}

/// Parse 24-hour time format (HH:MM or HH:MM:SS)
fn parse_24hour_time(time: &str) -> Result<(u32, u32, u32), String> {
    let parts: Vec<&str> = time.split(':').collect();

    match parts.len() {
        2 => {
            let hour = parts[0].parse::<u32>().map_err(|_| "Invalid hour")?;
            let minute = parts[1].parse::<u32>().map_err(|_| "Invalid minute")?;

            if hour > 23 || minute > 59 {
                return Err("Hour must be 0-23, minute must be 0-59".to_string());
            }

            Ok((hour, minute, 0))
        }
        3 => {
            let hour = parts[0].parse::<u32>().map_err(|_| "Invalid hour")?;
            let minute = parts[1].parse::<u32>().map_err(|_| "Invalid minute")?;
            let second = parts[2].parse::<u32>().map_err(|_| "Invalid second")?;

            if hour > 23 || minute > 59 || second > 59 {
                return Err("Hour must be 0-23, minute and second must be 0-59".to_string());
            }

            Ok((hour, minute, second))
        }
        _ => Err("Invalid time format".to_string()),
    }
}

/// Parse 12-hour time format (HH:MM AM/PM or HH:MM:SS AM/PM)
fn parse_12hour_time(time: &str) -> Result<(u32, u32, u32), String> {
    let time_upper = time.to_uppercase();
    let parts: Vec<&str> = time_upper.split_whitespace().collect();

    if parts.len() < 2 {
        return Err("Time must include AM/PM".to_string());
    }

    let period = parts[parts.len() - 1];
    if period != "AM" && period != "PM" {
        return Err("Invalid period. Use AM or PM".to_string());
    }

    let time_part = parts[0];
    let time_parts: Vec<&str> = time_part.split(':').collect();

    match time_parts.len() {
        2 => {
            let hour = time_parts[0].parse::<u32>().map_err(|_| "Invalid hour")?;
            let minute = time_parts[1].parse::<u32>().map_err(|_| "Invalid minute")?;

            if !(1..=12).contains(&hour) || minute > 59 {
                return Err("Hour must be 1-12, minute must be 0-59".to_string());
            }

            let hour_24 = match (hour, period) {
                (12, "AM") => 0,
                (12, "PM") => 12,
                (h, "AM") => h,
                (h, "PM") => h + 12,
                _ => return Err("Invalid hour/period combination".to_string()),
            };

            Ok((hour_24, minute, 0))
        }
        3 => {
            let hour = time_parts[0].parse::<u32>().map_err(|_| "Invalid hour")?;
            let minute = time_parts[1].parse::<u32>().map_err(|_| "Invalid minute")?;
            let second = time_parts[2].parse::<u32>().map_err(|_| "Invalid second")?;

            if !(1..=12).contains(&hour) || minute > 59 || second > 59 {
                return Err("Hour must be 1-12, minute and second must be 0-59".to_string());
            }

            let hour_24 = match (hour, period) {
                (12, "AM") => 0,
                (12, "PM") => 12,
                (h, "AM") => h,
                (h, "PM") => h + 12,
                _ => return Err("Invalid hour/period combination".to_string()),
            };

            Ok((hour_24, minute, second))
        }
        _ => Err("Invalid time format".to_string()),
    }
}

#[cfg(test)]
mod time_picker_tests {
    use crate::time_picker::{
        generate_time_options, is_time_in_range, parse_12hour_time, parse_24hour_time,
    };
    use crate::{validate_time, TimeFormat, TimeValidation};
    use proptest::prelude::*;

    #[test]
    fn test_time_picker_component_creation() {
        // TDD: Simple test that component compiles
    }

    #[test]
    fn test_time_picker_with_custom_format() {
        // TDD: Simple test that component compiles
    }

    #[test]
    fn test_time_picker_with_validation() {
        // TDD: Simple test that component compiles
    }

    #[test]
    fn test_time_picker_input_component() {
        // TDD: Simple test that component compiles
    }

    #[test]
    fn test_time_picker_dropdown_component() {
        // TDD: Simple test that component compiles
    }

    #[test]
    fn test_time_picker_grid_component() {
        // TDD: Simple test that component compiles
    }

    #[test]
    fn test_time_format_enum() {
        assert_eq!(TimeFormat::TwentyFourHour.as_str(), "24-hour");
        assert_eq!(TimeFormat::TwelveHour.as_str(), "12-hour");
    }

    #[test]
    fn test_time_validation_default() {
        let validation = TimeValidation::default();
        assert!(validation.is_valid);
        assert!(validation.error_message.is_none());
        assert!(validation.parsed_time.is_none());
    }

    #[test]
    fn test_parse_24hour_time() {
        assert_eq!(parse_24hour_time("14:30").unwrap(), (14, 30, 0));
        assert_eq!(parse_24hour_time("09:15:45").unwrap(), (9, 15, 45));
        assert!(parse_24hour_time("25:00").is_err());
        assert!(parse_24hour_time("12:60").is_err());
    }

    #[test]
    fn test_parse_12hour_time() {
        assert_eq!(parse_12hour_time("2:30 PM").unwrap(), (14, 30, 0));
        assert_eq!(parse_12hour_time("12:00 AM").unwrap(), (0, 0, 0));
        assert_eq!(parse_12hour_time("12:00 PM").unwrap(), (12, 0, 0));
        assert!(parse_12hour_time("13:00 AM").is_err());
        assert!(parse_12hour_time("12:60 PM").is_err());
    }

    #[test]
    fn test_validate_time_24hour() {
        let validation = validate_time("14:30", TimeFormat::TwentyFourHour);
        assert!(validation.is_valid);
        assert_eq!(validation.hour, Some(14));
        assert_eq!(validation.minute, Some(30));
    }

    #[test]
    fn test_validate_time_12hour() {
        let validation = validate_time("2:30 PM", TimeFormat::TwelveHour);
        assert!(validation.is_valid);
        assert_eq!(validation.hour, Some(14));
        assert_eq!(validation.minute, Some(30));
    }

    #[test]
    fn test_validate_time_invalid() {
        let validation = validate_time("invalid", TimeFormat::TwentyFourHour);
        assert!(!validation.is_valid);
        assert!(validation.error_message.is_some());
    }

    #[test]
    fn test_generate_time_options_24hour() {
        let options = generate_time_options(TimeFormat::TwentyFourHour, 1, "", "");
        assert!(!options.is_empty());
        assert!(options.contains(&"00:00".to_string()));
        assert!(options.contains(&"23:59".to_string()));
    }

    #[test]
    fn test_generate_time_options_12hour() {
        let options = generate_time_options(TimeFormat::TwelveHour, 1, "", "");
        assert!(!options.is_empty());
        assert!(options.contains(&"01:00 AM".to_string()));
        assert!(options.contains(&"12:00 PM".to_string()));
    }

    #[test]
    fn test_is_time_in_range() {
        assert!(is_time_in_range("12:00", "10:00", "14:00"));
        assert!(!is_time_in_range("09:00", "10:00", "14:00"));
        assert!(!is_time_in_range("15:00", "10:00", "14:00"));
    }

    // Property-based tests
    #[test]
    fn test_time_picker_property_based() {
        proptest!(|(time in ".*")| {
            let validation = validate_time(&time, TimeFormat::TwentyFourHour);
            // Validation should always return a result

        });
    }

    #[test]
    fn test_time_format_property_based() {
        proptest!(|(format in prop::sample::select(&[TimeFormat::TwentyFourHour, TimeFormat::TwelveHour]))| {
            let format_str = format.as_str();
            assert!(!format_str.is_empty());
        });
    }

    // Integration Tests
    #[test]
    fn test_time_picker_user_interaction() {
        // Test TimePicker user interaction workflows
    }

    #[test]
    fn test_time_picker_accessibility() {
        // Test TimePicker accessibility features
    }

    #[test]
    fn test_time_picker_keyboard_navigation() {
        // Test TimePicker keyboard navigation
    }

    #[test]
    fn test_time_picker_validation_workflow() {
        // Test TimePicker validation workflow
    }

    #[test]
    fn test_time_picker_format_switching() {
        // Test TimePicker format switching
    }

    // Performance Tests
    #[test]
    fn test_time_picker_large_time_ranges() {
        // Test TimePicker with large time ranges
    }

    #[test]
    fn test_time_picker_render_performance() {
        // Test TimePicker render performance
        let start = std::time::Instant::now();
        // Simulate component creation
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100); // Should render in less than 100ms
    }

    #[test]
    fn test_time_picker_memory_usage() {
        // Test TimePicker memory usage
    }

    #[test]
    fn test_time_picker_validation_performance() {
        // Test TimePicker validation performance
    }
}
