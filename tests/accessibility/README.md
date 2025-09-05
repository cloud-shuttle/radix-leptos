# Accessibility Tests

This directory contains accessibility tests to ensure Radix-Leptos components meet WCAG guidelines.

## Structure

```
accessibility/
├── keyboard-navigation/  # Keyboard navigation tests
├── screen-reader/       # Screen reader compatibility
├── color-contrast/      # Color contrast tests
├── focus-management/    # Focus handling tests
└── aria-compliance/     # ARIA attribute tests
```

## Running Accessibility Tests

```bash
# Run all accessibility tests
pnpm test --grep "accessibility"

# Run specific accessibility test
pnpm test tests/accessibility/keyboard-navigation.spec.ts

# Run with accessibility tools
pnpm test --grep "accessibility" --reporter=json
```

## Test Categories

### Keyboard Navigation
- **Tab Order**: Logical tab sequence
- **Arrow Keys**: Arrow key navigation
- **Enter/Space**: Activation keys
- **Escape**: Escape key handling
- **Focus Trapping**: Modal focus management

### Screen Reader Support
- **ARIA Labels**: Proper labeling
- **ARIA Descriptions**: Descriptive text
- **ARIA States**: State announcements
- **Live Regions**: Dynamic content updates
- **Landmark Roles**: Page structure

### Visual Accessibility
- **Color Contrast**: WCAG AA compliance
- **Focus Indicators**: Visible focus states
- **Text Scaling**: Responsive text sizing
- **High Contrast**: High contrast mode support
- **Motion Preferences**: Reduced motion support

### WCAG Compliance
- **Level A**: Basic accessibility requirements
- **Level AA**: Enhanced accessibility (target)
- **Level AAA**: Maximum accessibility (aspirational)

## Accessibility Standards

- **WCAG 2.1 AA**: Primary compliance target
- **Section 508**: US federal accessibility standards
- **EN 301 549**: European accessibility standards
- **ADA**: Americans with Disabilities Act compliance

## Testing Tools

- **axe-core**: Automated accessibility testing
- **Lighthouse**: Accessibility auditing
- **WAVE**: Web accessibility evaluation
- **NVDA**: Screen reader testing
- **VoiceOver**: macOS screen reader testing
- **JAWS**: Windows screen reader testing

## Test Guidelines

- **Automated Testing**: Use axe-core for automated checks
- **Manual Testing**: Test with actual screen readers
- **User Testing**: Include users with disabilities
- **Continuous Monitoring**: Regular accessibility audits
- **Documentation**: Document accessibility features
