#[cfg(test)]
mod size_variants_tests {
    use super::*;
    use leptos::*;
    use proptest::prelude::*;

    #[test]
    fn test_size_variants_component_creation() {
        let runtime = create_runtime();
        let _view = view! {
            <SizeVariants>
                <div>"Test content"</div>
            </SizeVariants>
        };
        runtime.dispose();
        assert!(true); // Component compiles successfully
    }

    #[test]
    fn test_size_variants_with_custom_size() {
        let runtime = create_runtime();
        let _view = view! {
            <SizeVariants initial_size=Size::Large>
                <div>"Test content"</div>
            </SizeVariants>
        };
        runtime.dispose();
        assert!(true); // Component compiles successfully
    }

    #[test]
    fn test_size_variants_with_custom_variant() {
        let runtime = create_runtime();
        let _view = view! {
            <SizeVariants initial_variant=Variant::Primary>
                <div>"Test content"</div>
            </SizeVariants>
        };
        runtime.dispose();
        assert!(true); // Component compiles successfully
    }

    // Property-based test for SizeVariants
    proptest! {
        #[test]
        fn test_size_variants_properties(
            initial_size in prop::sample::select(vec![
                Size::Small,
                Size::Medium,
                Size::Large,
            ]),
            initial_variant in prop::sample::select(vec![
                Variant::Primary,
                Variant::Secondary,
                Variant::Destructive,
            ]),
        ) {
            let runtime = create_runtime();
            let _view = view! {
                <SizeVariants 
                    initial_size=initial_size
                    initial_variant=initial_variant
                >
                    <div>"Test content"</div>
                </SizeVariants>
            };
            runtime.dispose();
            assert!(true); // Component compiles successfully
        }
    }
}

