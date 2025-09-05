#[cfg(test)]
mod tests {
    use super::*;
    use crate::theming::{
        AnimationVariables, BorderVariables, CSSVariables, PrimaryColors, SecondaryColors,
        SemanticColors, ShadowVariables, SpacingVariables, TypographyVariables,
    };
    use leptos::serde_json;

    #[test]
    fn test_css_variables_default() {
        let css_vars = CSSVariables::default();
        assert_eq!(css_vars.primary.primary_500, "#3b82f6");
        assert_eq!(css_vars.secondary.secondary_500, "#64748b");
        assert_eq!(css_vars.semantic.success, "#10b981");
        assert_eq!(css_vars.semantic.warning, "#f59e0b");
        assert_eq!(css_vars.semantic.error, "#ef4444");
        assert_eq!(css_vars.semantic.info, "#3b82f6");
    }

    #[test]
    fn test_css_variables_dark_theme() {
        let dark_theme = CSSVariables::dark_theme();
        assert_eq!(dark_theme.primary.primary_500, "#3b82f6");
        assert_eq!(dark_theme.secondary.secondary_500, "#64748b");
        // Dark theme should have different neutral colors
        assert_eq!(dark_theme.neutral.neutral_50, "#0a0a0a");
        assert_eq!(dark_theme.neutral.neutral_950, "#fafafa");
    }

    #[test]
    fn test_css_variables_to_css_string() {
        let css_vars = CSSVariables::default();
        let css_string = css_vars.to_css_string();

        assert!(css_string.contains("--primary-500: #3b82f6;"));
        assert!(css_string.contains("--secondary-500: #64748b;"));
        assert!(css_string.contains("--success: #10b981;"));
        assert!(css_string.contains("--warning: #f59e0b;"));
        assert!(css_string.contains("--error: #ef4444;"));
        assert!(css_string.contains("--info: #3b82f6;"));
    }

    #[test]
    fn test_css_variables_serialize() {
        let css_vars = CSSVariables::default();
        let json = serde_json::to_string(&css_vars).unwrap();
        assert!(json.contains("\"primary_500\":\"#3b82f6\""));
        assert!(json.contains("\"secondary_500\":\"#64748b\""));
        assert!(json.contains("\"success\":\"#10b981\""));
    }

    #[test]
    fn test_css_variables_deserialize() {
        let json = include_str!("test_data.json");
        let css_vars: CSSVariables = serde_json::from_str(json).unwrap();
        assert_eq!(css_vars.primary.primary_500, "#3b82f6");
        assert_eq!(css_vars.secondary.secondary_500, "#64748b");
    }

    #[test]
    fn test_primary_colors_default() {
        let primary = PrimaryColors::default();
        assert_eq!(primary.primary_50, "#eff6ff");
        assert_eq!(primary.primary_500, "#3b82f6");
        assert_eq!(primary.primary_950, "#172554");
    }

    #[test]
    fn test_secondary_colors_default() {
        let secondary = SecondaryColors::default();
        assert_eq!(secondary.secondary_50, "#f8fafc");
        assert_eq!(secondary.secondary_500, "#64748b");
        assert_eq!(secondary.secondary_950, "#020617");
    }

    #[test]
    fn test_semantic_colors_default() {
        let semantic = SemanticColors::default();
        assert_eq!(semantic.success, "#10b981");
        assert_eq!(semantic.warning, "#f59e0b");
        assert_eq!(semantic.error, "#ef4444");
        assert_eq!(semantic.info, "#3b82f6");
    }

    #[test]
    fn test_typography_variables_default() {
        let typography = TypographyVariables::default();
        assert_eq!(
            typography.font_family_sans,
            "ui-sans-serif, system-ui, sans-serif"
        );
        assert_eq!(typography.font_size_base, "1rem");
        assert_eq!(typography.font_weight_normal, "400");
    }

    #[test]
    fn test_spacing_variables_default() {
        let spacing = SpacingVariables::default();
        assert_eq!(spacing.space_0, "0px");
        assert_eq!(spacing.space_1, "0.25rem");
        assert_eq!(spacing.space_4, "1rem");
    }

    #[test]
    fn test_border_variables_default() {
        let border = BorderVariables::default();
        assert_eq!(border.border_width_0, "0px");
        assert_eq!(border.border_width_1, "1px");
        assert_eq!(border.border_radius_none, "0px");
    }

    #[test]
    fn test_shadow_variables_default() {
        let shadow = ShadowVariables::default();
        assert_eq!(shadow.shadow_sm, "0 1px 2px 0 rgb(0 0 0 / 0.05)");
        assert_eq!(
            shadow.shadow_md,
            "0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)"
        );
    }

    #[test]
    fn test_animation_variables_default() {
        let animation = AnimationVariables::default();
        assert_eq!(animation.duration_75, "75ms");
        assert_eq!(animation.duration_150, "150ms");
        assert_eq!(animation.ease_in_out, "cubic-bezier(0.4, 0, 0.2, 1)");
    }

    #[test]
    fn test_css_variables_clone() {
        let css_vars = CSSVariables::default();
        let cloned = css_vars.clone();
        assert_eq!(css_vars, cloned);
    }

    #[test]
    fn test_css_variables_debug() {
        let css_vars = CSSVariables::default();
        let debug_str = format!("{:?}", css_vars);
        assert!(debug_str.contains("CSSVariables"));
    }

    #[test]
    fn test_css_variables_partial_eq() {
        let css_vars1 = CSSVariables::default();
        let css_vars2 = CSSVariables::default();
        let css_vars3 = CSSVariables::dark_theme();

        assert_eq!(css_vars1, css_vars2);
        assert_ne!(css_vars1, css_vars3);
    }
}
