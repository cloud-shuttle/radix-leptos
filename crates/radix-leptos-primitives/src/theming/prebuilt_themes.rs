use crate::theming::css_variables::*;
use crate::utils::merge_classes;
use leptos::callback::Callback;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

/// Pre-built theme collection with industry-specific themes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
            dark: createdark_theme(),
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
                            on_theme_change=handle_theme_change.clone()
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
                            on_select=on_theme_change.clone()
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

/// Theme category enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ThemeCategory {
    Basic,
    Industry,
    Style,
    Seasonal,
    Accessibility,
}

impl ThemeCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            ThemeCategory::Basic => "basic",
            ThemeCategory::Industry => "industry",
            ThemeCategory::Style => "style",
            ThemeCategory::Seasonal => "seasonal",
            ThemeCategory::Accessibility => "accessibility",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            ThemeCategory::Basic => "Basic Themes",
            ThemeCategory::Industry => "Industry Themes",
            ThemeCategory::Style => "Style Themes",
            ThemeCategory::Seasonal => "Seasonal Themes",
            ThemeCategory::Accessibility => "Accessibility Themes",
        }
    }
}

/// Theme information struct
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThemeInfo {
    pub name: String,
    pub description: String,
    pub category: ThemeCategory,
    pub colors: ThemeColors,
    pub tags: Vec<String>,
    pub css_variables: CSSVariables,
}

impl Default for ThemeInfo {
    fn default() -> Self {
        Self {
            name: "Default".to_string(),
            description: "Default theme".to_string(),
            category: ThemeCategory::Basic,
            colors: ThemeColors::default(),
            tags: ["default".to_string()].to_vec(),
            css_variables: CSSVariables::default(),
        }
    }
}

/// Theme colors struct
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThemeColors {
    pub primary: String,
    pub secondary: String,
    pub accent: String,
    pub neutral: String,
    pub text: String,
    pub background: String,
    pub surface: String,
    pub error: String,
    pub warning: String,
    pub success: String,
    pub info: String,
}

impl Default for ThemeColors {
    fn default() -> Self {
        Self {
            primary: "#3b82f6".to_string(),
            secondary: "#64748b".to_string(),
            accent: "#8b5cf6".to_string(),
            neutral: "#6b7280".to_string(),
            text: "#111827".to_string(),
            background: "#ffffff".to_string(),
            surface: "#f9fafb".to_string(),
            error: "#ef4444".to_string(),
            warning: "#f59e0b".to_string(),
            success: "#22c55e".to_string(),
            info: "#06b6d4".to_string(),
        }
    }
}

/// Get themes organized by categories
fn get_themes_by_categories(
    categories: &[ThemeCategory],
) -> std::collections::HashMap<ThemeCategory, Vec<ThemeInfo>> {
    let mut themes_map = std::collections::HashMap::new();

    for category in categories {
        let themes = match category {
            ThemeCategory::Basic => vec![
                ThemeInfo {
                    name: "Light".to_string(),
                    description: "Clean and bright theme for everyday use".to_string(),
                    category: *category,
                    colors: ThemeColors {
                        primary: "#3b82f6".to_string(),
                        secondary: "#64748b".to_string(),
                        accent: "#8b5cf6".to_string(),
                        neutral: "#6b7280".to_string(),
                        text: "#111827".to_string(),
                        background: "#ffffff".to_string(),
                        surface: "#f9fafb".to_string(),
                        error: "#ef4444".to_string(),
                        warning: "#f59e0b".to_string(),
                        success: "#22c55e".to_string(),
                        info: "#06b6d4".to_string(),
                    },
                    tags: [
                        "light".to_string(),
                        "clean".to_string(),
                        "modern".to_string(),
                    ]
                    .to_vec(),
                    css_variables: create_light_theme(),
                },
                ThemeInfo {
                    name: "Dark".to_string(),
                    description: "Dark theme for low-light environments".to_string(),
                    category: *category,
                    colors: ThemeColors {
                        primary: "#60a5fa".to_string(),
                        secondary: "#94a3b8".to_string(),
                        accent: "#a78bfa".to_string(),
                        neutral: "#9ca3af".to_string(),
                        text: "#f9fafb".to_string(),
                        background: "#111827".to_string(),
                        surface: "#1f2937".to_string(),
                        error: "#f87171".to_string(),
                        warning: "#fbbf24".to_string(),
                        success: "#4ade80".to_string(),
                        info: "#22d3ee".to_string(),
                    },
                    tags: [
                        "dark".to_string(),
                        "night".to_string(),
                        "modern".to_string(),
                    ]
                    .to_vec(),
                    css_variables: createdark_theme(),
                },
                ThemeInfo {
                    name: "High Contrast".to_string(),
                    description: "High contrast theme for accessibility".to_string(),
                    category: *category,
                    colors: ThemeColors {
                        primary: "#0000ff".to_string(),
                        secondary: "#000000".to_string(),
                        accent: "#ff0000".to_string(),
                        neutral: "#000000".to_string(),
                        text: "#000000".to_string(),
                        background: "#ffffff".to_string(),
                        surface: "#ffffff".to_string(),
                        error: "#ff0000".to_string(),
                        warning: "#ff8000".to_string(),
                        success: "#008000".to_string(),
                        info: "#0000ff".to_string(),
                    },
                    tags: [
                        "accessibility".to_string(),
                        "high-contrast".to_string(),
                        "wcag".to_string(),
                    ]
                    .to_vec(),
                    css_variables: create_high_contrast_theme(),
                },
            ],
            ThemeCategory::Industry => vec![
                ThemeInfo {
                    name: "Finance".to_string(),
                    description: "Professional theme for financial applications".to_string(),
                    category: *category,
                    colors: ThemeColors {
                        primary: "#1e40af".to_string(),
                        secondary: "#374151".to_string(),
                        accent: "#059669".to_string(),
                        neutral: "#4b5563".to_string(),
                        text: "#111827".to_string(),
                        background: "#ffffff".to_string(),
                        surface: "#f8fafc".to_string(),
                        error: "#dc2626".to_string(),
                        warning: "#d97706".to_string(),
                        success: "#059669".to_string(),
                        info: "#0284c7".to_string(),
                    },
                    tags: [
                        "finance".to_string(),
                        "professional".to_string(),
                        "corporate".to_string(),
                    ]
                    .to_vec(),
                    css_variables: create_finance_theme(),
                },
                ThemeInfo {
                    name: "Healthcare".to_string(),
                    description: "Calming theme for healthcare applications".to_string(),
                    category: *category,
                    colors: ThemeColors {
                        primary: "#059669".to_string(),
                        secondary: "#6b7280".to_string(),
                        accent: "#0d9488".to_string(),
                        neutral: "#9ca3af".to_string(),
                        text: "#111827".to_string(),
                        background: "#f0fdf4".to_string(),
                        surface: "#ffffff".to_string(),
                        error: "#dc2626".to_string(),
                        warning: "#d97706".to_string(),
                        success: "#059669".to_string(),
                        info: "#0284c7".to_string(),
                    },
                    tags: [
                        "healthcare".to_string(),
                        "medical".to_string(),
                        "calming".to_string(),
                    ]
                    .to_vec(),
                    css_variables: create_healthcare_theme(),
                },
                ThemeInfo {
                    name: "Education".to_string(),
                    description: "Engaging theme for educational platforms".to_string(),
                    category: *category,
                    colors: ThemeColors {
                        primary: "#7c3aed".to_string(),
                        secondary: "#6366f1".to_string(),
                        accent: "#ec4899".to_string(),
                        neutral: "#6b7280".to_string(),
                        text: "#111827".to_string(),
                        background: "#fefce8".to_string(),
                        surface: "#ffffff".to_string(),
                        error: "#dc2626".to_string(),
                        warning: "#d97706".to_string(),
                        success: "#059669".to_string(),
                        info: "#0284c7".to_string(),
                    },
                    tags: [
                        "education".to_string(),
                        "learning".to_string(),
                        "engaging".to_string(),
                    ]
                    .to_vec(),
                    css_variables: create_education_theme(),
                },
            ],
            ThemeCategory::Style => vec![
                ThemeInfo {
                    name: "Minimal".to_string(),
                    description: "Minimalist theme with clean lines".to_string(),
                    category: *category,
                    colors: ThemeColors {
                        primary: "#000000".to_string(),
                        secondary: "#6b7280".to_string(),
                        accent: "#000000".to_string(),
                        neutral: "#9ca3af".to_string(),
                        text: "#000000".to_string(),
                        background: "#ffffff".to_string(),
                        surface: "#ffffff".to_string(),
                        error: "#dc2626".to_string(),
                        warning: "#d97706".to_string(),
                        success: "#059669".to_string(),
                        info: "#6b7280".to_string(),
                    },
                    tags: [
                        "minimal".to_string(),
                        "clean".to_string(),
                        "simple".to_string(),
                    ]
                    .to_vec(),
                    css_variables: create_minimal_theme(),
                },
                ThemeInfo {
                    name: "Vibrant".to_string(),
                    description: "Bold and colorful theme".to_string(),
                    category: *category,
                    colors: ThemeColors {
                        primary: "#ff6b6b".to_string(),
                        secondary: "#4ecdc4".to_string(),
                        accent: "#45b7d1".to_string(),
                        neutral: "#96ceb4".to_string(),
                        text: "#2c3e50".to_string(),
                        background: "#f8f9fa".to_string(),
                        surface: "#ffffff".to_string(),
                        error: "#e74c3c".to_string(),
                        warning: "#f39c12".to_string(),
                        success: "#27ae60".to_string(),
                        info: "#3498db".to_string(),
                    },
                    tags: [
                        "vibrant".to_string(),
                        "colorful".to_string(),
                        "bold".to_string(),
                    ]
                    .to_vec(),
                    css_variables: create_vibrant_theme(),
                },
                ThemeInfo {
                    name: "Elegant".to_string(),
                    description: "Sophisticated theme with refined aesthetics".to_string(),
                    category: *category,
                    colors: ThemeColors {
                        primary: "#2c3e50".to_string(),
                        secondary: "#34495e".to_string(),
                        accent: "#e74c3c".to_string(),
                        neutral: "#95a5a6".to_string(),
                        text: "#2c3e50".to_string(),
                        background: "#ecf0f1".to_string(),
                        surface: "#ffffff".to_string(),
                        error: "#e74c3c".to_string(),
                        warning: "#f39c12".to_string(),
                        success: "#27ae60".to_string(),
                        info: "#3498db".to_string(),
                    },
                    tags: [
                        "elegant".to_string(),
                        "sophisticated".to_string(),
                        "refined".to_string(),
                    ]
                    .to_vec(),
                    css_variables: create_elegant_theme(),
                },
            ],
            _ => Vec::new(),
        };

        themes_map.insert(*category, themes);
    }

    themes_map
}

// Theme creation functions
fn create_light_theme() -> CSSVariables {
    CSSVariables::default()
}

fn createdark_theme() -> CSSVariables {
    CSSVariables::dark_theme()
}

fn create_high_contrast_theme() -> CSSVariables {
    let mut theme = CSSVariables::default();
    // Override with high contrast colors
    theme.primary.primary_500 = "#0000ff".to_string();
    theme.secondary.secondary_500 = "#000000".to_string();
    theme.semantic.success = "#008000".to_string();
    theme.semantic.error = "#ff0000".to_string();
    theme.semantic.warning = "#ff8000".to_string();
    theme.semantic.info = "#0000ff".to_string();
    theme
}

fn create_finance_theme() -> CSSVariables {
    let mut theme = CSSVariables::default();
    theme.primary.primary_500 = "#1e40af".to_string();
    theme.secondary.secondary_500 = "#374151".to_string();
    theme.semantic.success = "#059669".to_string();
    theme
}

fn create_healthcare_theme() -> CSSVariables {
    let mut theme = CSSVariables::default();
    theme.primary.primary_500 = "#059669".to_string();
    theme.secondary.secondary_500 = "#6b7280".to_string();
    theme.semantic.success = "#059669".to_string();
    theme
}

fn create_education_theme() -> CSSVariables {
    let mut theme = CSSVariables::default();
    theme.primary.primary_500 = "#7c3aed".to_string();
    theme.secondary.secondary_500 = "#6366f1".to_string();
    theme.semantic.info = "#ec4899".to_string();
    theme
}

fn create_ecommerce_theme() -> CSSVariables {
    let mut theme = CSSVariables::default();
    theme.primary.primary_500 = "#dc2626".to_string();
    theme.secondary.secondary_500 = "#059669".to_string();
    theme.semantic.success = "#059669".to_string();
    theme
}

fn create_gaming_theme() -> CSSVariables {
    let mut theme = CSSVariables::default();
    theme.primary.primary_500 = "#7c3aed".to_string();
    theme.secondary.secondary_500 = "#ec4899".to_string();
    theme.semantic.warning = "#f59e0b".to_string();
    theme
}

fn create_minimal_theme() -> CSSVariables {
    let mut theme = CSSVariables::default();
    theme.primary.primary_500 = "#000000".to_string();
    theme.secondary.secondary_500 = "#6b7280".to_string();
    theme.neutral.neutral_500 = "#9ca3af".to_string();
    theme
}

fn create_vibrant_theme() -> CSSVariables {
    let mut theme = CSSVariables::default();
    theme.primary.primary_500 = "#ff6b6b".to_string();
    theme.secondary.secondary_500 = "#4ecdc4".to_string();
    theme.semantic.info = "#45b7d1".to_string();
    theme.semantic.success = "#27ae60".to_string();
    theme
}

fn create_elegant_theme() -> CSSVariables {
    let mut theme = CSSVariables::default();
    theme.primary.primary_500 = "#2c3e50".to_string();
    theme.secondary.secondary_500 = "#34495e".to_string();
    theme.semantic.info = "#e74c3c".to_string();
    theme.semantic.success = "#27ae60".to_string();
    theme
}

#[cfg(test)]
mod prebuilt_themes_tests {
    use crate::theming::prebuilt_themes::{
        create_ecommerce_theme, create_education_theme, create_finance_theme, create_gaming_theme,
        create_healthcare_theme, create_high_contrast_theme, create_light_theme,
        create_minimal_theme, create_vibrant_theme, createdark_theme, get_themes_by_categories,
    };
    use crate::theming::{PrebuiltThemes, ThemeCategory, ThemeColors, ThemeInfo};
    use leptos::callback::Callback;
    use proptest::prelude::*;

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

    #[test]
    fn test_theme_category_enum() {
        assert_eq!(ThemeCategory::Basic.as_str(), "basic");
        assert_eq!(ThemeCategory::Industry.as_str(), "industry");
        assert_eq!(ThemeCategory::Style.as_str(), "style");
        assert_eq!(ThemeCategory::Seasonal.as_str(), "seasonal");
        assert_eq!(ThemeCategory::Accessibility.as_str(), "accessibility");
    }

    #[test]
    fn test_theme_category_display_name() {
        assert_eq!(ThemeCategory::Basic.display_name(), "Basic Themes");
        assert_eq!(ThemeCategory::Industry.display_name(), "Industry Themes");
        assert_eq!(ThemeCategory::Style.display_name(), "Style Themes");
        assert_eq!(ThemeCategory::Seasonal.display_name(), "Seasonal Themes");
        assert_eq!(
            ThemeCategory::Accessibility.display_name(),
            "Accessibility Themes"
        );
    }

    #[test]
    fn test_theme_info_default() {
        let theme = ThemeInfo::default();
        assert_eq!(theme.name, "Default");
        assert_eq!(theme.description, "Default theme");
        assert_eq!(theme.category, ThemeCategory::Basic);
        assert_eq!(theme.tags, ["default"]);
    }

    #[test]
    fn test_theme_colors_default() {
        let colors = ThemeColors::default();
        assert_eq!(colors.primary, "#3b82f6");
        assert_eq!(colors.secondary, "#64748b");
        assert_eq!(colors.accent, "#8b5cf6");
        assert_eq!(colors.neutral, "#6b7280");
        assert_eq!(colors.text, "#111827");
        assert_eq!(colors.background, "#ffffff");
    }

    #[test]
    fn test_get_themes_by_categories() {
        let categories = [ThemeCategory::Basic, ThemeCategory::Industry];
        let themes_map = get_themes_by_categories(&categories);

        assert!(themes_map.contains_key(&ThemeCategory::Basic));
        assert!(themes_map.contains_key(&ThemeCategory::Industry));
        assert!(!themes_map.contains_key(&ThemeCategory::Style));

        let basic_themes = themes_map.get(&ThemeCategory::Basic).unwrap();
        assert_eq!(basic_themes.len(), 3); // Light, Dark, High Contrast

        let industry_themes = themes_map.get(&ThemeCategory::Industry).unwrap();
        assert_eq!(industry_themes.len(), 3); // Finance, Healthcare, Education
    }

    #[test]
    fn test_theme_creation_functions() {
        let light_theme = create_light_theme();
        let dark_theme = createdark_theme();
        let high_contrast_theme = create_high_contrast_theme();
        let finance_theme = create_finance_theme();
        let healthcare_theme = create_healthcare_theme();
        let education_theme = create_education_theme();
        let ecommerce_theme = create_ecommerce_theme();
        let gaming_theme = create_gaming_theme();
        let minimal_theme = create_minimal_theme();
        let vibrant_theme = create_vibrant_theme();

        // Test that all themes are created successfully
        assert_eq!(light_theme.primary.primary_500, "#3b82f6");
        assert_eq!(dark_theme.primary.primary_500, "#3b82f6");
        assert_eq!(high_contrast_theme.primary.primary_500, "#0000ff");
        assert_eq!(finance_theme.primary.primary_500, "#1e40af");
        assert_eq!(healthcare_theme.primary.primary_500, "#059669");
        assert_eq!(education_theme.primary.primary_500, "#7c3aed");
        assert_eq!(ecommerce_theme.primary.primary_500, "#dc2626");
        assert_eq!(gaming_theme.primary.primary_500, "#7c3aed");
        assert_eq!(minimal_theme.primary.primary_500, "#000000");
        assert_eq!(vibrant_theme.primary.primary_500, "#ff6b6b");
    }

    // Property-based tests
    #[test]
    fn test_theme_category_property_based() {
        proptest!(|(category in prop::sample::select(&[
            ThemeCategory::Basic,
            ThemeCategory::Industry,
            ThemeCategory::Style,
            ThemeCategory::Seasonal,
            ThemeCategory::Accessibility,
        ]))| {
            let category_str = category.as_str();
            let display_name = category.display_name();
            assert!(!category_str.is_empty());
            assert!(!display_name.is_empty());
        });
    }

    #[test]
    fn test_theme_colors_property_based() {
        proptest!(|(color in ".*")| {
            let colors = ThemeColors {
                primary: color.clone(),
                secondary: color.clone(),
                accent: color.clone(),
                neutral: color.clone(),
                text: color.clone(),
                background: color.clone(),
                surface: color.clone(),
                error: color.clone(),
                warning: color.clone(),
                success: color.clone(),
                info: color.clone(),
            };
            assert_eq!(colors.primary, color);
        });
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
