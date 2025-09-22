use super::color_schemes::{ThemeCategory, ThemeColors, ThemeInfo};
use super::theme_builder::{create_dark_theme, create_ecommerce_theme, create_gaming_theme, create_minimal_theme, create_vibrant_theme, create_elegant_theme};

/// Dark theme variants and configurations
pub struct DarkThemes;

impl DarkThemes {
    /// Get all dark theme variants
    pub fn get_dark_themes() -> Vec<ThemeInfo> {
        vec![
            ThemeInfo {
                name: "Dark".to_string(),
                description: "Dark theme for low-light environments".to_string(),
                category: ThemeCategory::Basic,
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
                css_variables: create_dark_theme(),
            },
            ThemeInfo {
                name: "Ecommerce".to_string(),
                description: "E-commerce focused dark theme".to_string(),
                category: ThemeCategory::Industry,
                colors: ThemeColors {
                    primary: "#dc2626".to_string(),
                    secondary: "#059669".to_string(),
                    accent: "#f59e0b".to_string(),
                    neutral: "#6b7280".to_string(),
                    text: "#f9fafb".to_string(),
                    background: "#0f172a".to_string(),
                    surface: "#1e293b".to_string(),
                    error: "#f87171".to_string(),
                    warning: "#fbbf24".to_string(),
                    success: "#4ade80".to_string(),
                    info: "#22d3ee".to_string(),
                },
                tags: [
                    "ecommerce".to_string(),
                    "shopping".to_string(),
                    "retail".to_string(),
                ]
                .to_vec(),
                css_variables: create_ecommerce_theme(),
            },
            ThemeInfo {
                name: "Gaming".to_string(),
                description: "Gaming-focused dark theme with vibrant accents".to_string(),
                category: ThemeCategory::Industry,
                colors: ThemeColors {
                    primary: "#7c3aed".to_string(),
                    secondary: "#ec4899".to_string(),
                    accent: "#f59e0b".to_string(),
                    neutral: "#6b7280".to_string(),
                    text: "#f9fafb".to_string(),
                    background: "#0f172a".to_string(),
                    surface: "#1e293b".to_string(),
                    error: "#f87171".to_string(),
                    warning: "#fbbf24".to_string(),
                    success: "#4ade80".to_string(),
                    info: "#22d3ee".to_string(),
                },
                tags: [
                    "gaming".to_string(),
                    "vibrant".to_string(),
                    "entertainment".to_string(),
                ]
                .to_vec(),
                css_variables: create_gaming_theme(),
            },
            ThemeInfo {
                name: "Minimal Dark".to_string(),
                description: "Minimalist dark theme with clean lines".to_string(),
                category: ThemeCategory::Style,
                colors: ThemeColors {
                    primary: "#ffffff".to_string(),
                    secondary: "#6b7280".to_string(),
                    accent: "#ffffff".to_string(),
                    neutral: "#9ca3af".to_string(),
                    text: "#ffffff".to_string(),
                    background: "#000000".to_string(),
                    surface: "#111827".to_string(),
                    error: "#f87171".to_string(),
                    warning: "#fbbf24".to_string(),
                    success: "#4ade80".to_string(),
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
                name: "Vibrant Dark".to_string(),
                description: "Bold and colorful dark theme".to_string(),
                category: ThemeCategory::Style,
                colors: ThemeColors {
                    primary: "#ff6b6b".to_string(),
                    secondary: "#4ecdc4".to_string(),
                    accent: "#45b7d1".to_string(),
                    neutral: "#96ceb4".to_string(),
                    text: "#f9fafb".to_string(),
                    background: "#1a1a1a".to_string(),
                    surface: "#2d2d2d".to_string(),
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
                name: "Elegant Dark".to_string(),
                description: "Sophisticated dark theme with refined aesthetics".to_string(),
                category: ThemeCategory::Style,
                colors: ThemeColors {
                    primary: "#e74c3c".to_string(),
                    secondary: "#34495e".to_string(),
                    accent: "#e74c3c".to_string(),
                    neutral: "#95a5a6".to_string(),
                    text: "#ecf0f1".to_string(),
                    background: "#2c3e50".to_string(),
                    surface: "#34495e".to_string(),
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
        ]
    }

    /// Get dark themes by category
    pub fn get_dark_themes_by_category(category: ThemeCategory) -> Vec<ThemeInfo> {
        Self::get_dark_themes()
            .into_iter()
            .filter(|theme| theme.category == category)
            .collect()
    }

    /// Check if a theme is a dark theme
    pub fn is_dark_theme(theme_name: &str) -> bool {
        let dark_themes = Self::get_dark_themes();
        dark_themes.iter().any(|theme| theme.name == theme_name)
    }

    /// Get all dark theme names
    pub fn get_dark_theme_names() -> Vec<String> {
        Self::get_dark_themes()
            .into_iter()
            .map(|theme| theme.name)
            .collect()
    }
}

#[cfg(test)]
mod dark_themes_tests {
    use super::*;

    #[test]
    fn test_get_dark_themes() {
        let themes = DarkThemes::get_dark_themes();
        assert!(!themes.is_empty());
        
        // Check that all themes have dark backgrounds
        for theme in &themes {
            assert!(theme.colors.background.contains("111") || 
                   theme.colors.background.contains("000") || 
                   theme.colors.background.contains("1a1") ||
                   theme.colors.background.contains("2c3") ||
                   theme.colors.background.contains("0f1"));
        }
    }

    #[test]
    fn test_get_dark_themes_by_category() {
        let basic_themes = DarkThemes::get_dark_themes_by_category(ThemeCategory::Basic);
        let industry_themes = DarkThemes::get_dark_themes_by_category(ThemeCategory::Industry);
        let style_themes = DarkThemes::get_dark_themes_by_category(ThemeCategory::Style);
        
        assert!(!basic_themes.is_empty());
        assert!(!industry_themes.is_empty());
        assert!(!style_themes.is_empty());
        
        // Verify categories
        for theme in &basic_themes {
            assert_eq!(theme.category, ThemeCategory::Basic);
        }
        for theme in &industry_themes {
            assert_eq!(theme.category, ThemeCategory::Industry);
        }
        for theme in &style_themes {
            assert_eq!(theme.category, ThemeCategory::Style);
        }
    }

    #[test]
    fn test_is_dark_theme() {
        assert!(DarkThemes::is_dark_theme("Dark"));
        assert!(DarkThemes::is_dark_theme("Gaming"));
        assert!(DarkThemes::is_dark_theme("Elegant Dark"));
        assert!(!DarkThemes::is_dark_theme("Light"));
        assert!(!DarkThemes::is_dark_theme("Nonexistent"));
    }

    #[test]
    fn test_get_dark_theme_names() {
        let names = DarkThemes::get_dark_theme_names();
        assert!(!names.is_empty());
        assert!(names.contains(&"Dark".to_string()));
        assert!(names.contains(&"Gaming".to_string()));
    }

    #[test]
    fn test_dark_theme_properties() {
        let themes = DarkThemes::get_dark_themes();
        
        for theme in &themes {
            // Dark themes should have dark backgrounds
            assert!(theme.colors.background.contains("111") || 
                   theme.colors.background.contains("000") || 
                   theme.colors.background.contains("1a1") ||
                   theme.colors.background.contains("2c3") ||
                   theme.colors.background.contains("0f1"));
            
            // Should have light text for contrast
            assert!(theme.colors.text.contains("f9f") || 
                   theme.colors.text.contains("fff") ||
                   theme.colors.text.contains("ecf"));
        }
    }
}
