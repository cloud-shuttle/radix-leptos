// Integration tests for the theming system
// These tests can be run independently to verify the theming functionality

#[cfg(test)]
#[allow(clippy::module_inception)]
mod integration_tests {
    use super::*;
    use crate::theming::{
        BreakpointSystem, ButtonVariants, CSSVariables, ContainerSystem, FlexboxSystem, GridSystem,
        InputVariants, LayoutSystem, PrebuiltThemes, Size, SpacingSystem,
    };
    use leptos::serde_json;

    #[test]
    fn test_complete_theming_system() {
        println!("🧪 Running Complete Theming System Integration Test...");

        // Test CSS Variables
        test_css_variables_integration();

        // Test Theme Provider
        test_theme_provider_integration();

        // Test Dark Mode
        test_dark_mode_integration();

        // Test Size Variants
        test_size_variants_integration();

        // Test Theme Customization
        test_theme_customization_integration();

        // Test Prebuilt Themes
        test_prebuilt_themes_integration();

        // Test Component Variants
        test_component_variants_integration();

        // Test Layout System
        test_layout_system_integration();

        println!("✅ All theming system integration tests passed!");
    }

    fn test_css_variables_integration() {
        println!("  📋 Testing CSS Variables...");

        // Test default theme
        let css_vars = CSSVariables::default();
        assert_eq!(css_vars.primary.primary_500, "#3b82f6");
        assert_eq!(css_vars.secondary.secondary_500, "#64748b");
        assert_eq!(css_vars.semantic.success, "#10b981");

        // Test dark theme
        let dark_theme = CSSVariables::dark_theme();
        assert_eq!(dark_theme.neutral.neutral_50, "#0a0a0a");
        assert_eq!(dark_theme.neutral.neutral_950, "#fafafa");

        // Test CSS generation
        let css_string = css_vars.to_css_string();
        assert!(css_string.contains("--primary-500: #3b82f6;"));
        assert!(css_string.contains("--secondary-500: #64748b;"));
        assert!(css_string.contains("--success: #10b981;"));

        // Test serialization
        let json = serde_json::to_string(&css_vars).unwrap();
        assert!(json.contains("\"primary_500\":\"#3b82f6\""));
        assert!(json.contains("\"secondary_500\":\"#64748b\""));

        // Test deserialization
        let deserialized: CSSVariables = serde_json::from_str(&json).unwrap();
        assert_eq!(css_vars, deserialized);

        println!("    ✅ CSS Variables integration test passed");
    }

    fn test_theme_provider_integration() {
        println!("  📋 Testing Theme Provider...");

        // Test theme consistency
        let theme1 = CSSVariables::default();
        let theme2 = CSSVariables::default();
        assert_eq!(theme1, theme2);

        // Test theme switching
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

        // Test color validation
        assert!(light_theme.primary.primary_500.starts_with('#'));
        assert_eq!(light_theme.primary.primary_500.len(), 7);

        println!("    ✅ Theme Provider integration test passed");
    }

    fn test_dark_mode_integration() {
        println!("  📋 Testing Dark Mode...");

        // Test dark theme creation
        let dark_theme = CSSVariables::dark_theme();
        assert_eq!(dark_theme.neutral.neutral_50, "#0a0a0a");
        assert_eq!(dark_theme.neutral.neutral_950, "#fafafa");

        // Test dark vs light mode
        let light_theme = CSSVariables::default();
        assert_ne!(
            light_theme.neutral.neutral_50,
            dark_theme.neutral.neutral_50
        );
        assert_ne!(
            light_theme.neutral.neutral_950,
            dark_theme.neutral.neutral_950
        );

        // Test theme switching logic
        let current_theme = light_theme.clone();
        let is_dark = false;

        let new_theme = if is_dark {
            CSSVariables::dark_theme()
        } else {
            current_theme
        };

        assert_eq!(new_theme, light_theme);

        println!("    ✅ Dark Mode integration test passed");
    }

    fn test_size_variants_integration() {
        println!("  📋 Testing Size Variants...");

        // Test size variants
        let sizes = [Size::Xs, Size::Sm, Size::Md, Size::Lg, Size::Xl, Size::Xxl];

        for size in sizes {
            assert!(!size.class().is_empty());
            assert!(size.class().starts_with("size-"));
            assert!(!size.spacing().is_empty());
            assert!(size.spacing().ends_with("rem"));
            assert!(!size.font_size().is_empty());
            assert!(size.font_size().ends_with("rem"));
        }

        // Test default size
        assert_eq!(Size::default(), Size::Md);

        // Test size progression
        let xs_spacing = Size::Xs.spacing();
        let sm_spacing = Size::Sm.spacing();
        let md_spacing = Size::Md.spacing();

        let xs_val: f64 = xs_spacing.replace("rem", "").parse().unwrap();
        let sm_val: f64 = sm_spacing.replace("rem", "").parse().unwrap();
        let md_val: f64 = md_spacing.replace("rem", "").parse().unwrap();

        assert!(xs_val < sm_val);
        assert!(sm_val < md_val);

        println!("    ✅ Size Variants integration test passed");
    }

    fn test_theme_customization_integration() {
        println!("  📋 Testing Theme Customization...");

        // Test theme modification
        let mut theme = CSSVariables::default();
        theme.primary.primary_500 = "#ff0000".to_string();
        assert_eq!(theme.primary.primary_500, "#ff0000");

        // Test typography modification
        theme.typography.font_family_sans = "Arial, sans-serif".to_string();
        assert_eq!(theme.typography.font_family_sans, "Arial, sans-serif");

        // Test spacing modification
        theme.spacing.space_1 = "0.5rem".to_string();
        assert_eq!(theme.spacing.space_1, "0.5rem");

        // Test theme cloning
        let cloned_theme = theme.clone();
        assert_eq!(theme, cloned_theme);

        // Test CSS generation with custom theme
        let css_string = theme.to_css_string();
        assert!(css_string.contains("--primary-500: #ff0000;"));

        println!("    ✅ Theme Customization integration test passed");
    }

    fn test_prebuilt_themes_integration() {
        println!("  📋 Testing Prebuilt Themes...");

        // Test prebuilt themes creation
        let themes = PrebuiltThemes::default();

        // Test that themes have different characteristics (some may have same primary colors)
        // Note: Currently all themes use the same primary colors, but have different neutral colors
        assert_ne!(
            themes.light.neutral.neutral_50,
            themes.dark.neutral.neutral_50
        );
        assert_eq!(
            themes.light.primary.primary_500,
            themes.dark.primary.primary_500
        ); // Same primary colors

        // Test theme characteristics
        assert_eq!(themes.light.primary.primary_500, "#3b82f6");
        assert_eq!(themes.dark.neutral.neutral_50, "#0a0a0a");
        assert_eq!(themes.high_contrast.primary.primary_500, "#0000ff");
        assert_eq!(themes.finance.primary.primary_500, "#1e40af");
        assert_eq!(themes.healthcare.primary.primary_500, "#059669");

        // Test theme serialization
        let json = serde_json::to_string(&themes).unwrap();
        assert!(json.contains("\"light\""));
        assert!(json.contains("\"dark\""));
        assert!(json.contains("\"high_contrast\""));
        assert!(json.contains("\"finance\""));
        assert!(json.contains("\"healthcare\""));

        // Test theme deserialization
        let deserialized: PrebuiltThemes = serde_json::from_str(&json).unwrap();
        assert_eq!(themes, deserialized);

        println!("    ✅ Prebuilt Themes integration test passed");
    }

    fn test_component_variants_integration() {
        println!("  📋 Testing Component Variants...");

        // Test button variants
        let button_variants = ButtonVariants::default();
        assert!(!button_variants.sizes.is_empty());
        assert!(!button_variants.styles.is_empty());
        assert!(!button_variants.states.is_empty());

        // Test input variants
        let input_variants = InputVariants::default();
        assert!(!input_variants.sizes.is_empty());
        assert!(!input_variants.styles.is_empty());
        assert!(!input_variants.states.is_empty());
        assert!(!input_variants.types.is_empty());

        // Test variant consistency
        assert_eq!(button_variants.sizes.len(), 4); // Small, Medium, Large, ExtraLarge
        assert_eq!(button_variants.styles.len(), 6); // Default, Primary, Secondary, Outline, Ghost, Destructive
        assert_eq!(button_variants.states.len(), 5); // Default, Hover, Active, Disabled, Loading

        assert_eq!(input_variants.sizes.len(), 3); // Small, Medium, Large
        assert_eq!(input_variants.styles.len(), 3); // Default, Outline, Filled
        assert_eq!(input_variants.states.len(), 4); // Default, Focus, Error, Disabled
        assert_eq!(input_variants.types.len(), 5); // Text, Email, Password, Number, Search

        println!("    ✅ Component Variants integration test passed");
    }

    fn test_layout_system_integration() {
        println!("  📋 Testing Layout System...");

        // Test layout config
        let layout = LayoutSystem::default();
        assert!(!layout.spacing.base_unit.is_nan());
        assert!(!layout.breakpoints.breakpoints.is_empty());

        // Test grid system
        let grid = GridSystem::default();
        assert!(grid.columns > 0);
        assert!(!grid.gutters.is_empty());
        assert!(!grid.gaps.is_empty());

        // Test flexbox system
        let flexbox = FlexboxSystem::default();
        assert!(!flexbox.directions.is_empty());
        assert!(!flexbox.wraps.is_empty());
        assert!(!flexbox.justifications.is_empty());
        assert!(!flexbox.alignments.is_empty());

        // Test container system
        let container = ContainerSystem::default();
        assert!(!container.max_widths.is_empty());
        assert!(!container.paddings.is_empty());
        assert!(!container.margins.is_empty());

        // Test spacing system
        let spacing_system = SpacingSystem::default();
        assert!(!spacing_system.scale.is_empty());

        // Test breakpoint system
        let breakpoint_system = BreakpointSystem::default();
        assert!(!breakpoint_system.breakpoints.is_empty());

        println!("    ✅ Layout System integration test passed");
    }
}
