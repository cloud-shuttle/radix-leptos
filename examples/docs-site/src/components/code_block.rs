use leptos::*;

#[component]
pub fn CodeBlock(
    #[prop(into)] code: String,
    #[prop(into, default = "rust".to_string())] language: String,
) -> impl IntoView {
    view! {
        <pre class="code-block">
            <code class=format!("language-{}", language)>
                {code}
            </code>
        </pre>
    }
}

#[component] 
pub fn InlineCode(
    #[prop(into)] code: String,
) -> impl IntoView {
    view! {
        <code>{code}</code>
    }
}