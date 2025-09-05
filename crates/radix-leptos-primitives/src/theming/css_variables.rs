
/// CSS variable system for theming
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CSSVariables {
    /// Primary color variables
    pub primary: PrimaryColors,
    /// Secondary color variables
    pub secondary: SecondaryColors,
    /// Neutral color variables
    pub neutral: NeutralColors,
    /// Semantic color variables
    pub semantic: SemanticColors,
    /// Typography variables
    pub typography: TypographyVariables,
    /// Spacing variables
    pub spacing: SpacingVariables,
    /// Border variables
    pub border: BorderVariables,
    /// Shadow variables
    pub shadow: ShadowVariables,
    /// Animation variables
    pub animation: AnimationVariables,
}

/// Primary color variables
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PrimaryColors {
    pub primary_50: String,
    pub primary_100: String,
    pub primary_200: String,
    pub primary_300: String,
    pub primary_400: String,
    pub primary_500: String,
    pub primary_600: String,
    pub primary_700: String,
    pub primary_800: String,
    pub primary_900: String,
    pub primary_950: String,
}

/// Secondary color variables
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SecondaryColors {
    pub secondary_50: String,
    pub secondary_100: String,
    pub secondary_200: String,
    pub secondary_300: String,
    pub secondary_400: String,
    pub secondary_500: String,
    pub secondary_600: String,
    pub secondary_700: String,
    pub secondary_800: String,
    pub secondary_900: String,
    pub secondary_950: String,
}

/// Neutral color variables
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NeutralColors {
    pub neutral_50: String,
    pub neutral_100: String,
    pub neutral_200: String,
    pub neutral_300: String,
    pub neutral_400: String,
    pub neutral_500: String,
    pub neutral_600: String,
    pub neutral_700: String,
    pub neutral_800: String,
    pub neutral_900: String,
    pub neutral_950: String,
}

/// Semantic color variables
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SemanticColors {
    pub success: String,
    pub warning: String,
    pub error: String,
    pub info: String,
}

/// Typography variables
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TypographyVariables {
    pub font_family_sans: String,
    pub font_family_serif: String,
    pub font_family_mono: String,
    pub font_size_xs: String,
    pub font_size_sm: String,
    pub font_size_base: String,
    pub font_size_lg: String,
    pub font_size_xl: String,
    pub font_size_2xl: String,
    pub font_size_3xl: String,
    pub font_size_4xl: String,
    pub font_size_5xl: String,
    pub font_size_6xl: String,
    pub font_weight_thin: String,
    pub font_weight_light: String,
    pub font_weight_normal: String,
    pub font_weight_medium: String,
    pub font_weight_semibold: String,
    pub font_weight_bold: String,
    pub font_weight_extrabold: String,
    pub font_weight_black: String,
    pub line_height_none: String,
    pub line_height_tight: String,
    pub line_height_snug: String,
    pub line_height_normal: String,
    pub line_height_relaxed: String,
    pub line_height_loose: String,
}

/// Spacing variables
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SpacingVariables {
    pub space_0: String,
    pub space_1: String,
    pub space_2: String,
    pub space_3: String,
    pub space_4: String,
    pub space_5: String,
    pub space_6: String,
    pub space_8: String,
    pub space_10: String,
    pub space_12: String,
    pub space_16: String,
    pub space_20: String,
    pub space_24: String,
    pub space_32: String,
    pub space_40: String,
    pub space_48: String,
    pub space_56: String,
    pub space_64: String,
}

/// Border variables
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BorderVariables {
    pub border_width_0: String,
    pub border_width_1: String,
    pub border_width_2: String,
    pub border_width_4: String,
    pub border_width_8: String,
    pub border_radius_none: String,
    pub border_radius_sm: String,
    pub border_radius_base: String,
    pub border_radius_md: String,
    pub border_radius_lg: String,
    pub border_radius_xl: String,
    pub border_radius_2xl: String,
    pub border_radius_3xl: String,
    pub border_radius_full: String,
}

/// Shadow variables
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ShadowVariables {
    pub shadow_sm: String,
    pub shadow_base: String,
    pub shadow_md: String,
    pub shadow_lg: String,
    pub shadow_xl: String,
    pub shadow_2xl: String,
    pub shadow_inner: String,
    pub shadow_none: String,
}

/// Animation variables
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AnimationVariables {
    pub duration_75: String,
    pub duration_100: String,
    pub duration_150: String,
    pub duration_200: String,
    pub duration_300: String,
    pub duration_500: String,
    pub duration_700: String,
    pub duration_1000: String,
    pub ease_linear: String,
    pub ease_in: String,
    pub ease_out: String,
    pub ease_in_out: String,
}

impl Default for CSSVariables {
    fn default() -> Self {
        Self {
            primary: PrimaryColors::default(),
            secondary: SecondaryColors::default(),
            neutral: NeutralColors::default(),
            semantic: SemanticColors::default(),
            typography: TypographyVariables::default(),
            spacing: SpacingVariables::default(),
            border: BorderVariables::default(),
            shadow: ShadowVariables::default(),
            animation: AnimationVariables::default(),
        }
    }
}

impl Default for PrimaryColors {
    fn default() -> Self {
        Self {
            primary_50: "#eff6ff".to_string(),
            primary_100: "#dbeafe".to_string(),
            primary_200: "#bfdbfe".to_string(),
            primary_300: "#93c5fd".to_string(),
            primary_400: "#60a5fa".to_string(),
            primary_500: "#3b82f6".to_string(),
            primary_600: "#2563eb".to_string(),
            primary_700: "#1d4ed8".to_string(),
            primary_800: "#1e40af".to_string(),
            primary_900: "#1e3a8a".to_string(),
            primary_950: "#172554".to_string(),
        }
    }
}

impl Default for SecondaryColors {
    fn default() -> Self {
        Self {
            secondary_50: "#f8fafc".to_string(),
            secondary_100: "#f1f5f9".to_string(),
            secondary_200: "#e2e8f0".to_string(),
            secondary_300: "#cbd5e1".to_string(),
            secondary_400: "#94a3b8".to_string(),
            secondary_500: "#64748b".to_string(),
            secondary_600: "#475569".to_string(),
            secondary_700: "#334155".to_string(),
            secondary_800: "#1e293b".to_string(),
            secondary_900: "#0f172a".to_string(),
            secondary_950: "#020617".to_string(),
        }
    }
}

impl Default for NeutralColors {
    fn default() -> Self {
        Self {
            neutral_50: "#fafafa".to_string(),
            neutral_100: "#f5f5f5".to_string(),
            neutral_200: "#e5e5e5".to_string(),
            neutral_300: "#d4d4d4".to_string(),
            neutral_400: "#a3a3a3".to_string(),
            neutral_500: "#737373".to_string(),
            neutral_600: "#525252".to_string(),
            neutral_700: "#404040".to_string(),
            neutral_800: "#262626".to_string(),
            neutral_900: "#171717".to_string(),
            neutral_950: "#0a0a0a".to_string(),
        }
    }
}

impl Default for SemanticColors {
    fn default() -> Self {
        Self {
            success: "#10b981".to_string(),
            warning: "#f59e0b".to_string(),
            error: "#ef4444".to_string(),
            info: "#3b82f6".to_string(),
        }
    }
}

impl Default for TypographyVariables {
    fn default() -> Self {
        Self {
            font_family_sans: "ui-sans-serif, system-ui, sans-serif".to_string(),
            font_family_serif: "ui-serif, Georgia, serif".to_string(),
            font_family_mono: "ui-monospace, SFMono-Regular, monospace".to_string(),
            font_size_xs: "0.75rem".to_string(),
            font_size_sm: "0.875rem".to_string(),
            font_size_base: "1rem".to_string(),
            font_size_lg: "1.125rem".to_string(),
            font_size_xl: "1.25rem".to_string(),
            font_size_2xl: "1.5rem".to_string(),
            font_size_3xl: "1.875rem".to_string(),
            font_size_4xl: "2.25rem".to_string(),
            font_size_5xl: "3rem".to_string(),
            font_size_6xl: "3.75rem".to_string(),
            font_weight_thin: "100".to_string(),
            font_weight_light: "300".to_string(),
            font_weight_normal: "400".to_string(),
            font_weight_medium: "500".to_string(),
            font_weight_semibold: "600".to_string(),
            font_weight_bold: "700".to_string(),
            font_weight_extrabold: "800".to_string(),
            font_weight_black: "900".to_string(),
            line_height_none: "1".to_string(),
            line_height_tight: "1.25".to_string(),
            line_height_snug: "1.375".to_string(),
            line_height_normal: "1.5".to_string(),
            line_height_relaxed: "1.625".to_string(),
            line_height_loose: "2".to_string(),
        }
    }
}

impl Default for SpacingVariables {
    fn default() -> Self {
        Self {
            space_0: "0px".to_string(),
            space_1: "0.25rem".to_string(),
            space_2: "0.5rem".to_string(),
            space_3: "0.75rem".to_string(),
            space_4: "1rem".to_string(),
            space_5: "1.25rem".to_string(),
            space_6: "1.5rem".to_string(),
            space_8: "2rem".to_string(),
            space_10: "2.5rem".to_string(),
            space_12: "3rem".to_string(),
            space_16: "4rem".to_string(),
            space_20: "5rem".to_string(),
            space_24: "6rem".to_string(),
            space_32: "8rem".to_string(),
            space_40: "10rem".to_string(),
            space_48: "12rem".to_string(),
            space_56: "14rem".to_string(),
            space_64: "16rem".to_string(),
        }
    }
}

impl Default for BorderVariables {
    fn default() -> Self {
        Self {
            border_width_0: "0px".to_string(),
            border_width_1: "1px".to_string(),
            border_width_2: "2px".to_string(),
            border_width_4: "4px".to_string(),
            border_width_8: "8px".to_string(),
            border_radius_none: "0px".to_string(),
            border_radius_sm: "0.125rem".to_string(),
            border_radius_base: "0.25rem".to_string(),
            border_radius_md: "0.375rem".to_string(),
            border_radius_lg: "0.5rem".to_string(),
            border_radius_xl: "0.75rem".to_string(),
            border_radius_2xl: "1rem".to_string(),
            border_radius_3xl: "1.5rem".to_string(),
            border_radius_full: "9999px".to_string(),
        }
    }
}

impl Default for ShadowVariables {
    fn default() -> Self {
        Self {
            shadow_sm: "0 1px 2px 0 rgb(0 0 0 / 0.05)".to_string(),
            shadow_base: "0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)".to_string(),
            shadow_md: "0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)".to_string(),
            shadow_lg: "0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)".to_string(),
            shadow_xl: "0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1)".to_string(),
            shadow_2xl: "0 25px 50px -12px rgb(0 0 0 / 0.25)".to_string(),
            shadow_inner: "inset 0 2px 4px 0 rgb(0 0 0 / 0.05)".to_string(),
            shadow_none: "0 0 #0000".to_string(),
        }
    }
}

impl Default for AnimationVariables {
    fn default() -> Self {
        Self {
            duration_75: "75ms".to_string(),
            duration_100: "100ms".to_string(),
            duration_150: "150ms".to_string(),
            duration_200: "200ms".to_string(),
            duration_300: "300ms".to_string(),
            duration_500: "500ms".to_string(),
            duration_700: "700ms".to_string(),
            duration_1000: "1000ms".to_string(),
            ease_linear: "linear".to_string(),
            ease_in: "cubic-bezier(0.4, 0, 1, 1)".to_string(),
            ease_out: "cubic-bezier(0, 0, 0.2, 1)".to_string(),
            ease_in_out: "cubic-bezier(0.4, 0, 0.2, 1)".to_string(),
        }
    }
}

impl CSSVariables {
    /// Create a dark theme variant
    pub fn dark_theme() -> Self {
        Self {
            primary: PrimaryColors::default(),
            secondary: SecondaryColors::default(),
            neutral: NeutralColors {
                neutral_50: "#0a0a0a".to_string(),
                neutral_100: "#171717".to_string(),
                neutral_200: "#262626".to_string(),
                neutral_300: "#404040".to_string(),
                neutral_400: "#525252".to_string(),
                neutral_500: "#737373".to_string(),
                neutral_600: "#a3a3a3".to_string(),
                neutral_700: "#d4d4d4".to_string(),
                neutral_800: "#e5e5e5".to_string(),
                neutral_900: "#f5f5f5".to_string(),
                neutral_950: "#fafafa".to_string(),
            },
            semantic: SemanticColors::default(),
            typography: TypographyVariables::default(),
            spacing: SpacingVariables::default(),
            border: BorderVariables::default(),
            shadow: ShadowVariables::default(),
            animation: AnimationVariables::default(),
        }
    }

    /// Convert CSS variables to CSS string
    pub fn to_css_string(&self) -> String {
        let mut css = String::new();
        
        // Primary colors
        css.push_str(&format!("--primary-50: {};", self.primary.primary_50));
        css.push_str(&format!("--primary-100: {};", self.primary.primary_100));
        css.push_str(&format!("--primary-200: {};", self.primary.primary_200));
        css.push_str(&format!("--primary-300: {};", self.primary.primary_300));
        css.push_str(&format!("--primary-400: {};", self.primary.primary_400));
        css.push_str(&format!("--primary-500: {};", self.primary.primary_500));
        css.push_str(&format!("--primary-600: {};", self.primary.primary_600));
        css.push_str(&format!("--primary-700: {};", self.primary.primary_700));
        css.push_str(&format!("--primary-800: {};", self.primary.primary_800));
        css.push_str(&format!("--primary-900: {};", self.primary.primary_900));
        css.push_str(&format!("--primary-950: {};", self.primary.primary_950));
        
        // Secondary colors
        css.push_str(&format!("--secondary-50: {};", self.secondary.secondary_50));
        css.push_str(&format!("--secondary-100: {};", self.secondary.secondary_100));
        css.push_str(&format!("--secondary-200: {};", self.secondary.secondary_200));
        css.push_str(&format!("--secondary-300: {};", self.secondary.secondary_300));
        css.push_str(&format!("--secondary-400: {};", self.secondary.secondary_400));
        css.push_str(&format!("--secondary-500: {};", self.secondary.secondary_500));
        css.push_str(&format!("--secondary-600: {};", self.secondary.secondary_600));
        css.push_str(&format!("--secondary-700: {};", self.secondary.secondary_700));
        css.push_str(&format!("--secondary-800: {};", self.secondary.secondary_800));
        css.push_str(&format!("--secondary-900: {};", self.secondary.secondary_900));
        css.push_str(&format!("--secondary-950: {};", self.secondary.secondary_950));
        
        // Neutral colors
        css.push_str(&format!("--neutral-50: {};", self.neutral.neutral_50));
        css.push_str(&format!("--neutral-100: {};", self.neutral.neutral_100));
        css.push_str(&format!("--neutral-200: {};", self.neutral.neutral_200));
        css.push_str(&format!("--neutral-300: {};", self.neutral.neutral_300));
        css.push_str(&format!("--neutral-400: {};", self.neutral.neutral_400));
        css.push_str(&format!("--neutral-500: {};", self.neutral.neutral_500));
        css.push_str(&format!("--neutral-600: {};", self.neutral.neutral_600));
        css.push_str(&format!("--neutral-700: {};", self.neutral.neutral_700));
        css.push_str(&format!("--neutral-800: {};", self.neutral.neutral_800));
        css.push_str(&format!("--neutral-900: {};", self.neutral.neutral_900));
        css.push_str(&format!("--neutral-950: {};", self.neutral.neutral_950));
        
        // Semantic colors
        css.push_str(&format!("--success: {};", self.semantic.success));
        css.push_str(&format!("--warning: {};", self.semantic.warning));
        css.push_str(&format!("--error: {};", self.semantic.error));
        css.push_str(&format!("--info: {};", self.semantic.info));
        
        // Typography
        css.push_str(&format!("--font-family-sans: {};", self.typography.font_family_sans));
        css.push_str(&format!("--font-family-serif: {};", self.typography.font_family_serif));
        css.push_str(&format!("--font-family-mono: {};", self.typography.font_family_mono));
        css.push_str(&format!("--font-size-xs: {};", self.typography.font_size_xs));
        css.push_str(&format!("--font-size-sm: {};", self.typography.font_size_sm));
        css.push_str(&format!("--font-size-base: {};", self.typography.font_size_base));
        css.push_str(&format!("--font-size-lg: {};", self.typography.font_size_lg));
        css.push_str(&format!("--font-size-xl: {};", self.typography.font_size_xl));
        css.push_str(&format!("--font-size-2xl: {};", self.typography.font_size_2xl));
        css.push_str(&format!("--font-size-3xl: {};", self.typography.font_size_3xl));
        css.push_str(&format!("--font-size-4xl: {};", self.typography.font_size_4xl));
        css.push_str(&format!("--font-size-5xl: {};", self.typography.font_size_5xl));
        css.push_str(&format!("--font-size-6xl: {};", self.typography.font_size_6xl));
        css.push_str(&format!("--font-weight-thin: {};", self.typography.font_weight_thin));
        css.push_str(&format!("--font-weight-light: {};", self.typography.font_weight_light));
        css.push_str(&format!("--font-weight-normal: {};", self.typography.font_weight_normal));
        css.push_str(&format!("--font-weight-medium: {};", self.typography.font_weight_medium));
        css.push_str(&format!("--font-weight-semibold: {};", self.typography.font_weight_semibold));
        css.push_str(&format!("--font-weight-bold: {};", self.typography.font_weight_bold));
        css.push_str(&format!("--font-weight-extrabold: {};", self.typography.font_weight_extrabold));
        css.push_str(&format!("--font-weight-black: {};", self.typography.font_weight_black));
        css.push_str(&format!("--line-height-none: {};", self.typography.line_height_none));
        css.push_str(&format!("--line-height-tight: {};", self.typography.line_height_tight));
        css.push_str(&format!("--line-height-snug: {};", self.typography.line_height_snug));
        css.push_str(&format!("--line-height-normal: {};", self.typography.line_height_normal));
        css.push_str(&format!("--line-height-relaxed: {};", self.typography.line_height_relaxed));
        css.push_str(&format!("--line-height-loose: {};", self.typography.line_height_loose));
        
        // Spacing
        css.push_str(&format!("--space-0: {};", self.spacing.space_0));
        css.push_str(&format!("--space-1: {};", self.spacing.space_1));
        css.push_str(&format!("--space-2: {};", self.spacing.space_2));
        css.push_str(&format!("--space-3: {};", self.spacing.space_3));
        css.push_str(&format!("--space-4: {};", self.spacing.space_4));
        css.push_str(&format!("--space-5: {};", self.spacing.space_5));
        css.push_str(&format!("--space-6: {};", self.spacing.space_6));
        css.push_str(&format!("--space-8: {};", self.spacing.space_8));
        css.push_str(&format!("--space-10: {};", self.spacing.space_10));
        css.push_str(&format!("--space-12: {};", self.spacing.space_12));
        css.push_str(&format!("--space-16: {};", self.spacing.space_16));
        css.push_str(&format!("--space-20: {};", self.spacing.space_20));
        css.push_str(&format!("--space-24: {};", self.spacing.space_24));
        css.push_str(&format!("--space-32: {};", self.spacing.space_32));
        css.push_str(&format!("--space-40: {};", self.spacing.space_40));
        css.push_str(&format!("--space-48: {};", self.spacing.space_48));
        css.push_str(&format!("--space-56: {};", self.spacing.space_56));
        css.push_str(&format!("--space-64: {};", self.spacing.space_64));
        
        // Border
        css.push_str(&format!("--border-width-0: {};", self.border.border_width_0));
        css.push_str(&format!("--border-width-1: {};", self.border.border_width_1));
        css.push_str(&format!("--border-width-2: {};", self.border.border_width_2));
        css.push_str(&format!("--border-width-4: {};", self.border.border_width_4));
        css.push_str(&format!("--border-width-8: {};", self.border.border_width_8));
        css.push_str(&format!("--border-radius-none: {};", self.border.border_radius_none));
        css.push_str(&format!("--border-radius-sm: {};", self.border.border_radius_sm));
        css.push_str(&format!("--border-radius-base: {};", self.border.border_radius_base));
        css.push_str(&format!("--border-radius-md: {};", self.border.border_radius_md));
        css.push_str(&format!("--border-radius-lg: {};", self.border.border_radius_lg));
        css.push_str(&format!("--border-radius-xl: {};", self.border.border_radius_xl));
        css.push_str(&format!("--border-radius-2xl: {};", self.border.border_radius_2xl));
        css.push_str(&format!("--border-radius-3xl: {};", self.border.border_radius_3xl));
        css.push_str(&format!("--border-radius-full: {};", self.border.border_radius_full));
        
        // Shadow
        css.push_str(&format!("--shadow-sm: {};", self.shadow.shadow_sm));
        css.push_str(&format!("--shadow-base: {};", self.shadow.shadow_base));
        css.push_str(&format!("--shadow-md: {};", self.shadow.shadow_md));
        css.push_str(&format!("--shadow-lg: {};", self.shadow.shadow_lg));
        css.push_str(&format!("--shadow-xl: {};", self.shadow.shadow_xl));
        css.push_str(&format!("--shadow-2xl: {};", self.shadow.shadow_2xl));
        css.push_str(&format!("--shadow-inner: {};", self.shadow.shadow_inner));
        css.push_str(&format!("--shadow-none: {};", self.shadow.shadow_none));
        
        // Animation
        css.push_str(&format!("--duration-75: {};", self.animation.duration_75));
        css.push_str(&format!("--duration-100: {};", self.animation.duration_100));
        css.push_str(&format!("--duration-150: {};", self.animation.duration_150));
        css.push_str(&format!("--duration-200: {};", self.animation.duration_200));
        css.push_str(&format!("--duration-300: {};", self.animation.duration_300));
        css.push_str(&format!("--duration-500: {};", self.animation.duration_500));
        css.push_str(&format!("--duration-700: {};", self.animation.duration_700));
        css.push_str(&format!("--duration-1000: {};", self.animation.duration_1000));
        css.push_str(&format!("--ease-linear: {};", self.animation.ease_linear));
        css.push_str(&format!("--ease-in: {};", self.animation.ease_in));
        css.push_str(&format!("--ease-out: {};", self.animation.ease_out));
        css.push_str(&format!("--ease-in-out: {};", self.animation.ease_in_out));
        
        css
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::*;

    #[test]
    fn test_css_variables_default() {
        let css_vars = CSSVariables::default();
        assert_eq!(css_vars.primary.primary_500, "#3b82f6");
        assert_eq!(css_vars.secondary.secondary_500, "#64748b");
        assert_eq!(css_vars.neutral.neutral_500, "#737373");
        assert_eq!(css_vars.semantic.success, "#10b981");
        assert_eq!(css_vars.typography.font_size_base, "1rem");
        assert_eq!(css_vars.spacing.space_4, "1rem");
        assert_eq!(css_vars.border.border_radius_base, "0.25rem");
        assert_eq!(css_vars.shadow.shadow_base, "0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)");
        assert_eq!(css_vars.animation.duration_300, "300ms");
    }

    #[test]
    fn test_css_variables_to_css_string() {
        let css_vars = CSSVariables::default();
        let css_string = css_vars.to_css_string();
        
        assert!(css_string.contains("--primary-500: #3b82f6;"));
        assert!(css_string.contains("--secondary-500: #64748b;"));
        assert!(css_string.contains("--neutral-500: #737373;"));
        assert!(css_string.contains("--success: #10b981;"));
        assert!(css_string.contains("--font-size-base: 1rem;"));
        assert!(css_string.contains("--space-4: 1rem;"));
        assert!(css_string.contains("--border-radius-base: 0.25rem;"));
        assert!(css_string.contains("--shadow-base: 0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1);"));
        assert!(css_string.contains("--duration-300: 300ms;"));
    }

    #[test]
    fn test_css_variables_clone() {
        let css_vars = CSSVariables::default();
        let cloned = css_vars.clone();
        assert_eq!(css_vars.primary.primary_500, cloned.primary.primary_500);
        assert_eq!(css_vars.secondary.secondary_500, cloned.secondary.secondary_500);
    }

    #[test]
    fn test_css_variables_serialize() {
        let css_vars = CSSVariables::default();
        let json = serde_json::to_string(&css_vars).unwrap();
        assert!(json.contains("\"primary_500\":\"#3b82f6\""));
        assert!(json.contains("\"secondary_500\":\"#64748b\""));
    }

    #[test]
    fn test_css_variables_deserialize() {
        let json = include_str!("test_data.json");
        let css_vars: CSSVariables = serde_json::from_str(json).unwrap();
        assert_eq!(css_vars.primary.primary_500, "#3b82f6");
        assert_eq!(css_vars.secondary.secondary_500, "#64748b");
    }
}