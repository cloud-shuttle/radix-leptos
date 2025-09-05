#[cfg(test)]
mod tests {
    use super::*;
    use crate::theming::{ButtonVariants, InputVariants, SizeVariant, StyleVariant, StateVariant, InputTypeVariant, ComponentVariants};

    #[test]
    fn test_button_variants_creation() {
        let variants = ButtonVariants::default();
        assert_eq!(variants.sizes.len(), 4); // Small, Medium, Large, ExtraLarge
        assert_eq!(variants.styles.len(), 6); // Default, Primary, Secondary, Outline, Ghost, Destructive
        assert_eq!(variants.states.len(), 5); // Default, Hover, Active, Disabled, Loading
    }

    #[test]
    fn test_input_variants_creation() {
        let variants = InputVariants::default();
        assert_eq!(variants.sizes.len(), 3); // Small, Medium, Large
        assert_eq!(variants.styles.len(), 3); // Default, Outline, Filled
        assert_eq!(variants.states.len(), 4); // Default, Focus, Error, Disabled
        assert_eq!(variants.types.len(), 5); // Text, Email, Password, Number, Search
    }

    #[test]
    fn test_size_variant_enum() {
        let size = SizeVariant::Large;
        assert_eq!(size.as_str(), "lg");
        assert_eq!(size.display_name(), "Large");
    }

    #[test]
    fn test_style_variant_enum() {
        let style = StyleVariant::Primary;
        assert_eq!(format!("{:?}", style), "Primary");
    }

    #[test]
    fn test_state_variant_enum() {
        let state = StateVariant::Hover;
        assert_eq!(format!("{:?}", state), "Hover");
    }

    #[test]
    fn test_component_variants_default() {
        let variants = ComponentVariants::default();
        assert_eq!(variants.button.sizes.len(), 4);
        assert_eq!(variants.input.sizes.len(), 3);
        assert_eq!(variants.card.sizes.len(), 3);
        assert_eq!(variants.badge.sizes.len(), 3);
        assert_eq!(variants.alert.sizes.len(), 3);
    }
}
