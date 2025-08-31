use leptos::*;
use uuid::Uuid;

/// Hook for generating stable, unique IDs for component accessibility
/// 
/// This hook generates a unique ID that remains stable across re-renders,
/// essential for ARIA attributes like `aria-labelledby` and `aria-describedby`.
/// 
/// # Arguments
/// 
/// * `prefix` - Optional prefix for the generated ID
/// 
/// # Example
/// 
/// ```rust
/// use leptos::*;
/// use radix_leptos_core::use_id;
/// 
/// #[component]
/// pub fn FormField() -> impl IntoView {
///     let field_id = use_id(Some("field"));
///     let label_id = use_id(Some("label"));
///     let desc_id = use_id(Some("description"));
///     
///     view! {
///         <div>
///             <label id=label_id for=field_id>"Email"</label>
///             <input 
///                 id=field_id 
///                 aria-labelledby=label_id
///                 aria-describedby=desc_id
///             />
///             <div id=desc_id>"Please enter a valid email address"</div>
///         </div>
///     }
/// }
/// ```
pub fn use_id(prefix: Option<String>) -> Signal<String> {
    let id = create_memo(move |_| {
        let uuid = Uuid::new_v4().to_string();
        let short_id = &uuid[..8]; // Use first 8 characters
        
        match &prefix {
            Some(p) => format!("{}-{}", p, short_id),
            None => format!("radix-{}", short_id),
        }
    });
    
    id.into()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_generates_unique_ids() {
        run_test(|cx| {
            let id1 = use_id(None);
            let id2 = use_id(None);
            
            // Should generate different IDs
            assert_ne!(id1.get(), id2.get());
            
            // Should start with "radix-"
            assert!(id1.get().starts_with("radix-"));
            assert!(id2.get().starts_with("radix-"));
        });
    }
    
    #[test]
    fn test_custom_prefix() {
        run_test(|cx| {
            let id = use_id(Some("button"));
            
            // Should start with custom prefix
            assert!(id.get().starts_with("button-"));
        });
    }
    
    #[test]
    fn test_id_stability() {
        run_test(|cx| {
            let id = use_id(None);
            let initial_id = id.get();
            
            // ID should remain stable
            assert_eq!(id.get(), initial_id);
        });
    }
    
    fn run_test<F>(f: F) where F: FnOnce(Scope) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let _ = create_runtime();
            run_scope(create_runtime(), f);
        });
    }
}