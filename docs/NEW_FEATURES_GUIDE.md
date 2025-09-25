# New Features Guide for Radix-Leptos

This comprehensive guide covers all the new features and enhancements available in Radix-Leptos.

## Table of Contents

- [Advanced Components](#advanced-components)
- [Advanced Theming](#advanced-theming)
- [Accessibility Enhancements](#accessibility-enhancements)
- [Data Visualization](#data-visualization)
- [Theme Builder](#theme-builder)
- [Accessibility Testing](#accessibility-testing)
- [Best Practices](#best-practices)

## Advanced Components

### Data Visualization Components

Radix-Leptos now includes powerful data visualization components for creating interactive charts and graphs.

#### Interactive Line Chart

```rust
use radix_leptos_primitives::components::advanced::*;

#[component]
pub fn ChartExample() -> impl IntoView {
    let data = vec![
        DataPoint { x: 0.0, y: 10.0, label: Some("Start".to_string()), color: None, metadata: HashMap::new() },
        DataPoint { x: 1.0, y: 20.0, label: Some("Point 1".to_string()), color: None, metadata: HashMap::new() },
        DataPoint { x: 2.0, y: 15.0, label: Some("Point 2".to_string()), color: None, metadata: HashMap::new() },
        DataPoint { x: 3.0, y: 25.0, label: Some("Point 3".to_string()), color: None, metadata: HashMap::new() },
    ];

    let config = ChartConfig {
        width: 800.0,
        height: 400.0,
        colors: vec!["#3b82f6".to_string(), "#ef4444".to_string()],
        animations: true,
        interactive: true,
        responsive: true,
        ..Default::default()
    };

    view! {
        <InteractiveLineChart
            data=data
            config=Some(config)
            on_point_click=Some(Callback::new(|point| {
                log::info!("Clicked point: {:?}", point);
            }))
            on_point_hover=Some(Callback::new(|point| {
                log::info!("Hovered point: {:?}", point);
            }))
        />
    }
}
```

#### Interactive Bar Chart

```rust
#[component]
pub fn BarChartExample() -> impl IntoView {
    let data = vec![
        DataPoint { x: 0.0, y: 20.0, label: Some("Category A".to_string()), color: None, metadata: HashMap::new() },
        DataPoint { x: 1.0, y: 35.0, label: Some("Category B".to_string()), color: None, metadata: HashMap::new() },
        DataPoint { x: 2.0, y: 15.0, label: Some("Category C".to_string()), color: None, metadata: HashMap::new() },
        DataPoint { x: 3.0, y: 40.0, label: Some("Category D".to_string()), color: None, metadata: HashMap::new() },
    ];

    view! {
        <InteractiveBarChart
            data=data
            config=Some(ChartConfig::default())
            on_bar_click=Some(Callback::new(|point| {
                log::info!("Clicked bar: {:?}", point);
            }))
            on_bar_hover=Some(Callback::new(|point| {
                log::info!("Hovered bar: {:?}", point);
            }))
        />
    }
}
```

#### Real-time Data Display

```rust
#[component]
pub fn RealTimeChart() -> impl IntoView {
    let (data, set_data) = create_signal(generate_sample_data());
    
    // Simulate real-time updates
    use_interval(1000, move || {
        set_data.update(|d| {
            d.push(DataPoint {
                x: d.len() as f64,
                y: (d.len() as f64 * 0.5 + (d.len() as f64 * 0.1).sin() * 10.0).max(0.0),
                label: Some(format!("Point {}", d.len())),
                color: None,
                metadata: HashMap::new(),
            });
        });
    });

    view! {
        <RealTimeDataDisplay
            data=data
            config=Some(ChartConfig::default())
            update_interval=Some(1000)
        />
    }
}
```

#### Analytics Dashboard

```rust
#[component]
pub fn AnalyticsDashboard() -> impl IntoView {
    let (metrics, set_metrics) = create_signal(HashMap::new());
    
    // Initialize metrics
    set_metrics.update(|m| {
        m.insert("total_views".to_string(), 1250.0);
        m.insert("unique_visitors".to_string(), 850.0);
        m.insert("conversion_rate".to_string(), 12.5);
        m.insert("revenue".to_string(), 25000.0);
    });

    view! {
        <AnalyticsDashboard
            metrics=metrics
        />
    }
}
```

## Advanced Theming

### Theme Builder

The new theme builder provides sophisticated theming capabilities with inheritance, composition, and validation.

#### Creating a Custom Theme

```rust
use radix_leptos_primitives::theming::advanced::*;

#[component]
pub fn CustomThemeExample() -> impl IntoView {
    let theme = AdvancedThemeBuilder::new("my-custom-theme".to_string())
        .with_base_theme("default".to_string())
        .with_colors(ColorPalette {
            primary: ColorScale {
                50: "#f0f9ff".to_string(),
                100: "#e0f2fe".to_string(),
                200: "#bae6fd".to_string(),
                300: "#7dd3fc".to_string(),
                400: "#38bdf8".to_string(),
                500: "#0ea5e9".to_string(),
                600: "#0284c7".to_string(),
                700: "#0369a1".to_string(),
                800: "#075985".to_string(),
                900: "#0c4a6e".to_string(),
                950: "#082f49".to_string(),
            },
            secondary: ColorScale::default().secondary,
            neutral: ColorScale::default().neutral,
            success: ColorScale::default().success,
            warning: ColorScale::default().warning,
            error: ColorScale::default().error,
            info: ColorScale::default().info,
            background: BackgroundColors::default(),
            surface: SurfaceColors::default(),
            text: TextColors::default(),
        })
        .with_typography(TypographyConfig {
            font_families: FontFamilies {
                sans: "Inter, system-ui, sans-serif".to_string(),
                serif: "Georgia, serif".to_string(),
                mono: "JetBrains Mono, monospace".to_string(),
                display: "Inter, system-ui, sans-serif".to_string(),
            },
            ..Default::default()
        })
        .with_custom_property("custom-color".to_string(), "#ff0000".to_string())
        .with_metadata(ThemeMetadata {
            version: "1.0.0".to_string(),
            author: "Your Name".to_string(),
            description: "My custom theme".to_string(),
            tags: vec!["custom".to_string(), "blue".to_string()],
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        })
        .build();

    // Validate the theme
    match ThemeValidator::validate(&theme) {
        Ok(_) => log::info!("Theme is valid"),
        Err(errors) => {
            for error in errors {
                log::error!("Theme validation error: {}", error);
            }
        }
    }

    // Export the theme
    let json = ThemeExporter::to_json(&theme).unwrap();
    let css = ThemeExporter::to_css_variables(&theme);
    
    view! {
        <div>
            <h2>"Custom Theme Example"</h2>
            <pre>{json}</pre>
            <pre>{css}</pre>
        </div>
    }
}
```

#### Theme Inheritance

```rust
#[component]
pub fn ThemeInheritanceExample() -> impl IntoView {
    let base_theme = AdvancedTheme::default();
    
    let overrides = AdvancedTheme {
        name: "dark-theme".to_string(),
        colors: ColorPalette {
            primary: ColorScale {
                500: "#3b82f6".to_string(),
                ..Default::default()
            },
            background: BackgroundColors {
                primary: "#0f172a".to_string(),
                secondary: "#1e293b".to_string(),
                tertiary: "#334155".to_string(),
                overlay: "rgba(0, 0, 0, 0.8)".to_string(),
            },
            ..Default::default()
        },
        ..Default::default()
    };
    
    let inherited_theme = ThemeInheritance::inherit_from(&base_theme, overrides);
    
    view! {
        <div>
            <h2>"Theme Inheritance Example"</h2>
            <p>"Inherited theme: " {inherited_theme.name}</p>
        </div>
    }
}
```

#### Theme Composition

```rust
#[component]
pub fn ThemeCompositionExample() -> impl IntoView {
    let theme1 = AdvancedTheme {
        name: "theme1".to_string(),
        colors: ColorPalette {
            primary: ColorScale {
                500: "#3b82f6".to_string(),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    };
    
    let theme2 = AdvancedTheme {
        name: "theme2".to_string(),
        colors: ColorPalette {
            secondary: ColorScale {
                500: "#ef4444".to_string(),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    };
    
    let composed_theme = ThemeComposition::compose(vec![theme1, theme2]);
    
    view! {
        <div>
            <h2>"Theme Composition Example"</h2>
            <p>"Composed theme: " {composed_theme.name}</p>
        </div>
    }
}
```

## Accessibility Enhancements

### Advanced Accessibility Features

Radix-Leptos now includes comprehensive accessibility features for building inclusive applications.

#### Screen Reader Optimization

```rust
use radix_leptos_primitives::accessibility::*;

#[component]
pub fn ScreenReaderExample() -> impl IntoView {
    let config = AccessibilityConfig {
        screen_reader_optimized: true,
        keyboard_navigation: true,
        focus_management: true,
        aria_live_regions: true,
        ..Default::default()
    };
    
    let accessibility = use_accessibility(config);
    
    let handle_announcement = move |_| {
        accessibility.screen_reader_optimizer.announce("Button clicked!".to_string());
    };

    view! {
        <div>
            <h2>"Screen Reader Example"</h2>
            <Button on_click=handle_announcement>
                "Click me for announcement"
            </Button>
        </div>
    }
}
```

#### Keyboard Navigation

```rust
#[component]
pub fn KeyboardNavigationExample() -> impl IntoView {
    let config = AccessibilityConfig {
        keyboard_navigation: true,
        focus_management: true,
        ..Default::default()
    };
    
    let accessibility = use_accessibility(config);
    
    let handle_keydown = move |event: web_sys::KeyboardEvent| {
        accessibility.keyboard_navigation_manager.handle_keyboard_navigation(event);
    };

    view! {
        <div on:keydown=handle_keydown>
            <h2>"Keyboard Navigation Example"</h2>
            <Button>"Button 1"</Button>
            <Button>"Button 2"</Button>
            <Button>"Button 3"</Button>
        </div>
    }
}
```

#### Focus Management

```rust
#[component]
pub fn FocusManagementExample() -> impl IntoView {
    let config = AccessibilityConfig {
        focus_management: true,
        ..Default::default()
    };
    
    let accessibility = use_accessibility(config);
    
    let handle_focus = move |event: web_sys::FocusEvent| {
        accessibility.focus_manager.handle_focus(event);
    };
    
    let handle_return_focus = move |_| {
        accessibility.focus_manager.return_focus();
    };

    view! {
        <div on:focus=handle_focus>
            <h2>"Focus Management Example"</h2>
            <Button>"Button 1"</Button>
            <Button>"Button 2"</Button>
            <Button on_click=handle_return_focus>
                "Return Focus"
            </Button>
        </div>
    }
}
```

#### ARIA Live Regions

```rust
#[component]
pub fn AriaLiveExample() -> impl IntoView {
    let config = AccessibilityConfig {
        aria_live_regions: true,
        ..Default::default()
    };
    
    let accessibility = use_accessibility(config);
    
    let (announcements, set_announcements) = create_signal(Vec::new());
    
    let handle_announcement = move |_| {
        let message = format!("Announcement {}", announcements().len() + 1);
        set_announcements.update(|a| a.push(message.clone()));
        accessibility.aria_live_manager.announce("announcements", message);
    };

    view! {
        <div>
            <h2>"ARIA Live Regions Example"</h2>
            <Button on_click=handle_announcement>
                "Make Announcement"
            </Button>
            <ul>
                {announcements().into_iter().map(|announcement| {
                    view! { <li>{announcement}</li> }
                }).collect::<Vec<_>>()}
            </ul>
        </div>
    }
}
```

#### High Contrast Mode Support

```rust
#[component]
pub fn HighContrastExample() -> impl IntoView {
    let config = AccessibilityConfig {
        high_contrast_mode: true,
        ..Default::default()
    };
    
    let accessibility = use_accessibility(config);
    
    let is_high_contrast = accessibility.high_contrast_support.check_high_contrast_mode();

    view! {
        <div>
            <h2>"High Contrast Mode Example"</h2>
            <p>"High contrast mode: " {if is_high_contrast { "Enabled" } else { "Disabled" }}</p>
            <Button>"Button with high contrast support"</Button>
        </div>
    }
}
```

#### Reduced Motion Support

```rust
#[component]
pub fn ReducedMotionExample() -> impl IntoView {
    let config = AccessibilityConfig {
        reduced_motion: true,
        ..Default::default()
    };
    
    let accessibility = use_accessibility(config);
    
    let is_reduced_motion = accessibility.reduced_motion_support.check_reduced_motion();

    view! {
        <div>
            <h2>"Reduced Motion Example"</h2>
            <p>"Reduced motion: " {if is_reduced_motion { "Enabled" } else { "Disabled" }}</p>
            <div class="animated-element">
                "This element respects reduced motion preferences"
            </div>
        </div>
    }
}
```

#### Accessibility Testing

```rust
#[component]
pub fn AccessibilityTestingExample() -> impl IntoView {
    let config = AccessibilityConfig::default();
    let accessibility = use_accessibility(config);
    
    let test_element = NodeRef::<leptos::html::Div>::new();
    
    let handle_test = move |_| {
        if let Some(element) = test_element.get() {
            let result = accessibility.accessibility_tester.test_component_accessibility(&element);
            log::info!("Accessibility score: {}", result.get_score());
            for error in result.errors {
                log::error!("Accessibility error: {}", error);
            }
            for warning in result.warnings {
                log::warn!("Accessibility warning: {}", warning);
            }
        }
    };

    view! {
        <div>
            <h2>"Accessibility Testing Example"</h2>
            <div
                node_ref=test_element
                role="button"
                tabindex="0"
                aria_label="Test button"
            >
                "Test Element"
            </div>
            <Button on_click=handle_test>
                "Test Accessibility"
            </Button>
        </div>
    }
}
```

## Best Practices

### Component Design

1. **Always include accessibility attributes**: Use proper ARIA labels, roles, and properties
2. **Support keyboard navigation**: Ensure all interactive elements are keyboard accessible
3. **Provide focus management**: Implement proper focus handling for modals and overlays
4. **Use semantic HTML**: Choose appropriate HTML elements for your content
5. **Test with screen readers**: Verify your components work with assistive technologies

### Theming

1. **Use the theme builder**: Leverage the advanced theme builder for consistent theming
2. **Validate themes**: Always validate your themes before using them
3. **Support inheritance**: Use theme inheritance for related themes
4. **Export and import**: Use theme export/import for sharing themes
5. **Test accessibility**: Ensure your themes meet accessibility standards

### Data Visualization

1. **Provide alternative text**: Include text descriptions for charts and graphs
2. **Support keyboard navigation**: Allow users to navigate data points with keyboard
3. **Use appropriate colors**: Ensure sufficient color contrast and support color blindness
4. **Provide data tables**: Offer tabular data as an alternative to visualizations
5. **Test with assistive technologies**: Verify your visualizations work with screen readers

## Conclusion

These new features provide powerful capabilities for building sophisticated, accessible, and well-themed applications with Radix-Leptos. By following the best practices and using the provided examples, you can create exceptional user experiences that work for everyone.

For more information, see the [API Reference](api-reference/new-features-api.md) and [Examples](examples/new-features-examples.md).
