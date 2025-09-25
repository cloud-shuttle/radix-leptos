use crate::theming::CSSVariables;
use leptos::prelude::*;
use leptos::serde_json;

/// Theme export preview component
#[component]
pub fn ThemeExportPreview(
    /// Theme to preview
    theme: ReadSignal<CSSVariables>,
    /// Additional CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// Inline styles
    #[prop(optional)]
    style: Option<String>,
) -> impl IntoView {
    let class = format!("theme-preview {}", class.unwrap_or_default());

    view! {
        <div class=class style=style>
            <div class="preview-header">
                <h3>"Theme Preview"</h3>
            </div>
            <div class="preview-content">
                <div class="preview-section">
                    <h4>"Colors"</h4>
                    <div class="color-preview">
                        <div class="color-swatch" style=move || format!("background-color: {}", theme.get().primary.primary_500)></div>
                        <div class="color-swatch" style=move || format!("background-color: {}", theme.get().secondary.secondary_500)></div>
                        <div class="color-swatch" style=move || format!("background-color: {}", theme.get().neutral.neutral_50)></div>
                        <div class="color-swatch" style=move || format!("background-color: {}", theme.get().semantic.error)></div>
                    </div>
                </div>
                <div class="preview-section">
                    <h4>"Typography"</h4>
                    <div class="typography-preview">
                        <p style=move || format!("font-size: {}", theme.get().typography.font_size_base)>"Base text"</p>
                        <p style=move || format!("font-size: {}", theme.get().typography.font_size_lg)>"Large text"</p>
                        <p style=move || format!("font-size: {}", theme.get().typography.font_size_xl)>"Extra large text"</p>
                    </div>
                </div>
                <div class="preview-section">
                    <h4>"Spacing"</h4>
                    <div class="spacing-preview">
                        <div class="spacing-box" style=move || format!("margin: {}", theme.get().spacing.space_2)>"Small"</div>
                        <div class="spacing-box" style=move || format!("margin: {}", theme.get().spacing.space_4)>"Medium"</div>
                        <div class="spacing-box" style=move || format!("margin: {}", theme.get().spacing.space_8)>"Large"</div>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Export theme as JSON string
pub fn export_theme_json(theme: &CSSVariables) -> String {
    serde_json::to_string(theme).unwrap_or_default()
}

/// Export theme as CSS variables
pub fn export_theme_css(theme: &CSSVariables) -> String {
    theme.to_css_string()
}

/// Export theme as JavaScript object
pub fn export_theme_js(theme: &CSSVariables) -> String {
    format!("const theme = {};", serde_json::to_string(theme).unwrap_or_default())
}
