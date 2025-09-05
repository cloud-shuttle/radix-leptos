#[cfg(test)]
mod css_variables_tests {
    use super::*;
    use leptos::*;
    use proptest::prelude::*;

    #[test]
    fn test_css_variables_default() {
        let css_vars = CSSVariables::default();
        assert_eq!(css_vars.primary.primary_500, "#3b82f6");
        assert_eq!(css_vars.secondary.secondary_500, "#64748b");
    }

    #[test]
    fn test_css_variables_dark_theme() {
        let dark_theme = CSSVariables::dark_theme();
        assert_eq!(dark_theme.primary.primary_500, "#3b82f6");
        assert_eq!(dark_theme.secondary.secondary_500, "#64748b");
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
    }

    #[test]
    fn test_css_variables_deserialize() {
        let json = include_str!("test_data.json");
        let css_vars: CSSVariables = serde_json::from_str(json).unwrap();
        assert_eq!(css_vars.primary.primary_500, "#3b82f6");
        assert_eq!(css_vars.secondary.secondary_500, "#64748b");
    }

    // Property-based test for CSS variables
    proptest! {
        #[test]
        fn test_css_variables_properties(
            primary_500 in "#[0-9a-fA-F]{6}",
            secondary_500 in "#[0-9a-fA-F]{6}",
            success in "#[0-9a-fA-F]{6}",
            warning in "#[0-9a-fA-F]{6}",
            error in "#[0-9a-fA-F]{6}",
            info in "#[0-9a-fA-F]{6}",
        ) {
            let mut css_vars = CSSVariables::default();
            css_vars.primary.primary_500 = primary_500.clone();
            css_vars.secondary.secondary_500 = secondary_500.clone();
            css_vars.semantic.success = success.clone();
            css_vars.semantic.warning = warning.clone();
            css_vars.semantic.error = error.clone();
            css_vars.semantic.info = info.clone();

            let css_string = css_vars.to_css_string();
            assert!(css_string.contains(&format!("--primary-500: {};", primary_500)));
            assert!(css_string.contains(&format!("--secondary-500: {};", secondary_500)));
            assert!(css_string.contains(&format!("--success: {};", success)));
            assert!(css_string.contains(&format!("--warning: {};", warning)));
            assert!(css_string.contains(&format!("--error: {};", error)));
            assert!(css_string.contains(&format!("--info: {};", info)));
        }
    }
}

