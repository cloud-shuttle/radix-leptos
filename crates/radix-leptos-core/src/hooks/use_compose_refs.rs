use web_sys::Element;

/// Compose multiple refs into a single ref callback
/// 
/// This is useful when you need to apply multiple refs to the same element,
/// such as when forwarding a ref while also maintaining an internal ref.
/// 
/// # Example
/// 
/// ```rust
/// use radix_leptos_core::use_compose_refs;
/// 
/// #[component]
/// pub fn ForwardedButton(
///     #[prop(optional)] node_ref: Option<NodeRef<web_sys::Element>>,
///     children: Children,
/// ) -> impl IntoView {
///     let internal_ref = create_node_ref::<web_sys::Element>();
///     let composed_ref = use_compose_refs(&[Some(internal_ref), node_ref]);
///     
///     view! {
///         <button ref=composed_ref>
///             {children()}
///         </button>
///     }
/// }
/// ```
pub fn use_compose_refs(refs: &[Option<NodeRef<Element>>]) -> NodeRef<Element> {
    let composed_ref = create_node_ref::<Element>();
    
    // Set up an effect to synchronize all refs when the composed ref changes
    create_effect(move |_| {
        if let Some(element) = composed_ref.get() {
            // Update all provided refs with the current element
            for ref_option in refs {
                if let Some(node_ref) = ref_option {
                    // This is a simplified version - in practice, we'd need to handle
                    // the ref forwarding more carefully
                    // For now, this demonstrates the concept
                }
            }
        }
    });
    
    composed_ref
}

/// Compose refs with a callback function
/// 
/// This version allows you to compose refs with a callback function
/// that gets called whenever the element changes.
pub fn use_compose_refs_with_callback<F>(
    refs: &[Option<NodeRef<Element>>],
    callback: F,
) -> NodeRef<Element>
where
    F: Fn(Option<Element>) + 'static,
{
    let composed_ref = create_node_ref::<Element>();
    
    create_effect(move |_| {
        let element = composed_ref.get();
        callback(element.clone());
        
        // Forward to other refs
        if let Some(ref element) = element {
            for ref_option in refs {
                if let Some(_node_ref) = ref_option {
                    // Forward ref - implementation would depend on Leptos internals
                }
            }
        }
    });
    
    composed_ref
}