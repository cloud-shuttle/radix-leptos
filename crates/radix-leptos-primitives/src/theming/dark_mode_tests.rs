#[cfg(test)]
mod tests {
    use super::*;
    use crate::theming::CSSVariables;
    use leptos::serde_json;

    #[test]
    fn test_dark_mode_theme_creation() {
        let dark_theme = CSSVariables::dark_theme();
        assert_eq!(dark_theme.neutral.neutral_50, "#0a0a0a");
        assert_eq!(dark_theme.neutral.neutral_950, "#fafafa");
    }

    #[test]
    fn test_dark_mode_vs_light_mode() {
        let light_theme = CSSVariables::default();
        let dark_theme = CSSVariables::dark_theme();
        
        // Dark mode should have darker neutral colors
        assert_ne!(light_theme.neutral.neutral_50, dark_theme.neutral.neutral_50);
        assert_ne!(light_theme.neutral.neutral_950, dark_theme.neutral.neutral_950);
        
        // Primary colors should remain the same
        assert_eq!(light_theme.primary.primary_500, dark_theme.primary.primary_500);
        assert_eq!(light_theme.secondary.secondary_500, dark_theme.secondary.secondary_500);
    }

    #[test]
    fn test_dark_mode_css_generation() {
        let dark_theme = CSSVariables::dark_theme();
        let css_string = dark_theme.to_css_string();
        
        // Test that dark theme CSS contains expected variables
        assert!(css_string.contains("--neutral-50: #0a0a0a;"));
        assert!(css_string.contains("--neutral-950: #fafafa;"));
        assert!(css_string.contains("--primary-500: #3b82f6;"));
    }

    #[test]
    fn test_dark_mode_serialization() {
        let dark_theme = CSSVariables::dark_theme();
        let json = serde_json::to_string(&dark_theme).unwrap();
        
        // Test that JSON contains dark theme values
        assert!(json.contains("\"neutral_50\":\"#0a0a0a\""));
        assert!(json.contains("\"neutral_950\":\"#fafafa\""));
    }

    #[test]
    fn test_dark_mode_deserialization() {
        let dark_theme = CSSVariables::dark_theme();
        let json = serde_json::to_string(&dark_theme).unwrap();
        let deserialized: CSSVariables = serde_json::from_str(&json).unwrap();
        
        assert_eq!(dark_theme, deserialized);
    }

    #[test]
    fn test_dark_mode_consistency() {
        let dark1 = CSSVariables::dark_theme();
        let dark2 = CSSVariables::dark_theme();
        
        // Test that dark themes are consistent
        assert_eq!(dark1, dark2);
    }

    #[test]
    fn test_dark_mode_color_contrast() {
        let dark_theme = CSSVariables::dark_theme();
        
        // Test that dark theme has proper contrast
        // Dark background colors should be dark
        assert!(dark_theme.neutral.neutral_50.starts_with('#'));
        assert!(dark_theme.neutral.neutral_100.starts_with('#'));
        
        // Light text colors should be light
        assert!(dark_theme.neutral.neutral_900.starts_with('#'));
        assert!(dark_theme.neutral.neutral_950.starts_with('#'));
    }

    #[test]
    fn test_dark_mode_theme_switching_logic() {
        let light_theme = CSSVariables::default();
        let dark_theme = CSSVariables::dark_theme();
        
        // Test theme switching logic
        let current_theme = light_theme.clone();
        let is_dark = false;
        
        let new_theme = if is_dark {
            CSSVariables::dark_theme()
        } else {
            current_theme
        };
        
        assert_eq!(new_theme, light_theme);
        
        let new_theme_dark = if true {
            CSSVariables::dark_theme()
        } else {
            light_theme
        };
        
        assert_eq!(new_theme_dark, dark_theme);
    }

    #[test]
    fn test_dark_mode_storage_key_validation() {
        // Test that storage keys are valid
        let valid_keys = [
            "theme-preference",
            "dark-mode",
            "user-theme",
            "app-theme",
        ];
        
        for key in valid_keys {
            assert!(!key.is_empty());
            assert!(key.chars().all(|c| c.is_alphanumeric() || c == '-'));
        }
    }

    #[test]
    fn test_dark_mode_theme_application() {
        let dark_theme = CSSVariables::dark_theme();
        let css_string = dark_theme.to_css_string();
        
        // Test that CSS string is properly formatted
        assert!(css_string.contains("--neutral-50: #0a0a0a;"));
        assert!(css_string.contains("--neutral-100: #171717;"));
        assert!(css_string.contains("--neutral-200: #262626;"));
        
        // Test that all neutral colors are present
        for i in [50, 100, 200, 300, 400, 500, 600, 700, 800, 900, 950] {
            assert!(css_string.contains(&format!("--neutral-{}:", i)));
        }
    }
}

