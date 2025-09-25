use crate::theming::css_variables::CSSVariables;
use super::color_schemes::{ThemeCategory, ThemeColors, ThemeInfo};

// Theme creation functions
pub fn create_light_theme() -> CSSVariables {
    CSSVariables::default()
}

pub fn createdark_theme() -> CSSVariables {
    CSSVariables::dark_theme()
}

pub fn create_high_contrast_theme() -> CSSVariables {
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

pub fn create_finance_theme() -> CSSVariables {
    let mut theme = CSSVariables::default();
    theme.primary.primary_500 = "#1e40af".to_string();
    theme.secondary.secondary_500 = "#374151".to_string();
    theme.semantic.success = "#059669".to_string();
    theme
}

pub fn create_healthcare_theme() -> CSSVariables {
    let mut theme = CSSVariables::default();
    theme.primary.primary_500 = "#059669".to_string();
    theme.secondary.secondary_500 = "#6b7280".to_string();
    theme.semantic.success = "#059669".to_string();
    theme
}

pub fn create_education_theme() -> CSSVariables {
    let mut theme = CSSVariables::default();
    theme.primary.primary_500 = "#7c3aed".to_string();
    theme.secondary.secondary_500 = "#6366f1".to_string();
    theme.semantic.info = "#ec4899".to_string();
    theme
}

pub fn create_ecommerce_theme() -> CSSVariables {
    let mut theme = CSSVariables::default();
    theme.primary.primary_500 = "#dc2626".to_string();
    theme.secondary.secondary_500 = "#059669".to_string();
    theme.semantic.success = "#059669".to_string();
    theme
}

pub fn create_gaming_theme() -> CSSVariables {
    let mut theme = CSSVariables::default();
    theme.primary.primary_500 = "#7c3aed".to_string();
    theme.secondary.secondary_500 = "#ec4899".to_string();
    theme.semantic.warning = "#f59e0b".to_string();
    theme
}

pub fn create_minimal_theme() -> CSSVariables {
    let mut theme = CSSVariables::default();
    theme.primary.primary_500 = "#000000".to_string();
    theme.secondary.secondary_500 = "#6b7280".to_string();
    theme.neutral.neutral_500 = "#9ca3af".to_string();
    theme
}

pub fn create_vibrant_theme() -> CSSVariables {
    let mut theme = CSSVariables::default();
    theme.primary.primary_500 = "#ff6b6b".to_string();
    theme.secondary.secondary_500 = "#4ecdc4".to_string();
    theme.semantic.info = "#45b7d1".to_string();
    theme.semantic.success = "#27ae60".to_string();
    theme
}

pub fn create_elegant_theme() -> CSSVariables {
    let mut theme = CSSVariables::default();
    theme.primary.primary_500 = "#2c3e50".to_string();
    theme.secondary.secondary_500 = "#34495e".to_string();
    theme.semantic.info = "#e74c3c".to_string();
    theme.semantic.success = "#27ae60".to_string();
    theme
}

/// Get themes organized by categories
pub fn get_themes_by_categories(
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

#[cfg(test)]
mod theme_builder_tests {
    use super::*;

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
    fn test_theme_category_property_based() {
        use proptest::prelude::*;
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
}
