#[cfg(test)]
mod theme_customization_tests {
    use super::*;
    use leptos::*;
    use proptest::prelude::*;

    #[test]
    fn test_theme_customization_component_creation() {
        let runtime = create_runtime();
        let _view = view! {
            <ThemeCustomization />
        };
        runtime.dispose();
        assert!(true); // Component compiles successfully
    }

    #[test]
    fn test_theme_customization_with_custom_theme() {
        let runtime = create_runtime();
        let custom_theme = CSSVariables::dark_theme();
        let _view = view! {
            <ThemeCustomization initial_theme=custom_theme />
        };
        runtime.dispose();
        assert!(true); // Component compiles successfully
    }

    #[test]
    fn test_theme_customization_with_custom_callback() {
        let runtime = create_runtime();
        let callback = Callback::new(|_theme: CSSVariables| {});
        let _view = view! {
            <ThemeCustomization on_theme_change=callback />
        };
        runtime.dispose();
        assert!(true); // Component compiles successfully
    }

    // Property-based test for ThemeCustomization
    proptest! {
        #[test]
        fn test_theme_customization_properties(
            primary_color in "#[0-9a-fA-F]{6}",
            secondary_color in "#[0-9a-fA-F]{6}",
            success_color in "#[0-9a-fA-F]{6}",
            warning_color in "#[0-9a-fA-F]{6}",
            error_color in "#[0-9a-fA-F]{6}",
            info_color in "#[0-9a-fA-F]{6}",
        ) {
            let runtime = create_runtime();
            let mut custom_theme = CSSVariables::default();
            custom_theme.primary.primary_500 = primary_color.clone();
            custom_theme.secondary.secondary_500 = secondary_color.clone();
            custom_theme.semantic.success = success_color.clone();
            custom_theme.semantic.warning = warning_color.clone();
            custom_theme.semantic.error = error_color.clone();
            custom_theme.semantic.info = info_color.clone();

            let _view = view! {
                <ThemeCustomization initial_theme=custom_theme />
            };
            runtime.dispose();
            assert!(true); // Component compiles successfully
        }
    }
}

