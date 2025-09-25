#!/bin/bash
# Fix syntax errors in test files

echo "Fixing test syntax errors..."

# Fix ValidationRule syntax - add missing closing braces
find crates/ -name "*.rs" -path "*/tests/*" -exec sed -i '' 's/ValidationRule {rule_type: ValidationRuleType::Required, message: "\([^"]*\)"/ValidationRule { rule_type: ValidationRuleType::Required, message: "\1" }/g' {} \;
find crates/ -name "*.rs" -path "*/tests/*" -exec sed -i '' 's/ValidationRule {rule_type: ValidationRuleType::Email, message: "\([^"]*\)"/ValidationRule { rule_type: ValidationRuleType::Email, message: "\1" }/g' {} \;
find crates/ -name "*.rs" -path "*/tests/*" -exec sed -i '' 's/ValidationRule {rule_type: ValidationRuleType::Phone, message: "\([^"]*\)"/ValidationRule { rule_type: ValidationRuleType::Phone, message: "\1" }/g' {} \;

echo "Test syntax errors fixed."
echo "Please review changes and test compilation with: cargo test"
