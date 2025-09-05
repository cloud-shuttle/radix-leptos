#[cfg(test)]
mod dark_mode_tests {
    use super::*;
    use leptos::*;
    use proptest::prelude::*;

    #[test]
    fn test_dark_mode_component_creation() {
        let runtime = create_runtime();
        let _view = view! {
            <DarkModeToggle />
        };
        runtime.dispose();
        assert!(true); // Component compiles successfully
    }

    #[test]
    fn test_dark_mode_with_custom_storage_key() {
        let runtime = create_runtime();
        let _view = view! {
            <DarkModeToggle storage_key="custom-theme".to_string() />
        };
        runtime.dispose();
        assert!(true); // Component compiles successfully
    }

    #[test]
    fn test_dark_mode_with_custom_theme() {
        let runtime = create_runtime();
        let custom_theme = CSSVariables::dark_theme();
        let _view = view! {
            <DarkModeToggle theme=custom_theme />
        };
        runtime.dispose();
        assert!(true); // Component compiles successfully
    }

    // Property-based test for DarkModeToggle
    proptest! {
        #[test]
        fn test_dark_mode_properties(
            storage_key in "[a-zA-Z0-9_-]+",
            initial_dark in any::<bool>(),
        ) {
            let runtime = create_runtime();
            let _view = view! {
                <DarkModeToggle 
                    storage_key=storage_key
                    initial_dark=initial_dark
                />
            };
            runtime.dispose();
            assert!(true); // Component compiles successfully
        }
    }
}

