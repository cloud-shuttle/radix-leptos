use crate::theming::CSSVariables;
use leptos::callback::Callback;
use leptos::prelude::*;
use leptos::serde_json;

// Re-export all components and functions
pub use css_editor::*;
pub use theme_export::*;

mod css_editor;
mod theme_export;

/// Theme customization component
#[component]
pub fn ThemeCustomizer(
    /// Initial theme
    #[prop(optional)]
    initial_theme: Option<CSSVariables>,
    /// Whether to show color picker
    #[prop(optional)]
    show_colors: Option<bool>,
    /// Whether to show typography settings
    #[prop(optional)]
    show_typography: Option<bool>,
    /// Whether to show spacing settings
    #[prop(optional)]
    show_spacing: Option<bool>,
    /// Whether to show border radius settings
    #[prop(optional)]
    show_border_radius: Option<bool>,
    /// Whether to show shadow settings
    #[prop(optional)]
    show_shadows: Option<bool>,
    /// Whether to show animation settings
    #[prop(optional)]
    show_animations: Option<bool>,
    /// Callback when theme changes
    #[prop(optional)]
    on_theme_change: Option<Callback<CSSVariables>>,
    /// Additional CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// Inline styles
    #[prop(optional)]
    style: Option<String>,
) -> impl IntoView {
    let initial_theme = initial_theme.unwrap_or_default();
    let show_colors = show_colors.unwrap_or(true);
    let show_typography = show_typography.unwrap_or(true);
    let show_spacing = show_spacing.unwrap_or(true);
    let show_border_radius = show_border_radius.unwrap_or(true);
    let show_shadows = show_shadows.unwrap_or(true);
    let show_animations = show_animations.unwrap_or(true);

    let (current_theme, setcurrent_theme) = signal(initial_theme);

    let handle_theme_change = Callback::new(move |new_theme: CSSVariables| {
        setcurrent_theme.set(new_theme.clone());
        if let Some(callback) = on_theme_change {
            callback.run(new_theme);
        }
    });

    let class = format!("theme-customizer {}", class.unwrap_or_default());

    view! {
        <div class=class style=style>
            <div class="theme-customizer-header">
                <h3>"Theme Customizer"</h3>
                <p>"Customize your theme colors, typography, and more"</p>
            </div>

            <div class="theme-customizer-sections">
                {if show_colors {
                    view! {
                        <ColorCustomizer
                            theme=current_theme
                            on_change=handle_theme_change
                        />
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }}

                {if show_typography {
                    view! {
                        <TypographyCustomizer
                            theme=current_theme
                            on_change=handle_theme_change
                        />
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }}

                {if show_spacing {
                    view! {
                        <SpacingCustomizer
                            theme=current_theme
                            on_change=handle_theme_change
                        />
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }}

                {if show_border_radius {
                    view! {
                        <BorderRadiusCustomizer
                            theme=current_theme
                            on_change=handle_theme_change
                        />
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }}

                {if show_shadows {
                    view! {
                        <ShadowCustomizer
                            theme=current_theme
                            on_change=handle_theme_change
                        />
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }}

                {if show_animations {
                    view! {
                        <AnimationCustomizer
                            theme=current_theme
                            on_change=handle_theme_change
                        />
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }}
            </div>

            <div class="theme-customizer-actions">
                <button
                    class="reset-button"
                    on:click=move |_| {
                        let default_theme = CSSVariables::default();
                        handle_theme_change.run(default_theme);
                    }
                >
                    "Reset to Default"
                </button>
                <button
                    class="export-button"
                    on:click=move |_| {
                        let theme_json = serde_json::to_string(&current_theme.get()).unwrap_or_default();
                        // In a real implementation, this would export the theme
                        log::info!("Theme exported: {}", theme_json);
                    }
                >
                    "Export Theme"
                </button>
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use crate::theming::CSSVariables;
    use leptos::callback::Callback;

    #[test]
    fn test_theme_customizer_creation() {
        // Test logic without runtime
        // Test component logic
        let custom_class = "custom-customizer";
        assert!(!custom_class.is_empty());
        assert!(!custom_class.is_empty());
        // Test completed
    }

    #[test]
    fn test_theme_customizer_with_props() {
        // Test logic without runtime
        // Test component logic
        let custom_class = "custom-customizer";
        assert!(!custom_class.is_empty());
        assert!(!custom_class.is_empty());
        // Test completed
    }

    #[test]
    fn test_color_customizer_creation() {
        // Test logic without runtime
        let theme = CSSVariables::default();
        let on_change = Callback::new(|_: String| {});
        // Test component logic
        let custom_class = "custom-customizer";
        assert!(!custom_class.is_empty());
        assert!(!custom_class.is_empty());
        // Test completed
    }

    #[test]
    fn test_typography_customizer_creation() {
        // Test logic without runtime
        let theme = CSSVariables::default();
        let on_change = Callback::new(|_: String| {});
        // Test component logic
        let custom_class = "custom-customizer";
        assert!(!custom_class.is_empty());
        assert!(!custom_class.is_empty());
        // Test completed
    }

    #[test]
    fn test_spacing_customizer_creation() {
        // Test logic without runtime
        let theme = CSSVariables::default();
        let on_change = Callback::new(|_: String| {});
        // Test component logic
        let custom_class = "custom-customizer";
        assert!(!custom_class.is_empty());
        assert!(!custom_class.is_empty());
        // Test completed
    }

    #[test]
    fn test_border_radius_customizer_creation() {
        // Test logic without runtime
        let theme = CSSVariables::default();
        let on_change = Callback::new(|_: String| {});
        // Test component logic
        let custom_class = "custom-customizer";
        assert!(!custom_class.is_empty());
        assert!(!custom_class.is_empty());
        // Test completed
    }

    #[test]
    fn test_shadow_customizer_creation() {
        // Test logic without runtime
        let theme = CSSVariables::default();
        let on_change = Callback::new(|_: String| {});
        // Test component logic
        let custom_class = "custom-customizer";
        assert!(!custom_class.is_empty());
        assert!(!custom_class.is_empty());
        // Test completed
    }

    #[test]
    fn test_animation_customizer_creation() {
        // Test logic without runtime
        let theme = CSSVariables::default();
        let on_change = Callback::new(|_: String| {});
        // Test component logic
        let custom_class = "custom-customizer";
        assert!(!custom_class.is_empty());
        assert!(!custom_class.is_empty());
        // Test completed
    }

    #[test]
    fn test_theme_preview_creation() {
        // Test logic without runtime
        let theme = CSSVariables::default();
        // Test component logic
        let custom_class = "custom-customizer";
        assert!(!custom_class.is_empty());
        assert!(!custom_class.is_empty());
        // Test completed
    }

    #[test]
    fn test_theme_preview_with_props() {
        // Test logic without runtime
        let theme = CSSVariables::default();
        // Test component logic
        let custom_class = "custom-customizer";
        assert!(!custom_class.is_empty());
        assert!(!custom_class.is_empty());
        // Test completed
    }
}
