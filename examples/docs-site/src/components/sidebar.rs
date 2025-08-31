use leptos::*;
use leptos_router::*;

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <div>
            // Logo/Brand
            <div style="margin-bottom: 2rem;">
                <h2 style="margin: 0; font-size: 1.5rem; font-weight: 700;">
                    "Radix-Leptos"
                </h2>
                <p style="margin: 0.5rem 0 0 0; font-size: 0.875rem; color: var(--color-text-muted);">
                    "v0.1.0-alpha"
                </p>
            </div>

            // Getting Started
            <div class="nav-section">
                <div class="nav-title">"Getting Started"</div>
                <A href="/" class="nav-link">"Overview"</A>
                <A href="/getting-started" class="nav-link">"Installation"</A>
            </div>

            // Core Concepts
            <div class="nav-section">
                <div class="nav-title">"Core Concepts"</div>
                <A href="/accessibility" class="nav-link">"Accessibility"</A>
                <A href="/styling" class="nav-link">"Styling"</A>
                <A href="/composition" class="nav-link">"Composition"</A>
            </div>

            // Components
            <div class="nav-section">
                <div class="nav-title">"Components"</div>
                
                // Core Primitives
                <div style="margin: 0.5rem 0;">
                    <div style="font-size: 0.75rem; color: var(--color-text-muted); margin-bottom: 0.5rem;">
                        "PRIMITIVES"
                    </div>
                    <A href="/components/portal" class="nav-link">"Portal"</A>
                    <A href="/components/slot" class="nav-link">"Slot"</A>
                    <A href="/components/visually-hidden" class="nav-link">"VisuallyHidden"</A>
                    <A href="/components/presence" class="nav-link">"Presence"</A>
                </div>

                // Form Components
                <div style="margin: 0.5rem 0;">
                    <div style="font-size: 0.75rem; color: var(--color-text-muted); margin-bottom: 0.5rem;">
                        "FORM"
                    </div>
                    <A href="/components/button" class="nav-link">"Button"</A>
                    <A href="/components/label" class="nav-link">"Label"</A>
                    <A href="/components/switch" class="nav-link">"Switch"</A>
                    <A href="/components/checkbox" class="nav-link">"Checkbox"</A>
                    <A href="/components/radio-group" class="nav-link">"RadioGroup"</A>
                </div>

                // Overlay Components
                <div style="margin: 0.5rem 0;">
                    <div style="font-size: 0.75rem; color: var(--color-text-muted); margin-bottom: 0.5rem;">
                        "OVERLAY"
                    </div>
                    <A href="/components/dialog" class="nav-link">"Dialog"</A>
                    <A href="/components/popover" class="nav-link">"Popover"</A>
                    <A href="/components/tooltip" class="nav-link">"Tooltip"</A>
                    <A href="/components/dropdown-menu" class="nav-link">"DropdownMenu"</A>
                </div>

                // Layout Components
                <div style="margin: 0.5rem 0;">
                    <div style="font-size: 0.75rem; color: var(--color-text-muted); margin-bottom: 0.5rem;">
                        "LAYOUT"
                    </div>
                    <A href="/components/accordion" class="nav-link">"Accordion"</A>
                    <A href="/components/tabs" class="nav-link">"Tabs"</A>
                    <A href="/components/collapsible" class="nav-link">"Collapsible"</A>
                </div>
            </div>

            // Playground
            <div class="nav-section">
                <div class="nav-title">"Tools"</div>
                <A href="/playground" class="nav-link">"Playground"</A>
                <A href="/examples" class="nav-link">"Examples"</A>
            </div>
        </div>
    }
}