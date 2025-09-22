use serde::{Deserialize, Serialize};

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
    pub css_variables: crate::theming::css_variables::CSSVariables,
}

impl Default for ThemeInfo {
    fn default() -> Self {
        Self {
            name: "Default".to_string(),
            description: "Default theme".to_string(),
            category: ThemeCategory::Basic,
            colors: ThemeColors::default(),
            tags: ["default".to_string()].to_vec(),
            css_variables: crate::theming::css_variables::CSSVariables::default(),
        }
    }
}

#[cfg(test)]
mod color_schemes_tests {
    use super::*;

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
    fn test_theme_colors_property_based() {
        use proptest::prelude::*;
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
}
