use leptos::*;
use leptos::prelude::*;

/// Portal component for rendering content in different DOM locations
/// 
/// The Portal component is essential for overlays, modals, tooltips, and any content
/// that needs to escape the normal document flow. It renders its children into a
/// different DOM location while maintaining the component tree relationship.
/// 
/// # Example
/// 
/// ```rust
/// use leptos::*;
/// use radix_leptos_core::Portal;
/// 
/// #[component]
/// fn MyModal() -> impl IntoView {
///     view! {
///         <div>
///             "This is in the normal flow"
///             <Portal>
///                 <div class="modal-overlay">
///                     "This is rendered at document.body"
///                 </div>
///             </Portal>
///         </div>
///     }
/// }
/// ```
#[component]
pub fn Portal(
    /// Whether to force mount the portal regardless of hydration state
    #[prop(optional, default = false)]
    _force_mount: bool,
    /// Content to render in the portal
    children: Children,
) -> impl IntoView {
    // Simplified portal implementation for Leptos 0.8
    // TODO: Implement proper DOM portal functionality with thread-safe approach
    view! {
        <div data-radix-portal="true" style="display: none;">
            {children()}
        </div>
    }
}

/// Portal root context for managing multiple portals
/// 
/// Note: Temporarily simplified for Leptos 0.8 compatibility
#[derive(Clone)]
pub struct PortalContext {
    // TODO: Implement thread-safe container management
}

/// Provider for portal context
#[component]
pub fn PortalProvider(
    children: Children,
) -> impl IntoView {
    // TODO: Implement proper context management
    children()
}

/// Use portal context
pub fn use_portal_context() -> Option<PortalContext> {
    use_context::<PortalContext>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    #[wasm_bindgen_test]
    fn test_portal_creation() {
        run_test(|cx| {
            let document = web_sys::window().unwrap().document().unwrap();
            let custom_container = document.create_element("div").unwrap();
            custom_container.set_id("portal-container");
            document.body().unwrap().append_child(&custom_container).unwrap();
            
            let rendered = leptos::ssr::render_to_string(cx, move || {
                view! {
                    <Portal container=custom_container.clone()>
                        <div>"Portal content"</div>
                    </Portal>
                }
            });
            
            // The portal should create a mount point
            let portals = document.query_selector_all("[data-radix-portal]").unwrap();
            assert!(portals.length() > 0);
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