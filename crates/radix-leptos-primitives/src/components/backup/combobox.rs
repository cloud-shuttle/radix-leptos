use wasm_bindgen::JsCast;

/// Combobox context for sharing state between components
#[derive(Clone)]
pub struct ComboboxContext {
    pub isopen: Signal<bool>,
    pub set_isopen: WriteSignal<bool>,
    pub search_query: Signal<String>,
    pub set_search_query: WriteSignal<String>,
    pub selected_value: Signal<Option<String>>,
    pub setselected_value: WriteSignal<Option<String>>,
    pub focused_index: Signal<usize>,
    pub setfocused_index: WriteSignal<usize>,
    pub options: Signal<Vec<ComboboxOption>>,
    pub filtered_options: Memo<Vec<ComboboxOption>>,
    pub on_change: Option<Callback<String>>,
    pub on_search: Option<Callback<String>>,
    pub disabled: bool,
    pub combobox_id: String,
}

/// Combobox option structure
#[derive(Clone, Debug, PartialEq)]
pub struct ComboboxOption {
    pub value: String,
    pub label: String,
    pub disabled: Option<bool>,
    pub group: Option<String>,
}

impl ComboboxOption {
    pub fn new(value: String, label: String) -> Self {
        Self {
            value,
            label,
            disabled: None,
            group: None,
        }
    }

    pub fn withdisabled(mut self, disabled: bool) -> Self {
        self.disabled = Some(disabled);
        self
    }

    pub fn with_group(mut self, group: String) -> Self {
        self.group = Some(group);
        self
    }
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

/// Main Combobox component with searchable dropdown functionality
#[component]
pub fn Combobox(
    /// Available options for selection
    #[prop(optional)]
    options: Vec<ComboboxOption>,
    /// Currently selected value
    #[prop(optional)]
    value: Option<String>,
    /// Placeholder text for the input
    #[prop(optional)]
    placeholder: Option<String>,
    /// Whether the combobox is disabled
    #[prop(optional, default = false)]
    disabled: bool,
    /// Whether the combobox supports search/filtering
    #[prop(optional, default = true)]
    __searchable: bool,
    /// Whether the combobox supports multiple selection
    #[prop(optional, default = false)]
    _multi_select: bool,
    /// Change event handler
    #[prop(optional)]
    on_change: Option<Callback<String>>,
    /// Search event handler
    #[prop(optional)]
    on_search: Option<Callback<String>>,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// Child content (trigger, content, and items)
    children: Children,
) -> impl IntoView {
    let __combobox_id = generate_id("combobox");
    
    // Reactive state
    let (isopen, set_isopen) = signal(false);
    let (search_query, set_search_query) = signal(String::new());
    let (selected_value, setselected_value) = signal(value.clone());
    let (focused_index, setfocused_index) = signal(0);
    let (options_signal, _) = signal(options.clone());
    
    // Filter options based on search query
    let filtered_options = Memo::new(move |_| {
        let current_options = options_signal.get();
        if search_query.get().is_empty() {
            current_options
                .cloned()
                .collect::<Vec<_>>()
        }
    });
    
    // Create context
    let context = ComboboxContext {
        isopen: isopen.into(),
        set_isopen,
        search_query: search_query.into(),
        set_search_query,
        selected_value: selected_value.into(),
        setselected_value,
        focused_index: focused_index.into(),
        setfocused_index,
        options: options_signal.into(),
        filtered_options,
        on_change,
        on_search,
        disabled,
        combobox_id: combobox_id.clone(),
    };
    
    // Build base classes
    let base_classes = "radix-combobox";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    // Provide the context
    provide_context(context);
    
    view! {
        <div
            id=combobox_id.clone()
            class=combined_class
            data-value=selected_value.get().unwrap_or_default()
            data-disabled=disabled
            data-open=isopen.get()
            data-searchable=searchable
            data-multi-select=multi_select
            role="combobox"
            aria-expanded=isopen.get()
            aria-haspopup="listbox"
            aria-controls=format!("{}-listbox", combobox_id.clone())
        >
            {children()}
        </div>
    }
}

/// ComboboxTrigger component for the input field
#[component]
pub fn ComboboxTrigger(
    /// Whether the trigger is disabled
    #[prop(optional, default = false)]
    disabled: bool,
    /// Placeholder text
    #[prop(optional)]
    placeholder: Option<String>,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Click event handler
    #[prop(optional)]
    on_click: Option<Callback<web_sys::MouseEvent>>,
    /// Keydown event handler
    #[prop(optional)]
    on_keydown: Option<Callback<web_sys::KeyboardEvent>>,
    /// Input change event handler
    #[prop(optional)]
    on_input: Option<Callback<web_sys::Event>>,
    /// Child content
    children: Children,
) -> impl IntoView {
    let __trigger_id = generate_id("combobox-trigger");
    
    // Get context
    let context = use_context::<ComboboxContext>().expect("ComboboxTrigger must be used within Combobox");
    
    // Handle toggle dropdown
    let handle_click = move |event: web_sys::MouseEvent| {
        if !context.disabled {
            context.set_isopen.update(|open| *open = !*open);
            if !context.isopen.get() {
                context.set_search_query.set(String::new());
                context.setfocused_index.set(0);
            }
        }
        
        // Call custom click handler if provided
        if let Some(callback) = on_click.clone() {
            callback.run(event);
        }
    };
    
    // Handle keyboard navigation
    let handle_keydown = move |event: web_sys::KeyboardEvent| {
        match event.key().as_str() {
            "ArrowDown" | "ArrowUp" => {
                event.prevent_default();
                if !context.isopen.get() {
                    context.set_isopen.set(true);
                }
            }
            "Enter" | " " => {
                event.prevent_default();
                context.set_isopen.update(|open| *open = !*open);
            }
            "Escape" => {
                event.prevent_default();
                context.set_isopen.set(false);
                context.set_search_query.set(String::new());
            }
            _ => {}
        }
        
        // Call custom keydown handler if provided
        if let Some(callback) = on_keydown.clone() {
            callback.run(event);
        }
    };
    
    // Build base classes
    let base_classes = "radix-combobox-trigger";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <div
            id=trigger_id
            class=combined_class
            style=style.unwrap_or_default()
            data-disabled=context.disabled || disabled
            data-open=context.isopen.get()
            role="button"
            tabindex="0"
            aria-haspopup="listbox"
            aria-expanded=context.isopen.get()
            on:click=handle_click
            on:keydown=handle_keydown
        >
            {children()}
        </div>
    }
}

/// ComboboxContent component for the dropdown
#[component]
pub fn ComboboxContent(
    /// Whether the content is visible (optional, will use context if not provided)
    #[prop(optional)]
    open: Option<bool>,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Child content (items)
    children: Children,
) -> impl IntoView {
    let __content_id = generate_id("combobox-content");
    
    // Get context
    let context = use_context::<ComboboxContext>().expect("ComboboxContent must be used within Combobox");
    
    // Use provided open state or context state
    let isopen = open.unwrap_or_else(|| context.isopen.get());
    
    // Build base classes
    let base_classes = "radix-combobox-content";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <div
            id=content_id
            class=combined_class
            }
        >
            {children()}
        </div>
    }
}

/// ComboboxItem component for individual options
#[component]
pub fn ComboboxItem(
    /// Option value
    #[prop(optional)]
    value: String,
    /// Option label
    #[prop(optional)]
    label: String,
    /// Whether the item is disabled
    #[prop(optional, default = false)]
    disabled: bool,
    /// Whether the item is focused (optional, will use context if not provided)
    #[prop(optional, default = false)]
    _focused: bool,
    /// Whether the item is selected (optional, will use context if not provided)
    #[prop(optional, default = false)]
    selected: bool,
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Click event handler
    #[prop(optional)]
    on_click: Option<Callback<web_sys::MouseEvent>>,
    /// Child content
    children: Children,
) -> impl IntoView {
    let __item_id = generate_id("combobox-item");
    
    // Get context
    let context = use_context::<ComboboxContext>().expect("ComboboxItem must be used within Combobox");
    
    // Get current option index and state
    let current_value = value.clone();
    let isfocused = if focused {
        focused
    
    let isselected = if selected {
        selected
    
    // Handle option selection
    let handle_click = move |event: web_sys::MouseEvent| {
        // Find the option in the filtered list
        let filtered_options = context.filtered_options.get();
        if let Some(option) = filtered_options.iter().find(|opt| opt.value == current_value) {
            if !option.disabled.unwrap_or(false) {
                context.setselected_value.set(Some(option.value.clone()));
                context.set_isopen.set(false);
                context.set_search_query.set(String::new());
                
                // Call on_change callback if provided
                if let Some(on_change) = context.on_change.clone() {
                    on_change.run(option.value.clone());
                }
            }
        }
        
        // Call custom click handler if provided
        if let Some(callback) = on_click.clone() {
            callback.run(event);
        }
    };
    
    // Build base classes
    let base_classes = "radix-combobox-item";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <div
            id=item_id
            class=combined_class
            style=style.unwrap_or_default()
            data-value=value
            data-disabled=disabled
            data-focused=isfocused
            data-selected=isselected
            role="option"
            aria-selected=isselected
            aria-disabled=disabled
            tabindex="-1"
            on:click=handle_click
        >
            {children()}
        </div>
    }
}

/// ComboboxInput component for search functionality
#[component]
pub fn ComboboxInput(
    /// Input value (optional, will use context if not provided)
    #[prop(optional)]
    value: Option<String>,
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
    /// Input change event handler
    #[prop(optional)]
    on_input: Option<Callback<web_sys::Event>>,
    /// Keydown event handler
    #[prop(optional)]
    on_keydown: Option<Callback<web_sys::KeyboardEvent>>,
) -> impl IntoView {
    let __input_id = generate_id("combobox-input");
    
    // Get context
    let context = use_context::<ComboboxContext>().expect("ComboboxInput must be used within Combobox");
    
    // Use provided value or context search query
    let input_value = value.unwrap_or_else(|| context.search_query.get());
    
    // Handle search input changes
    let handle_input = move |event: web_sys::Event| {
        if let Some(target) = event.target() {
            if let Ok(input) = target.dyn_into::<web_sys::HtmlInputElement>() {
                let value = input.value();
                context.set_search_query.set(value.clone());
                context.setfocused_index.set(0);
                
                // Open dropdown when typing
                if !context.isopen.get() {
                    context.set_isopen.set(true);
                }
                
                // Call on_search callback if provided
                if let Some(on_search) = context.on_search.clone() {
                    on_search.run(value);
                }
            }
        }
        
        // Call custom input handler if provided
        if let Some(callback) = on_input.clone() {
            callback.run(event);
        }
    };
    
    // Handle keyboard navigation
    let handle_keydown = move |event: web_sys::KeyboardEvent| {
        match event.key().as_str() {
            "ArrowDown" => {
                event.prevent_default();
                let current_index = context.focused_index.get();
                let options_len = context.filtered_options.get().len();
                if current_index < options_len - 1 {
                    context.setfocused_index.set(current_index + 1);
                }
            }
            "ArrowUp" => {
                event.prevent_default();
                let current_index = context.focused_index.get();
                if current_index > 0 {
                    context.setfocused_index.set(current_index - 1);
                }
            }
            "Enter" => {
                event.prevent_default();
                let options = context.filtered_options.get();
                if let Some(option) = options.get(context.focused_index.get()) {
                    if !option.disabled.unwrap_or(false) {
                        context.setselected_value.set(Some(option.value.clone()));
                        context.set_isopen.set(false);
                        context.set_search_query.set(String::new());
                        
                        // Call on_change callback if provided
                        if let Some(on_change) = context.on_change.clone() {
                            on_change.run(option.value.clone());
                        }
                    }
                }
            }
            "Escape" => {
                event.prevent_default();
                context.set_isopen.set(false);
                context.set_search_query.set(String::new());
            }
            _ => {}
        }
        
        // Call custom keydown handler if provided
        if let Some(callback) = on_keydown.clone() {
            callback.run(event);
        }
    };
    
    // Build base classes
    let base_classes = "radix-combobox-input";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <input
            id=input_id
            type="text"
            class=combined_class
            style=style.unwrap_or_default()
            value=input_value
            placeholder=placeholder.unwrap_or_default()
            disabled=context.disabled || disabled
            role="combobox"
            aria-autocomplete="list"
            aria-expanded=context.isopen.get()
            aria-controls=format!("{}-listbox", context.combobox_id)
            on:input=handle_input
            on:keydown=handle_keydown
        />
    }
}

/// ComboboxOptions component for automatically rendering filtered options
#[component]
pub fn ComboboxOptions(
    /// CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
) -> impl IntoView {
    // Get context
    let context = use_context::<ComboboxContext>().expect("ComboboxOptions must be used within Combobox");
    
    // Get filtered options
    let filtered_options = context.filtered_options;
    
    // Build base classes
    let base_classes = "radix-combobox-options";
    let combined_class = merge_classes(Some(base_classes), class.as_deref())
        .unwrap_or_else(|| base_classes.to_string());
    
    view! {
        <div class=combined_class style=style.unwrap_or_default()>
            {move || {
                filtered_options.get().into_iter().enumerate().map(|(index, option)| {
                    let isfocused = index == context.focused_index.get();
                    let isselected = context.selected_value.get().as_ref() == Some(&option.value);
                    
                    view! {
                        <ComboboxItem
                            value=option.value.clone()
                            label=option.label.clone()
                            disabled=option.disabled.unwrap_or(false)
                            focused=isfocused
                            selected=isselected
                        >
                            {option.label}
                        </ComboboxItem>
                    }
                }).collect::<Vec<_>>()
            }}
        </div>
    }
}
