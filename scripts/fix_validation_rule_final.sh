#!/bin/bash
# Fix ValidationRule struct syntax in test files - final fix

echo "Fixing ValidationRule struct syntax in test files..."

# Fix ValidationRule struct syntax - replace with proper struct initialization
find crates/ -name "*.rs" -path "*/tests/*" -exec sed -i '' 's/ValidationRule {ValidationRuleType::Required, "\([^"]*\)"}/ValidationRule { rule_type: ValidationRuleType::Required, message: "\1".to_string(), value: None }/g' {} \;
find crates/ -name "*.rs" -path "*/tests/*" -exec sed -i '' 's/ValidationRule {ValidationRuleType::Email, "\([^"]*\)"}/ValidationRule { rule_type: ValidationRuleType::Email, message: "\1".to_string(), value: None }/g' {} \;
find crates/ -name "*.rs" -path "*/tests/*" -exec sed -i '' 's/ValidationRule {ValidationRuleType::Phone, "\([^"]*\)"}/ValidationRule { rule_type: ValidationRuleType::Phone, message: "\1".to_string(), value: None }/g' {} \;
find crates/ -name "*.rs" -path "*/tests/*" -exec sed -i '' 's/ValidationRule {ValidationRuleType::Number, "\([^"]*\)"}/ValidationRule { rule_type: ValidationRuleType::Number, message: "\1".to_string(), value: None }/g' {} \;

echo "ValidationRule struct syntax fixed."
echo "Please review changes and test compilation with: cargo test --workspace"
