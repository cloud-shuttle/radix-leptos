use super::color_schemes::{ThemeCategory, ThemeColors, ThemeInfo};
use super::theme_builder::{create_light_theme, create_high_contrast_theme, create_finance_theme, create_healthcare_theme, create_education_theme};

/// Light theme variants and configurations
pub struct LightThemes;

impl LightThemes {
    /// Get all light theme variants
    pub fn get_light_themes() -> Vec<ThemeInfo> {
        vec![
            ThemeInfo {
                name: "Light".to_string(),
                description: "Clean and bright theme for everyday use".to_string(),
                category: ThemeCategory::Basic,
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
                name: "High Contrast".to_string(),
                description: "High contrast theme for accessibility".to_string(),
                category: ThemeCategory::Accessibility,
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
            ThemeInfo {
                name: "Finance".to_string(),
                description: "Professional theme for financial applications".to_string(),
                category: ThemeCategory::Industry,
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
                category: ThemeCategory::Industry,
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
                category: ThemeCategory::Industry,
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
        ]
    }

    /// Get light themes by category
    pub fn get_light_themes_by_category(category: ThemeCategory) -> Vec<ThemeInfo> {
        Self::get_light_themes()
            .into_iter()
            .filter(|theme| theme.category == category)
            .collect()
    }

    /// Check if a theme is a light theme
    pub fn is_light_theme(theme_name: &str) -> bool {
        let light_themes = Self::get_light_themes();
        light_themes.iter().any(|theme| theme.name == theme_name)
    }
}

#[cfg(test)]
mod light_themes_tests {
    use super::*;

    #[test]
    fn test_get_light_themes() {
        let themes = LightThemes::get_light_themes();
        assert!(!themes.is_empty());
        
        // Check that all themes have light backgrounds
        for theme in &themes {
            assert!(theme.colors.background.contains("fff") || theme.colors.background.contains("f0f") || theme.colors.background.contains("fef"));
        }
    }

    #[test]
    fn test_get_light_themes_by_category() {
        let basic_themes = LightThemes::get_light_themes_by_category(ThemeCategory::Basic);
        let industry_themes = LightThemes::get_light_themes_by_category(ThemeCategory::Industry);
        
        assert!(!basic_themes.is_empty());
        assert!(!industry_themes.is_empty());
        
        // Verify categories
        for theme in &basic_themes {
            assert_eq!(theme.category, ThemeCategory::Basic);
        }
        for theme in &industry_themes {
            assert_eq!(theme.category, ThemeCategory::Industry);
        }
    }

    #[test]
    fn test_is_light_theme() {
        assert!(LightThemes::is_light_theme("Light"));
        assert!(LightThemes::is_light_theme("Finance"));
        assert!(LightThemes::is_light_theme("Healthcare"));
        assert!(!LightThemes::is_light_theme("Dark"));
        assert!(!LightThemes::is_light_theme("Nonexistent"));
    }

    #[test]
    fn test_light_theme_properties() {
        let themes = LightThemes::get_light_themes();
        
        for theme in &themes {
            // Light themes should have light backgrounds
            assert!(theme.colors.background.contains("fff") || 
                   theme.colors.background.contains("f0f") || 
                   theme.colors.background.contains("fef"));
            
            // Should have dark text for contrast
            assert!(theme.colors.text.contains("111") || 
                   theme.colors.text.contains("000"));
        }
    }
}
