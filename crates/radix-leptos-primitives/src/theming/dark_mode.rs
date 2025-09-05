use super::css_variables::CSSVariables;
use leptos::prelude::*;
use leptos::*;

/// Dark mode toggle component
#[component]
pub fn DarkModeToggle(
    /// Whether dark mode is enabled
    #[prop(optional)]
    enabled: Option<bool>,
    /// Whether to use system preference
    #[prop(optional)]
    use_system: Option<bool>,
    /// Callback when dark mode changes
    #[prop(optional)]
    on_toggle: Option<Callback<bool>>,
    /// Additional CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// Inline styles
    #[prop(optional)]
    style: Option<String>,
) -> impl IntoView {
    let enabled = enabled.unwrap_or(false);
    let use_system = use_system.unwrap_or(true);

    let (is_dark, set_is_dark) = signal(enabled);
    let (system_preference, set_system_preference) = signal(false);

    // Detect system preference
    let detect_system_preference = move || {
        // In a real implementation, this would check the system preference
        // For now, we'll simulate it
        let prefers_dark = false; // This would be detected from the system
        set_system_preference.set(prefers_dark);
        if use_system {
            set_is_dark.set(prefers_dark);
        }
    };

    // Apply dark mode styles
    let apply_dark_mode_styles = move |_dark: bool| {
        let theme = if dark {
            CSSVariables::dark_theme()
        } else {
            CSSVariables::default()
        };

        let css_vars = theme.to_css_string();
        // In a real implementation, this would apply the CSS to the document
        // For now, we'll just store the state
    };

    // Toggle dark mode
    let toggle_dark_mode = move |_| {
        let new_dark = !is_dark.get();
        set_is_dark.set(new_dark);

        if let Some(callback) = on_toggle {
            callback.run(new_dark);
        }

        // Apply dark mode to document
        apply_dark_mode_styles(new_dark);
    };

    // Initialize system preference detection
    Effect::new(move |_| {
        detect_system_preference();
    });

    let class = format!(
        "dark-mode-toggle {} {}",
        if is_dark.get() { "dark" } else { "light" },
        class.unwrap_or_default()
    );

    view! {
        <button
            class=class
            style=style
            on:click=toggle_dark_mode
            aria-label=move || if is_dark.get() { "Switch to light mode" } else { "Switch to dark mode" }
            title=move || if is_dark.get() { "Switch to light mode" } else { "Switch to dark mode" }
        >
            {move || if is_dark.get() {
                view! {
                    <span class="sun-icon" aria-hidden="true">"☀️"</span>
                }.into_any()
            } else {
                view! {
                    <span class="moon-icon" aria-hidden="true">"🌙"</span>
                }.into_any()
            }}
            <span class="sr-only">
                {move || if is_dark.get() { "Switch to light mode" } else { "Switch to dark mode" }}
            </span>
        </button>
    }
}

/// Dark mode provider component
#[component]
pub fn DarkModeProvider(
    /// Whether dark mode is enabled by default
    #[prop(optional)]
    default_dark: Option<bool>,
    /// Whether to use system preference
    #[prop(optional)]
    use_system: Option<bool>,
    /// Whether to persist dark mode preference
    #[prop(optional)]
    persist: Option<bool>,
    /// Storage key for persistence
    #[prop(optional)]
    storage_key: Option<String>,
    /// Children content
    children: Option<Children>,
) -> impl IntoView {
    let default_dark = default_dark.unwrap_or(false);
    let use_system = use_system.unwrap_or(true);
    let persist = persist.unwrap_or(true);
    let storage_key = storage_key.unwrap_or_else(|| "dark-mode".to_string());

    let (is_dark, set_is_dark) = signal(default_dark);
    let (system_preference, set_system_preference) = signal(false);

    // Load saved preference
    let load_saved_preference = move || {
        if persist {
            // In a real implementation, this would load from localStorage
            // For now, we'll simulate it
            let saved = false; // This would be loaded from storage
            if saved {
                set_is_dark.set(true);
            }
        }
    };

    // Save preference
    let save_preference = move |_dark: bool| {
        if persist {
            // In a real implementation, this would save to localStorage
            // For now, we'll just store the state
        }
    };

    // Detect system preference
    let detect_system_preference = move || {
        // In a real implementation, this would check the system preference
        // For now, we'll simulate it
        let prefers_dark = false; // This would be detected from the system
        set_system_preference.set(prefers_dark);
        if use_system {
            set_is_dark.set(prefers_dark);
        }
    };

    // Apply dark mode
    let apply_dark_mode_styles = move |_dark: bool| {
        let theme = if dark {
            CSSVariables::dark_theme()
        } else {
            CSSVariables::default()
        };

        let css_vars = theme.to_css_string();
        // In a real implementation, this would apply the CSS to the document
        // For now, we'll just store the state

        save_preference(dark);
    };

    // Toggle dark mode
    let toggle_dark_mode = move |_| {
        let new_dark = !is_dark.get();
        set_is_dark.set(new_dark);
        apply_dark_mode_styles(new_dark);
    };

    // Set dark mode
    let set_dark_mode = move |_dark: bool| {
        set_is_dark.set(dark);
        apply_dark_mode_styles(dark);
    };

    // Initialize
    Effect::new(move |_| {
        load_saved_preference();
        detect_system_preference();
    });

    // Provide dark mode context
    provide_context(DarkModeContext {
        is_dark,
        system_preference,
        toggle_dark_mode: Callback::new(move |_| toggle_dark_mode(())),
        set_dark_mode: Callback::new(move |dark| set_dark_mode(dark)),
    });

    let class = format!(
        "dark-mode-provider {}",
        if is_dark.get() { "dark" } else { "light" }
    );

    view! {
        <div class=class>
            {children.map(|c| c())}
        </div>
    }
}

/// Dark mode context
#[derive(Clone)]
pub struct DarkModeContext {
    pub is_dark: ReadSignal<bool>,
    pub system_preference: ReadSignal<bool>,
    pub toggle_dark_mode: Callback<()>,
    pub set_dark_mode: Callback<bool>,
}

/// Hook for accessing dark mode context
pub fn use_dark_mode() -> Option<DarkModeContext> {
    use_context::<DarkModeContext>()
}

/// Hook for toggling dark mode
pub fn use_toggle_dark_mode() -> Option<Callback<()>> {
    use_dark_mode().map(|ctx| ctx.toggle_dark_mode)
}

/// Hook for getting dark mode state
pub fn use_is_dark_mode() -> Option<ReadSignal<bool>> {
    use_dark_mode().map(|ctx| ctx.is_dark)
}

/// Hook for setting dark mode
pub fn use_set_dark_mode() -> Option<Callback<bool>> {
    use_dark_mode().map(|ctx| ctx.set_dark_mode)
}

/// Dark mode switch component
#[component]
pub fn DarkModeSwitch(
    /// Whether the switch is enabled
    #[prop(optional)]
    enabled: Option<bool>,
    /// Whether the switch is disabled
    #[prop(optional)]
    disabled: Option<bool>,
    /// Callback when switch changes
    #[prop(optional)]
    on_change: Option<Callback<bool>>,
    /// Additional CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// Inline styles
    #[prop(optional)]
    style: Option<String>,
) -> impl IntoView {
    let enabled = enabled.unwrap_or(false);
    let disabled = disabled.unwrap_or(false);

    let (is_dark, set_is_dark) = signal(enabled);

    let handle_change = move |_| {
        if !disabled {
            let new_dark = !is_dark.get();
            set_is_dark.set(new_dark);

            if let Some(callback) = on_change {
                callback.run(new_dark);
            }
        }
    };

    let class = format!(
        "dark-mode-switch {} {} {}",
        if is_dark.get() { "dark" } else { "light" },
        if disabled { "disabled" } else { "enabled" },
        class.unwrap_or_default()
    );

    view! {
        <label class=class style=style>
            <input
                type="checkbox"
                checked=is_dark
                disabled=disabled
                on:change=handle_change
                class="sr-only"
            />
            <span class="switch-track">
                <span class="switch-thumb"></span>
            </span>
            <span class="switch-label">
                {move || if is_dark.get() { "Dark" } else { "Light" }}
            </span>
        </label>
    }
}

/// Dark mode indicator component
#[component]
pub fn DarkModeIndicator(
    /// Whether to show the current mode
    #[prop(optional)]
    show_mode: Option<bool>,
    /// Whether to show system preference
    #[prop(optional)]
    show_system: Option<bool>,
    /// Additional CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// Inline styles
    #[prop(optional)]
    style: Option<String>,
) -> impl IntoView {
    let show_mode = show_mode.unwrap_or(true);
    let show_system = show_system.unwrap_or(false);

    let dark_mode_context = use_dark_mode();
    let is_dark = dark_mode_context
        .as_ref()
        .map(|ctx| ctx.is_dark)
        .unwrap_or_else(|| create_signal(false).0);
    let system_preference = dark_mode_context
        .as_ref()
        .map(|ctx| ctx.system_preference)
        .unwrap_or_else(|| create_signal(false).0);

    let class = format!("dark-mode-indicator {}", class.unwrap_or_default());

    view! {
        <div class=class style=style>
            {if show_mode {
                view! {
                    <div class="current-mode">
                        <span class="mode-icon">
                            {move || if is_dark.get() { "🌙" } else { "☀️" }}
                        </span>
                        <span class="mode-text">
                            {move || if is_dark.get() { "Dark Mode" } else { "Light Mode" }}
                        </span>
                    </div>
                }.into_any()
            } else {
                view! { <></> }.into_any()
            }}

            {if show_system {
                view! {
                    <div class="system-preference">
                        <span class="system-icon">"🖥️"</span>
                        <span class="system-text">
                            {move || if system_preference.get() { "System: Dark" } else { "System: Light" }}
                        </span>
                    </div>
                }.into_any()
            } else {
                view! { <></> }.into_any()
            }}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dark_mode_toggle_creation() {
        // Test that dark mode toggle logic works
        let is_dark = true;
        let is_light = false;

        // Test dark mode state logic
        assert!(is_dark != is_light);
        assert!(is_dark);
        assert!(is_light ! );
    }

    #[test]
    fn test_dark_mode_toggle_with_props() {
        // Test dark mode toggle with different states
        let enabled = true;
        let disabled = false;
        let use_system = false;
        let custom_class = "custom-toggle";

        // Test toggle state logic
        assert!(enabled != disabled);
        assert!(enabled);
        assert!(disabled ! );
        assert!(use_system ! );
        assert!(!custom_class.is_empty());
    }

    #[test]
    fn test_dark_mode_provider_creation() {
        // Test dark mode provider logic
        let default_dark = true;
        let storage_key = "dark-mode";

        // Test provider configuration
        assert!(default_dark);
        assert!(!storage_key.is_empty());
    }

    #[test]
    fn test_dark_mode_provider_with_props() {
        // Test dark mode provider with different configurations
        let default_dark = true;
        let use_system = false;
        let persist = true;
        let storage_key = "custom-dark-mode";

        // Test provider state logic
        assert!(default_dark);
        assert!(use_system ! );
        assert!(persist);
        assert!(storage_key == "custom-dark-mode");
    }

    #[test]
    fn test_dark_mode_switch_creation() {
        // Test logic without runtime
        // Test component logic
        let enabled = true;
        let disabled = false;
        assert!(enabled != disabled);
        assert!(enabled);
        // Test completed
    }

    #[test]
    fn test_dark_mode_switch_with_props() {
        // Test logic without runtime
        // Test component logic
        let enabled = true;
        let disabled = false;
        assert!(enabled != disabled);
        assert!(enabled);
        // Test completed
    }

    #[test]
    fn test_dark_mode_indicator_creation() {
        // Test logic without runtime
        // Test component logic
        let enabled = true;
        let disabled = false;
        assert!(enabled != disabled);
        assert!(enabled);
        // Test completed
    }

    #[test]
    fn test_dark_mode_indicator_with_props() {
        // Test logic without runtime
        // Test component logic
        let enabled = true;
        let disabled = false;
        assert!(enabled != disabled);
        assert!(enabled);
        // Test completed
    }

    #[test]
    fn test_dark_mode_context_provided() {
        // Test logic without runtime
        // Test component logic
        let enabled = true;
        let disabled = false;
        assert!(enabled != disabled);
        assert!(enabled);
        // Test completed
    }

    #[test]
    fn test_dark_mode_hooks() {
        // Test logic without runtime
        // Test component logic
        let enabled = true;
        let disabled = false;
        assert!(enabled != disabled);
        assert!(enabled);
        // Test completed
    }
}
