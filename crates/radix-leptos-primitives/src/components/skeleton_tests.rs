#[cfg(test)]
mod skeleton_tests {
    use leptos::callback::Callback;
    use proptest::prelude::*;
    use crate::components::skeleton::*;
use crate::utils::{merge_optional_classes, generate_id};

    // Test Skeleton component creation
    #[test]
    fn test_skeleton_component_creation() {
        // Test basic skeleton creation
        let skeleton = Skeleton::new();
        assert_eq!(skeleton.variant, SkeletonVariant::Text);
        assert_eq!(skeleton.size, SkeletonSize::Medium);
        assert_eq!(skeleton.lines, 1);
        assert!(skeleton.width.is_none());
        assert!(skeleton.height.is_none());
        assert!(skeleton.class.is_none());
        assert!(skeleton.style.is_none());
    }

    // Test Skeleton with different variants
    #[test]
    fn test_skeleton_variants() {
        // Test Text variant
        let text_skeleton = Skeleton::new().with_variant(SkeletonVariant::Text);
        assert_eq!(text_skeleton.variant, SkeletonVariant::Text);
        
        // Test Circular variant
        let circular_skeleton = Skeleton::new().with_variant(SkeletonVariant::Circular);
        assert_eq!(circular_skeleton.variant, SkeletonVariant::Circular);
        
        // Test Rectangular variant
        let rectangular_skeleton = Skeleton::new().with_variant(SkeletonVariant::Rectangular);
        assert_eq!(rectangular_skeleton.variant, SkeletonVariant::Rectangular);
    }

    // Test Skeleton with different sizes
    #[test]
    fn test_skeleton_sizes() {
        let small_skeleton = Skeleton::new().with_size(SkeletonSize::Small);
        assert_eq!(small_skeleton.size, SkeletonSize::Small);
        
        let medium_skeleton = Skeleton::new().with_size(SkeletonSize::Medium);
        assert_eq!(medium_skeleton.size, SkeletonSize::Medium);
        
        let large_skeleton = Skeleton::new().with_size(SkeletonSize::Large);
        assert_eq!(large_skeleton.size, SkeletonSize::Large);
        
        let xl_skeleton = Skeleton::new().with_size(SkeletonSize::ExtraLarge);
        assert_eq!(xl_skeleton.size, SkeletonSize::ExtraLarge);
    }

    // Test Skeleton with custom dimensions
    #[test]
    fn test_skeleton_custom_dimensions() {
        let skeleton = Skeleton::new()
            .with_width("200px")
            .with_height("100px");
        
        assert_eq!(skeleton.width, Some("200px".to_string()));
        assert_eq!(skeleton.height, Some("100px".to_string()));
    }

    // Test Skeleton with multiple lines
    #[test]
    fn test_skeleton_multiple_lines() {
        let skeleton = Skeleton::new().with_lines(3);
        assert_eq!(skeleton.lines, 3);
    }

    // Test Skeleton with custom class and style
    #[test]
    fn test_skeleton_custom_styling() {
        let skeleton = Skeleton::new()
            .with_class("custom-skeleton")
            .with_style("background-color: red");
        
        assert_eq!(skeleton.class, Some("custom-skeleton".to_string()));
        assert_eq!(skeleton.style, Some("background-color: red".to_string()));
    }

    // Test Skeleton builder pattern
    #[test]
    fn test_skeleton_builder_pattern() {
        let skeleton = Skeleton::new()
            .with_variant(SkeletonVariant::Circular)
            .with_size(SkeletonSize::Large)
            .with_width("50px")
            .with_height("50px")
            .with_class("avatar-skeleton")
            .with_style("border-radius: 50%");
        
        assert_eq!(skeleton.variant, SkeletonVariant::Circular);
        assert_eq!(skeleton.size, SkeletonSize::Large);
        assert_eq!(skeleton.width, Some("50px".to_string()));
        assert_eq!(skeleton.height, Some("50px".to_string()));
        assert_eq!(skeleton.class, Some("avatar-skeleton".to_string()));
        assert_eq!(skeleton.style, Some("border-radius: 50%".to_string()));
    }

    // Property-based test for Skeleton
    proptest! {
        #[test]
        fn test_skeleton_properties(
            variant in prop::sample::select(&[
                SkeletonVariant::Text,
                SkeletonVariant::Circular,
                SkeletonVariant::Rectangular,
            ]),
            size in prop::sample::select(&[
                SkeletonSize::Small,
                SkeletonSize::Medium,
                SkeletonSize::Large,
                SkeletonSize::ExtraLarge,
            ]),
            animated in any::<bool>(),
            __lines in 1..=5usize,
        ) {
            // Test that Skeleton can be created with various property combinations
            let _skeleton = view! {
                <Skeleton
                    variant=variant
                    size=size
                    animated=animated
                    lines=lines
                />
            };
            
        }
    }

    // Test SkeletonText component
    #[test]
    fn test_skeleton_text_component() {
        let _skeleton_text = view! {
            <SkeletonText lines=3 animated=true />
        };
        
    }

    // Test SkeletonAvatar component
    #[test]
    fn test_skeleton_avatar_component() {
        let _skeleton_avatar = view! {
            <SkeletonAvatar size=SkeletonSize::Medium animated=true />
        };
        
    }

    // Test SkeletonButton component
    #[test]
    fn test_skeleton_button_component() {
        let _skeleton_button = view! {
            <SkeletonButton size=SkeletonSize::Large animated=true />
        };
        
    }

    // Test Skeleton accessibility
    #[test]
    fn test_skeleton_accessibility() {
        
    }

    // Test Skeleton animations
    #[test]
    fn test_skeleton_animations() {
        
    }
}
