use super::PaginationPage;
use crate::utils::{merge_optional_classes, generate_id};



/// Calculate visible page range
pub fn calculate_page_range(
    current_page: usize,
    total_pages: usize,
    maxvisible: usize,
) -> (usize, usize) {
    if total_pages <= maxvisible {
        return (1, total_pages);
    }

    let halfvisible = maxvisible / 2;
    let mut start = current_page.saturating_sub(halfvisible);
    let mut end = start + maxvisible - 1;

    if end > total_pages {
        end = total_pages;
        start = end.saturating_sub(maxvisible - 1);
    }

    (start, end)
}

/// Helper function to generate page numbers for pagination
pub fn generate_page_numbers(
    current_page: usize,
    total_pages: usize,
    maxvisible: usize,
) -> Vec<PaginationPage> {
    if total_pages <= maxvisible {
        return (1..=total_pages)
            .map(|page| PaginationPage::new(page).withcurrent(page == current_page))
            .collect();
    }

    let (start, end) = calculate_page_range(current_page, total_pages, maxvisible);
    let mut pages = Vec::new();

    // Add first page if not in range
    if start > 1 {
        pages.push(PaginationPage::new(1));
        if start > 2 {
            pages.push(PaginationPage::new(0).withdisabled(true)); // Placeholder for ellipsis
        }
    }

    // Add visible pages
    for page in start..=end {
        pages.push(PaginationPage::new(page).withcurrent(page == current_page));
    }

    // Add last page if not in range
    if end < total_pages {
        if end < total_pages - 1 {
            pages.push(PaginationPage::new(0).withdisabled(true)); // Placeholder for ellipsis
        }
        pages.push(PaginationPage::new(total_pages));
    }

    pages
}

/// Helper function to generate page numbers for pagination
/// This function returns a vector of page numbers that should be displayed
/// It handles ellipsis for large page counts
pub fn getvisible_page_numbers(
    current_page: usize,
    total_pages: usize,
    maxvisible: usize,
) -> Vec<usize> {
    if total_pages <= maxvisible {
        return (1..=total_pages).collect();
    }

    let (start, end) = calculate_page_range(current_page, total_pages, maxvisible);
    let mut pages = Vec::new();

    // Add first page if not in range
    if start > 1 {
        pages.push(1);
        if start > 2 {
            pages.push(0); // Placeholder for ellipsis
        }
    }

    // Add visible pages
    for page in start..=end {
        pages.push(page);
    }

    // Add last page if not in range
    if end < total_pages {
        if end < total_pages - 1 {
            pages.push(0); // Placeholder for ellipsis
        }
        pages.push(total_pages);
    }

    pages
}

#[cfg(test)]
mod helpers_tests {
    use super::*;
use crate::utils::{merge_optional_classes, generate_id};

    #[test]
    fn test_generate_id() {
        let id1 = generate_id("test");
        let id2 = generate_id("test");
        assert!(id1 != id2);
        assert!(id1.starts_with("test-"));
        assert!(id2.starts_with("test-"));
    }

    #[test]
    fn test_merge_optional_classes() {
        assert_eq!(
            merge_optional_classes(Some("class1"), Some("class2")),
            Some("class1 class2".to_string())
        );
        assert_eq!(
            merge_optional_classes(Some("class1"), None),
            Some("class1".to_string())
        );
        assert_eq!(
            merge_optional_classes(None, Some("class2")),
            Some("class2".to_string())
        );
        assert_eq!(merge_optional_classes(None, None), None);
    }

    #[test]
    fn test_calculate_page_range() {
        // Test case where total pages <= max visible
        let (start, end) = calculate_page_range(1, 5, 10);
        assert_eq!(start, 1);
        assert_eq!(end, 5);

        // Test case where current page is in the middle
        let (start, end) = calculate_page_range(5, 20, 7);
        assert_eq!(start, 2);
        assert_eq!(end, 8);

        // Test case where current page is near the end
        let (start, end) = calculate_page_range(18, 20, 7);
        assert_eq!(start, 14);
        assert_eq!(end, 20);
    }

    #[test]
    fn test_generate_page_numbers() {
        let pages = generate_page_numbers(5, 10, 5);
        assert!(!pages.is_empty());
        
        // Should contain current page
        let current_page = pages.iter().find(|p| p._current);
        assert!(current_page.is_some());
        assert_eq!(current_page.unwrap().number, 5);
    }

    #[test]
    fn test_getvisible_page_numbers() {
        let pages = getvisible_page_numbers(5, 10, 5);
        assert!(!pages.is_empty());
        assert!(pages.contains(&5)); // Should contain current page
    }
}
