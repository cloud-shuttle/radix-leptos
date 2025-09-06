#[cfg(test)]
mod tests {
    use crate::theming::CSSVariables;
    use leptos::serde_json;

    #[test]
    fn test_theme_provider_default_theme() {
        let theme = CSSVariables::default();
        assert_eq!(theme.primary.primary_500, "#3b82f6");
        assert_eq!(theme.secondary.secondary_500, "#64748b");
    }

    #[test]
    fn test_theme_providerdark_theme() {
        let dark_theme = CSSVariables::dark_theme();
        assert_eq!(dark_theme.neutral.neutral_50, "#0a0a0a");
        assert_eq!(dark_theme.neutral.neutral_950, "#fafafa");
    }

    #[test]
    fn test_theme_provider_theme_switching() {
        let light_theme = CSSVariables::default();
        let dark_theme = CSSVariables::dark_theme();

        // Test that themes are different
        assert_ne!(
            light_theme.neutral.neutral_50,
            dark_theme.neutral.neutral_50
        );
        assert_ne!(
            light_theme.neutral.neutral_950,
            dark_theme.neutral.neutral_950
        );

        // Test that primary colors remain the same
        assert_eq!(
            light_theme.primary.primary_500,
            dark_theme.primary.primary_500
        );
    }

    #[test]
    fn test_theme_provider_css_generation() {
        let theme = CSSVariables::default();
        let css_string = theme.to_css_string();

        // Test that CSS string contains expected variables
        assert!(css_string.contains("--primary-500: #3b82f6;"));
        assert!(css_string.contains("--secondary-500: #64748b;"));
        assert!(css_string.contains("--success: #10b981;"));
        assert!(css_string.contains("--warning: #f59e0b;"));
        assert!(css_string.contains("--error: #ef4444;"));
        assert!(css_string.contains("--info: #3b82f6;"));
    }

    #[test]
    fn test_theme_provider_serialization() {
        let theme = CSSVariables::default();
        let json = serde_json::to_string(&theme).unwrap();

        // Test that JSON contains expected fields
        assert!(json.contains("\"primary_500\":\"#3b82f6\""));
        assert!(json.contains("\"secondary_500\":\"#64748b\""));
        assert!(json.contains("\"success\":\"#10b981\""));
    }

    #[test]
    fn test_theme_provider_deserialization() {
        let theme = CSSVariables::default();
        let json = serde_json::to_string(&theme).unwrap();
        let deserialized: CSSVariables = serde_json::from_str(&json).unwrap();

        assert_eq!(theme, deserialized);
    }

    #[test]
    fn test_theme_provider_theme_consistency() {
        let theme1 = CSSVariables::default();
        let theme2 = CSSVariables::default();

        // Test that default themes are consistent
        assert_eq!(theme1, theme2);

        // Test that dark themes are consistent
        let dark1 = CSSVariables::dark_theme();
        let dark2 = CSSVariables::dark_theme();
        assert_eq!(dark1, dark2);
    }

    #[test]
    fn test_theme_provider_color_validation() {
        let theme = CSSVariables::default();

        // Test that colors are valid hex values
        assert!(theme.primary.primary_500.starts_with('#'));
        assert!(theme.secondary.secondary_500.starts_with('#'));
        assert!(theme.semantic.success.starts_with('#'));
        assert!(theme.semantic.warning.starts_with('#'));
        assert!(theme.semantic.error.starts_with('#'));
        assert!(theme.semantic.info.starts_with('#'));

        // Test that hex values are 7 characters long (#RRGGBB)
        assert_eq!(theme.primary.primary_500.len(), 7);
        assert_eq!(theme.secondary.secondary_500.len(), 7);
        assert_eq!(theme.semantic.success.len(), 7);
    }

    #[test]
    fn test_theme_provider_typography_consistency() {
        let theme = CSSVariables::default();

        // Test that typography values are reasonable
        assert!(!theme.typography.font_family_sans.is_empty());
        assert!(!theme.typography.font_size_base.is_empty());
        assert!(!theme.typography.font_weight_normal.is_empty());

        // Test that font sizes contain valid units
        assert!(
            theme.typography.font_size_base.contains("rem")
                || theme.typography.font_size_base.contains("px")
        );
    }

    #[test]
    fn test_theme_provider_spacing_consistency() {
        let theme = CSSVariables::default();

        // Test that spacing values are reasonable
        assert!(!theme.spacing.space_0.is_empty());
        assert!(!theme.spacing.space_1.is_empty());
        assert!(!theme.spacing.space_4.is_empty());

        // Test that spacing values contain valid units
        assert!(theme.spacing.space_0.contains("px") || theme.spacing.space_0.contains("rem"));
    }
}
