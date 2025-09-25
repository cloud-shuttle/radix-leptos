#!/bin/bash
# Fix test import issues

echo "Fixing test import issues..."

# Fix missing imports in test files
find crates/ -name "*.rs" -path "*/tests/*" -exec sed -i '' 's/use super::\*;/use super::*;\nuse crate::utils::{generate_id, merge_classes, merge_optional_classes};\nuse crate::form_validation::{is_valid_email, is_valid_date, is_valid_time, is_valid_phone};/g' {} \;

# Fix ValidationRule::new() calls - replace with ValidationRule { ... }
find crates/ -name "*.rs" -path "*/tests/*" -exec sed -i '' 's/ValidationRule::new(/ValidationRule {/g' {} \;
find crates/ -name "*.rs" -path "*/tests/*" -exec sed -i '' 's/ValidationRuleType::Required, "\([^"]*\)"/rule_type: ValidationRuleType::Required, message: "\1"/g' {} \;
find crates/ -name "*.rs" -path "*/tests/*" -exec sed -i '' 's/ValidationRuleType::Email, "\([^"]*\)"/rule_type: ValidationRuleType::Email, message: "\1"/g' {} \;
find crates/ -name "*.rs" -path "*/tests/*" -exec sed -i '' 's/ValidationRuleType::Phone, "\([^"]*\)"/rule_type: ValidationRuleType::Phone, message: "\1"/g' {} \;

# Fix string to String conversions
find crates/ -name "*.rs" -path "*/tests/*" -exec sed -i '' 's/add_rule("\([^"]*\)",/add_rule("\1".to_string(),/g' {} \;

# Fix ButtonSize::Medium to ButtonSize::Default
find crates/ -name "*.rs" -path "*/tests/*" -exec sed -i '' 's/ButtonSize::Medium/ButtonSize::Default/g' {} \;

# Fix AlertDialogSize references - replace with AlertDialogProps
find crates/ -name "*.rs" -path "*/tests/*" -exec sed -i '' 's/AlertDialogSize/AlertDialogProps/g' {} \;

echo "Test import issues fixed."
echo "Please review changes and test compilation with: cargo test"
