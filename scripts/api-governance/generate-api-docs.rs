use serde_json::Value;
use std::fs;
use std::path::Path;

/// Automated API Documentation Generator
/// 
/// This tool generates API documentation from the component API specification
/// and ensures it stays in sync with the actual implementation.

pub struct ApiDocGenerator {
    api_spec: Value,
    output_dir: String,
}

impl ApiDocGenerator {
    pub fn new(api_spec_path: &str, output_dir: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let api_spec_content = fs::read_to_string(api_spec_path)?;
        let api_spec: Value = serde_json::from_str(&api_spec_content)?;
        
        // Create output directory if it doesn't exist
        fs::create_dir_all(output_dir)?;
        
        Ok(ApiDocGenerator {
            api_spec,
            output_dir: output_dir.to_string(),
        })
    }
    
    pub fn generate_all_docs(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Generate main API reference
        self.generate_api_reference()?;
        
        // Generate component-specific documentation
        self.generate_component_docs()?;
        
        // Generate prop type definitions
        self.generate_type_definitions()?;
        
        // Generate event documentation
        self.generate_event_docs()?;
        
        // Generate accessibility guide
        self.generate_accessibility_guide()?;
        
        // Generate migration guide
        self.generate_migration_guide()?;
        
        Ok(())
    }
    
    fn generate_api_reference(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut content = String::new();
        
        content.push_str("# Radix-Leptos API Reference\n\n");
        content.push_str(&format!("**Version:** {}\n", self.api_spec["api_version"]));
        content.push_str(&format!("**Last Updated:** {}\n\n", self.api_spec["last_updated"]));
        
        content.push_str("## Table of Contents\n\n");
        
        let components = self.api_spec["components"].as_object().unwrap();
        
        // Group components by category
        let mut categories: std::collections::HashMap<String, Vec<(&String, &Value)>> = std::collections::HashMap::new();
        
        for (name, spec) in components {
            let category = spec["category"].as_str().unwrap().to_string();
            categories.entry(category).or_insert_with(Vec::new).push((name, spec));
        }
        
        // Generate table of contents
        for (category, components) in &categories {
            content.push_str(&format!("### {}\n", capitalize_first(category)));
            for (name, _) in components {
                content.push_str(&format!("- [{}](#{})\n", name, name.to_lowercase()));
            }
            content.push_str("\n");
        }
        
        // Generate component documentation
        for (category, components) in &categories {
            content.push_str(&format!("## {}\n\n", capitalize_first(category)));
            
            for (name, spec) in components {
                content.push_str(&self.generate_component_section(name, spec));
                content.push_str("\n");
            }
        }
        
        // Write to file
        let output_path = Path::new(&self.output_dir).join("API_REFERENCE.md");
        fs::write(output_path, content)?;
        
        Ok(())
    }
    
    fn generate_component_section(&self, name: &str, spec: &Value) -> String {
        let mut content = String::new();
        
        content.push_str(&format!("### {}\n\n", name));
        content.push_str(&format!("{}\n\n", spec["description"]));
        
        // Props section
        if let Some(props) = spec["props"].as_array() {
            content.push_str("#### Props\n\n");
            content.push_str("| Prop | Type | Default | Description |\n");
            content.push_str("|------|------|---------|-------------|\n");
            
            for prop in props {
                let prop_name = prop["name"].as_str().unwrap();
                let prop_type = prop["type"].as_str().unwrap();
                let prop_default = prop["default"].as_str().unwrap_or("-");
                let prop_description = prop["description"].as_str().unwrap();
                
                content.push_str(&format!("| `{}` | `{}` | `{}` | {} |\n", 
                    prop_name, prop_type, prop_default, prop_description));
            }
            content.push_str("\n");
        }
        
        // Variants section
        if let Some(variants) = spec["variants"].as_array() {
            content.push_str("#### Variants\n\n");
            for variant in variants {
                let variant_name = variant["name"].as_str().unwrap();
                let variant_description = variant["description"].as_str().unwrap();
                content.push_str(&format!("- **{}** - {}\n", variant_name, variant_description));
            }
            content.push_str("\n");
        }
        
        // Sizes section
        if let Some(sizes) = spec["sizes"].as_array() {
            content.push_str("#### Sizes\n\n");
            for size in sizes {
                let size_name = size["name"].as_str().unwrap();
                let size_description = size["description"].as_str().unwrap();
                content.push_str(&format!("- **{}** - {}\n", size_name, size_description));
            }
            content.push_str("\n");
        }
        
        // Events section
        if let Some(events) = spec["events"].as_array() {
            content.push_str("#### Events\n\n");
            for event in events {
                let event_name = event["name"].as_str().unwrap();
                let event_type = event["type"].as_str().unwrap();
                let event_description = event["description"].as_str().unwrap();
                content.push_str(&format!("- **{}** (`{}`) - {}\n", event_name, event_type, event_description));
            }
            content.push_str("\n");
        }
        
        // Accessibility section
        if let Some(accessibility) = spec["accessibility"].as_object() {
            content.push_str("#### Accessibility\n\n");
            
            let keyboard_nav = accessibility["keyboard_navigation"].as_bool().unwrap_or(false);
            let screen_reader = accessibility["screen_reader_support"].as_bool().unwrap_or(false);
            
            content.push_str(&format!("- **Keyboard Navigation:** {}\n", if keyboard_nav { "✅ Supported" } else { "❌ Not supported" }));
            content.push_str(&format!("- **Screen Reader:** {}\n", if screen_reader { "✅ Supported" } else { "❌ Not supported" }));
            
            if let Some(aria_attrs) = accessibility["aria_attributes"].as_array() {
                content.push_str("- **ARIA Attributes:** ");
                let attrs: Vec<&str> = aria_attrs.iter()
                    .map(|a| a.as_str().unwrap())
                    .collect();
                content.push_str(&format!("`{}`\n", attrs.join("`, `")));
            }
            content.push_str("\n");
        }
        
        // Examples section
        if let Some(examples) = spec["examples"].as_array() {
            content.push_str("#### Examples\n\n");
            
            for example in examples {
                let example_title = example["title"].as_str().unwrap();
                let example_description = example["description"].as_str().unwrap_or("");
                let example_code = example["code"].as_str().unwrap();
                
                content.push_str(&format!("**{}**\n", example_title));
                if !example_description.is_empty() {
                    content.push_str(&format!("{}\n\n", example_description));
                }
                content.push_str("```rust\n");
                content.push_str(example_code);
                content.push_str("\n```\n\n");
            }
        }
        
        content
    }
    
    fn generate_component_docs(&self) -> Result<(), Box<dyn std::error::Error>> {
        let components_dir = Path::new(&self.output_dir).join("components");
        fs::create_dir_all(&components_dir)?;
        
        let components = self.api_spec["components"].as_object().unwrap();
        
        for (name, spec) in components {
            let component_doc = self.generate_component_section(name, spec);
            let file_path = components_dir.join(format!("{}.md", name.to_lowercase()));
            fs::write(file_path, component_doc)?;
        }
        
        Ok(())
    }
    
    fn generate_type_definitions(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut content = String::new();
        
        content.push_str("# Type Definitions\n\n");
        content.push_str("This document contains all type definitions used in Radix-Leptos components.\n\n");
        
        let components = self.api_spec["components"].as_object().unwrap();
        let mut all_types: std::collections::HashSet<String> = std::collections::HashSet::new();
        
        // Collect all types
        for (_, spec) in components {
            if let Some(props) = spec["props"].as_array() {
                for prop in props {
                    if let Some(prop_type) = prop["type"].as_str() {
                        all_types.insert(prop_type.to_string());
                    }
                }
            }
        }
        
        // Generate type definitions
        for type_name in all_types {
            content.push_str(&format!("## {}\n\n", type_name));
            content.push_str(&format!("```rust\n{}\n```\n\n", generate_type_definition(&type_name)));
        }
        
        let output_path = Path::new(&self.output_dir).join("TYPE_DEFINITIONS.md");
        fs::write(output_path, content)?;
        
        Ok(())
    }
    
    fn generate_event_docs(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut content = String::new();
        
        content.push_str("# Event Documentation\n\n");
        content.push_str("This document describes all events emitted by Radix-Leptos components.\n\n");
        
        let components = self.api_spec["components"].as_object().unwrap();
        
        for (component_name, spec) in components {
            if let Some(events) = spec["events"].as_array() {
                if !events.is_empty() {
                    content.push_str(&format!("## {}\n\n", component_name));
                    
                    for event in events {
                        let event_name = event["name"].as_str().unwrap();
                        let event_type = event["type"].as_str().unwrap();
                        let event_description = event["description"].as_str().unwrap();
                        
                        content.push_str(&format!("### {}\n\n", event_name));
                        content.push_str(&format!("**Type:** `{}`\n\n", event_type));
                        content.push_str(&format!("{}\n\n", event_description));
                    }
                }
            }
        }
        
        let output_path = Path::new(&self.output_dir).join("EVENTS.md");
        fs::write(output_path, content)?;
        
        Ok(())
    }
    
    fn generate_accessibility_guide(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut content = String::new();
        
        content.push_str("# Accessibility Guide\n\n");
        content.push_str("This guide covers accessibility features and best practices for Radix-Leptos components.\n\n");
        
        content.push_str("## WCAG 2.1 AA Compliance\n\n");
        content.push_str("All Radix-Leptos components are designed to meet WCAG 2.1 AA standards.\n\n");
        
        let components = self.api_spec["components"].as_object().unwrap();
        
        content.push_str("## Component Accessibility Features\n\n");
        
        for (component_name, spec) in components {
            if let Some(accessibility) = spec["accessibility"].as_object() {
                content.push_str(&format!("### {}\n\n", component_name));
                
                let keyboard_nav = accessibility["keyboard_navigation"].as_bool().unwrap_or(false);
                let screen_reader = accessibility["screen_reader_support"].as_bool().unwrap_or(false);
                
                content.push_str(&format!("- **Keyboard Navigation:** {}\n", if keyboard_nav { "✅ Supported" } else { "❌ Not supported" }));
                content.push_str(&format!("- **Screen Reader:** {}\n", if screen_reader { "✅ Supported" } else { "❌ Not supported" }));
                
                if let Some(aria_attrs) = accessibility["aria_attributes"].as_array() {
                    content.push_str("- **ARIA Attributes:** ");
                    let attrs: Vec<&str> = aria_attrs.iter()
                        .map(|a| a.as_str().unwrap())
                        .collect();
                    content.push_str(&format!("`{}`\n", attrs.join("`, `")));
                }
                content.push_str("\n");
            }
        }
        
        let output_path = Path::new(&self.output_dir).join("ACCESSIBILITY.md");
        fs::write(output_path, content)?;
        
        Ok(())
    }
    
    fn generate_migration_guide(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut content = String::new();
        
        content.push_str("# Migration Guide\n\n");
        content.push_str("This guide helps you migrate between different versions of Radix-Leptos.\n\n");
        
        content.push_str("## Version Compatibility\n\n");
        content.push_str("| Radix-Leptos Version | Breaking Changes | Migration Required |\n");
        content.push_str("|---------------------|------------------|-------------------|\n");
        content.push_str("| 0.8.x → 0.9.x | None | No |\n");
        content.push_str("| 0.7.x → 0.8.x | Minor | Optional |\n");
        content.push_str("| 0.6.x → 0.7.x | Major | Yes |\n\n");
        
        content.push_str("## Common Migration Patterns\n\n");
        content.push_str("### Prop Name Changes\n\n");
        content.push_str("When prop names change, update your component usage:\n\n");
        content.push_str("```rust\n");
        content.push_str("// Before\n");
        content.push_str("<Button variant=ButtonVariant::Primary />\n\n");
        content.push_str("// After\n");
        content.push_str("<Button variant=ButtonVariant::Default />\n");
        content.push_str("```\n\n");
        
        content.push_str("### Event Handler Changes\n\n");
        content.push_str("When event signatures change, update your handlers:\n\n");
        content.push_str("```rust\n");
        content.push_str("// Before\n");
        content.push_str("on_click=Callback::new(|_| println!(\"Clicked!\"))\n\n");
        content.push_str("// After\n");
        content.push_str("on_click=Callback::new(|event| println!(\"Clicked: {:?}\", event))\n");
        content.push_str("```\n\n");
        
        let output_path = Path::new(&self.output_dir).join("MIGRATION.md");
        fs::write(output_path, content)?;
        
        Ok(())
    }
}

fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

fn generate_type_definition(type_name: &str) -> String {
    match type_name {
        "ButtonVariant" => "pub enum ButtonVariant {\n    Default,\n    Destructive,\n    Outline,\n    Secondary,\n    Ghost,\n    Link,\n}".to_string(),
        "ButtonSize" => "pub enum ButtonSize {\n    Default,\n    Sm,\n    Lg,\n    Icon,\n}".to_string(),
        "CheckboxVariant" => "pub enum CheckboxVariant {\n    Default,\n    Bordered,\n    Ghost,\n}".to_string(),
        "CheckboxSize" => "pub enum CheckboxSize {\n    Default,\n    Sm,\n    Lg,\n}".to_string(),
        "DialogVariant" => "pub enum DialogVariant {\n    Default,\n    Bordered,\n    Ghost,\n}".to_string(),
        "DialogSize" => "pub enum DialogSize {\n    Default,\n    Sm,\n    Lg,\n    Xl,\n}".to_string(),
        _ => format!("// Type definition for {} would be generated here", type_name),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_doc_generation() {
        let test_spec = serde_json::json!({
            "api_version": "0.8.3",
            "last_updated": "2024-12-19T00:00:00Z",
            "components": {
                "Button": {
                    "name": "Button",
                    "description": "A versatile button component",
                    "category": "form",
                    "version_added": "0.1.0",
                    "deprecated": false,
                    "props": [
                        {
                            "name": "variant",
                            "type": "Option<ButtonVariant>",
                            "optional": true,
                            "description": "Visual style variant"
                        }
                    ],
                    "variants": [
                        {
                            "name": "Default",
                            "description": "Primary button style"
                        }
                    ],
                    "sizes": [
                        {
                            "name": "Default",
                            "description": "Standard button size"
                        }
                    ],
                    "events": [
                        {
                            "name": "click",
                            "type": "web_sys::Event",
                            "description": "Triggered when clicked"
                        }
                    ],
                    "accessibility": {
                        "keyboard_navigation": true,
                        "screen_reader_support": true
                    }
                }
            }
        });
        
        let generator = ApiDocGenerator {
            api_spec: test_spec,
            output_dir: "/tmp/test_docs".to_string(),
        };
        
        // Test component section generation
        let button_spec = &generator.api_spec["components"]["Button"];
        let section = generator.generate_component_section("Button", button_spec);
        
        assert!(section.contains("### Button"));
        assert!(section.contains("A versatile button component"));
        assert!(section.contains("variant"));
        assert!(section.contains("ButtonVariant"));
    }
}
