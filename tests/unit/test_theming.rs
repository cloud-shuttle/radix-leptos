// Simple test runner for theming system
// This file can be run independently to test our theming functionality

use radix_leptos_primitives::theming::*;

fn main() {
    println!("🧪 Running Theming System Tests...\n");
    
    let mut passed = 0;
    let mut failed = 0;
    
    // Test CSS Variables
    println!("📋 Testing CSS Variables...");
    if test_css_variables() {
        passed += 1;
        println!("✅ CSS Variables tests passed");
    } else {
        failed += 1;
        println!("❌ CSS Variables tests failed");
    }
    
    // Test Theme Provider
    println!("\n📋 Testing Theme Provider...");
    if test_theme_provider() {
        passed += 1;
        println!("✅ Theme Provider tests passed");
    } else {
        failed += 1;
        println!("❌ Theme Provider tests failed");
    }
    
    // Test Dark Mode
    println!("\n📋 Testing Dark Mode...");
    if test_dark_mode() {
        passed += 1;
        println!("✅ Dark Mode tests passed");
    } else {
        failed += 1;
        println!("❌ Dark Mode tests failed");
    }
    
    // Test Size Variants
    println!("\n📋 Testing Size Variants...");
    if test_size_variants() {
        passed += 1;
        println!("✅ Size Variants tests passed");
    } else {
        failed += 1;
        println!("❌ Size Variants tests failed");
    }
    
    // Test Theme Customization
    println!("\n📋 Testing Theme Customization...");
    if test_theme_customization() {
        passed += 1;
        println!("✅ Theme Customization tests passed");
    } else {
        failed += 1;
        println!("❌ Theme Customization tests failed");
    }
    
    // Test Prebuilt Themes
    println!("\n📋 Testing Prebuilt Themes...");
    if test_prebuilt_themes() {
        passed += 1;
        println!("✅ Prebuilt Themes tests passed");
    } else {
        failed += 1;
        println!("❌ Prebuilt Themes tests failed");
    }
    
    // Test Component Variants
    println!("\n📋 Testing Component Variants...");
    if test_component_variants() {
        passed += 1;
        println!("✅ Component Variants tests passed");
    } else {
        failed += 1;
        println!("❌ Component Variants tests failed");
    }
    
    // Test Layout System
    println!("\n📋 Testing Layout System...");
    if test_layout_system() {
        passed += 1;
        println!("✅ Layout System tests passed");
    } else {
        failed += 1;
        println!("❌ Layout System tests failed");
    }
    
    // Summary
    println!("\n" + "=".repeat(50).as_str());
    println!("🎯 Test Summary:");
    println!("✅ Passed: {}", passed);
    println!("❌ Failed: {}", failed);
    println!("📊 Total: {}", passed + failed);
    
    if failed == 0 {
        println!("\n🎉 All theming tests passed! The system is working correctly.");
    } else {
        println!("\n⚠️  Some tests failed. Please check the implementation.");
    }
}

fn test_css_variables() -> bool {
    let mut all_passed = true;
    
    // Test default theme
    let css_vars = CSSVariables::default();
    if css_vars.primary.primary_500 != "#3b82f6" {
        println!("  ❌ Default primary color incorrect");
        all_passed = false;
    }
    
    // Test dark theme
    let dark_theme = CSSVariables::dark_theme();
    if dark_theme.neutral.neutral_50 != "#0a0a0a" {
        println!("  ❌ Dark theme neutral color incorrect");
        all_passed = false;
    }
    
    // Test CSS generation
    let css_string = css_vars.to_css_string();
    if !css_string.contains("--primary-500: #3b82f6;") {
        println!("  ❌ CSS generation failed");
        all_passed = false;
    }
    
    // Test serialization
    match serde_json::to_string(&css_vars) {
        Ok(json) => {
            if !json.contains("\"primary_500\":\"#3b82f6\"") {
                println!("  ❌ Serialization failed");
                all_passed = false;
            }
        }
        Err(_) => {
            println!("  ❌ Serialization error");
            all_passed = false;
        }
    }
    
    all_passed
}

fn test_theme_provider() -> bool {
    let mut all_passed = true;
    
    // Test theme consistency
    let theme1 = CSSVariables::default();
    let theme2 = CSSVariables::default();
    if theme1 != theme2 {
        println!("  ❌ Theme consistency failed");
        all_passed = false;
    }
    
    // Test theme switching
    let light_theme = CSSVariables::default();
    let dark_theme = CSSVariables::dark_theme();
    if light_theme.neutral.neutral_50 == dark_theme.neutral.neutral_50 {
        println!("  ❌ Theme switching failed");
        all_passed = false;
    }
    
    all_passed
}

fn test_dark_mode() -> bool {
    let mut all_passed = true;
    
    // Test dark theme creation
    let dark_theme = CSSVariables::dark_theme();
    if dark_theme.neutral.neutral_50 != "#0a0a0a" {
        println!("  ❌ Dark theme creation failed");
        all_passed = false;
    }
    
    // Test dark vs light mode
    let light_theme = CSSVariables::default();
    if light_theme.neutral.neutral_50 == dark_theme.neutral.neutral_50 {
        println!("  ❌ Dark vs light mode failed");
        all_passed = false;
    }
    
    all_passed
}

fn test_size_variants() -> bool {
    let mut all_passed = true;
    
    // Test size variants
    let sizes = vec![Size::Xs, Size::Sm, Size::Md, Size::Lg, Size::Xl, Size::Xxl];
    
    for size in sizes {
        if size.class().is_empty() {
            println!("  ❌ Size class is empty");
            all_passed = false;
        }
        if size.spacing().is_empty() {
            println!("  ❌ Size spacing is empty");
            all_passed = false;
        }
        if size.font_size().is_empty() {
            println!("  ❌ Size font size is empty");
            all_passed = false;
        }
    }
    
    // Test default size
    if Size::default() != Size::Md {
        println!("  ❌ Default size incorrect");
        all_passed = false;
    }
    
    all_passed
}

fn test_theme_customization() -> bool {
    let mut all_passed = true;
    
    // Test theme modification
    let mut theme = CSSVariables::default();
    theme.primary.primary_500 = "#ff0000".to_string();
    if theme.primary.primary_500 != "#ff0000" {
        println!("  ❌ Theme modification failed");
        all_passed = false;
    }
    
    // Test theme cloning
    let cloned_theme = theme.clone();
    if theme != cloned_theme {
        println!("  ❌ Theme cloning failed");
        all_passed = false;
    }
    
    all_passed
}

fn test_prebuilt_themes() -> bool {
    let mut all_passed = true;
    
    // Test prebuilt themes creation
    let themes = PrebuiltThemes::default();
    
    // Test that all themes are different
    if themes.light.primary.primary_500 == themes.dark.primary.primary_500 {
        println!("  ❌ Light and dark themes should be different");
        all_passed = false;
    }
    
    if themes.finance.primary.primary_500 == themes.healthcare.primary.primary_500 {
        println!("  ❌ Finance and healthcare themes should be different");
        all_passed = false;
    }
    
    // Test theme serialization
    match serde_json::to_string(&themes) {
        Ok(json) => {
            if !json.contains("\"light\"") || !json.contains("\"dark\"") {
                println!("  ❌ Prebuilt themes serialization failed");
                all_passed = false;
            }
        }
        Err(_) => {
            println!("  ❌ Prebuilt themes serialization error");
            all_passed = false;
        }
    }
    
    all_passed
}

fn test_component_variants() -> bool {
    let mut all_passed = true;
    
    // Test button variants
    let button_variants = ButtonVariants::default();
    if button_variants.sizes.is_empty() {
        println!("  ❌ Button variants sizes are empty");
        all_passed = false;
    }
    
    if button_variants.styles.is_empty() {
        println!("  ❌ Button variants styles are empty");
        all_passed = false;
    }
    
    // Test input variants
    let input_variants = InputVariants::default();
    if input_variants.sizes.is_empty() {
        println!("  ❌ Input variants sizes are empty");
        all_passed = false;
    }
    
    if input_variants.styles.is_empty() {
        println!("  ❌ Input variants styles are empty");
        all_passed = false;
    }
    
    all_passed
}

fn test_layout_system() -> bool {
    let mut all_passed = true;
    
    // Test layout config
    let layout = LayoutConfig::default();
    if layout.scale.is_empty() {
        println!("  ❌ Layout scale is empty");
        all_passed = false;
    }
    
    if layout.breakpoints.is_empty() {
        println!("  ❌ Layout breakpoints are empty");
        all_passed = false;
    }
    
    // Test grid system
    let grid = GridSystem::default();
    if grid.columns == 0 {
        println!("  ❌ Grid columns is zero");
        all_passed = false;
    }
    
    // Test flexbox system
    let flexbox = FlexboxSystem::default();
    if flexbox.direction.is_empty() {
        println!("  ❌ Flexbox direction is empty");
        all_passed = false;
    }
    
    all_passed
}
