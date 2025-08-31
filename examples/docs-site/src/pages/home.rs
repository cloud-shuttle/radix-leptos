use leptos::*;
use crate::components::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div>
            <header style="margin-bottom: 3rem;">
                <h1 style="font-size: 3rem; margin-bottom: 1rem; background: linear-gradient(135deg, #6366f1 0%, #8b5cf6 100%); -webkit-background-clip: text; -webkit-text-fill-color: transparent; background-clip: text;">
                    "Radix-Leptos"
                </h1>
                <p style="font-size: 1.25rem; color: var(--color-text-muted); max-width: 600px;">
                    "Accessible, unstyled UI components for Leptos applications. Built with Rust's type safety and Leptos's fine-grained reactivity."
                </p>
                <div style="margin-top: 2rem;">
                    <a href="/getting-started" class="btn btn-primary" style="margin-right: 1rem;">
                        "Get Started"
                    </a>
                    <a href="/components" class="btn btn-secondary">
                        "Browse Components"
                    </a>
                </div>
            </header>

            <section style="margin-bottom: 4rem;">
                <h2 style="margin-bottom: 2rem;">"Why Radix-Leptos?"</h2>
                <div class="example-grid">
                    <div class="example-card">
                        <div class="example-preview">
                            <div style="text-align: center; color: var(--color-primary);">
                                <svg width="48" height="48" viewBox="0 0 24 24" fill="currentColor">
                                    <path d="M12 2C13.1 2 14 2.9 14 4C14 5.1 13.1 6 12 6C10.9 6 10 5.1 10 4C10 2.9 10.9 2 12 2ZM21 9V7L15 7.5V9M15 11.5L21 12V10L15 9.5M21 16V14L15 14.5V16M15 18.5L21 19V17L15 16.5M9 7H3V9H9V7ZM9 10H3V12H9V10ZM9 13H3V15H9V13ZM9 16H3V18H9V16Z"/>
                                </svg>
                            </div>
                        </div>
                        <div class="example-info">
                            <div class="example-title">"Accessibility First"</div>
                            <div class="example-description">
                                "Built with WAI-ARIA patterns, keyboard navigation, and screen reader support from the ground up."
                            </div>
                        </div>
                    </div>

                    <div class="example-card">
                        <div class="example-preview">
                            <div style="text-align: center; color: var(--color-primary);">
                                <svg width="48" height="48" viewBox="0 0 24 24" fill="currentColor">
                                    <path d="M12,2A3,3 0 0,1 15,5V11A3,3 0 0,1 12,14A3,3 0 0,1 9,11V5A3,3 0 0,1 12,2M19,11C19,14.53 16.39,17.44 13,17.93V21H11V17.93C7.61,17.44 5,14.53 5,11H7A5,5 0 0,0 12,16A5,5 0 0,0 17,11H19Z"/>
                                </svg>
                            </div>
                        </div>
                        <div class="example-info">
                            <div class="example-title">"Type Safe"</div>
                            <div class="example-description">
                                "Leverage Rust's type system for compile-time guarantees and better developer experience."
                            </div>
                        </div>
                    </div>

                    <div class="example-card">
                        <div class="example-preview">
                            <div style="text-align: center; color: var(--color-primary);">
                                <svg width="48" height="48" viewBox="0 0 24 24" fill="currentColor">
                                    <path d="M12,16A3,3 0 0,1 9,13C9,11.88 9.61,10.9 10.5,10.39L20.21,4.77L14.68,14.35C14.18,15.33 13.17,16 12,16M12,3C13.81,3 15.5,3.5 16.97,4.32L14.87,5.53C14,5.19 13,5 12,5A7,7 0 0,0 5,12C5,15.04 6.91,17.62 9.66,18.51L9.89,20.49C5.94,19.38 3,15.98 3,12A9,9 0 0,1 12,3M18.18,6.2C19.91,8.2 21,10.92 21,14C21,17.23 18.96,19.85 16,20.69L14.18,19.43C15.83,18.65 17,16.95 17,15C17,13.55 16.4,12.21 15.46,11.19L18.18,6.2Z"/>
                                </svg>
                            </div>
                        </div>
                        <div class="example-info">
                            <div class="example-title">"Performant"</div>
                            <div class="example-description">
                                "Built on Leptos's fine-grained reactivity system for optimal rendering performance."
                            </div>
                        </div>
                    </div>

                    <div class="example-card">
                        <div class="example-preview">
                            <div style="text-align: center; color: var(--color-primary);">
                                <svg width="48" height="48" viewBox="0 0 24 24" fill="currentColor">
                                    <path d="M12,2A10,10 0 0,0 2,12A10,10 0 0,0 12,22A10,10 0 0,0 22,12A10,10 0 0,0 12,2M11,14L6.5,9.5L7.91,8.09L11,11.18L16.09,6.09L17.5,7.5L11,14Z"/>
                                </svg>
                            </div>
                        </div>
                        <div class="example-info">
                            <div class="example-title">"Unstyled"</div>
                            <div class="example-description">
                                "Complete control over styling with data attributes and CSS custom properties."
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            <section style="margin-bottom: 4rem;">
                <h2 style="margin-bottom: 2rem;">"Quick Example"</h2>
                
                <ComponentPlayground
                    title="Dialog Component"
                    description="A modal dialog with focus management and accessibility features."
                    code=r#"use leptos::*;
use radix_leptos::*;

#[component]
fn DialogExample() -> impl IntoView {
    view! {
        <DialogRoot>
            <DialogTrigger class="btn btn-primary">
                "Open Dialog"
            </DialogTrigger>
            <DialogContent class="dialog-content">
                <DialogTitle class="dialog-title">
                    "Edit Profile"
                </DialogTitle>
                <DialogDescription class="dialog-description">
                    "Make changes to your profile here."
                </DialogDescription>
                <div class="dialog-body">
                    <label>
                        "Name"
                        <input type="text" />
                    </label>
                </div>
                <div class="dialog-footer">
                    <DialogClose class="btn btn-secondary">
                        "Cancel"
                    </DialogClose>
                    <button class="btn btn-primary">
                        "Save Changes"
                    </button>
                </div>
            </DialogContent>
        </DialogRoot>
    }
}"#
                >
                    <div style="text-align: center;">
                        <div style="padding: 2rem; border: 2px dashed var(--color-border); border-radius: var(--border-radius); color: var(--color-text-muted);">
                            "Dialog component will render here once implemented"
                        </div>
                    </div>
                </ComponentPlayground>
            </section>

            <section>
                <h2 style="margin-bottom: 2rem;">"Getting Started"</h2>
                <p style="margin-bottom: 2rem; font-size: 1.1rem;">
                    "Ready to build accessible applications with Radix-Leptos? Follow our getting started guide to set up your first project."
                </p>
                
                <div style="background-color: var(--color-surface); padding: 2rem; border-radius: var(--border-radius); border-left: 4px solid var(--color-primary);">
                    <h3 style="margin-top: 0;">"Installation"</h3>
                    <CodeBlock 
                        code=r#"[dependencies]
radix-leptos = "0.1"
leptos = { version = "0.6", features = ["csr"] }"#
                        language="toml"
                    />
                    <p style="margin-bottom: 0;">
                        "Add Radix-Leptos to your Cargo.toml and start building accessible components right away."
                    </p>
                </div>
            </section>
        </div>
    }
}