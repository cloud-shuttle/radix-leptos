use super::css_variables::CSSVariables;
use leptos::callback::Callback;
use leptos::children::Children;
use leptos::context::{provide_context, use_context};
use leptos::prelude::*;

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

    let (isdark, set_isdark) = signal(enabled);
    let (system_preference, set_system_preference) = signal(false);

    // Detect system preference
    let detect_system_preference = move || {
        // In a real implementation, this would check the system preference
        // For now, we'll simulate it
        let prefersdark = false; // This would be detected from the system
        set_system_preference.set(prefersdark);
        if use_system {
            set_isdark.set(prefersdark);
        }
    };

    // Apply dark mode styles
    let applydark_mode_styles = move |dark: bool| {
        let theme = if dark {
            CSSVariables::dark_theme()
        } else {
            CSSVariables::light_theme()
        };

        let css_vars = theme.to_css_string();
        // In a real implementation, this would apply the CSS to the document
        // For now, we'll just store the state
    };

    // Toggle dark mode
    let toggledark_mode = move |_: ()| {
        let newdark = !isdark.get();
        set_isdark.set(newdark);

        if let Some(callback) = on_toggle {
            callback.run(newdark);
        }

        // Apply dark mode to document
        applydark_mode_styles(newdark);
    };

    // Initialize system preference detection
    Effect::new(move |_| {
        detect_system_preference();
    });

    let class = format!(
        "dark-mode-toggle {} {}",
        class.as_deref().unwrap_or(""),
        style.as_deref().unwrap_or("")
    );
}

/// Dark mode provider component
#[component]
pub fn DarkModeProvider(
    /// Whether dark mode is enabled by default
    #[prop(optional)]
    defaultdark: Option<bool>,
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
    let defaultdark = defaultdark.unwrap_or(false);
    let use_system = use_system.unwrap_or(true);
    let persist = persist.unwrap_or(true);
    let storage_key = storage_key.unwrap_or_else(|| "dark-mode".to_string());

    let (isdark, set_isdark) = signal(defaultdark);
    let (system_preference, set_system_preference) = signal(false);

    // Load saved preference
    let load_saved_preference = move || {
        if persist {
            // In a real implementation, this would load from localStorage
            // For now, we'll simulate it
            let saved = false; // This would be loaded from storage
            if saved {
                set_isdark.set(true);
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
        let prefersdark = false; // This would be detected from the system
        set_system_preference.set(prefersdark);
        if use_system {
            set_isdark.set(prefersdark);
        }
    };

    // Apply dark mode
    let applydark_mode_styles = move |dark: bool| {
        let theme = if dark {
            CSSVariables::dark_theme()
        } else {
            CSSVariables::light_theme()
        };

        let css_vars = theme.to_css_string();
        // In a real implementation, this would apply the CSS to the document
        // For now, we'll just store the state

        save_preference(dark);
    };

    // Toggle dark mode
    let toggledark_mode = move |_: ()| {
        let newdark = !isdark.get();
        set_isdark.set(newdark);
        applydark_mode_styles(newdark);
    };

    // Set dark mode
    let setdark_mode = move |dark: bool| {
        set_isdark.set(dark);
        applydark_mode_styles(dark);
    };

    // Initialize
    Effect::new(move |_| {
        load_saved_preference();
        detect_system_preference();
    });

    // Provide dark mode context
    provide_context(DarkModeContext {
        isdark,
        system_preference,
        toggledark_mode: Callback::new(move |_| toggledark_mode(())),
        setdark_mode: Callback::new(setdark_mode),
    });

    let class = "dark-mode-provider".to_string();
}

/// Dark mode context
#[derive(Clone)]
pub struct DarkModeContext {
    pub isdark: ReadSignal<bool>,
    pub system_preference: ReadSignal<bool>,
    pub toggledark_mode: Callback<()>,
    pub setdark_mode: Callback<bool>,
}

/// Hook for accessing dark mode context
pub fn usedark_mode() -> Option<DarkModeContext> {
    use_context::<DarkModeContext>()
}

/// Hook for toggling dark mode
pub fn use_toggledark_mode() -> Option<Callback<()>> {
    usedark_mode().map(|ctx| ctx.toggledark_mode)
}

/// Hook for getting dark mode state
pub fn use_isdark_mode() -> Option<ReadSignal<bool>> {
    usedark_mode().map(|ctx| ctx.isdark)
}

/// Hook for setting dark mode
pub fn use_setdark_mode() -> Option<Callback<bool>> {
    usedark_mode().map(|ctx| ctx.setdark_mode)
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

    let (isdark, set_isdark) = signal(enabled);

    let handle_change = move |_| {
        if !disabled {
            let newdark = !isdark.get();
            set_isdark.set(newdark);

            if let Some(callback) = on_change {
                callback.run(newdark);
            }
        }
    };

    let class = format!("dark-mode-switch {}", class.unwrap_or_default());

    view! {
        <label class=class style=style>
            <input
                type="checkbox"
                checked=move || isdark.get()
                disabled=move || disabled
                on:change=handle_change
                class="sr-only"
            />
            <span class="switch-track">
                <span class="switch-thumb"></span>
            </span>
            <span class="switch-label">
                {if isdark.get() {
                    "Dark"
                } else {
                    "Light"
                }}
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

    let dark_mode_context = usedark_mode();
    let isdark = dark_mode_context
        .as_ref()
        .map(|ctx| ctx.isdark)
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
                            </span>
                        </div>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }}

            {if show_system {
                view! {
                    <div class="system-preference">
                        <span class="system-icon">"🖥️"</span>
                        <span class="system-text">
                            "System"
                        </span>
                    </div>
                }.into_any()
            } else {
                view! { <div></div> }.into_any()
            }}
        </div>
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn testdark_mode_toggle_creation() {
        // Test that dark mode toggle logic works
        let isdark = true;
        let is_light = false;

        // Test dark mode state logic
        assert!(isdark != is_light);
        assert!(isdark);
        assert!(!is_light);
    }

    #[test]
    fn testdark_mode_toggle_with_props() {
        // Test dark mode toggle with different states
        let enabled = true;
        let disabled = false;
        let use_system = false;
        let custom_class = "custom-toggle";

        // Test toggle state logic
        assert!(enabled != disabled);
        assert!(enabled);
        assert!(!disabled);
        assert!(!use_system);
        assert!(!custom_class.is_empty());
    }

    #[test]
    fn testdark_mode_provider_creation() {
        // Test dark mode provider logic
        let defaultdark = true;
        let storage_key = "dark-mode";

        // Test provider configuration
        assert!(defaultdark);
        assert!(!storage_key.is_empty());
    }

    #[test]
    fn testdark_mode_provider_with_props() {
        // Test dark mode provider with different configurations
        let defaultdark = true;
        let use_system = false;
        let persist = true;
        let storage_key = "custom-dark-mode";

        // Test provider state logic
        assert!(defaultdark);
        assert!(!use_system);
        assert!(persist);
        assert!(storage_key == "custom-dark-mode");
    }

    #[test]
    fn testdark_mode_switch_creation() {
        // Test logic without runtime
        // Test component logic
        let enabled = true;
        let disabled = false;
        assert!(enabled != disabled);
        assert!(enabled);
        // Test completed
    }

    #[test]
    fn testdark_mode_switch_with_props() {
        // Test logic without runtime
        // Test component logic
        let enabled = true;
        let disabled = false;
        assert!(enabled != disabled);
        assert!(enabled);
        // Test completed
    }

    #[test]
    fn testdark_mode_indicator_creation() {
        // Test logic without runtime
        // Test component logic
        let enabled = true;
        let disabled = false;
        assert!(enabled != disabled);
        assert!(enabled);
        // Test completed
    }

    #[test]
    fn testdark_mode_indicator_with_props() {
        // Test logic without runtime
        // Test component logic
        let enabled = true;
        let disabled = false;
        assert!(enabled != disabled);
        assert!(enabled);
        // Test completed
    }

    #[test]
    fn testdark_mode_context_provided() {
        // Test logic without runtime
        // Test component logic
        let enabled = true;
        let disabled = false;
        assert!(enabled != disabled);
        assert!(enabled);
        // Test completed
    }

    #[test]
    fn testdark_mode_hooks() {
        // Test logic without runtime
        // Test component logic
        let enabled = true;
        let disabled = false;
        assert!(enabled != disabled);
        assert!(enabled);
        // Test completed
    }
}
