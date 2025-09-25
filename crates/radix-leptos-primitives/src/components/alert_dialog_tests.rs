#[cfg(test)]
mod alert_dialog_tests {
    use leptos::callback::Callback;
    use proptest::prelude::*;
    use crate::components::alert_dialog::*;
use crate::utils::{merge_optional_classes, generate_id};

    // Test AlertDialog component creation
    #[test]
    fn test_alert_dialog_component_creation() {
        // Test basic alert dialog creation
        let dialog = AlertDialog::new();
        assert_eq!(dialog.variant, AlertDialogVariant::Default);
        assert_eq!(dialog.size, AlertDialogSize::Medium);
        assert!(!dialog.open);
        assert!(dialog.title.is_none());
        assert!(dialog.description.is_none());
        assert!(dialog.class.is_none());
        assert!(dialog.style.is_none());
    }

    // Test AlertDialog with different variants
    #[test]
    fn test_alert_dialog_variants() {
        // Test Default variant
        let default_dialog = AlertDialog::new().with_variant(AlertDialogVariant::Default);
        assert_eq!(default_dialog.variant, AlertDialogVariant::Default);
        
        // Test Destructive variant
        let destructive_dialog = AlertDialog::new().with_variant(AlertDialogVariant::Destructive);
        assert_eq!(destructive_dialog.variant, AlertDialogVariant::Destructive);
        
        // Test Warning variant
        let warning_dialog = AlertDialog::new().with_variant(AlertDialogVariant::Warning);
        assert_eq!(warning_dialog.variant, AlertDialogVariant::Warning);
    }

    // Test AlertDialog with different sizes
    #[test]
    fn test_alert_dialog_sizes() {
        let small_dialog = AlertDialog::new().with_size(AlertDialogSize::Small);
        assert_eq!(small_dialog.size, AlertDialogSize::Small);
        
        let medium_dialog = AlertDialog::new().with_size(AlertDialogSize::Medium);
        assert_eq!(medium_dialog.size, AlertDialogSize::Medium);
        
        let large_dialog = AlertDialog::new().with_size(AlertDialogSize::Large);
        assert_eq!(large_dialog.size, AlertDialogSize::Large);
    }

    // Test AlertDialog with content
    #[test]
    fn test_alert_dialog_content() {
        let dialog = AlertDialog::new()
            .with_title("Confirm Action")
            .with_description("Are you sure you want to proceed?");
        
        assert_eq!(dialog.title, Some("Confirm Action".to_string()));
        assert_eq!(dialog.description, Some("Are you sure you want to proceed?".to_string()));
    }

    // Test AlertDialog open state
    #[test]
    fn test_alert_dialogopen_state() {
        let closed_dialog = AlertDialog::new();
        assert!(!closed_dialog.open);
        
        let open_dialog = AlertDialog::new().withopen(true);
        assert!(open_dialog.open);
    }

    // Test AlertDialog builder pattern
    #[test]
    fn test_alert_dialog_builder_pattern() {
        let dialog = AlertDialog::new()
            .with_variant(AlertDialogVariant::Destructive)
            .with_size(AlertDialogSize::Large)
            .with_title("Delete Item")
            .with_description("This action cannot be undone.")
            .withopen(true)
            .with_class("custom-dialog")
            .with_style("z-index: 1000");
        
        assert_eq!(dialog.variant, AlertDialogVariant::Destructive);
        assert_eq!(dialog.size, AlertDialogSize::Large);
        assert_eq!(dialog.title, Some("Delete Item".to_string()));
        assert_eq!(dialog.description, Some("This action cannot be undone.".to_string()));
        assert!(dialog.open);
        assert_eq!(dialog.class, Some("custom-dialog".to_string()));
        assert_eq!(dialog.style, Some("z-index: 1000".to_string()));
    }

    // Property-based test for AlertDialog
    proptest! {
        #[test]
        fn test_alert_dialog_properties(
            open in any::<bool>(),
            variant in prop::sample::select(&[
                AlertDialogVariant::Default,
                AlertDialogVariant::Destructive,
                AlertDialogVariant::Warning,
            ]),
            size in prop::sample::select(&[
                AlertDialogSize::Small,
                AlertDialogSize::Medium,
                AlertDialogSize::Large,
            ]),
        ) {
            // Test that AlertDialog can be created with various property combinations
            let _dialog = view! {
                <AlertDialog
                    open=open
                    variant=variant
                    size=size
                    onopen_change=Callback::new(|_| {})
                >
                    <AlertDialogTitle>"Test Title"</AlertDialogTitle>
                    <AlertDialogDescription>"Test Description"</AlertDialogDescription>
                    <AlertDialogAction on_click=Callback::new(|_| {})>
                        "Confirm"
                    </AlertDialogAction>
                    <AlertDialogCancel on_click=Callback::new(|_| {})>
                        "Cancel"
                    </AlertDialogCancel>
                </AlertDialog>
            };
            
        }
    }

    // Test AlertDialog accessibility
    #[test]
    fn test_alert_dialog_accessibility() {
        
    }

    // Test AlertDialog keyboard navigation
    #[test]
    fn test_alert_dialog_keyboard_navigation() {
        
    }
}
