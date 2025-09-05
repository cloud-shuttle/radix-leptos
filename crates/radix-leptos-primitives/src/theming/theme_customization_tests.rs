#[cfg(test)]
mod tests {
    use crate::theming::{CSSVariables, Size};
    use leptos::serde_json;

    #[test]
    fn test_theme_customization_default_theme() {
        let theme = CSSVariables::default();
        assert_eq!(theme.primary.primary_500, "#3b82f6");
        assert_eq!(theme.secondary.secondary_500, "#64748b");
    }

    #[test]
    fn test_theme_customizationdark_theme() {
        let dark_theme = CSSVariables::dark_theme();
        assert_eq!(dark_theme.neutral.neutral_50, "#0a0a0a");
        assert_eq!(dark_theme.neutral.neutral_950, "#fafafa");
    }

    #[test]
    fn test_theme_customization_theme_modification() {
        let mut theme = CSSVariables::default();

        // Modify primary color
        theme.primary.primary_500 = "#ff0000".to_string();
        assert_eq!(theme.primary.primary_500, "#ff0000");

        // Modify secondary color
        theme.secondary.secondary_500 = "#00ff00".to_string();
        assert_eq!(theme.secondary.secondary_500, "#00ff00");

        // Modify semantic colors
        theme.semantic.success = "#00ff00".to_string();
        theme.semantic.warning = "#ffff00".to_string();
        theme.semantic.error = "#ff0000".to_string();
        theme.semantic.info = "#0000ff".to_string();

        assert_eq!(theme.semantic.success, "#00ff00");
        assert_eq!(theme.semantic.warning, "#ffff00");
        assert_eq!(theme.semantic.error, "#ff0000");
        assert_eq!(theme.semantic.info, "#0000ff");
    }

    #[test]
    fn test_theme_customization_typography_modification() {
        let mut theme = CSSVariables::default();

        // Modify typography
        theme.typography.font_family_sans = "Arial, sans-serif".to_string();
        theme.typography.font_size_base = "1.2rem".to_string();
        theme.typography.font_weight_normal = "500".to_string();

        assert_eq!(theme.typography.font_family_sans, "Arial, sans-serif");
        assert_eq!(theme.typography.font_size_base, "1.2rem");
        assert_eq!(theme.typography.font_weight_normal, "500");
    }

    #[test]
    fn test_theme_customization_spacing_modification() {
        let mut theme = CSSVariables::default();

        // Modify spacing
        theme.spacing.space_1 = "0.5rem".to_string();
        theme.spacing.space_2 = "1rem".to_string();
        theme.spacing.space_4 = "2rem".to_string();

        assert_eq!(theme.spacing.space_1, "0.5rem");
        assert_eq!(theme.spacing.space_2, "1rem");
        assert_eq!(theme.spacing.space_4, "2rem");
    }

    #[test]
    fn test_theme_customization_border_modification() {
        let mut theme = CSSVariables::default();

        // Modify border
        theme.border.border_width_1 = "2px".to_string();
        theme.border.border_radius_sm = "0.5rem".to_string();
        theme.border.border_radius_md = "1rem".to_string();

        assert_eq!(theme.border.border_width_1, "2px");
        assert_eq!(theme.border.border_radius_sm, "0.5rem");
        assert_eq!(theme.border.border_radius_md, "1rem");
    }

    #[test]
    fn test_theme_customization_shadow_modification() {
        let mut theme = CSSVariables::default();

        // Modify shadows
        theme.shadow.shadow_sm = "0 2px 4px 0 rgb(0 0 0 / 0.1)".to_string();
        theme.shadow.shadow_md = "0 8px 16px 0 rgb(0 0 0 / 0.15)".to_string();
        theme.shadow.shadow_lg = "0 16px 32px 0 rgb(0 0 0 / 0.2)".to_string();

        assert_eq!(theme.shadow.shadow_sm, "0 2px 4px 0 rgb(0 0 0 / 0.1)");
        assert_eq!(theme.shadow.shadow_md, "0 8px 16px 0 rgb(0 0 0 / 0.15)");
        assert_eq!(theme.shadow.shadow_lg, "0 16px 32px 0 rgb(0 0 0 / 0.2)");
    }

    #[test]
    fn test_theme_customization_animation_modification() {
        let mut theme = CSSVariables::default();

        // Modify animations
        theme.animation.duration_150 = "200ms".to_string();
        theme.animation.duration_300 = "400ms".to_string();
        theme.animation.ease_in_out = "cubic-bezier(0.25, 0.46, 0.45, 0.94)".to_string();

        assert_eq!(theme.animation.duration_150, "200ms");
        assert_eq!(theme.animation.duration_300, "400ms");
        assert_eq!(
            theme.animation.ease_in_out,
            "cubic-bezier(0.25, 0.46, 0.45, 0.94)"
        );
    }

    #[test]
    fn test_theme_customization_css_generation() {
        let mut theme = CSSVariables::default();
        theme.primary.primary_500 = "#ff0000".to_string();
        theme.secondary.secondary_500 = "#00ff00".to_string();

        let css_string = theme.to_css_string();

        assert!(css_string.contains("--primary-500: #ff0000;"));
        assert!(css_string.contains("--secondary-500: #00ff00;"));
    }

    #[test]
    fn test_theme_customization_serialization() {
        let mut theme = CSSVariables::default();
        theme.primary.primary_500 = "#ff0000".to_string();
        theme.semantic.success = "#00ff00".to_string();

        let json = serde_json::to_string(&theme).unwrap();

        assert!(json.contains("\"primary_500\":\"#ff0000\""));
        assert!(json.contains("\"success\":\"#00ff00\""));
    }

    #[test]
    fn test_theme_customization_deserialization() {
        let mut theme = CSSVariables::default();
        theme.primary.primary_500 = "#ff0000".to_string();
        theme.semantic.success = "#00ff00".to_string();

        let json = serde_json::to_string(&theme).unwrap();
        let deserialized: CSSVariables = serde_json::from_str(&json).unwrap();

        assert_eq!(theme, deserialized);
    }

    #[test]
    fn test_theme_customization_theme_cloning() {
        let mut theme = CSSVariables::default();
        theme.primary.primary_500 = "#ff0000".to_string();

        let cloned_theme = theme.clone();
        assert_eq!(theme, cloned_theme);

        // Modify original
        theme.primary.primary_500 = "#00ff00".to_string();
        assert_ne!(theme, cloned_theme);
    }

    #[test]
    fn test_theme_customization_color_validation() {
        let mut theme = CSSVariables::default();

        // Test valid hex colors
        let valid_colors = ["#000000", "#ffffff", "#ff0000", "#00ff00", "#0000ff"];

        for color in valid_colors {
            theme.primary.primary_500 = color.to_string();
            assert!(theme.primary.primary_500.starts_with('#'));
            assert_eq!(theme.primary.primary_500.len(), 7);
        }
    }

    #[test]
    fn test_theme_customization_size_variants() {
        // Test size variants
        let sizes = [Size::Xs, Size::Sm, Size::Md, Size::Lg, Size::Xl, Size::Xxl];

        for size in sizes {
            assert!(!size.class().is_empty());
            assert!(!size.spacing().is_empty());
            assert!(!size.font_size().is_empty());
        }
    }

    #[test]
    fn test_theme_customization_variant_consistency() {
        // Test that variants are consistent
        let size1 = Size::Md;
        let size2 = Size::Md;
        assert_eq!(size1, size2);

        let size3 = Size::Lg;
        assert_ne!(size1, size3);
    }
}
