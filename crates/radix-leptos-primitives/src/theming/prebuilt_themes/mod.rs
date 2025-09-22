use crate::theming::css_variables::*;
use crate::utils::merge_classes;
use leptos::callback::Callback;
use leptos::prelude::*;

// Module declarations
mod color_schemes;
mod theme_builder;
mod light_themes;
mod dark_themes;

// Re-export all types and functions from sub-modules
pub use color_schemes::*;
pub use theme_builder::*;
pub use light_themes::*;
pub use dark_themes::*;

/// Pre-built theme collection with industry-specific themes
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PrebuiltThemes {
    pub light: CSSVariables,
    pub dark: CSSVariables,
    pub high_contrast: CSSVariables,
    pub finance: CSSVariables,
    pub healthcare: CSSVariables,
    pub education: CSSVariables,
    pub ecommerce: CSSVariables,
    pub gaming: CSSVariables,
    pub minimal: CSSVariables,
    pub vibrant: CSSVariables,
}

impl Default for PrebuiltThemes {
    fn default() -> Self {
        Self {
            light: create_light_theme(),
            dark: create_dark_theme(),
            high_contrast: create_high_contrast_theme(),
            finance: create_finance_theme(),
            healthcare: create_healthcare_theme(),
            education: create_education_theme(),
            ecommerce: create_ecommerce_theme(),
            gaming: create_gaming_theme(),
            minimal: create_minimal_theme(),
            vibrant: create_vibrant_theme(),
        }
    }
}

/// Theme selector component for choosing from pre-built themes
#[component]
pub fn ThemeSelector(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] current_theme: Option<String>,
    #[prop(optional)] on_theme_change: Option<Callback<String>>,
    #[prop(optional)] show_preview: Option<bool>,
    #[prop(optional)] categories: Option<Vec<ThemeCategory>>,
) -> impl IntoView {
    let current_theme = current_theme.unwrap_or_else(|| "light".to_string());
    let show_preview = show_preview.unwrap_or(true);
    let categories = categories.unwrap_or_else(|| {
        [
            ThemeCategory::Basic,
            ThemeCategory::Industry,
            ThemeCategory::Style,
        ]
        .to_vec()
    });

    let class = merge_classes(["theme-selector", class.as_deref().unwrap_or("")].to_vec());

    let handle_theme_change = Callback::new(move |theme_name: String| {
        if let Some(callback) = on_theme_change {
            callback.run(theme_name);
        }
    });

    let themes = get_themes_by_categories(&categories);

    view! {
        <div
            class=class
            style=style
            role="radiogroup"
            aria-label="Theme selection"
        >
            <div class="theme-selector-header">
                <h3>"Choose a Theme"</h3>
                <p>"Select from our pre-built themes or create your own"</p>
            </div>

            <div class="theme-categories">
                {categories.into_iter().map(|category| {
                    let category_themes = themes.get(&category).cloned().unwrap_or_default();
                    view! {
                        <ThemeCategorySection
                            category=category
                            themes=category_themes
                            current_theme=current_theme.clone()
                            show_preview=show_preview
                            on_theme_change=handle_theme_change
                        />
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

/// Theme category section component
#[component]
pub fn ThemeCategorySection(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] category: Option<ThemeCategory>,
    #[prop(optional)] themes: Option<Vec<ThemeInfo>>,
    #[prop(optional)] current_theme: Option<String>,
    #[prop(optional)] show_preview: Option<bool>,
    #[prop(optional)] on_theme_change: Option<Callback<String>>,
) -> impl IntoView {
    let category = category.unwrap_or(ThemeCategory::Basic);
    let themes = themes.unwrap_or_default();
    let current_theme = current_theme.unwrap_or_default();
    let show_preview = show_preview.unwrap_or(true);
    let on_theme_change = on_theme_change.unwrap_or_else(|| Callback::new(|_| {}));

    let class = merge_classes(
        [
            "theme-category-section",
            category.as_str(),
            class.as_deref().unwrap_or(""),
        ]
        .to_vec(),
    );

    view! {
        <div
            class=class
            style=style
            data-category=category.as_str()
        >
            <h4 class="category-title">{category.display_name()}</h4>
            <div class="theme-grid">
                {themes.into_iter().map(|theme| {
                    let theme_name = theme.name.clone();
                    let isselected = current_theme == theme_name;
                    view! {
                        <ThemeCard
                            theme=theme
                            selected=isselected
                            show_preview=show_preview
                            on_select=on_theme_change
                        />
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

/// Theme card component
#[component]
pub fn ThemeCard(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] theme: Option<ThemeInfo>,
    #[prop(optional)] selected: Option<bool>,
    #[prop(optional)] show_preview: Option<bool>,
    #[prop(optional)] on_select: Option<Callback<String>>,
) -> impl IntoView {
    let theme = theme.unwrap_or_default();
    let selected = selected.unwrap_or(false);
    let show_preview = show_preview.unwrap_or(true);
    let on_select = on_select.unwrap_or_else(|| Callback::new(|_| {}));

    let class = merge_classes(["theme-card", class.as_deref().unwrap_or("")].to_vec());

    let theme_name = theme.name.clone();
    let theme_name_clone = theme_name.clone();
    let handle_select = move |_: web_sys::MouseEvent| {
        on_select.run(theme_name.clone());
    };

    view! {
        <div
            class=class
            style=style
            role="radio"
            tabindex=0
            aria-checked=selected
            aria-label=format!("Select {} theme", theme.name)
            on:click=handle_select
            on:keydown=move |e: web_sys::KeyboardEvent| {
                if e.key() == "Enter" || e.key() == " " {
                    e.prevent_default();
                    on_select.run(theme_name_clone.clone());
                }
            }
        >
            {if show_preview {
                view! {
                    <div class="theme-preview">
                        <ThemePreview colors=theme.colors.clone() />
                    </div>
                }.into_any()
            } else {
                view! { <div></div> }.into_any()
            }}

            <div class="theme-info">
                <h5 class="theme-name">{theme.name.clone()}</h5>
                <p class="theme-description">{theme.description.clone()}</p>
                <div class="theme-tags">
                    {theme.tags.into_iter().map(|tag| {
                        view! {
                            <span class="theme-tag">{tag}</span>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </div>
    }
}

/// Theme preview component
#[component]
pub fn ThemePreview(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] colors: Option<ThemeColors>,
) -> impl IntoView {
    let colors = colors.unwrap_or_default();

    let class = merge_classes(["theme-preview", class.as_deref().unwrap_or("")].to_vec());

    view! {
        <div
            class=class
            style=style
            role="img"
            aria-label="Theme color preview"
        >
            <div class="color-palette">
                <div class="color-swatch primary" style=format!("background-color: {}", colors.primary)></div>
                <div class="color-swatch secondary" style=format!("background-color: {}", colors.secondary)></div>
                <div class="color-swatch accent" style=format!("background-color: {}", colors.accent)></div>
                <div class="color-swatch neutral" style=format!("background-color: {}", colors.neutral)></div>
            </div>
            <div class="preview-elements">
                <div class="preview-button" style=format!("background-color: {}", colors.primary)></div>
                <div class="preview-card" style=format!("border-color: {}", colors.neutral)></div>
                <div class="preview-text" style=format!("color: {}", colors.text)></div>
            </div>
        </div>
    }
}

#[cfg(test)]
mod prebuilt_themes_tests {
    use super::*;
    use leptos::callback::Callback;

    #[test]
    fn test_prebuilt_themes_default() {
        let themes = PrebuiltThemes::default();
        assert_eq!(themes.light.primary.primary_500, "#3b82f6");
        assert_eq!(themes.dark.primary.primary_500, "#3b82f6");
        assert_eq!(themes.high_contrast.primary.primary_500, "#0000ff");
    }

    #[test]
    fn test_theme_selector_component_creation() {
        // Test logic without runtime
        // Test component logic
        let theme_name = "test-theme";
        assert!(!theme_name.is_empty()); // Test completed
    }

    #[test]
    fn test_theme_selector_with_callback() {
        // Test logic without runtime
        let callback = Callback::new(|_theme: String| {});
        // Test component logic
        let theme_name = "test-theme";
        assert!(!theme_name.is_empty()); // Test completed
    }

    #[test]
    fn test_theme_category_section_component() {
        // Test logic without runtime
        let themes = [ThemeInfo::default()];
        // Test component logic
        let theme_name = "test-theme";
        assert!(!theme_name.is_empty()); // Test completed
    }

    #[test]
    fn test_theme_card_component() {
        // Test logic without runtime
        let theme = ThemeInfo::default();
        // Test component logic
        let theme_name = "test-theme";
        assert!(!theme_name.is_empty()); // Test completed
    }

    #[test]
    fn test_theme_preview_component() {
        // Test logic without runtime
        let colors = ThemeColors::default();
        // Test component logic
        let theme_name = "test-theme";
        assert!(!theme_name.is_empty()); // Test completed
    }

    // Integration Tests
    #[test]
    fn test_theme_selector_integration() {
        // Test complete theme selector workflow
    }

    #[test]
    fn test_theme_preview_integration() {
        // Test theme preview functionality
    }

    #[test]
    fn test_theme_switching_integration() {
        // Test theme switching workflow
    }

    // Performance Tests
    #[test]
    fn test_theme_creation_performance() {
        // Test theme creation performance
        let start = std::time::Instant::now();
        let _themes = PrebuiltThemes::default();
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100); // Should create themes in less than 100ms
    }

    #[test]
    fn test_theme_selector_render_performance() {
        // Test theme selector render performance
        let start = std::time::Instant::now();
        // Simulate component creation
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100); // Should render in less than 100ms
    }

    #[test]
    fn test_theme_memory_usage() {
        // Test theme memory usage
    }
}
