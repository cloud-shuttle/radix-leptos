// Simple tests for theming system that don't rely on complex proptest infrastructure
#[cfg(test)]
#[allow(clippy::module_inception)]
mod simple_tests {
    use super::*;
    use crate::theming::{ButtonVariants, CSSVariables, LayoutSystem, PrebuiltThemes, Size};

    #[test]
    fn test_css_variables_basic() {
        let css_vars = CSSVariables::default();
        assert_eq!(css_vars.primary.primary_500, "#3b82f6");
        assert_eq!(css_vars.secondary.secondary_500, "#64748b");
        assert_eq!(css_vars.semantic.success, "#10b981");
    }

    #[test]
    fn test_dark_theme_basic() {
        let dark_theme = CSSVariables::dark_theme();
        assert_eq!(dark_theme.neutral.neutral_50, "#0a0a0a");
        assert_eq!(dark_theme.neutral.neutral_950, "#fafafa");
    }

    #[test]
    fn test_size_variants_basic() {
        let size = Size::Md;
        assert_eq!(size.class(), "size-md");
        assert_eq!(size.spacing(), "1rem");
        assert_eq!(size.font_size(), "1rem");
    }

    #[test]
    fn test_prebuilt_themes_basic() {
        let themes = PrebuiltThemes::default();
        assert_eq!(themes.light.primary.primary_500, "#3b82f6");
        assert_eq!(themes.dark.neutral.neutral_50, "#0a0a0a");
        assert_eq!(themes.finance.primary.primary_500, "#1e40af");
    }

    #[test]
    fn test_component_variants_basic() {
        let button_variants = ButtonVariants::default();
        assert!(!button_variants.sizes.is_empty());
        assert!(!button_variants.styles.is_empty());
        assert!(!button_variants.states.is_empty());
    }

    #[test]
    fn test_layout_system_basic() {
        let layout = LayoutSystem::default();
        assert_eq!(layout.spacing.base_unit, 4.0);
        assert!(!layout.breakpoints.breakpoints.is_empty());
    }

    #[test]
    fn test_css_generation() {
        let css_vars = CSSVariables::default();
        let css_string = css_vars.to_css_string();
        assert!(css_string.contains("--primary-500: #3b82f6;"));
        assert!(css_string.contains("--secondary-500: #64748b;"));
        assert!(css_string.contains("--success: #10b981;"));
    }

    #[test]
    fn test_theme_consistency() {
        let theme1 = CSSVariables::default();
        let theme2 = CSSVariables::default();
        assert_eq!(theme1, theme2);
    }

    #[test]
    fn test_theme_differences() {
        let light_theme = CSSVariables::default();
        let dark_theme = CSSVariables::dark_theme();
        assert_ne!(
            light_theme.neutral.neutral_50,
            dark_theme.neutral.neutral_50
        );
        assert_eq!(
            light_theme.primary.primary_500,
            dark_theme.primary.primary_500
        );
    }
}
