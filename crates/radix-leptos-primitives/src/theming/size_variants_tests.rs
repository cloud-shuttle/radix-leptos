#[cfg(test)]
mod tests {
    use super::*;
    use crate::theming::Size;

    #[test]
    fn test_size_variants_creation() {
        let sizes = vec![
            Size::Xs,
            Size::Sm,
            Size::Md,
            Size::Lg,
            Size::Xl,
            Size::Xxl,
        ];
        
        assert_eq!(sizes.len(), 6);
    }

    #[test]
    fn test_size_variants_default() {
        let default_size = Size::default();
        assert_eq!(default_size, Size::Md);
    }

    #[test]
    fn test_size_variants_class() {
        assert_eq!(Size::Xs.class(), "size-xs");
        assert_eq!(Size::Sm.class(), "size-sm");
        assert_eq!(Size::Md.class(), "size-md");
        assert_eq!(Size::Lg.class(), "size-lg");
        assert_eq!(Size::Xl.class(), "size-xl");
        assert_eq!(Size::Xxl.class(), "size-xxl");
    }

    #[test]
    fn test_size_variants_spacing() {
        assert_eq!(Size::Xs.spacing(), "0.25rem");
        assert_eq!(Size::Sm.spacing(), "0.5rem");
        assert_eq!(Size::Md.spacing(), "1rem");
        assert_eq!(Size::Lg.spacing(), "1.5rem");
        assert_eq!(Size::Xl.spacing(), "2rem");
        assert_eq!(Size::Xxl.spacing(), "3rem");
    }

    #[test]
    fn test_size_variants_font_size() {
        assert_eq!(Size::Xs.font_size(), "0.75rem");
        assert_eq!(Size::Sm.font_size(), "0.875rem");
        assert_eq!(Size::Md.font_size(), "1rem");
        assert_eq!(Size::Lg.font_size(), "1.125rem");
        assert_eq!(Size::Xl.font_size(), "1.25rem");
        assert_eq!(Size::Xxl.font_size(), "1.5rem");
    }

    #[test]
    fn test_size_variants_clone() {
        let size = Size::Lg;
        let cloned = size.clone();
        assert_eq!(size, cloned);
    }

    #[test]
    fn test_size_variants_debug() {
        let size = Size::Md;
        let debug_str = format!("{:?}", size);
        assert!(debug_str.contains("Md"));
    }

    #[test]
    fn test_size_variants_partial_eq() {
        let size1 = Size::Lg;
        let size2 = Size::Lg;
        let size3 = Size::Sm;
        
        assert_eq!(size1, size2);
        assert_ne!(size1, size3);
    }

    #[test]
    fn test_size_variants_all_have_valid_classes() {
        let sizes = vec![Size::Xs, Size::Sm, Size::Md, Size::Lg, Size::Xl, Size::Xxl];
        
        for size in sizes {
            let class = size.class();
            assert!(!class.is_empty());
            assert!(class.starts_with("size-"));
            assert!(class.len() > 5); // At least "size-x"
        }
    }

    #[test]
    fn test_size_variants_all_have_valid_spacing() {
        let sizes = vec![Size::Xs, Size::Sm, Size::Md, Size::Lg, Size::Xl, Size::Xxl];
        
        for size in sizes {
            let spacing = size.spacing();
            assert!(!spacing.is_empty());
            assert!(spacing.ends_with("rem"));
            assert!(spacing.len() > 3); // At least "0.25rem"
        }
    }

    #[test]
    fn test_size_variants_all_have_valid_font_sizes() {
        let sizes = vec![Size::Xs, Size::Sm, Size::Md, Size::Lg, Size::Xl, Size::Xxl];
        
        for size in sizes {
            let font_size = size.font_size();
            assert!(!font_size.is_empty());
            assert!(font_size.ends_with("rem"));
            assert!(font_size.len() > 3); // At least "0.75rem"
        }
    }

    #[test]
    fn test_size_variants_spacing_progression() {
        // Test that spacing increases with size
        let xs_spacing = Size::Xs.spacing();
        let sm_spacing = Size::Sm.spacing();
        let md_spacing = Size::Md.spacing();
        let lg_spacing = Size::Lg.spacing();
        let xl_spacing = Size::Xl.spacing();
        let xxl_spacing = Size::Xxl.spacing();
        
        // Convert to numbers for comparison
        let xs_val: f64 = xs_spacing.replace("rem", "").parse().unwrap();
        let sm_val: f64 = sm_spacing.replace("rem", "").parse().unwrap();
        let md_val: f64 = md_spacing.replace("rem", "").parse().unwrap();
        let lg_val: f64 = lg_spacing.replace("rem", "").parse().unwrap();
        let xl_val: f64 = xl_spacing.replace("rem", "").parse().unwrap();
        let xxl_val: f64 = xxl_spacing.replace("rem", "").parse().unwrap();
        
        assert!(xs_val < sm_val);
        assert!(sm_val < md_val);
        assert!(md_val < lg_val);
        assert!(lg_val < xl_val);
        assert!(xl_val < xxl_val);
    }

    #[test]
    fn test_size_variants_font_size_progression() {
        // Test that font sizes increase with size
        let xs_font = Size::Xs.font_size();
        let sm_font = Size::Sm.font_size();
        let md_font = Size::Md.font_size();
        let lg_font = Size::Lg.font_size();
        let xl_font = Size::Xl.font_size();
        let xxl_font = Size::Xxl.font_size();
        
        // Convert to numbers for comparison
        let xs_val: f64 = xs_font.replace("rem", "").parse().unwrap();
        let sm_val: f64 = sm_font.replace("rem", "").parse().unwrap();
        let md_val: f64 = md_font.replace("rem", "").parse().unwrap();
        let lg_val: f64 = lg_font.replace("rem", "").parse().unwrap();
        let xl_val: f64 = xl_font.replace("rem", "").parse().unwrap();
        let xxl_val: f64 = xxl_font.replace("rem", "").parse().unwrap();
        
        assert!(xs_val < sm_val);
        assert!(sm_val < md_val);
        assert!(md_val < lg_val);
        assert!(lg_val < xl_val);
        assert!(xl_val < xxl_val);
    }
}