# API Governance

This directory contains tools and processes for maintaining API consistency, detecting breaking changes, and ensuring comprehensive API documentation for Radix-Leptos.

## Overview

API Governance ensures that:
- Component APIs are well-defined and consistent
- Breaking changes are detected and documented
- Documentation stays in sync with implementation
- API contracts are validated through automated testing

## Components

### 1. API Specification

- **`component-api-schema.json`** - JSON Schema defining the structure of component API specifications
- **`component-api-spec.json`** - Complete API specification for all components

### 2. Contract Validation

- **`component_api_validation.rs`** - Automated tests that validate component implementations against the API specification
- Ensures all components have required props, variants, sizes, and events
- Validates accessibility features and documentation completeness

### 3. Breaking Change Detection

- **`breaking-change-detector.rs`** - Tool that compares API specifications between versions
- Detects removed props, variants, sizes, events, and components
- Generates migration guides for breaking changes
- Categorizes changes by severity (Major, Minor, Patch)

### 4. Documentation Generation

- **`generate-api-docs.rs`** - Automated documentation generator
- Creates comprehensive API reference from specification
- Generates component-specific documentation
- Creates type definitions, event documentation, and accessibility guides

### 5. Governance Script

- **`run-api-governance.sh`** - Master script that runs all governance checks
- Validates API specification against schema
- Runs contract validation tests
- Checks for breaking changes
- Generates and validates documentation
- Creates compliance reports

## Usage

### Running API Governance Checks

```bash
# Run all API governance checks
./scripts/api-governance/run-api-governance.sh
```

### Adding New Components

1. **Update API Specification**
   ```bash
   # Edit the component specification
   vim docs/api-spec/component-api-spec.json
   ```

2. **Validate Specification**
   ```bash
   # Check JSON schema compliance
   ajv validate -s docs/api-spec/component-api-schema.json -d docs/api-spec/component-api-spec.json
   ```

3. **Update Contract Tests**
   ```bash
   # Add component to validation tests
   vim tests/api-contract/component_api_validation.rs
   ```

4. **Generate Documentation**
   ```bash
   # Regenerate API documentation
   cargo run --bin generate-api-docs -- docs/api-spec/component-api-spec.json docs/api-reference
   ```

### Detecting Breaking Changes

```bash
# Compare with previous version
cargo run --bin breaking-change-detector -- \
  docs/api-spec/component-api-spec.json \
  docs/api-spec/component-api-spec-previous.json
```

## API Specification Format

### Component Structure

```json
{
  "name": "ComponentName",
  "description": "Brief description of the component",
  "category": "form|feedback|media|navigation|advanced|mobile",
  "version_added": "0.1.0",
  "deprecated": false,
  "props": [
    {
      "name": "prop_name",
      "type": "Option<PropType>",
      "optional": true,
      "default": "default_value",
      "description": "Description of the prop",
      "examples": ["example1", "example2"]
    }
  ],
  "variants": [
    {
      "name": "VariantName",
      "description": "Description of the variant",
      "css_classes": ["class1", "class2"]
    }
  ],
  "sizes": [
    {
      "name": "SizeName",
      "description": "Description of the size",
      "css_classes": ["size-class"]
    }
  ],
  "events": [
    {
      "name": "event_name",
      "type": "EventType",
      "description": "When the event is triggered"
    }
  ],
  "accessibility": {
    "aria_attributes": ["aria-label", "aria-disabled"],
    "keyboard_navigation": true,
    "screen_reader_support": true
  },
  "examples": [
    {
      "title": "Example Title",
      "description": "Description of the example",
      "code": "<Component>Example</Component>"
    }
  ]
}
```

## Testing Strategy

### Contract Validation Tests

- **Component Instantiation** - Verify components can be created with all prop combinations
- **Prop Validation** - Ensure all props match specification
- **Variant Validation** - Verify all variants are available
- **Size Validation** - Check all sizes are supported
- **Event Validation** - Confirm all events are properly defined
- **Accessibility Validation** - Ensure accessibility features are documented

### Breaking Change Tests

- **Prop Removal** - Detect removed props
- **Type Changes** - Identify changed prop types
- **Variant Removal** - Find removed variants
- **Size Removal** - Detect removed sizes
- **Event Removal** - Identify removed events
- **Component Removal** - Find removed components

## Best Practices

### API Design

1. **Consistency** - Use consistent naming patterns across components
2. **Optionality** - Make props optional when they have sensible defaults
3. **Types** - Use specific types rather than generic ones
4. **Documentation** - Provide clear descriptions and examples
5. **Accessibility** - Always specify accessibility features

### Versioning

1. **Semantic Versioning** - Follow semver for API changes
2. **Breaking Changes** - Require major version bump
3. **Deprecation** - Mark deprecated features clearly
4. **Migration Guides** - Provide clear migration paths

### Documentation

1. **Completeness** - Document all props, variants, sizes, and events
2. **Examples** - Provide practical usage examples
3. **Accessibility** - Document accessibility features
4. **Migration** - Include migration guides for changes

## Integration with CI/CD

### Pre-commit Hooks

```bash
# Add to .git/hooks/pre-commit
#!/bin/bash
./scripts/api-governance/run-api-governance.sh
```

### GitHub Actions

```yaml
name: API Governance
on: [push, pull_request]
jobs:
  api-governance:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Run API Governance
        run: ./scripts/api-governance/run-api-governance.sh
```

## Troubleshooting

### Common Issues

1. **Schema Validation Errors**
   - Check JSON syntax in API specification
   - Verify all required fields are present
   - Ensure types match schema definitions

2. **Contract Test Failures**
   - Update tests when adding new components
   - Ensure component implementations match specification
   - Check prop types and optionality

3. **Breaking Change Detection**
   - Ensure previous API specification exists
   - Check that changes are properly categorized
   - Verify migration guides are complete

### Getting Help

- Check the API specification examples
- Review existing component implementations
- Run individual governance tools for debugging
- Consult the component documentation

## Future Enhancements

- [ ] OpenAPI/Swagger integration
- [ ] Automated API client generation
- [ ] Visual API documentation
- [ ] Interactive component playground
- [ ] API performance monitoring
- [ ] Automated migration tooling
