#[cfg(test)]
mod tests {
    use super::*;
    use crate::theming::{CSSVariables, PrebuiltThemes};
    use leptos::serde_json;

    #[test]
    fn test_prebuilt_themes_creation() {
        let themes = PrebuiltThemes::default();
        
        // Test that all themes are created
        assert_eq!(themes.light.primary.primary_500, "#3b82f6");
        assert_eq!(themes.dark.neutral.neutral_50, "#0a0a0a");
        assert_eq!(themes.high_contrast.primary.primary_500, "#0000ff");
        assert_eq!(themes.finance.primary.primary_500, "#1e40af");
        assert_eq!(themes.healthcare.primary.primary_500, "#059669");
        assert_eq!(themes.education.primary.primary_500, "#7c3aed");
        assert_eq!(themes.ecommerce.primary.primary_500, "#dc2626");
        assert_eq!(themes.gaming.primary.primary_500, "#7c2d12");
        assert_eq!(themes.minimal.primary.primary_500, "#374151");
        assert_eq!(themes.vibrant.primary.primary_500, "#ec4899");
    }

    #[test]
    fn test_prebuilt_themes_light_theme() {
        let themes = PrebuiltThemes::default();
        let light_theme = &themes.light;
        
        // Test light theme characteristics
        assert_eq!(light_theme.primary.primary_500, "#3b82f6");
        assert_eq!(light_theme.secondary.secondary_500, "#64748b");
        assert_eq!(light_theme.semantic.success, "#10b981");
        assert_eq!(light_theme.semantic.warning, "#f59e0b");
        assert_eq!(light_theme.semantic.error, "#ef4444");
        assert_eq!(light_theme.semantic.info, "#3b82f6");
    }

    #[test]
    fn test_prebuilt_themes_dark_theme() {
        let themes = PrebuiltThemes::default();
        let dark_theme = &themes.dark;
        
        // Test dark theme characteristics
        assert_eq!(dark_theme.neutral.neutral_50, "#0a0a0a");
        assert_eq!(dark_theme.neutral.neutral_950, "#fafafa");
        assert_eq!(dark_theme.primary.primary_500, "#3b82f6");
        assert_eq!(dark_theme.secondary.secondary_500, "#64748b");
    }

    #[test]
    fn test_prebuilt_themes_high_contrast_theme() {
        let themes = PrebuiltThemes::default();
        let high_contrast_theme = &themes.high_contrast;
        
        // Test high contrast theme characteristics
        assert_eq!(high_contrast_theme.primary.primary_500, "#0000ff");
        assert_eq!(high_contrast_theme.secondary.secondary_500, "#000000");
        assert_eq!(high_contrast_theme.semantic.success, "#00ff00");
        assert_eq!(high_contrast_theme.semantic.warning, "#ffff00");
        assert_eq!(high_contrast_theme.semantic.error, "#ff0000");
        assert_eq!(high_contrast_theme.semantic.info, "#0000ff");
    }

    #[test]
    fn test_prebuilt_themes_finance_theme() {
        let themes = PrebuiltThemes::default();
        let finance_theme = &themes.finance;
        
        // Test finance theme characteristics
        assert_eq!(finance_theme.primary.primary_500, "#1e40af");
        assert_eq!(finance_theme.secondary.secondary_500, "#374151");
        assert_eq!(finance_theme.semantic.success, "#059669");
        assert_eq!(finance_theme.semantic.warning, "#d97706");
        assert_eq!(finance_theme.semantic.error, "#dc2626");
        assert_eq!(finance_theme.semantic.info, "#0284c7");
    }

    #[test]
    fn test_prebuilt_themes_healthcare_theme() {
        let themes = PrebuiltThemes::default();
        let healthcare_theme = &themes.healthcare;
        
        // Test healthcare theme characteristics
        assert_eq!(healthcare_theme.primary.primary_500, "#059669");
        assert_eq!(healthcare_theme.secondary.secondary_500, "#374151");
        assert_eq!(healthcare_theme.semantic.success, "#10b981");
        assert_eq!(healthcare_theme.semantic.warning, "#f59e0b");
        assert_eq!(healthcare_theme.semantic.error, "#ef4444");
        assert_eq!(healthcare_theme.semantic.info, "#06b6d4");
    }

    #[test]
    fn test_prebuilt_themes_education_theme() {
        let themes = PrebuiltThemes::default();
        let education_theme = &themes.education;
        
        // Test education theme characteristics
        assert_eq!(education_theme.primary.primary_500, "#7c3aed");
        assert_eq!(education_theme.secondary.secondary_500, "#374151");
        assert_eq!(education_theme.semantic.success, "#10b981");
        assert_eq!(education_theme.semantic.warning, "#f59e0b");
        assert_eq!(education_theme.semantic.error, "#ef4444");
        assert_eq!(education_theme.semantic.info, "#3b82f6");
    }

    #[test]
    fn test_prebuilt_themes_ecommerce_theme() {
        let themes = PrebuiltThemes::default();
        let ecommerce_theme = &themes.ecommerce;
        
        // Test ecommerce theme characteristics
        assert_eq!(ecommerce_theme.primary.primary_500, "#dc2626");
        assert_eq!(ecommerce_theme.secondary.secondary_500, "#374151");
        assert_eq!(ecommerce_theme.semantic.success, "#10b981");
        assert_eq!(ecommerce_theme.semantic.warning, "#f59e0b");
        assert_eq!(ecommerce_theme.semantic.error, "#ef4444");
        assert_eq!(ecommerce_theme.semantic.info, "#3b82f6");
    }

    #[test]
    fn test_prebuilt_themes_gaming_theme() {
        let themes = PrebuiltThemes::default();
        let gaming_theme = &themes.gaming;
        
        // Test gaming theme characteristics
        assert_eq!(gaming_theme.primary.primary_500, "#7c2d12");
        assert_eq!(gaming_theme.secondary.secondary_500, "#374151");
        assert_eq!(gaming_theme.semantic.success, "#10b981");
        assert_eq!(gaming_theme.semantic.warning, "#f59e0b");
        assert_eq!(gaming_theme.semantic.error, "#ef4444");
        assert_eq!(gaming_theme.semantic.info, "#3b82f6");
    }

    #[test]
    fn test_prebuilt_themes_minimal_theme() {
        let themes = PrebuiltThemes::default();
        let minimal_theme = &themes.minimal;
        
        // Test minimal theme characteristics
        assert_eq!(minimal_theme.primary.primary_500, "#374151");
        assert_eq!(minimal_theme.secondary.secondary_500, "#6b7280");
        assert_eq!(minimal_theme.semantic.success, "#10b981");
        assert_eq!(minimal_theme.semantic.warning, "#f59e0b");
        assert_eq!(minimal_theme.semantic.error, "#ef4444");
        assert_eq!(minimal_theme.semantic.info, "#3b82f6");
    }

    #[test]
    fn test_prebuilt_themes_vibrant_theme() {
        let themes = PrebuiltThemes::default();
        let vibrant_theme = &themes.vibrant;
        
        // Test vibrant theme characteristics
        assert_eq!(vibrant_theme.primary.primary_500, "#ec4899");
        assert_eq!(vibrant_theme.secondary.secondary_500, "#374151");
        assert_eq!(vibrant_theme.semantic.success, "#10b981");
        assert_eq!(vibrant_theme.semantic.warning, "#f59e0b");
        assert_eq!(vibrant_theme.semantic.error, "#ef4444");
        assert_eq!(vibrant_theme.semantic.info, "#3b82f6");
    }

    #[test]
    fn test_prebuilt_themes_serialization() {
        let themes = PrebuiltThemes::default();
        let json = serde_json::to_string(&themes).unwrap();
        
        // Test that JSON contains all theme names
        assert!(json.contains("\"light\""));
        assert!(json.contains("\"dark\""));
        assert!(json.contains("\"high_contrast\""));
        assert!(json.contains("\"finance\""));
        assert!(json.contains("\"healthcare\""));
        assert!(json.contains("\"education\""));
        assert!(json.contains("\"ecommerce\""));
        assert!(json.contains("\"gaming\""));
        assert!(json.contains("\"minimal\""));
        assert!(json.contains("\"vibrant\""));
    }

    #[test]
    fn test_prebuilt_themes_deserialization() {
        let themes = PrebuiltThemes::default();
        let json = serde_json::to_string(&themes).unwrap();
        let deserialized: PrebuiltThemes = serde_json::from_str(&json).unwrap();
        
        assert_eq!(themes, deserialized);
    }

    #[test]
    fn test_prebuilt_themes_clone() {
        let themes = PrebuiltThemes::default();
        let cloned = themes.clone();
        
        assert_eq!(themes, cloned);
    }

    #[test]
    fn test_prebuilt_themes_debug() {
        let themes = PrebuiltThemes::default();
        let debug_str = format!("{:?}", themes);
        
        assert!(debug_str.contains("PrebuiltThemes"));
    }

    #[test]
    fn test_prebuilt_themes_partial_eq() {
        let themes1 = PrebuiltThemes::default();
        let themes2 = PrebuiltThemes::default();
        
        assert_eq!(themes1, themes2);
    }

    #[test]
    fn test_prebuilt_themes_css_generation() {
        let themes = PrebuiltThemes::default();
        
        // Test that all themes can generate CSS
        let light_css = themes.light.to_css_string();
        let dark_css = themes.dark.to_css_string();
        let high_contrast_css = themes.high_contrast.to_css_string();
        
        assert!(light_css.contains("--primary-500: #3b82f6;"));
        assert!(dark_css.contains("--neutral-50: #0a0a0a;"));
        assert!(high_contrast_css.contains("--primary-500: #0000ff;"));
    }

    #[test]
    fn test_prebuilt_themes_theme_differences() {
        let themes = PrebuiltThemes::default();
        
        // Test that themes are different from each other
        assert_ne!(themes.light.primary.primary_500, themes.dark.primary.primary_500);
        assert_ne!(themes.light.primary.primary_500, themes.high_contrast.primary.primary_500);
        assert_ne!(themes.finance.primary.primary_500, themes.healthcare.primary.primary_500);
        assert_ne!(themes.education.primary.primary_500, themes.ecommerce.primary.primary_500);
        assert_ne!(themes.gaming.primary.primary_500, themes.minimal.primary.primary_500);
        assert_ne!(themes.minimal.primary.primary_500, themes.vibrant.primary.primary_500);
    }

    #[test]
    fn test_prebuilt_themes_color_validation() {
        let themes = PrebuiltThemes::default();
        
        // Test that all themes have valid colors
        let all_themes = vec![
            &themes.light, &themes.dark, &themes.high_contrast,
            &themes.finance, &themes.healthcare, &themes.education,
            &themes.ecommerce, &themes.gaming, &themes.minimal, &themes.vibrant,
        ];
        
        for theme in all_themes {
            // Test primary colors
            assert!(theme.primary.primary_500.starts_with('#'));
            assert_eq!(theme.primary.primary_500.len(), 7);
            
            // Test secondary colors
            assert!(theme.secondary.secondary_500.starts_with('#'));
            assert_eq!(theme.secondary.secondary_500.len(), 7);
            
            // Test semantic colors
            assert!(theme.semantic.success.starts_with('#'));
            assert!(theme.semantic.warning.starts_with('#'));
            assert!(theme.semantic.error.starts_with('#'));
            assert!(theme.semantic.info.starts_with('#'));
        }
    }
}
