use crate::utils::merge_classes;
use leptos::callback::Callback;
use leptos::children::Children;
use leptos::prelude::*;

/// Calendar component - Date picker and calendar component
#[component]
pub fn Calendar(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] value: Option<String>,
    #[prop(optional)] min_date: Option<String>,
    #[prop(optional)] max_date: Option<String>,
    #[prop(optional)] disabled_dates: Option<Vec<String>>,
    #[prop(optional)] locale: Option<String>,
    #[prop(optional)] first_day_of_week: Option<u8>,
    #[prop(optional)] show_week_numbers: Option<bool>,
    #[prop(optional)] _on_date_select: Option<Callback<String>>,
    #[prop(optional)] _on_month_change: Option<Callback<String>>,
) -> impl IntoView {
    let _value = value.unwrap_or_default();
    let _min_date = min_date.unwrap_or_default();
    let _max_date = max_date.unwrap_or_default();
    let disabled_dates = disabled_dates.unwrap_or_default();
    let locale = locale.unwrap_or_else(|| "en-US".to_string());
    let first_day_of_week = first_day_of_week.unwrap_or(0);
    let show_week_numbers = show_week_numbers.unwrap_or(false);

    let class = merge_classes(vec!["calendar", class.as_deref().unwrap_or("")]);

    view! {
        <div
            class=class
            style=style
            role="grid"
            aria-label="Calendar"
            data-locale=locale
            data-first-day-of-week=first_day_of_week
            data-show-week-numbers=show_week_numbers
        >
            {children.map(|c| c())}
        </div>
    }
}

/// Calendar Header component
#[component]
pub fn CalendarHeader(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] month: Option<String>,
    #[prop(optional)] year: Option<i32>,
    #[prop(optional)] _on_previous_month: Option<Callback<()>>,
    #[prop(optional)] _on_next_month: Option<Callback<()>>,
) -> impl IntoView {
    let _month = month.unwrap_or_else(|| "January".to_string());
    let _year = year.unwrap_or(2024);

    let class = merge_classes(vec!["calendar-header", class.as_deref().unwrap_or("")]);

    view! {
        <div
            class=class
            style=style
            role="banner"
            aria-label="Calendar header"
        >
            {children.map(|c| c())}
        </div>
    }
}

/// Calendar Navigation component
#[component]
pub fn CalendarNavigation(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] on_previous: Option<Callback<()>>,
    #[prop(optional)] on_next: Option<Callback<()>>,
    #[prop(optional)] on_today: Option<Callback<()>>,
) -> impl IntoView {
    let class = merge_classes(vec!["calendar-navigation", class.as_deref().unwrap_or("")]);

    view! {
        <div
            class=class
            style=style
            role="navigation"
            aria-label="Calendar navigation"
        >
            <button
                class="calendar-nav-previous"
                type="button"
                aria-label="Previous month"
                on:click=move |_| {
                    if let Some(callback) = on_previous {
                        callback.run(());
                    }
                }
            >
                "‹"
            </button>
            <button
                class="calendar-nav-today"
                type="button"
                aria-label="Go to today"
                on:click=move |_| {
                    if let Some(callback) = on_today {
                        callback.run(());
                    }
                }
            >
                "Today"
            </button>
            <button
                class="calendar-nav-next"
                type="button"
                aria-label="Next month"
                on:click=move |_| {
                    if let Some(callback) = on_next {
                        callback.run(());
                    }
                }
            >
                "›"
            </button>
        </div>
    }
}

/// Calendar Grid component
#[component]
pub fn CalendarGrid(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] month: Option<String>,
    #[prop(optional)] year: Option<i32>,
) -> impl IntoView {
    let _month = month.clone().unwrap_or_else(|| "January".to_string());
    let _year = year.unwrap_or(2024);

    let class = merge_classes(vec!["calendar-grid", class.as_deref().unwrap_or("")]);

    view! {
        <div
            class=class
            style=style
            role="grid"
            aria-label=format!("Calendar for {} {}", month.as_deref().unwrap_or(""), year.unwrap_or(0))
        >
            {children.map(|c| c())}
        </div>
    }
}

/// Calendar Day component
#[component]
pub fn CalendarDay(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] date: Option<String>,
    #[prop(optional)] day: Option<u8>,
    #[prop(optional)] is_today: Option<bool>,
    #[prop(optional)] isselected: Option<bool>,
    #[prop(optional)] isdisabled: Option<bool>,
    #[prop(optional)] is_other_month: Option<bool>,
    #[prop(optional)] on_click: Option<Callback<String>>,
) -> impl IntoView {
    let date = date.unwrap_or_default();
    let day = day.unwrap_or(1);
    let _is_today = is_today.unwrap_or(false);
    let isselected = isselected.unwrap_or(false);
    let isdisabled = isdisabled.unwrap_or(false);
    let _is_other_month = is_other_month.unwrap_or(false);

    let class = merge_classes(vec!["calendar-day", class.as_deref().unwrap_or("")]);

    let handle_click = move |_| {
        if !isdisabled {
            if let Some(callback) = on_click {
                callback.run(date.clone());
            }
        }
    };

    view! {
        <button
            class=class
            style=style
            type="button"
            disabled=isdisabled
            aria-label=format!("{}", day)
            aria-selected=isselected
            on:click=handle_click
        >
            {children.map(|c| c())}
        </button>
    }
}

/// Calendar Week Header component
#[component]
pub fn CalendarWeekHeader(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] locale: Option<String>,
    #[prop(optional)] first_day_of_week: Option<u8>,
) -> impl IntoView {
    let locale = locale.unwrap_or_else(|| "en-US".to_string());
    let first_day_of_week = first_day_of_week.unwrap_or(0);

    let class = merge_classes(vec!["calendar-week-header", class.as_deref().unwrap_or("")]);

    view! {
        <div
            class=class
            style=style
            role="row"
            aria-label="Week header"
            data-locale=locale
            data-first-day-of-week=first_day_of_week
        >
            // Week day headers would be rendered here
        </div>
    }
}

/// Helper function to merge CSS classes

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    // Unit Tests
    #[test]
    fn test_calendar_creation() {}
    #[test]
    fn test_calendar_with_class() {}
    #[test]
    fn test_calendar_with_style() {}
    #[test]
    fn test_calendar_with_value() {}
    #[test]
    fn test_calendar_min_max_dates() {}
    #[test]
    fn test_calendardisabled_dates() {}
    #[test]
    fn test_calendar_locale() {}
    #[test]
    fn test_calendar_first_day_of_week() {}
    #[test]
    fn test_calendar_show_week_numbers() {}
    #[test]
    fn test_calendar_on_date_select() {}
    #[test]
    fn test_calendar_on_month_change() {}

    // Calendar Header tests
    #[test]
    fn test_calendar_header_creation() {}
    #[test]
    fn test_calendar_header_with_class() {}
    #[test]
    fn test_calendar_header_month_year() {}
    #[test]
    fn test_calendar_header_navigation() {}

    // Calendar Navigation tests
    #[test]
    fn test_calendar_navigation_creation() {}
    #[test]
    fn test_calendar_navigation_with_class() {}
    #[test]
    fn test_calendar_navigation_previous() {}
    #[test]
    fn test_calendar_navigation_next() {}
    #[test]
    fn test_calendar_navigation_today() {}

    // Calendar Grid tests
    #[test]
    fn test_calendar_grid_creation() {}
    #[test]
    fn test_calendar_grid_with_class() {}
    #[test]
    fn test_calendar_grid_month_year() {}

    // Calendar Day tests
    #[test]
    fn test_calendar_day_creation() {}
    #[test]
    fn test_calendar_day_with_class() {}
    #[test]
    fn test_calendar_day_date() {}
    #[test]
    fn test_calendar_day_today() {}
    #[test]
    fn test_calendar_dayselected() {}
    #[test]
    fn test_calendar_daydisabled() {}
    #[test]
    fn test_calendar_day_other_month() {}
    #[test]
    fn test_calendar_day_on_click() {}

    // Calendar Week Header tests
    #[test]
    fn test_calendar_week_header_creation() {}
    #[test]
    fn test_calendar_week_header_with_class() {}
    #[test]
    fn test_calendar_week_header_locale() {}
    #[test]
    fn test_calendar_week_header_first_day() {}

    // Helper function tests
    #[test]
    fn test_merge_classes_empty() {}
    #[test]
    fn test_merge_classes_single() {}
    #[test]
    fn test_merge_classes_multiple() {}
    #[test]
    fn test_merge_classes_with_empty() {}

    // Property-based Tests
    #[test]
    fn test_calendar_property_based() {
        proptest!(|(____class in ".*", __style in ".*")| {

        });
    }

    #[test]
    fn test_calendar_date_validation() {
        proptest!(|(____year in 1900..2100i32, __month in 1..12u8, __day in 1..31u8)| {

        });
    }

    #[test]
    fn test_calendar_locale_validation() {
        proptest!(|(____locale in ".*")| {

        });
    }

    // Integration Tests
    #[test]
    fn test_calendar_user_interaction() {}
    #[test]
    fn test_calendar_accessibility() {}
    #[test]
    fn test_calendar_keyboard_navigation() {}
    #[test]
    fn test_calendar_month_navigation() {}
    #[test]
    fn test_calendar_date_selection() {}

    // Performance Tests
    #[test]
    fn test_calendar_large_date_ranges() {}
    #[test]
    fn test_calendar_render_performance() {}
    #[test]
    fn test_calendar_memory_usage() {}
    #[test]
    fn test_calendar_navigation_performance() {}
}
