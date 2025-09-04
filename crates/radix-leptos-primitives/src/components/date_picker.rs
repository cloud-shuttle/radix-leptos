use leptos::*;
use leptos::prelude::*;
use chrono::Datelike;
use wasm_bindgen::JsCast;

/// Date picker component with calendar interface
#[component]
pub fn DatePicker(
    /// The currently selected date
    #[prop(optional)]
    value: Option<chrono::NaiveDate>,
    /// Callback when date changes
    #[prop(optional)]
    on_change: Option<Callback<chrono::NaiveDate>>,
    /// Whether the date picker is disabled
    #[prop(optional, default = false)]
    disabled: bool,
    /// Placeholder text
    #[prop(optional)]
    placeholder: Option<String>,
    /// CSS classes to apply
    #[prop(optional)]
    class: Option<String>,
    /// Whether to show the calendar popup
    #[prop(optional, default = false)]
    open: bool,
    /// Callback when open state changes
    #[prop(optional)]
    on_open_change: Option<Callback<bool>>,
) -> impl IntoView {
    let (is_open, set_is_open) = signal(open);
    let (selected_date, set_selected_date) = signal(value);
    let (current_month, set_current_month) = signal(chrono::Utc::now().naive_utc().date());

    // Update open state when prop changes
    Effect::new(move |_| {
        set_is_open.set(open);
    });

    // Handle date selection
    let handle_date_select = move |date: chrono::NaiveDate| {
        set_selected_date.set(Some(date));
        set_is_open.set(false);
        
        if let Some(callback) = on_change {
            callback.run(date);
        }
        
        if let Some(callback) = on_open_change {
            callback.run(false);
        }
    };

    // Handle month navigation
    let go_to_previous_month = move |_| {
        let current = current_month.get();
        if let Some(prev_month) = current.pred_opt() {
            set_current_month.set(prev_month);
        }
    };

    let go_to_next_month = move |_| {
        let current = current_month.get();
        if let Some(next_month) = current.succ_opt() {
            set_current_month.set(next_month);
        }
    };

    // Generate calendar days for current month
    let calendar_days = move || {
        let month = current_month.get();
        let year = month.year();
        let month_num = month.month();
        
        let first_day = chrono::NaiveDate::from_ymd_opt(year, month_num, 1).unwrap();
        let last_day = chrono::NaiveDate::from_ymd_opt(year, month_num + 1, 1)
            .unwrap_or_else(|| chrono::NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap())
            .pred_opt()
            .unwrap();
        
        let start_weekday = first_day.weekday().num_days_from_monday() as u32;
        let total_days = last_day.day();
        
        let mut days = Vec::new();
        
        // Add empty cells for days before month starts
        for _ in 0..start_weekday {
            days.push(None);
        }
        
        // Add days of the month
        for day in 1..=total_days {
            let date = chrono::NaiveDate::from_ymd_opt(year, month_num, day).unwrap();
            days.push(Some(date));
        }
        
        days
    };

    // Format date for display
    let format_date = move |date: chrono::NaiveDate| {
        date.format("%B %d, %Y").to_string()
    };

    // Check if date is today
    let is_today = move |date: chrono::NaiveDate| {
        date == chrono::Utc::now().naive_utc().date()
    };

    // Check if date is selected
    let is_selected = move |date: chrono::NaiveDate| {
        selected_date.get().map_or(false, |selected| selected == date)
    };

    view! {
        <div class={format!("date-picker {}", class.unwrap_or_default())}>
            // Trigger button
            <button
                type="button"
                class="date-picker-trigger"
                disabled=disabled
                on:click=move |_| {
                    let new_state = !is_open.get();
                    set_is_open.set(new_state);
                    if let Some(callback) = on_open_change {
                        callback.run(new_state);
                    }
                }
            >
                <span class="date-picker-value">
                    {move || selected_date.get().map_or(
                        placeholder.clone().unwrap_or_else(|| "Select date".to_string()),
                        format_date
                    )}
                </span>
                <span class="date-picker-icon">"📅"</span>
            </button>

            // Calendar popup
            <Show when=move || is_open.get()>
                <div class="date-picker-calendar">
                    // Calendar header
                    <div class="calendar-header">
                        <button
                            type="button"
                            class="calendar-nav-button"
                            on:click=go_to_previous_month
                        >
                            "‹"
                        </button>
                        <h3 class="calendar-month">
                            {move || current_month.get().format("%B %Y").to_string()}
                        </h3>
                        <button
                            type="button"
                            class="calendar-nav-button"
                            on:click=go_to_next_month
                        >
                            "›"
                        </button>
                    </div>

                    // Calendar grid
                    <div class="calendar-grid">
                        // Weekday headers
                        <div class="calendar-weekdays">
                            <div class="calendar-weekday">"Mon"</div>
                            <div class="calendar-weekday">"Tue"</div>
                            <div class="calendar-weekday">"Wed"</div>
                            <div class="calendar-weekday">"Thu"</div>
                            <div class="calendar-weekday">"Fri"</div>
                            <div class="calendar-weekday">"Sat"</div>
                            <div class="calendar-weekday">"Sun"</div>
                        </div>

                        // Calendar days
                        <div class="calendar-days">
                            {move || calendar_days().into_iter().map(|maybe_date| {
                                if let Some(date) = maybe_date {
                                    let date_clone = date;
                                    let is_today_date = is_today(date);
                                    let is_selected_date = is_selected(date);
                                    
                                    view! {
                                        <button
                                            type="button"
                                            class={format!(
                                                "calendar-day {} {}",
                                                if is_today_date { "today" } else { "" },
                                                if is_selected_date { "selected" } else { "" }
                                            )}
                                            on:click=move |_| handle_date_select(date_clone)
                                        >
                                            {date.day().to_string()}
                                        </button>
                                    }
                                } else {
                                    view! {
                                        <button
                                            type="button"
                                            class="calendar-day empty"
                                            disabled=true
                                        >
                                            {String::new()}
                                        </button>
                                    }
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    </div>

                    // Today button
                    <div class="calendar-footer">
                        <button
                            type="button"
                            class="calendar-today-button"
                            on:click=move |_| {
                                let today = chrono::Utc::now().naive_utc().date();
                                handle_date_select(today);
                            }
                        >
                            "Today"
                        </button>
                    </div>
                </div>
            </Show>
        </div>
    }
}

/// Date picker input component (simplified version)
#[component]
pub fn DatePickerInput(
    /// The currently selected date
    #[prop(optional)]
    value: Option<chrono::NaiveDate>,
    /// Callback when date changes
    #[prop(optional)]
    on_change: Option<Callback<chrono::NaiveDate>>,
    /// Whether the input is disabled
    #[prop(optional, default = false)]
    disabled: bool,
    /// Placeholder text
    #[prop(optional)]
    placeholder: Option<String>,
    /// CSS classes to apply
    #[prop(optional)]
    class: Option<String>,
) -> impl IntoView {
    let (is_open, set_is_open) = signal(false);
    let (input_value, set_input_value) = signal(String::new());
    let (selected_date, set_selected_date) = signal(value);

    // Update input value when date changes
    Effect::new(move |_| {
        if let Some(date) = selected_date.get() {
            set_input_value.set(date.format("%Y-%m-%d").to_string());
        }
    });

    let handle_input_change = move |event: web_sys::Event| {
        let target = event.target().unwrap();
        let input_element = target.unchecked_ref::<web_sys::HtmlInputElement>();
        set_input_value.set(input_element.value());
    };

    let handle_date_select = move |date: chrono::NaiveDate| {
        set_selected_date.set(Some(date));
        set_is_open.set(false);
        
        if let Some(callback) = on_change {
            callback.run(date);
        }
    };

    view! {
        <div class={format!("date-picker-input {}", class.unwrap_or_default())}>
            <input
                type="text"
                class="date-picker-input-field"
                placeholder=placeholder.unwrap_or_else(|| "YYYY-MM-DD".to_string())
                value=move || input_value.get()
                disabled=disabled
                on:input=handle_input_change
                on:focus=move |_| set_is_open.set(true)
            />
            
            <Show when=move || is_open.get()>
                <DatePicker
                    value=selected_date.get().expect("Date should be available when calendar is open")
                    on_change=Callback::new(handle_date_select)
                    open=true
                    on_open_change=Callback::new(move |open| set_is_open.set(open))
                />
            </Show>
        </div>
    }
}
