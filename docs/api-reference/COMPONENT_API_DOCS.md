# Radix-Leptos Component API Documentation

This document provides comprehensive API documentation for all Radix-Leptos components, generated using TDD principles and comprehensive testing.

## Table of Contents

- [Button](#button)
- [Input](#input)
- [Select](#select)
- [Checkbox](#checkbox)
- [RadioGroup](#radiogroup)
- [Dialog](#dialog)
- [Alert](#alert)
- [Badge](#badge)
- [Card](#card)
- [Tabs](#tabs)
- [Accordion](#accordion)
- [Avatar](#avatar)
- [Progress](#progress)
- [Switch](#switch)
- [Slider](#slider)
- [Tooltip](#tooltip)
- [Popover](#popover)
- [DropdownMenu](#dropdownmenu)
- [Sheet](#sheet)
- [Table](#table)
- [Skeleton](#skeleton)
- [And more...](#additional-components)

---

## Button

A versatile button component with multiple variants and sizes.

### Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `variant` | `ButtonVariant` | `Default` | Visual style variant |
| `size` | `ButtonSize` | `Default` | Button size |
| `disabled` | `bool` | `false` | Whether button is disabled |
| `on_click` | `Callback<web_sys::Event>` | - | Click event handler |
| `class` | `String` | - | Additional CSS classes |
| `aria_label` | `String` | - | Accessibility label |
| `children` | `Children` | - | Button content |

### Variants

- `Default` - Primary button style
- `Destructive` - Destructive action style
- `Outline` - Outlined button style
- `Secondary` - Secondary button style
- `Ghost` - Ghost button style
- `Link` - Link button style

### Sizes

- `Default` - Standard button size
- `Sm` - Small button size
- `Lg` - Large button size
- `Icon` - Icon-only button size

### Example

```rust
use leptos::*;
use radix_leptos_primitives::*;

#[component]
fn MyComponent() -> impl IntoView {
    let handle_click = Callback::new(|_| {
        println!("Button clicked!");
    });

    view! {
        <Button 
            variant=ButtonVariant::Primary
            size=ButtonSize::Lg
            on_click=handle_click
        >
            "Click me"
        </Button>
    }
}
```

### Accessibility

- Supports keyboard navigation (Tab, Enter, Space)
- Proper ARIA attributes
- Focus management
- Screen reader compatible

---

## Input

A text input component with validation and accessibility features.

### Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `String` | - | Input value |
| `on_input` | `Callback<String>` | - | Input change handler |
| `placeholder` | `String` | - | Placeholder text |
| `type` | `String` | `"text"` | Input type |
| `disabled` | `bool` | `false` | Whether input is disabled |
| `required` | `bool` | `false` | Whether input is required |
| `aria_invalid` | `bool` | `false` | ARIA invalid state |
| `aria_describedby` | `String` | - | ARIA described by |
| `class` | `String` | - | Additional CSS classes |

### Example

```rust
use leptos::*;
use radix_leptos_primitives::*;

#[component]
fn MyForm() -> impl IntoView {
    let (email, set_email) = create_signal("".to_string());

    let handle_input = Callback::new(move |new_value: String| {
        set_email.set(new_value);
    });

    view! {
        <div>
            <Label for_control="email">"Email Address"</Label>
            <Input 
                id="email"
                type="email"
                value=email
                on_input=handle_input
                placeholder="Enter your email"
                required=true
            />
        </div>
    }
}
```

### Accessibility

- Proper label association
- ARIA attributes for validation states
- Keyboard navigation support
- Screen reader announcements

---

## Select

A dropdown select component with search and keyboard navigation.

### Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `String` | - | Selected value |
| `on_value_change` | `Callback<String>` | - | Value change handler |
| `disabled` | `bool` | `false` | Whether select is disabled |
| `class` | `String` | - | Additional CSS classes |

### Sub-components

- `SelectTrigger` - The clickable trigger element
- `SelectValue` - Displays the selected value
- `SelectContent` - The dropdown content container
- `SelectItem` - Individual selectable items
- `SelectGroup` - Groups related items
- `SelectLabel` - Labels for groups

### Example

```rust
use leptos::*;
use radix_leptos_primitives::*;

#[component]
fn MySelect() -> impl IntoView {
    let (selected, set_selected) = create_signal("".to_string());

    let handle_change = Callback::new(move |new_value: String| {
        set_selected.set(new_value);
    });

    view! {
        <Select value=selected on_value_change=handle_change>
            <SelectTrigger>
                <SelectValue placeholder="Select an option" />
            </SelectTrigger>
            <SelectContent>
                <SelectItem value="option1">"Option 1"</SelectItem>
                <SelectItem value="option2">"Option 2"</SelectItem>
                <SelectItem value="option3">"Option 3"</SelectItem>
            </SelectContent>
        </Select>
    }
}
```

### Accessibility

- Full keyboard navigation (Arrow keys, Enter, Escape)
- ARIA attributes for screen readers
- Focus management
- Proper labeling

---

## Checkbox

A checkbox input component with proper accessibility.

### Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `checked` | `bool` | `false` | Whether checkbox is checked |
| `on_checked_change` | `Callback<bool>` | - | Checked state change handler |
| `disabled` | `bool` | `false` | Whether checkbox is disabled |
| `required` | `bool` | `false` | Whether checkbox is required |
| `aria_label` | `String` | - | Accessibility label |
| `class` | `String` | - | Additional CSS classes |
| `children` | `Children` | - | Checkbox label content |

### Example

```rust
use leptos::*;
use radix_leptos_primitives::*;

#[component]
fn MyCheckbox() -> impl IntoView {
    let (checked, set_checked) = create_signal(false);

    let handle_change = Callback::new(move |new_checked: bool| {
        set_checked.set(new_checked);
    });

    view! {
        <div>
            <Checkbox 
                checked=checked
                on_checked_change=handle_change
            >
                "I agree to the terms and conditions"
            </Checkbox>
        </div>
    }
}
```

### Accessibility

- Proper ARIA attributes
- Keyboard navigation (Space to toggle)
- Screen reader support
- Label association

---

## RadioGroup

A radio button group component for single selection.

### Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `String` | - | Selected value |
| `on_value_change` | `Callback<String>` | - | Value change handler |
| `disabled` | `bool` | `false` | Whether group is disabled |
| `class` | `String` | - | Additional CSS classes |
| `children` | `Children` | - | Radio group content |

### Sub-components

- `RadioGroupItem` - Individual radio button
- `Label` - Label for radio button

### Example

```rust
use leptos::*;
use radix_leptos_primitives::*;

#[component]
fn MyRadioGroup() -> impl IntoView {
    let (selected, set_selected) = create_signal("".to_string());

    let handle_change = Callback::new(move |new_value: String| {
        set_selected.set(new_value);
    });

    view! {
        <RadioGroup value=selected on_value_change=handle_change>
            <div>
                <RadioGroupItem value="option1" id="radio1" />
                <Label for_control="radio1">"Option 1"</Label>
            </div>
            <div>
                <RadioGroupItem value="option2" id="radio2" />
                <Label for_control="radio2">"Option 2"</Label>
            </div>
            <div>
                <RadioGroupItem value="option3" id="radio3" />
                <Label for_control="radio3">"Option 3"</Label>
            </div>
        </RadioGroup>
    }
}
```

### Accessibility

- Proper ARIA attributes for radio groups
- Keyboard navigation (Arrow keys)
- Screen reader support
- Focus management

---

## Dialog

A modal dialog component with focus management and accessibility.

### Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `open` | `bool` | `false` | Whether dialog is open |
| `on_open_change` | `Callback<bool>` | - | Open state change handler |
| `class` | `String` | - | Additional CSS classes |

### Sub-components

- `DialogTrigger` - Element that opens the dialog
- `DialogContent` - The dialog content container
- `DialogHeader` - Dialog header section
- `DialogTitle` - Dialog title
- `DialogDescription` - Dialog description
- `DialogFooter` - Dialog footer section
- `DialogClose` - Element that closes the dialog

### Example

```rust
use leptos::*;
use radix_leptos_primitives::*;

#[component]
fn MyDialog() -> impl IntoView {
    let (open, set_open) = create_signal(false);

    let handle_open_change = Callback::new(move |new_open: bool| {
        set_open.set(new_open);
    });

    view! {
        <Dialog open=open on_open_change=handle_open_change>
            <DialogTrigger>
                <Button>"Open Dialog"</Button>
            </DialogTrigger>
            <DialogContent>
                <DialogHeader>
                    <DialogTitle>"Dialog Title"</DialogTitle>
                    <DialogDescription>
                        "This is a dialog description"
                    </DialogDescription>
                </DialogHeader>
                <div>
                    "Dialog content goes here"
                </div>
                <DialogFooter>
                    <DialogClose>
                        <Button variant=ButtonVariant::Secondary>"Cancel"</Button>
                    </DialogClose>
                    <Button>"Confirm"</Button>
                </DialogFooter>
            </DialogContent>
        </Dialog>
    }
}
```

### Accessibility

- Focus trap when open
- Escape key to close
- Proper ARIA attributes
- Screen reader announcements
- Focus restoration on close

---

## Alert

An alert component for displaying important messages.

### Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `variant` | `AlertVariant` | `Default` | Alert variant |
| `class` | `String` | - | Additional CSS classes |
| `children` | `Children` | - | Alert content |

### Variants

- `Default` - Standard alert style
- `Destructive` - Error/destructive alert style

### Sub-components

- `AlertTitle` - Alert title
- `AlertDescription` - Alert description

### Example

```rust
use leptos::*;
use radix_leptos_primitives::*;

#[component]
fn MyAlert() -> impl IntoView {
    view! {
        <Alert variant=AlertVariant::Destructive>
            <AlertTitle>"Error"</AlertTitle>
            <AlertDescription>
                "Something went wrong. Please try again."
            </AlertDescription>
        </Alert>
    }
}
```

### Accessibility

- Proper ARIA attributes
- Screen reader announcements
- Color contrast compliance
- Semantic markup

---

## Badge

A badge component for displaying status or labels.

### Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `variant` | `BadgeVariant` | `Default` | Badge variant |
| `class` | `String` | - | Additional CSS classes |
| `children` | `Children` | - | Badge content |

### Variants

- `Default` - Standard badge style
- `Secondary` - Secondary badge style
- `Destructive` - Destructive badge style
- `Outline` - Outlined badge style

### Example

```rust
use leptos::*;
use radix_leptos_primitives::*;

#[component]
fn MyBadge() -> impl IntoView {
    view! {
        <div>
            <Badge variant=BadgeVariant::Default>"Default"</Badge>
            <Badge variant=BadgeVariant::Secondary>"Secondary"</Badge>
            <Badge variant=BadgeVariant::Destructive>"Error"</Badge>
            <Badge variant=BadgeVariant::Outline>"Outline"</Badge>
        </div>
    }
}
```

### Accessibility

- Proper semantic markup
- Color contrast compliance
- Screen reader support

---

## Card

A card component for grouping related content.

### Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `class` | `String` | - | Additional CSS classes |
| `children` | `Children` | - | Card content |

### Sub-components

- `CardHeader` - Card header section
- `CardTitle` - Card title
- `CardDescription` - Card description
- `CardContent` - Card main content
- `CardFooter` - Card footer section

### Example

```rust
use leptos::*;
use radix_leptos_primitives::*;

#[component]
fn MyCard() -> impl IntoView {
    view! {
        <Card>
            <CardHeader>
                <CardTitle>"Card Title"</CardTitle>
                <CardDescription>"Card description"</CardDescription>
            </CardHeader>
            <CardContent>
                "This is the main card content"
            </CardContent>
            <CardFooter>
                <Button>"Action"</Button>
            </CardFooter>
        </Card>
    }
}
```

### Accessibility

- Proper semantic structure
- Screen reader navigation
- Focus management

---

## Tabs

A tabs component for organizing content into panels.

### Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `String` | - | Active tab value |
| `on_value_change` | `Callback<String>` | - | Tab change handler |
| `class` | `String` | - | Additional CSS classes |
| `children` | `Children` | - | Tabs content |

### Sub-components

- `TabsList` - Container for tab triggers
- `TabsTrigger` - Individual tab trigger
- `TabsContent` - Tab panel content

### Example

```rust
use leptos::*;
use radix_leptos_primitives::*;

#[component]
fn MyTabs() -> impl IntoView {
    let (active_tab, set_active_tab) = create_signal("tab1".to_string());

    let handle_tab_change = Callback::new(move |new_tab: String| {
        set_active_tab.set(new_tab);
    });

    view! {
        <Tabs value=active_tab on_value_change=handle_tab_change>
            <TabsList>
                <TabsTrigger value="tab1">"Tab 1"</TabsTrigger>
                <TabsTrigger value="tab2">"Tab 2"</TabsTrigger>
                <TabsTrigger value="tab3">"Tab 3"</TabsTrigger>
            </TabsList>
            <TabsContent value="tab1">
                "Content for Tab 1"
            </TabsContent>
            <TabsContent value="tab2">
                "Content for Tab 2"
            </TabsContent>
            <TabsContent value="tab3">
                "Content for Tab 3"
            </TabsContent>
        </Tabs>
    }
}
```

### Accessibility

- Full keyboard navigation (Arrow keys, Home, End)
- Proper ARIA attributes
- Focus management
- Screen reader support

---

## Accordion

An accordion component for collapsible content sections.

### Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `type` | `AccordionType` | `Single` | Accordion type |
| `collapsible` | `bool` | `false` | Whether items can be collapsed |
| `class` | `String` | - | Additional CSS classes |
| `children` | `Children` | - | Accordion content |

### Types

- `Single` - Only one item can be open at a time
- `Multiple` - Multiple items can be open simultaneously

### Sub-components

- `AccordionItem` - Individual accordion item
- `AccordionTrigger` - Clickable trigger for item
- `AccordionContent` - Collapsible content

### Example

```rust
use leptos::*;
use radix_leptos_primitives::*;

#[component]
fn MyAccordion() -> impl IntoView {
    view! {
        <Accordion type=AccordionType::Single collapsible=true>
            <AccordionItem value="item1">
                <AccordionTrigger>"Item 1"</AccordionTrigger>
                <AccordionContent>
                    "Content for item 1"
                </AccordionContent>
            </AccordionItem>
            <AccordionItem value="item2">
                <AccordionTrigger>"Item 2"</AccordionTrigger>
                <AccordionContent>
                    "Content for item 2"
                </AccordionContent>
            </AccordionItem>
        </Accordion>
    }
}
```

### Accessibility

- Full keyboard navigation
- Proper ARIA attributes
- Screen reader support
- Focus management

---

## Additional Components

### Avatar
- User profile image component with fallback
- Supports different sizes and shapes
- Proper alt text and accessibility

### Progress
- Progress bar component
- Supports different variants and sizes
- Accessible progress announcements

### Switch
- Toggle switch component
- Proper ARIA attributes
- Keyboard navigation support

### Slider
- Range slider component
- Supports single and range values
- Full keyboard navigation

### Tooltip
- Tooltip component for additional information
- Proper positioning and timing
- Screen reader support

### Popover
- Popover component for contextual content
- Proper focus management
- Keyboard navigation

### DropdownMenu
- Dropdown menu component
- Full keyboard navigation
- Proper ARIA attributes

### Sheet
- Side panel component
- Proper focus management
- Keyboard navigation

### Table
- Data table component
- Proper semantic structure
- Screen reader support

### Skeleton
- Loading skeleton component
- Proper accessibility attributes
- Screen reader announcements

---

## Testing

All components are thoroughly tested using TDD principles:

### Unit Tests
- Component rendering
- Props validation
- State management
- Event handling
- Edge cases

### Accessibility Tests
- WCAG 2.1 AA compliance
- Keyboard navigation
- Screen reader compatibility
- Focus management
- ARIA attributes

### Performance Tests
- Bundle size optimization
- Build time optimization
- Runtime performance
- Memory efficiency

### Integration Tests
- Component interactions
- Complex user workflows
- Cross-component compatibility

---

## Best Practices

### Accessibility
- Always provide proper labels
- Use semantic HTML elements
- Ensure keyboard navigation
- Test with screen readers
- Maintain color contrast

### Performance
- Use lazy loading for heavy components
- Optimize bundle size
- Minimize re-renders
- Use proper memoization

### Development
- Follow TDD principles
- Write comprehensive tests
- Document all props and usage
- Provide interactive examples
- Maintain backward compatibility

---

## Contributing

When adding new components:

1. Write failing tests first (RED)
2. Implement minimal functionality (GREEN)
3. Refactor and improve (REFACTOR)
4. Add comprehensive documentation
5. Ensure accessibility compliance
6. Optimize for performance
7. Add interactive examples

For more information, see the [Contributing Guide](../developer-guide/CONTRIBUTING.md).
