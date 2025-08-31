use leptos::*;

#[component]
pub fn ComponentPlayground(
    #[prop(into)] title: String,
    #[prop(into)] description: String,
    #[prop(into)] code: String,
    children: Children,
) -> impl IntoView {
    let (active_tab, set_active_tab) = create_signal("preview".to_string());
    
    view! {
        <div class="playground">
            <div class="playground-tabs">
                <button 
                    class=move || {
                        if active_tab.get() == "preview" { 
                            "playground-tab active" 
                        } else { 
                            "playground-tab" 
                        }
                    }
                    on:click=move |_| set_active_tab.set("preview".to_string())
                >
                    "Preview"
                </button>
                <button 
                    class=move || {
                        if active_tab.get() == "code" { 
                            "playground-tab active" 
                        } else { 
                            "playground-tab" 
                        }
                    }
                    on:click=move |_| set_active_tab.set("code".to_string())
                >
                    "Code"
                </button>
            </div>

            <Show
                when=move || active_tab.get() == "preview"
                fallback=move || view! {
                    <div class="playground-code">
                        <CodeBlock code=code.clone() language="rust"/>
                    </div>
                }
            >
                <div class="playground-preview">
                    {children()}
                </div>
            </Show>
        </div>
        
        <div style="margin-top: 1rem;">
            <h3>{title}</h3>
            <p style="color: var(--color-text-muted);">{description}</p>
        </div>
    }
}

#[component]
pub fn ExampleCard(
    #[prop(into)] title: String,
    #[prop(into)] description: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="example-card">
            <div class="example-preview">
                {children()}
            </div>
            <div class="example-info">
                <div class="example-title">{title}</div>
                <div class="example-description">{description}</div>
            </div>
        </div>
    }
}