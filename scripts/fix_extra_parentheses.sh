#!/bin/bash
# Fix extra closing parentheses in test files

echo "Fixing extra closing parentheses..."

# Remove extra closing parentheses from ValidationRule calls
find crates/ -name "*.rs" -path "*/tests/*" -exec sed -i '' 's/ValidationRule { rule_type: ValidationRuleType::Required, message: "\([^"]*\)".to_string(), value: None })/ValidationRule { rule_type: ValidationRuleType::Required, message: "\1".to_string(), value: None }/g' {} \;
find crates/ -name "*.rs" -path "*/tests/*" -exec sed -i '' 's/ValidationRule { rule_type: ValidationRuleType::Email, message: "\([^"]*\)".to_string(), value: None })/ValidationRule { rule_type: ValidationRuleType::Email, message: "\1".to_string(), value: None }/g' {} \;
find crates/ -name "*.rs" -path "*/tests/*" -exec sed -i '' 's/ValidationRule { rule_type: ValidationRuleType::Phone, message: "\([^"]*\)".to_string(), value: None })/ValidationRule { rule_type: ValidationRuleType::Phone, message: "\1".to_string(), value: None }/g' {} \;

echo "Extra closing parentheses fixed."
echo "Please review changes and test compilation with: cargo test"
