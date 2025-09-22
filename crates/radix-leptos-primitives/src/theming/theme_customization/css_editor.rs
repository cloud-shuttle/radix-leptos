use crate::theming::CSSVariables;
use leptos::callback::Callback;
use leptos::prelude::*;

/// Color customizer component
#[component]
pub fn ColorCustomizer(
    /// Current theme
    theme: ReadSignal<CSSVariables>,
    /// Callback when theme changes
    on_change: Callback<CSSVariables>,
) -> impl IntoView {
    let handle_color_change = move |color_type: &str, value: String| {
        let mut new_theme = theme.get();
        match color_type {
            "primary" => new_theme.primary.primary_500 = value,
            "primary-foreground" => new_theme.primary.primary_50 = value,
            "secondary" => new_theme.secondary.secondary_500 = value,
            "secondary-foreground" => new_theme.secondary.secondary_50 = value,
            "background" => new_theme.neutral.neutral_50 = value,
            "foreground" => new_theme.neutral.neutral_900 = value,
            "destructive" => new_theme.semantic.error = value,
            "destructive-foreground" => new_theme.semantic.success = value,
            _ => {}
        }
        on_change.run(new_theme);
    };

    view! {
        <div class="color-customizer">
            <h4>"Colors"</h4>
            <div class="color-controls">
                <div class="color-control">
                    <label>"Primary"</label>
                    <input
                        type="color"
                        value=move || theme.get().primary.primary_500
                        on:change=move |ev| {
                            let value = event_target_value(&ev);
                            handle_color_change("primary", value);
                        }
                    />
                </div>
                <div class="color-control">
                    <label>"Primary Foreground"</label>
                    <input
                        type="color"
                        value=move || theme.get().primary.primary_50
                        on:change=move |ev| {
                            let value = event_target_value(&ev);
                            handle_color_change("primary-foreground", value);
                        }
                    />
                </div>
                <div class="color-control">
                    <label>"Secondary"</label>
                    <input
                        type="color"
                        value=move || theme.get().secondary.secondary_500
                        on:change=move |ev| {
                            let value = event_target_value(&ev);
                            handle_color_change("secondary", value);
                        }
                    />
                </div>
                <div class="color-control">
                    <label>"Background"</label>
                    <input
                        type="color"
                        value=move || theme.get().neutral.neutral_50
                        on:change=move |ev| {
                            let value = event_target_value(&ev);
                            handle_color_change("background", value);
                        }
                    />
                </div>
                <div class="color-control">
                    <label>"Foreground"</label>
                    <input
                        type="color"
                        value=move || theme.get().neutral.neutral_900
                        on:change=move |ev| {
                            let value = event_target_value(&ev);
                            handle_color_change("foreground", value);
                        }
                    />
                </div>
                <div class="color-control">
                    <label>"Destructive"</label>
                    <input
                        type="color"
                        value=move || theme.get().semantic.error
                        on:change=move |ev| {
                            let value = event_target_value(&ev);
                            handle_color_change("destructive", value);
                        }
                    />
                </div>
            </div>
        </div>
    }
}

/// Typography customizer component
#[component]
pub fn TypographyCustomizer(
    /// Current theme
    theme: ReadSignal<CSSVariables>,
    /// Callback when theme changes
    on_change: Callback<CSSVariables>,
) -> impl IntoView {
    let handle_typography_change = move |typography_type: &str, value: String| {
        let mut new_theme = theme.get();
        match typography_type {
            "font-size-base" => new_theme.typography.font_size_base = value,
            "font-size-lg" => new_theme.typography.font_size_lg = value,
            "font-size-xl" => new_theme.typography.font_size_xl = value,
            "font-weight-normal" => new_theme.typography.font_weight_normal = value,
            "font-weight-medium" => new_theme.typography.font_weight_medium = value,
            "font-weight-bold" => new_theme.typography.font_weight_bold = value,
            "line-height-normal" => new_theme.typography.line_height_normal = value,
            _ => {}
        }
        on_change.run(new_theme);
    };

    view! {
        <div class="typography-customizer">
            <h4>"Typography"</h4>
            <div class="typography-controls">
                <div class="typography-control">
                    <label>"Base Font Size"</label>
                    <input
                        type="text"
                        value=move || theme.get().typography.font_size_base
                        on:change=move |ev| {
                            let value = event_target_value(&ev);
                            handle_typography_change("font-size-base", value);
                        }
                    />
                </div>
                <div class="typography-control">
                    <label>"Large Font Size"</label>
                    <input
                        type="text"
                        value=move || theme.get().typography.font_size_lg
                        on:change=move |ev| {
                            let value = event_target_value(&ev);
                            handle_typography_change("font-size-lg", value);
                        }
                    />
                </div>
                <div class="typography-control">
                    <label>"Extra Large Font Size"</label>
                    <input
                        type="text"
                        value=move || theme.get().typography.font_size_xl
                        on:change=move |ev| {
                            let value = event_target_value(&ev);
                            handle_typography_change("font-size-xl", value);
                        }
                    />
                </div>
                <div class="typography-control">
                    <label>"Normal Font Weight"</label>
                    <select
                        prop:value=move || theme.get().typography.font_weight_normal
                        on:change=move |ev| {
                            let value = event_target_value(&ev);
                            handle_typography_change("font-weight-normal", value);
                        }
                    >
                        <option value="300">"Light"</option>
                        <option value="400">"Normal"</option>
                        <option value="500">"Medium"</option>
                        <option value="600">"Semibold"</option>
                        <option value="700">"Bold"</option>
                    </select>
                </div>
                <div class="typography-control">
                    <label>"Medium Font Weight"</label>
                    <select
                        prop:value=move || theme.get().typography.font_weight_medium
                        on:change=move |ev| {
                            let value = event_target_value(&ev);
                            handle_typography_change("font-weight-medium", value);
                        }
                    >
                        <option value="300">"Light"</option>
                        <option value="400">"Normal"</option>
                        <option value="500">"Medium"</option>
                        <option value="600">"Semibold"</option>
                        <option value="700">"Bold"</option>
                    </select>
                </div>
                <div class="typography-control">
                    <label>"Bold Font Weight"</label>
                    <select
                        prop:value=move || theme.get().typography.font_weight_bold
                        on:change=move |ev| {
                            let value = event_target_value(&ev);
                            handle_typography_change("font-weight-bold", value);
                        }
                    >
                        <option value="300">"Light"</option>
                        <option value="400">"Normal"</option>
                        <option value="500">"Medium"</option>
                        <option value="600">"Semibold"</option>
                        <option value="700">"Bold"</option>
                    </select>
                </div>
                <div class="typography-control">
                    <label>"Line Height"</label>
                    <input
                        type="text"
                        value=move || theme.get().typography.line_height_normal
                        on:change=move |ev| {
                            let value = event_target_value(&ev);
                            handle_typography_change("line-height-normal", value);
                        }
                    />
                </div>
            </div>
        </div>
    }
}

/// Spacing customizer component
#[component]
pub fn SpacingCustomizer(
    /// Current theme
    theme: ReadSignal<CSSVariables>,
    /// Callback when theme changes
    on_change: Callback<CSSVariables>,
) -> impl IntoView {
    let handle_spacing_change = move |spacing_type: &str, value: String| {
        let mut new_theme = theme.get();
        match spacing_type {
            "spacing-sm" => new_theme.spacing.space_2 = value,
            "spacing-md" => new_theme.spacing.space_4 = value,
            "spacing-lg" => new_theme.spacing.space_8 = value,
            "spacing-xl" => new_theme.spacing.space_16 = value,
            _ => {}
        }
        on_change.run(new_theme);
    };

    view! {
        <div class="spacing-customizer">
            <h4>"Spacing"</h4>
            <div class="spacing-controls">
                <div class="spacing-control">
                    <label>"Small Spacing"</label>
                    <input
                        type="text"
                        value=move || theme.get().spacing.space_2
                        on:change=move |ev| {
                            let value = event_target_value(&ev);
                            handle_spacing_change("spacing-sm", value);
                        }
                    />
                </div>
                <div class="spacing-control">
                    <label>"Medium Spacing"</label>
                    <input
                        type="text"
                        value=move || theme.get().spacing.space_4
                        on:change=move |ev| {
                            let value = event_target_value(&ev);
                            handle_spacing_change("spacing-md", value);
                        }
                    />
                </div>
                <div class="spacing-control">
                    <label>"Large Spacing"</label>
                    <input
                        type="text"
                        value=move || theme.get().spacing.space_8
                        on:change=move |ev| {
                            let value = event_target_value(&ev);
                            handle_spacing_change("spacing-lg", value);
                        }
                    />
                </div>
                <div class="spacing-control">
                    <label>"Extra Large Spacing"</label>
                    <input
                        type="text"
                        value=move || theme.get().spacing.space_16
                        on:change=move |ev| {
                            let value = event_target_value(&ev);
                            handle_spacing_change("spacing-xl", value);
                        }
                    />
                </div>
            </div>
        </div>
    }
}

/// Border radius customizer component
#[component]
pub fn BorderRadiusCustomizer(
    /// Current theme
    theme: ReadSignal<CSSVariables>,
    /// Callback when theme changes
    on_change: Callback<CSSVariables>,
) -> impl IntoView {
    let handle_border_radius_change = move |radius_type: &str, value: String| {
        let mut new_theme = theme.get();
        match radius_type {
            "radius-sm" => new_theme.border.border_radius_sm = value,
            "radius-md" => new_theme.border.border_radius_md = value,
            "radius-lg" => new_theme.border.border_radius_lg = value,
            "radius-xl" => new_theme.border.border_radius_xl = value,
            _ => {}
        }
        on_change.run(new_theme);
    };

    view! {
        <div class="border-radius-customizer">
            <h4>"Border Radius"</h4>
            <div class="border-radius-controls">
                <div class="border-radius-control">
                    <label>"Small Radius"</label>
                    <input
                        type="text"
                        value=move || theme.get().border.border_radius_sm
                        on:change=move |ev| {
                            let value = event_target_value(&ev);
                            handle_border_radius_change("radius-sm", value);
                        }
                    />
                </div>
                <div class="border-radius-control">
                    <label>"Medium Radius"</label>
                    <input
                        type="text"
                        value=move || theme.get().border.border_radius_md
                        on:change=move |ev| {
                            let value = event_target_value(&ev);
                            handle_border_radius_change("radius-md", value);
                        }
                    />
                </div>
                <div class="border-radius-control">
                    <label>"Large Radius"</label>
                    <input
                        type="text"
                        value=move || theme.get().border.border_radius_lg
                        on:change=move |ev| {
                            let value = event_target_value(&ev);
                            handle_border_radius_change("radius-lg", value);
                        }
                    />
                </div>
                <div class="border-radius-control">
                    <label>"Extra Large Radius"</label>
                    <input
                        type="text"
                        value=move || theme.get().border.border_radius_xl
                        on:change=move |ev| {
                            let value = event_target_value(&ev);
                            handle_border_radius_change("radius-xl", value);
                        }
                    />
                </div>
            </div>
        </div>
    }
}

/// Shadow customizer component
#[component]
pub fn ShadowCustomizer(
    /// Current theme
    theme: ReadSignal<CSSVariables>,
    /// Callback when theme changes
    on_change: Callback<CSSVariables>,
) -> impl IntoView {
    let handle_shadow_change = move |shadow_type: &str, value: String| {
        let mut new_theme = theme.get();
        match shadow_type {
            "shadow-sm" => new_theme.shadow.shadow_sm = value,
            "shadow-md" => new_theme.shadow.shadow_md = value,
            "shadow-lg" => new_theme.shadow.shadow_lg = value,
            "shadow-xl" => new_theme.shadow.shadow_xl = value,
            _ => {}
        }
        on_change.run(new_theme);
    };

    view! {
        <div class="shadow-customizer">
            <h4>"Shadows"</h4>
            <div class="shadow-controls">
                <div class="shadow-control">
                    <label>"Small Shadow"</label>
                    <input
                        type="text"
                        value=move || theme.get().shadow.shadow_sm
                        on:change=move |ev| {
                            let value = event_target_value(&ev);
                            handle_shadow_change("shadow-sm", value);
                        }
                    />
                </div>
                <div class="shadow-control">
                    <label>"Medium Shadow"</label>
                    <input
                        type="text"
                        value=move || theme.get().shadow.shadow_md
                        on:change=move |ev| {
                            let value = event_target_value(&ev);
                            handle_shadow_change("shadow-md", value);
                        }
                    />
                </div>
                <div class="shadow-control">
                    <label>"Large Shadow"</label>
                    <input
                        type="text"
                        value=move || theme.get().shadow.shadow_lg
                        on:change=move |ev| {
                            let value = event_target_value(&ev);
                            handle_shadow_change("shadow-lg", value);
                        }
                    />
                </div>
                <div class="shadow-control">
                    <label>"Extra Large Shadow"</label>
                    <input
                        type="text"
                        value=move || theme.get().shadow.shadow_xl
                        on:change=move |ev| {
                            let value = event_target_value(&ev);
                            handle_shadow_change("shadow-xl", value);
                        }
                    />
                </div>
            </div>
        </div>
    }
}

/// Animation customizer component
#[component]
pub fn AnimationCustomizer(
    /// Current theme
    theme: ReadSignal<CSSVariables>,
    /// Callback when theme changes
    on_change: Callback<CSSVariables>,
) -> impl IntoView {
    let handle_animation_change = move |animation_type: &str, value: String| {
        let mut new_theme = theme.get();
        match animation_type {
            "duration-fast" => new_theme.animation.duration_150 = value,
            "duration-normal" => new_theme.animation.duration_300 = value,
            "duration-slow" => new_theme.animation.duration_500 = value,
            "easing-in" => new_theme.animation.ease_in = value,
            "easing-out" => new_theme.animation.ease_out = value,
            "easing-in-out" => new_theme.animation.ease_in_out = value,
            _ => {}
        }
        on_change.run(new_theme);
    };

    view! {
        <div class="animation-customizer">
            <h4>"Animations"</h4>
            <div class="animation-controls">
                <div class="animation-control">
                    <label>"Fast Duration"</label>
                    <input
                        type="text"
                        value=move || theme.get().animation.duration_150
                        on:change=move |ev| {
                            let value = event_target_value(&ev);
                            handle_animation_change("duration-fast", value);
                        }
                    />
                </div>
                <div class="animation-control">
                    <label>"Normal Duration"</label>
                    <input
                        type="text"
                        value=move || theme.get().animation.duration_300
                        on:change=move |ev| {
                            let value = event_target_value(&ev);
                            handle_animation_change("duration-normal", value);
                        }
                    />
                </div>
                <div class="animation-control">
                    <label>"Slow Duration"</label>
                    <input
                        type="text"
                        value=move || theme.get().animation.duration_500
                        on:change=move |ev| {
                            let value = event_target_value(&ev);
                            handle_animation_change("duration-slow", value);
                        }
                    />
                </div>
                <div class="animation-control">
                    <label>"Easing In"</label>
                    <input
                        type="text"
                        value=move || theme.get().animation.ease_in
                        on:change=move |ev| {
                            let value = event_target_value(&ev);
                            handle_animation_change("easing-in", value);
                        }
                    />
                </div>
                <div class="animation-control">
                    <label>"Easing Out"</label>
                    <input
                        type="text"
                        value=move || theme.get().animation.ease_out
                        on:change=move |ev| {
                            let value = event_target_value(&ev);
                            handle_animation_change("easing-out", value);
                        }
                    />
                </div>
                <div class="animation-control">
                    <label>"Easing In-Out"</label>
                    <input
                        type="text"
                        value=move || theme.get().animation.ease_in_out
                        on:change=move |ev| {
                            let value = event_target_value(&ev);
                            handle_animation_change("easing-in-out", value);
                        }
                    />
                </div>
            </div>
        </div>
    }
}
