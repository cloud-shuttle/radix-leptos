use wasm_bindgen::JsCast;
use chrono::{NaiveDate, Datelike, Duration, Utc};

/// DatePicker context for state management
#[derive(Clone)]
pub struct DatePickerContext {
    pub isopen: Signal<bool>,
    pub set_isopen: WriteSignal<bool>,
    pub current_month: Signal<NaiveDate>,
    pub setcurrent_month: WriteSignal<NaiveDate>,
    pub selected_date: Signal<Option<NaiveDate>>,
    pub setselected_date: WriteSignal<Option<NaiveDate>>,
    pub input_value: Signal<String>,
    pub set_input_value: WriteSignal<String>,
    pub on_change: Option<Callback<NaiveDate>>,
    pub on_calendaropen: Option<Callback<()>>,
    pub on_calendar_close: Option<Callback<()>>,
    pub disabled: bool,
    pub date_picker_id: String,
}

/// Generate a simple unique ID for components
fn generate_id(prefix: &str) -> String {
    static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let id = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    format!("{}-{}", prefix, id)
}

/// Merge CSS classes
fn merge_classes(existing: Option<&str>, additional: Option<&str>) -> Option<String> {
    match (existing, additional) {
        (Some(a), Some(b)) => Some(format!("{} {}", a, b)),
        (Some(a), None) => Some(a.to_string()),
        (None, Some(b)) => Some(b.to_string()),
        (None, None) => None,
    }
}

/// Main DatePicker component with calendar functionality
#[component]
pub fn DatePicker(
    /// Currently selected date
    #[prop(optional)]
    selected_date: Option<NaiveDate>,
    /// Minimum selectable date
    #[prop(optional)]
    _min_date: Option<NaiveDate>,
    /// Maximum selectable date
    #[prop(optional)]
    _max_date: Option<NaiveDate>,
    /// Dates that are disabled for selection
    #[prop(optional)]
    disabled_dates: Option<Vec<NaiveDate>>,
    /// Date format string
    #[prop(optional)]
    _format: Option<String>,
    /// Locale for date formatting
    #[prop(optional)]
    _locale: Option<String>,
    /// Change event handler
    #[prop(optional)]
    on_change: Option<Callback<NaiveDate>>,
    /// Calendar open event handler
    #[prop(optional)]
    on_calendaropen: Option<Callback<()>>,
    /// Calendar close event handler
    #[prop(optional)]
    on_calendar_close: Option<Callback<()>>,
    /// Whether the date picker is disabled
    #[prop(optional, default = false)]
    disabled: bool,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// Child content (trigger, calendar, etc.)
    children: Children,
) -> impl IntoView {
    let __date_picker_id = generate_id("date-picker");
    
    // Reactive state
    let (isopen, set_isopen) = signal(false);
    let (current_month, setcurrent_month) = signal(selected_date.unwrap_or_else(|| Utc::now().naive_utc().date()));
    let (selected_date_state, setselected_date) = signal(selected_date);
    let (input_value, set_input_value) = signal(selected_date.map(|d| d.format("%Y-%m-%d").to_string()).unwrap_or_default());
    
    // Create context
    let context = DatePickerContext {
        isopen: isopen.into(),
        set_isopen,
        current_month: current_month.into(),
        setcurrent_month,
        selected_date: selected_date_state.into(),
        setselected_date,
        input_value: input_value.into(),
        set_input_value,
        on_change,
        on_calendaropen,
        on_calendar_close,
        disabled,
        date_picker_id: date_picker_id.clone(),
    };
    
    // Build base classes
    let base_classes = "radix-date-picker";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    // Provide the context
    provide_context(context);
    
    view! {
        <div
            id=date_picker_id
            class=combined_class
            data-selected-date=selected_date_state.get().map(|d| d.format("%Y-%m-%d").to_string()).unwrap_or_default()
            data-open=isopen.get()
            role="application"
            aria-label="Date picker"
        >
            {children()}
        </div>
    }
}

/// DatePickerTrigger component for the input field
#[component]
pub fn DatePickerTrigger(
    /// Whether the trigger is disabled
    #[prop(optional, default = false)]
    disabled: bool,
    /// Placeholder text
    #[prop(optional)]
    _placeholder: Option<String>,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content
    children: Children,
) -> impl IntoView {
    let context = use_context::<DatePickerContext>().expect("DatePickerTrigger must be used within DatePicker");
    let __trigger_id = generate_id("date-picker-trigger");
    
    let handle_click = move |_event: web_sys::MouseEvent| {
        if !context.disabled {
            let newopen_state = !context.isopen.get();
            context.set_isopen.set(newopen_state);
            
            if newopen_state {
                if let Some(callback) = context.on_calendaropen.clone() {
                    callback.run(());
                }
            }
        }
    };
    
    let handle_keydown = move |event: web_sys::KeyboardEvent| {
        if !context.disabled {
            match event.key().as_str() {
                "Enter" | " " => {
                    event.prevent_default();
                    let newopen_state = !context.isopen.get();
                    context.set_isopen.set(newopen_state);
                    
                    if newopen_state {
                        if let Some(callback) = context.on_calendaropen.clone() {
                            callback.run(());
                        }
                    }
                }
                "Escape" => {
                    if context.isopen.get() {
                        context.set_isopen.set(false);
                        if let Some(callback) = context.on_calendar_close.clone() {
                            callback.run(());
                        }
                    }
                }
                _ => {}
            }
        }
    };
    
    // Build base classes
    let base_classes = "radix-date-picker-trigger";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <div
            id=trigger_id
            class=combined_class
            style=style.unwrap_or_default()
            data-disabled=context.disabled
            data-open=context.isopen.get()
            role="button"
            tabindex="0"
            aria-haspopup="dialog"
            aria-expanded=context.isopen.get()
            on:click=handle_click
            on:keydown=handle_keydown
        >
            {children()}
        </div>
    }
}

/// DatePickerCalendar component for the calendar grid
#[component]
pub fn DatePickerCalendar(
    /// Whether the calendar is visible
    #[prop(optional, default = false)]
    _open: bool,
    /// Current month to display
    #[prop(optional)]
    current_month: Option<NaiveDate>,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content (calendar grid)
    children: Children,
) -> impl IntoView {
    let context = use_context::<DatePickerContext>().expect("DatePickerCalendar must be used within DatePicker");
    let __calendar_id = generate_id("date-picker-calendar");
    
    // Build base classes
    let base_classes = "radix-date-picker-calendar";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    let display_style = if context.isopen.get() {
        style.unwrap_or_default()
    
    view! {
        <div
            id=calendar_id
            class=combined_class
            style=display_style
            role="dialog"
            aria-label="Calendar"
            aria-modal="true"
        >
            {children()}
        </div>
    }
}

/// DatePickerGrid component for the calendar days
#[component]
pub fn DatePickerGrid(
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content (calendar days)
    children: Children,
) -> impl IntoView {
    let context = use_context::<DatePickerContext>().expect("DatePickerGrid must be used within DatePicker");
    let __grid_id = generate_id("date-picker-grid");
    
    // Build base classes
    let base_classes = "radix-date-picker-grid";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <div
            id=grid_id
            class=combined_class
            style=style.unwrap_or_default()
            role="grid"
            aria-label="Calendar days"
        >
            // Header with month/year and navigation
            <div class="radix-date-picker-header">
                <div class="radix-date-picker-month-year">
                    {move || format!("{} {}", 
                        context.current_month.get().format("%B").to_string(),
                        context.current_month.get().year()
                    )}
                </div>
                <div class="radix-date-picker-nav">
                    <button
                        class="radix-date-picker-nav-prev"
                        on:click=move |_| {
                            let current = context.current_month.get();
                            let prev_month = if current.month() == 1 {
                                NaiveDate::from_ymd_opt(current.year() - 1, 12, 1).unwrap()
                            context.setcurrent_month.set(prev_month);
                        }
                    >
                        "‹"
                    </button>
                    <button
                        class="radix-date-picker-nav-next"
                        on:click=move |_| {
                            let current = context.current_month.get();
                            let next_month = if current.month() == 12 {
                                NaiveDate::from_ymd_opt(current.year() + 1, 1, 1).unwrap()
                            context.setcurrent_month.set(next_month);
                        }
                    >
                        "›"
                    </button>
                </div>
            </div>
            
            // Weekday headers
            <div class="radix-date-picker-weekdays">
                <div class="radix-date-picker-weekday">"Mon"</div>
                <div class="radix-date-picker-weekday">"Tue"</div>
                <div class="radix-date-picker-weekday">"Wed"</div>
                <div class="radix-date-picker-weekday">"Thu"</div>
                <div class="radix-date-picker-weekday">"Fri"</div>
                <div class="radix-date-picker-weekday">"Sat"</div>
                <div class="radix-date-picker-weekday">"Sun"</div>
            </div>
            
            // Calendar days placeholder
            <div class="radix-date-picker-days">
                <div class="radix-date-picker-days-placeholder">
                    "Calendar grid implementation coming soon..."
                </div>
            </div>
        </div>
    }
}

/// DatePickerDay component for individual calendar days
#[component]
pub fn DatePickerDay(
    /// The date this day represents
    #[prop(optional)]
    _date: Option<NaiveDate>,
    /// Whether the day is selected
    #[prop(optional, default = false)]
    selected: bool,
    /// Whether the day is disabled
    #[prop(optional, default = false)]
    disabled: bool,
    /// Whether the day is today
    #[prop(optional, default = false)]
    ___today: bool,
    /// Whether the day is outside the current month
    #[prop(optional, default = false)]
    ___outside_month: bool,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content
    children: Children,
) -> impl IntoView {
    let __day_id = generate_id("date-picker-day");
    
    // Build base classes
    let base_classes = "radix-date-picker-day";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <div
            id=day_id
            class=combined_class
            style=style.unwrap_or_default()
            role="gridcell"
            tabindex="-1"
        >
            {children()}
        </div>
    }
}

/// DatePickerInput component for the date input field
#[component]
pub fn DatePickerInput(
    /// Input value
    #[prop(optional)]
    _value: Option<String>,
    /// Placeholder text
    #[prop(optional)]
    placeholder: Option<String>,
    /// Whether the input is disabled
    #[prop(optional, default = false)]
    disabled: bool,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
) -> impl IntoView {
    let context = use_context::<DatePickerContext>().expect("DatePickerInput must be used within DatePicker");
    let __input_id = generate_id("date-picker-input");
    
    let handle_input = move |event: web_sys::Event| {
        if let Some(target) = event.target() {
            if let Ok(input) = target.dyn_into::<web_sys::HtmlInputElement>() {
                let value = input.value();
                context.set_input_value.set(value.clone());
                
                // Try to parse the date
                if let Ok(date) = NaiveDate::parse_from_str(&value, "%Y-%m-%d") {
                    context.setselected_date.set(Some(date));
                    if let Some(callback) = context.on_change.clone() {
                        callback.run(date);
                    }
                }
            }
        }
    };
    
    let handle_keydown = move |event: web_sys::KeyboardEvent| {
        match event.key().as_str() {
            "Enter" => {
                event.prevent_default();
                let value = context.input_value.get();
                if let Ok(date) = NaiveDate::parse_from_str(&value, "%Y-%m-%d") {
                    context.setselected_date.set(Some(date));
                    context.set_isopen.set(false);
                    
                    if let Some(callback) = context.on_change.clone() {
                        callback.run(date);
                    }
                    if let Some(callback) = context.on_calendar_close.clone() {
                        callback.run(());
                    }
                }
            }
            "Escape" => {
                if context.isopen.get() {
                    context.set_isopen.set(false);
                    if let Some(callback) = context.on_calendar_close.clone() {
                        callback.run(());
                    }
                }
            }
            _ => {}
        }
    };
    
    // Build base classes
    let base_classes = "radix-date-picker-input";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <input
            id=input_id
            type="text"
            class=combined_class
            style=style.unwrap_or_default()
            value=context.input_value.get()
            placeholder=placeholder.unwrap_or_default()
            disabled=context.disabled
            role="textbox"
            aria-autocomplete="none"
            aria-expanded=context.isopen.get()
            aria-controls=context.date_picker_id.clone()
            on:input=handle_input
            on:keydown=handle_keydown
        />
    }
}
