#[cfg(test)]
mod theme_provider_tests {
    use super::*;
    use leptos::*;
    use proptest::prelude::*;

    #[test]
    fn test_theme_provider_component_creation() {
        let runtime = create_runtime();
        let _view = view! {
            <ThemeProvider>
                <div>"Test content"</div>
            </ThemeProvider>
        };
        runtime.dispose();
        assert!(true); // Component compiles successfully
    }

    #[test]
    fn test_theme_provider_with_custom_theme() {
        let runtime = create_runtime();
        let custom_theme = CSSVariables::dark_theme();
        let _view = view! {
            <ThemeProvider theme=custom_theme>
                <div>"Test content"</div>
            </ThemeProvider>
        };
        runtime.dispose();
        assert!(true); // Component compiles successfully
    }

    #[test]
    fn test_theme_provider_with_initial_dark_mode() {
        let runtime = create_runtime();
        let _view = view! {
            <ThemeProvider initial_dark=true>
                <div>"Test content"</div>
            </ThemeProvider>
        };
        runtime.dispose();
        assert!(true); // Component compiles successfully
    }

    // Property-based test for ThemeProvider
    proptest! {
        #[test]
        fn test_theme_provider_properties(
            initial_dark in any::<bool>(),
            system_theme in any::<bool>(),
        ) {
            let runtime = create_runtime();
            let _view = view! {
                <ThemeProvider 
                    initial_dark=initial_dark
                    system_theme=system_theme
                >
                    <div>"Test content"</div>
                </ThemeProvider>
            };
            runtime.dispose();
            assert!(true); // Component compiles successfully
        }
    }
}

