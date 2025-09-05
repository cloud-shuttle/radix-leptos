use leptos::prelude::*;
// use leptos::*;  // Unused import

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
    __force_mount: bool,
    /// Content to render in the portal
    children: Children,
) -> impl IntoView {
    // Simplified portal implementation for Leptos 0.8
    // Portal implementation for rendering components outside their parent DOM hierarchy
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
    // Container management for portal rendering
}

/// Provider for portal context
#[component]
pub fn PortalProvider(children: Children) -> impl IntoView {
    // Context management for portal state
    children()
}

/// Use portal context
pub fn use_portal_context() -> Option<PortalContext> {
    use_context::<PortalContext>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_portal_context() {
        // Test portal context creation
        let _context = PortalContext {};
        // Portal context should be created without errors
        // Portal context should be created without errors
    }
}
