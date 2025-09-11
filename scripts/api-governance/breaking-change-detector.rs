use serde_json::{Value, Map};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// API Breaking Change Detector
/// 
/// This tool detects breaking changes in the component API by comparing
/// the current API specification with previous versions.

#[derive(Debug, Clone)]
pub struct BreakingChange {
    pub change_type: ChangeType,
    pub component: String,
    pub description: String,
    pub severity: Severity,
    pub migration_guide: Option<String>,
}

#[derive(Debug, Clone)]
pub enum ChangeType {
    RemovedProp,
    RemovedVariant,
    RemovedSize,
    RemovedEvent,
    ChangedPropType,
    ChangedDefaultValue,
    RemovedComponent,
    DeprecatedComponent,
}

#[derive(Debug, Clone)]
pub enum Severity {
    Major,    // Breaking change requiring major version bump
    Minor,    // Non-breaking change requiring minor version bump
    Patch,    // Bug fix or non-breaking change
}

pub struct ApiChangeDetector {
    current_spec: Value,
    previous_spec: Value,
}

impl ApiChangeDetector {
    pub fn new(current_spec_path: &str, previous_spec_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let current_spec = fs::read_to_string(current_spec_path)?;
        let previous_spec = fs::read_to_string(previous_spec_path)?;
        
        let current: Value = serde_json::from_str(&current_spec)?;
        let previous: Value = serde_json::from_str(&previous_spec)?;
        
        Ok(ApiChangeDetector {
            current_spec: current,
            previous_spec: previous,
        })
    }
    
    pub fn detect_breaking_changes(&self) -> Vec<BreakingChange> {
        let mut changes = Vec::new();
        
        // Check for removed components
        changes.extend(self.detect_removed_components());
        
        // Check for deprecated components
        changes.extend(self.detect_deprecated_components());
        
        // Check for component-level changes
        changes.extend(self.detect_component_changes());
        
        changes
    }
    
    fn detect_removed_components(&self) -> Vec<BreakingChange> {
        let mut changes = Vec::new();
        
        let current_components = self.current_spec["components"].as_object().unwrap();
        let previous_components = self.previous_spec["components"].as_object().unwrap();
        
        for (component_name, _) in previous_components {
            if !current_components.contains_key(component_name) {
                changes.push(BreakingChange {
                    change_type: ChangeType::RemovedComponent,
                    component: component_name.clone(),
                    description: format!("Component '{}' has been removed", component_name),
                    severity: Severity::Major,
                    migration_guide: Some(format!("Replace '{}' with an alternative component or implement custom solution", component_name)),
                });
            }
        }
        
        changes
    }
    
    fn detect_deprecated_components(&self) -> Vec<BreakingChange> {
        let mut changes = Vec::new();
        
        let current_components = self.current_spec["components"].as_object().unwrap();
        let previous_components = self.previous_spec["components"].as_object().unwrap();
        
        for (component_name, current_spec) in current_components {
            if let Some(previous_spec) = previous_components.get(component_name) {
                let current_deprecated = current_spec["deprecated"].as_bool().unwrap_or(false);
                let previous_deprecated = previous_spec["deprecated"].as_bool().unwrap_or(false);
                
                if current_deprecated && !previous_deprecated {
                    changes.push(BreakingChange {
                        change_type: ChangeType::DeprecatedComponent,
                        component: component_name.clone(),
                        description: format!("Component '{}' has been deprecated", component_name),
                        severity: Severity::Minor,
                        migration_guide: Some(format!("Consider migrating from '{}' to the recommended alternative", component_name)),
                    });
                }
            }
        }
        
        changes
    }
    
    fn detect_component_changes(&self) -> Vec<BreakingChange> {
        let mut changes = Vec::new();
        
        let current_components = self.current_spec["components"].as_object().unwrap();
        let previous_components = self.previous_spec["components"].as_object().unwrap();
        
        for (component_name, current_spec) in current_components {
            if let Some(previous_spec) = previous_components.get(component_name) {
                // Check for prop changes
                changes.extend(self.detect_prop_changes(component_name, current_spec, previous_spec));
                
                // Check for variant changes
                changes.extend(self.detect_variant_changes(component_name, current_spec, previous_spec));
                
                // Check for size changes
                changes.extend(self.detect_size_changes(component_name, current_spec, previous_spec));
                
                // Check for event changes
                changes.extend(self.detect_event_changes(component_name, current_spec, previous_spec));
            }
        }
        
        changes
    }
    
    fn detect_prop_changes(&self, component_name: &str, current: &Value, previous: &Value) -> Vec<BreakingChange> {
        let mut changes = Vec::new();
        
        let current_props = current["props"].as_array().unwrap_or(&vec![]);
        let previous_props = previous["props"].as_array().unwrap_or(&vec![]);
        
        // Create maps for easier lookup
        let current_prop_map: HashMap<String, &Value> = current_props.iter()
            .map(|p| (p["name"].as_str().unwrap().to_string(), p))
            .collect();
        
        let previous_prop_map: HashMap<String, &Value> = previous_props.iter()
            .map(|p| (p["name"].as_str().unwrap().to_string(), p))
            .collect();
        
        // Check for removed props
        for (prop_name, _) in &previous_prop_map {
            if !current_prop_map.contains_key(prop_name) {
                changes.push(BreakingChange {
                    change_type: ChangeType::RemovedProp,
                    component: component_name.to_string(),
                    description: format!("Prop '{}' has been removed from component '{}'", prop_name, component_name),
                    severity: Severity::Major,
                    migration_guide: Some(format!("Remove usage of '{}' prop from '{}' component", prop_name, component_name)),
                });
            }
        }
        
        // Check for type changes
        for (prop_name, current_prop) in &current_prop_map {
            if let Some(previous_prop) = previous_prop_map.get(prop_name) {
                let current_type = current_prop["type"].as_str().unwrap();
                let previous_type = previous_prop["type"].as_str().unwrap();
                
                if current_type != previous_type {
                    changes.push(BreakingChange {
                        change_type: ChangeType::ChangedPropType,
                        component: component_name.to_string(),
                        description: format!("Prop '{}' type changed from '{}' to '{}' in component '{}'", 
                            prop_name, previous_type, current_type, component_name),
                        severity: Severity::Major,
                        migration_guide: Some(format!("Update '{}' prop usage to match new type '{}'", prop_name, current_type)),
                    });
                }
            }
        }
        
        changes
    }
    
    fn detect_variant_changes(&self, component_name: &str, current: &Value, previous: &Value) -> Vec<BreakingChange> {
        let mut changes = Vec::new();
        
        let current_variants = current["variants"].as_array().unwrap_or(&vec![]);
        let previous_variants = previous["variants"].as_array().unwrap_or(&vec![]);
        
        let current_variant_names: std::collections::HashSet<String> = current_variants.iter()
            .map(|v| v["name"].as_str().unwrap().to_string())
            .collect();
        
        let previous_variant_names: std::collections::HashSet<String> = previous_variants.iter()
            .map(|v| v["name"].as_str().unwrap().to_string())
            .collect();
        
        // Check for removed variants
        for variant_name in &previous_variant_names {
            if !current_variant_names.contains(variant_name) {
                changes.push(BreakingChange {
                    change_type: ChangeType::RemovedVariant,
                    component: component_name.to_string(),
                    description: format!("Variant '{}' has been removed from component '{}'", variant_name, component_name),
                    severity: Severity::Major,
                    migration_guide: Some(format!("Replace '{}' variant with an available alternative in '{}' component", variant_name, component_name)),
                });
            }
        }
        
        changes
    }
    
    fn detect_size_changes(&self, component_name: &str, current: &Value, previous: &Value) -> Vec<BreakingChange> {
        let mut changes = Vec::new();
        
        let current_sizes = current["sizes"].as_array().unwrap_or(&vec![]);
        let previous_sizes = previous["sizes"].as_array().unwrap_or(&vec![]);
        
        let current_size_names: std::collections::HashSet<String> = current_sizes.iter()
            .map(|s| s["name"].as_str().unwrap().to_string())
            .collect();
        
        let previous_size_names: std::collections::HashSet<String> = previous_sizes.iter()
            .map(|s| s["name"].as_str().unwrap().to_string())
            .collect();
        
        // Check for removed sizes
        for size_name in &previous_size_names {
            if !current_size_names.contains(size_name) {
                changes.push(BreakingChange {
                    change_type: ChangeType::RemovedSize,
                    component: component_name.to_string(),
                    description: format!("Size '{}' has been removed from component '{}'", size_name, component_name),
                    severity: Severity::Major,
                    migration_guide: Some(format!("Replace '{}' size with an available alternative in '{}' component", size_name, component_name)),
                });
            }
        }
        
        changes
    }
    
    fn detect_event_changes(&self, component_name: &str, current: &Value, previous: &Value) -> Vec<BreakingChange> {
        let mut changes = Vec::new();
        
        let current_events = current["events"].as_array().unwrap_or(&vec![]);
        let previous_events = previous["events"].as_array().unwrap_or(&vec![]);
        
        let current_event_names: std::collections::HashSet<String> = current_events.iter()
            .map(|e| e["name"].as_str().unwrap().to_string())
            .collect();
        
        let previous_event_names: std::collections::HashSet<String> = previous_events.iter()
            .map(|e| e["name"].as_str().unwrap().to_string())
            .collect();
        
        // Check for removed events
        for event_name in &previous_event_names {
            if !current_event_names.contains(event_name) {
                changes.push(BreakingChange {
                    change_type: ChangeType::RemovedEvent,
                    component: component_name.to_string(),
                    description: format!("Event '{}' has been removed from component '{}'", event_name, component_name),
                    severity: Severity::Major,
                    migration_guide: Some(format!("Remove event handler for '{}' event in '{}' component", event_name, component_name)),
                });
            }
        }
        
        changes
    }
    
    pub fn generate_migration_report(&self, changes: &[BreakingChange]) -> String {
        let mut report = String::new();
        
        report.push_str("# API Breaking Changes Report\n\n");
        report.push_str(&format!("Generated on: {}\n\n", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));
        
        // Group changes by severity
        let major_changes: Vec<&BreakingChange> = changes.iter().filter(|c| matches!(c.severity, Severity::Major)).collect();
        let minor_changes: Vec<&BreakingChange> = changes.iter().filter(|c| matches!(c.severity, Severity::Minor)).collect();
        
        if !major_changes.is_empty() {
            report.push_str("## 🚨 Major Breaking Changes\n\n");
            for change in major_changes {
                report.push_str(&format!("### {}\n", change.component));
                report.push_str(&format!("**Type:** {:?}\n", change.change_type));
                report.push_str(&format!("**Description:** {}\n", change.description));
                if let Some(guide) = &change.migration_guide {
                    report.push_str(&format!("**Migration Guide:** {}\n", guide));
                }
                report.push_str("\n");
            }
        }
        
        if !minor_changes.is_empty() {
            report.push_str("## ⚠️ Minor Changes\n\n");
            for change in minor_changes {
                report.push_str(&format!("### {}\n", change.component));
                report.push_str(&format!("**Type:** {:?}\n", change.change_type));
                report.push_str(&format!("**Description:** {}\n", change.description));
                if let Some(guide) = &change.migration_guide {
                    report.push_str(&format!("**Migration Guide:** {}\n", guide));
                }
                report.push_str("\n");
            }
        }
        
        if changes.is_empty() {
            report.push_str("## ✅ No Breaking Changes Detected\n\n");
            report.push_str("The API is backward compatible with the previous version.\n");
        }
        
        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_breaking_change_detection() {
        // Create test specifications
        let current_spec = serde_json::json!({
            "components": {
                "Button": {
                    "props": [
                        {"name": "variant", "type": "Option<ButtonVariant>", "optional": true},
                        {"name": "size", "type": "Option<ButtonSize>", "optional": true}
                    ]
                }
            }
        });
        
        let previous_spec = serde_json::json!({
            "components": {
                "Button": {
                    "props": [
                        {"name": "variant", "type": "Option<ButtonVariant>", "optional": true},
                        {"name":size", "type": "Option<ButtonSize>", "optional": true},
                        {"name": "disabled", "type": "Option<bool>", "optional": true}
                    ]
                }
            }
        });
        
        let detector = ApiChangeDetector {
            current_spec,
            previous_spec,
        };
        
        let changes = detector.detect_breaking_changes();
        
        // Should detect removed 'disabled' prop
        assert!(!changes.is_empty());
        assert!(changes.iter().any(|c| matches!(c.change_type, ChangeType::RemovedProp)));
    }
}
