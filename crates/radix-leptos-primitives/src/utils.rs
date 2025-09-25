/// Utility function to merge CSS classes
pub fn merge_classes(classes: Vec<&str>) -> String {
    classes
        .into_iter()
        .filter(|class| !class.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

/// Performance-optimized CSS class merging with caching
pub fn merge_classes_optimized(classes: &[&str]) -> String {
    if classes.is_empty() {
        return String::new();
    }

    if classes.len() == 1 {
        return classes[0].to_string();
    }

    // Use string cache for frequently used combinations
    crate::performance::get_global_string_cache().get_or_insert(&classes.join(" "))
}

/// Utility function to merge optional CSS classes
pub fn merge_optional_classes(existing: Option<&str>, additional: Option<&str>) -> Option<String> {
    match (existing, additional) {
        (Some(existing), Some(additional)) => {
            if existing.is_empty() && additional.is_empty() {
                None
            } else if existing.is_empty() {
                Some(additional.to_string())
            } else if additional.is_empty() {
                Some(existing.to_string())
            } else {
                Some(format!("{} {}", existing, additional))
            }
        }
        (Some(existing), None) => {
            if existing.is_empty() {
                None
            } else {
                Some(existing.to_string())
            }
        }
        (None, Some(additional)) => {
            if additional.is_empty() {
                None
            } else {
                Some(additional.to_string())
            }
        }
        (None, None) => None,
    }
}

/// Utility function to generate unique IDs
pub fn generate_id(prefix: &str) -> String {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("{}-{}", prefix, id)
}
