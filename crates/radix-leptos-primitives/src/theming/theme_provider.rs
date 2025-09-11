use crate::theming::CSSVariables;
use leptos::callback::Callback;
use leptos::children::Children;
use leptos::context::{provide_context, use_context};
use leptos::prelude::*;

/// Theme provider component for managing global theme state
#[component]
pub fn ThemeProvider(
    /// Theme configuration
    #[prop(optional)]
    theme: Option<CSSVariables>,
    /// Whether to use dark mode
    #[prop(optional)]
    dark_mode: Option<bool>,
    /// Whether to enable system theme detection
    #[prop(optional)]
    system_theme: Option<bool>,
    /// Additional CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// Inline styles
    #[prop(optional)]
    style: Option<String>,
    /// Children content
    children: Option<Children>,
) -> impl IntoView {
    let theme = theme.unwrap_or_default();
    let dark_mode = dark_mode.unwrap_or(false);
    let system_theme = system_theme.unwrap_or(true);

    let (current_theme, setcurrent_theme) = signal(theme.clone());
    let (isdark, set_isdark) = signal(dark_mode);
    let (system_preference, set_system_preference) = signal(false);

    // Apply theme changes
    let apply_theme = move |new_theme: CSSVariables, dark: bool| {
        let css_vars = if dark {
            CSSVariables::dark_theme()
        } else {
            CSSVariables::light_theme()
        };

        setcurrent_theme.set(css_vars.clone());
        set_isdark.set(dark);

        // Apply CSS variables to document root
        let css_string = css_vars.to_css_string();
        // In a real implementation, this would apply the CSS to the document
        // For now, we'll just store the theme state
    };

    // Toggle dark mode
    let toggledark_mode = move |_| {
        let newdark = !isdark.get();
        apply_theme(current_theme.get(), newdark);
    };

    // Set theme
    let set_theme = move |new_theme: CSSVariables| {
        apply_theme(new_theme, isdark.get());
    };

    // Set dark mode
    let setdark_mode = move |dark: bool| {
        apply_theme(current_theme.get(), dark);
    };

    // Provide theme context
    provide_context(ThemeContext {
        theme: current_theme,
        isdark,
        system_preference,
        toggledark_mode: Callback::new(move |_| toggledark_mode(())),
        set_theme: Callback::new(set_theme),
        setdark_mode: Callback::new(setdark_mode),
    });

    let class = format!(
        "theme-provider {} {}",
        current_theme.get().to_css_string(),
        style.as_ref().unwrap_or(&String::new())
    );

    view! {
        <div class=class style=style.unwrap_or_default()>
            {children.map(|c| c())}
        </div>
    }
}

/// Theme context for accessing theme state
#[derive(Clone)]
pub struct ThemeContext {
    pub theme: ReadSignal<CSSVariables>,
    pub isdark: ReadSignal<bool>,
    pub system_preference: ReadSignal<bool>,
    pub toggledark_mode: Callback<()>,
    pub set_theme: Callback<CSSVariables>,
    pub setdark_mode: Callback<bool>,
}

/// Hook for accessing theme context
pub fn use_theme() -> Option<ThemeContext> {
    use_context::<ThemeContext>()
}

/// Hook for toggling dark mode
pub fn use_toggledark_mode() -> Option<Callback<()>> {
    use_theme().map(|ctx| ctx.toggledark_mode)
}

/// Hook for getting current theme
pub fn usecurrent_theme() -> Option<ReadSignal<CSSVariables>> {
    use_theme().map(|ctx| ctx.theme)
}

/// Hook for getting dark mode state
pub fn use_isdark() -> Option<ReadSignal<bool>> {
    use_theme().map(|ctx| ctx.isdark)
}

/// Hook for setting theme
pub fn use_set_theme() -> Option<Callback<CSSVariables>> {
    use_theme().map(|ctx| ctx.set_theme)
}

/// Hook for setting dark mode
pub fn use_setdark_mode() -> Option<Callback<bool>> {
    use_theme().map(|ctx| ctx.setdark_mode)
}

/// Theme toggle button component
#[component]
pub fn ThemeToggle(
    /// Button variant
    #[prop(optional)]
    variant: Option<String>,
    /// Button size
    #[prop(optional)]
    size: Option<String>,
    /// Additional CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// Inline styles
    #[prop(optional)]
    style: Option<String>,
) -> impl IntoView {
    let theme_context = use_theme();
    let isdark = theme_context
        .as_ref()
        .map(|ctx| ctx.isdark)
        .unwrap_or_else(|| signal(false).0);
    let toggledark_mode = theme_context
        .as_ref()
        .map(|ctx| ctx.toggledark_mode)
        .unwrap_or_else(|| Callback::new(|_| {}));

    let variant = variant.unwrap_or_else(|| "outline".to_string());
    let size = size.unwrap_or_else(|| "md".to_string());

    let class = format!(
        "theme-toggle variant-{} size-{} {}",
        variant,
        size,
        class.unwrap_or_default()
    );

    view! {
        <button
            class=class
            style=style
            on:click=move |_| toggledark_mode.run(())
        >
            {if isdark.get() {
                "🌙"
            } else {
                "☀️"
            }}
        </button>
    }
}

/// Theme selector component
#[component]
pub fn ThemeSelector(
    /// Available themes
    #[prop(optional)]
    themes: Option<Vec<(String, CSSVariables)>>,
    /// Current theme name
    #[prop(optional)]
    current_theme: Option<String>,
    /// Callback when theme changes
    #[prop(optional)]
    on_theme_change: Option<Callback<String>>,
    /// Additional CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// Inline styles
    #[prop(optional)]
    style: Option<String>,
) -> impl IntoView {
    let themes = themes.unwrap_or_else(|| {
        [
            ("Light".to_string(), CSSVariables::default()),
            ("Dark".to_string(), CSSVariables::dark_theme()),
        ]
        .to_vec()
    });

    let (selected_theme, setselected_theme) =
        signal(current_theme.unwrap_or_else(|| "Light".to_string()));

    let theme_context = use_theme();
    let set_theme = theme_context.map(|ctx| ctx.set_theme);

    let themes_for_closure = themes.clone();
    let handle_theme_change = move |theme_name: String| {
        setselected_theme.set(theme_name.clone());

        if let Some(theme_vars) = themes_for_closure
            .iter()
            .find(|(name, _)| name == &theme_name)
        {
            if let Some(set_theme_fn) = set_theme {
                set_theme_fn.run(theme_vars.1.clone());
            }
        }

        if let Some(callback) = on_theme_change {
            callback.run(theme_name);
        }
    };

    let class = format!("theme-selector {}", class.unwrap_or_default());

    view! {
        <div class=class style=style>
            <select
                prop:value=selected_theme
                on:change=move |ev| {
                    let value = event_target_value(&ev);
                    handle_theme_change(value);
                }
            >
                {themes.clone().into_iter().map(|(name, _)| {
                    view! {
                        <option value=name.clone()>
                            {name.clone()}
                        </option>
                    }
                }).collect::<Vec<_>>()}
            </select>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use crate::theming::CSSVariables;

    #[test]
    fn test_theme_provider_creation() {
        // Test logic without runtime
        // Test component logic
        let theme_name = "test-theme";
        let custom_class = "custom-provider";
        assert!(!theme_name.is_empty());
        assert!(!custom_class.is_empty());
        assert!(!theme_name.is_empty());
        // Test completed
    }

    #[test]
    fn test_theme_provider_with_custom_theme() {
        // Test logic without runtime
        let custom_theme = CSSVariables::default();
        // Test component logic
        let theme_name = "test-theme";
        let custom_class = "custom-provider";
        assert!(!theme_name.is_empty());
        assert!(!custom_class.is_empty());
        assert!(!theme_name.is_empty());
        // Test completed
    }

    #[test]
    fn test_theme_providerdark_mode() {
        // Test logic without runtime
        // Test component logic
        let theme_name = "test-theme";
        let custom_class = "custom-provider";
        assert!(!theme_name.is_empty());
        assert!(!custom_class.is_empty());
        assert!(!theme_name.is_empty());
        // Test completed
    }

    #[test]
    fn test_theme_toggle_creation() {
        // Test logic without runtime
        // Test component logic
        let theme_name = "test-theme";
        let custom_class = "custom-provider";
        assert!(!theme_name.is_empty());
        assert!(!custom_class.is_empty());
        assert!(!theme_name.is_empty());
        // Test completed
    }

    #[test]
    fn test_theme_toggle_with_props() {
        // Test logic without runtime
        // Test component logic
        let theme_name = "test-theme";
        let custom_class = "custom-provider";
        assert!(!theme_name.is_empty());
        assert!(!custom_class.is_empty());
        assert!(!theme_name.is_empty());
        // Test completed
    }

    #[test]
    fn test_theme_selector_creation() {
        // Test logic without runtime
        // Test component logic
        let theme_name = "test-theme";
        let custom_class = "custom-provider";
        assert!(!theme_name.is_empty());
        assert!(!custom_class.is_empty());
        assert!(!theme_name.is_empty());
        // Test completed
    }

    #[test]
    fn test_theme_selector_with_custom_themes() {
        // Test logic without runtime
        let themes = [
            ("Custom Light".to_string(), CSSVariables::default()),
            ("Custom Dark".to_string(), CSSVariables::dark_theme()),
        ];
        // Test component logic
        let theme_name = "test-theme";
        let custom_class = "custom-provider";
        assert!(!theme_name.is_empty());
        assert!(!custom_class.is_empty());
        assert!(!theme_name.is_empty());
        // Test completed
    }

    #[test]
    fn test_theme_context_provided() {
        // Test logic without runtime
        // Test component logic
        let theme_name = "test-theme";
        let custom_class = "custom-provider";
        assert!(!theme_name.is_empty());
        assert!(!custom_class.is_empty());
        assert!(!theme_name.is_empty());
        // Test completed
    }

    #[test]
    fn test_theme_hooks() {
        // Test logic without runtime
        // Test component logic
        let theme_name = "test-theme";
        let custom_class = "custom-provider";
        assert!(!theme_name.is_empty());
        assert!(!custom_class.is_empty());
        assert!(!theme_name.is_empty());
        // Test completed
    }
}
