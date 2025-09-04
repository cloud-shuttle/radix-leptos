use leptos::*;
use leptos::prelude::*;

/// Option item for multi-select
#[derive(Clone, Debug, PartialEq)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

impl SelectOption {
    pub fn new(value: String, label: String) -> Self {
        Self {
            value,
            label,
            disabled: false,
        }
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Simple multi-select component
#[component]
pub fn MultiSelect(
    /// Available options
    #[prop(into)]
    options: Vec<SelectOption>,
    /// Currently selected values
    #[prop(optional)]
    value: Option<Vec<String>>,
    /// Callback when selection changes
    #[prop(optional)]
    on_change: Option<Callback<Vec<String>>>,
    /// Whether the select is disabled
    #[prop(optional, default = false)]
    disabled: bool,
    /// Placeholder text
    #[prop(optional)]
    placeholder: Option<String>,
    /// CSS classes to apply
    #[prop(optional)]
    class: Option<String>,
) -> impl IntoView {
    let (selected_values, set_selected_values) = signal(value.unwrap_or_default());

    // Handle option selection/deselection
    let toggle_option = move |option_value: String| {
        let mut current = selected_values.get();
        if current.contains(&option_value) {
            current.retain(|v| v != &option_value);
        } else {
            current.push(option_value);
        }
        set_selected_values.set(current.clone());
        
        if let Some(callback) = on_change {
            callback.call(current);
        }
    };

    // Remove a selected value
    let remove_value = move |value_to_remove: String| {
        let mut current = selected_values.get();
        current.retain(|v| v != &value_to_remove);
        set_selected_values.set(current.clone());
        
        if let Some(callback) = on_change {
            callback.call(current);
        }
    };

    view! {
        <div class={format!("multi-select {}", class.unwrap_or_default())}>
            <div class="multi-select-trigger">
                <div class="multi-select-values">
                    {move || if selected_values.get().is_empty() {
                        view! {
                            <span class="multi-select-placeholder">
                                {placeholder.clone().unwrap_or_else(|| "Select options...".to_string())}
                            </span>
                        }
                    } else {
                        view! {
                            <div class="multi-select-tags">
                                {selected_values.get().into_iter().map(|value| {
                                    let value_clone = value.clone();
                                    view! {
                                        <span class="multi-select-tag">
                                            <span class="tag-label">{value}</span>
                                            <button
                                                type="button"
                                                class="tag-remove"
                                                on:click=move |e| {
                                                    e.stop_propagation();
                                                    remove_value(value_clone.clone());
                                                }
                                            >
                                                "×"
                                            </button>
                                        </span>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        }
                    }}
                </div>
            </div>

            <div class="multi-select-options">
                {move || {
                    let options_list = options.iter().map(|option| {
                        let option_value = option.value.clone();
                        let is_selected = selected_values.get().contains(&option.value);
                        let is_disabled = option.disabled;
                        
                        view! {
                            <div
                                class={format!(
                                    "multi-select-option {} {}",
                                    if is_selected { "selected" } else { "" },
                                    if is_disabled { "disabled" } else { "" }
                                )}
                                role="option"
                                aria-selected=is_selected
                                aria-disabled=is_disabled
                                on:click=move |_| {
                                    if !is_disabled {
                                        toggle_option(option_value.clone());
                                    }
                                }}
                            >
                                <input
                                    type="checkbox"
                                    checked=is_selected
                                    disabled=is_disabled
                                    readOnly=true
                                />
                                <span class="option-label">{option.label}</span>
                            </div>
                        }
                    }).collect::<Vec<_>>();
                    
                    options_list
                }}
            </div>
        </div>
    }
}

/// Simplified multi-select for basic use cases
#[component]
pub fn MultiSelectBasic(
    /// Available options as simple strings
    #[prop(into)]
    options: Vec<String>,
    /// Currently selected values
    #[prop(optional)]
    value: Option<Vec<String>>,
    /// Callback when selection changes
    #[prop(optional)]
    on_change: Option<Callback<Vec<String>>>,
    /// Whether the select is disabled
    #[prop(optional, default = false)]
    disabled: bool,
    /// Placeholder text
    #[prop(optional)]
    placeholder: Option<String>,
    /// CSS classes to apply
    #[prop(optional)]
    class: Option<String>,
) -> impl IntoView {
    let select_options = move || {
        options
            .iter()
            .map(|opt| SelectOption::new(opt.clone(), opt.clone()))
            .collect()
    };

    view! {
        <MultiSelect
            options=select_options()
            value=value
            on_change=on_change
            disabled=disabled
            placeholder=placeholder
            class=class
        />
    }
}
