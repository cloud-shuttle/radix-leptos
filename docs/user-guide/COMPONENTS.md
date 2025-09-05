# Radix-Leptos Components Documentation

## Table of Contents
1. [Button Component](#button-component)
2. [Label Component](#label-component)
3. [Separator Component](#separator-component)
4. [Dialog Component](#dialog-component)
5. [Checkbox Component](#checkbox-component)
6. [Switch Component](#switch-component)
7. [RadioGroup Component](#radiogroup-component)
8. [TextInput Component](#textinput-component)
9. [General Styling](#general-styling)
10. [Testing](#testing)
11. [Additional Resources](#additional-resources)

---

## Button Component

### API Reference

```rust
#[component]
pub fn Button(
    #[prop(optional, default = ButtonVariant::Default)]
    variant: ButtonVariant,
    #[prop(optional, default = false)]
    disabled: bool,
    #[prop(optional)]
    class: Option<String>,
    #[prop(optional)]
    style: Option<String>,
    #[prop(optional)]
    on_click: Option<Callback<web_sys::MouseEvent>>,
    children: Children,
) -> impl IntoView
```

### Variants
- `ButtonVariant::Default` - Primary button style
- `ButtonVariant::Secondary` - Secondary button style
- `ButtonVariant::Outline` - Outlined button style
- `ButtonVariant::Ghost` - Ghost button style

### Usage Examples

```rust
// Basic button
<Button on_click=handle_click>
    "Click me"
</Button>

// Button with variant
<Button variant=ButtonVariant::Secondary on_click=handle_click>
    "Secondary Button"
</Button>

// Disabled button
<Button disabled=true>
    "Disabled Button"
</Button>

// Button with custom styling
<Button 
    class="custom-button".to_string()
    style="background-color: red;".to_string()
    on_click=handle_click
>
    "Custom Button"
</Button>
```

---

## Label Component

### API Reference

```rust
#[component]
pub fn Label(
    #[prop(optional)]
    for_control: Option<String>,
    #[prop(optional)]
    class: Option<String>,
    #[prop(optional)]
    style: Option<String>,
    children: Children,
) -> impl IntoView
```

### Usage Examples

```rust
// Basic label
<Label>
    "Form Label"
</Label>

// Label with form control association
<Label for_control="input-id".to_string()>
    "Input Label"
</Label>

// Label with custom styling
<Label 
    class="custom-label".to_string()
    style="color: blue;".to_string()
>
    "Styled Label"
</Label>
```

---

## Separator Component

### API Reference

```rust
#[component]
pub fn Separator(
    #[prop(optional, default = SeparatorOrientation::Horizontal)]
    orientation: SeparatorOrientation,
    #[prop(optional, default = SeparatorVariant::Solid)]
    variant: SeparatorVariant,
    #[prop(optional)]
    class: Option<String>,
    #[prop(optional)]
    style: Option<String>,
    children: Children,
) -> impl IntoView
```

### Orientations
- `SeparatorOrientation::Horizontal` - Horizontal separator
- `SeparatorOrientation::Vertical` - Vertical separator

### Variants
- `SeparatorVariant::Solid` - Solid line
- `SeparatorVariant::Dashed` - Dashed line
- `SeparatorVariant::Dotted` - Dotted line

### Usage Examples

```rust
// Horizontal separator
<Separator orientation=SeparatorOrientation::Horizontal>
    "Section divider"
</Separator>

// Vertical separator
<Separator orientation=SeparatorOrientation::Vertical>
    "Vertical divider"
</Separator>

// Custom styled separator
<Separator 
    orientation=SeparatorOrientation::Horizontal
    variant=SeparatorVariant::Dashed
    class="custom-separator".to_string()
>
    "Custom Separator"
</Separator>
```

---

## Dialog Component

### API Reference

```rust
#[component]
pub fn DialogRoot(
    #[prop(optional, default = false)]
    default_open: bool,
    #[prop(optional)]
    on_open_change: Option<Callback<bool>>,
    children: Children,
) -> impl IntoView

#[component]
pub fn DialogTrigger(
    #[prop(optional)]
    class: Option<String>,
    #[prop(optional)]
    style: Option<String>,
    children: Children,
) -> impl IntoView

#[component]
pub fn DialogContent(
    #[prop(optional)]
    class: Option<String>,
    #[prop(optional)]
    style: Option<String>,
    children: Children,
) -> impl IntoView

#[component]
pub fn DialogTitle(
    #[prop(optional)]
    class: Option<String>,
    #[prop(optional)]
    style: Option<String>,
    children: Children,
) -> impl IntoView

#[component]
pub fn DialogDescription(
    #[prop(optional)]
    class: Option<String>,
    #[prop(optional)]
    style: Option<String>,
    children: Children,
) -> impl IntoView

#[component]
pub fn DialogClose(
    #[prop(optional)]
    class: Option<String>,
    #[prop(optional)]
    style: Option<String>,
    children: Children,
) -> impl IntoView
```

### Usage Examples

```rust
<DialogRoot default_open=dialog_open.get() on_open_change=handle_dialog_change>
    <DialogTrigger>
        <Button>"Open Dialog"</Button>
    </DialogTrigger>
    <DialogContent>
        <DialogTitle>"Dialog Title"</DialogTitle>
        <DialogDescription>
            "This is a dialog description."
        </DialogDescription>
        <div>
            <p>"Dialog content goes here"</p>
            <DialogClose>
                <Button>"Close"</Button>
            </DialogClose>
        </div>
    </DialogContent>
</DialogRoot>
```

---

## Checkbox Component

### API Reference

```rust
#[component]
pub fn Checkbox(
    #[prop(optional, default = CheckboxState::Unchecked)]
    state: CheckboxState,
    #[prop(optional, default = false)]
    disabled: bool,
    #[prop(optional, default = false)]
    required: bool,
    #[prop(optional)]
    name: Option<String>,
    #[prop(optional)]
    value: Option<String>,
    #[prop(optional)]
    class: Option<String>,
    #[prop(optional)]
    style: Option<String>,
    #[prop(optional)]
    on_change: Option<Callback<CheckboxState>>,
    #[prop(optional)]
    on_click: Option<Callback<web_sys::MouseEvent>>,
    children: Children,
) -> impl IntoView

#[component]
pub fn CheckboxWithLabel(
    #[prop(optional, default = CheckboxState::Unchecked)]
    state: CheckboxState,
    #[prop(optional, default = false)]
    disabled: bool,
    #[prop(optional, default = false)]
    required: bool,
    #[prop(optional)]
    name: Option<String>,
    #[prop(optional)]
    value: Option<String>,
    #[prop(optional)]
    class: Option<String>,
    #[prop(optional)]
    style: Option<String>,
    children: Children,
) -> impl IntoView
```

### States
- `CheckboxState::Unchecked` - Unchecked state
- `CheckboxState::Checked` - Checked state
- `CheckboxState::Indeterminate` - Indeterminate state

### Usage Examples

```rust
// Basic checkbox
<Checkbox 
    state=checkbox_state.get() 
    on_change=handle_checkbox_change
>
    "Accept terms"
</Checkbox>

// Checkbox with label
<CheckboxWithLabel 
    state=CheckboxState::Checked
    required=true
>
    "I agree to the terms and conditions"
</CheckboxWithLabel>

// Disabled checkbox
<CheckboxWithLabel 
    state=CheckboxState::Unchecked
    disabled=true
>
    "Disabled option"
</CheckboxWithLabel>
```

---

## Switch Component

### API Reference

```rust
#[component]
pub fn Switch(
    #[prop(optional, default = SwitchState::Off)]
    state: SwitchState,
    #[prop(optional, default = false)]
    disabled: bool,
    #[prop(optional, default = false)]
    required: bool,
    #[prop(optional)]
    name: Option<String>,
    #[prop(optional)]
    value: Option<String>,
    #[prop(optional)]
    class: Option<String>,
    #[prop(optional)]
    style: Option<String>,
    #[prop(optional)]
    on_change: Option<Callback<SwitchState>>,
    #[prop(optional)]
    on_click: Option<Callback<web_sys::MouseEvent>>,
    children: Children,
) -> impl IntoView

#[component]
pub fn SwitchWithLabel(
    #[prop(optional, default = SwitchState::Off)]
    state: SwitchState,
    #[prop(optional, default = false)]
    disabled: bool,
    #[prop(optional, default = false)]
    required: bool,
    #[prop(optional)]
    name: Option<String>,
    #[prop(optional)]
    value: Option<String>,
    #[prop(optional)]
    class: Option<String>,
    #[prop(optional)]
    style: Option<String>,
    children: Children,
) -> impl IntoView
```

### States
- `SwitchState::Off` - Switch is off
- `SwitchState::On` - Switch is on

### Usage Examples

```rust
// Basic switch
<Switch 
    state=switch_state.get() 
    on_change=handle_switch_change
>
    "Enable notifications"
</Switch>

// Switch with label
<SwitchWithLabel 
    state=SwitchState::On
>
    "Dark mode"
</SwitchWithLabel>

// Disabled switch
<SwitchWithLabel 
    state=SwitchState::Off
    disabled=true
>
    "Premium feature"
</SwitchWithLabel>
```

### Accessibility Features
- **ARIA Attributes**: `role="switch"`, `aria-checked`, `aria-required`, `aria-disabled`
- **Keyboard Navigation**: Space and Enter keys toggle the switch
- **Form Integration**: Hidden checkbox input for form submission
- **Visual Indicators**: SVG icons for on/off states

---

## RadioGroup Component

### API Reference

```rust
#[component]
pub fn RadioGroup(
    #[prop(optional)]
    value: Option<String>,
    #[prop(optional, default = false)]
    disabled: bool,
    #[prop(optional, default = false)]
    required: bool,
    #[prop(optional)]
    name: Option<String>,
    #[prop(optional)]
    class: Option<String>,
    #[prop(optional)]
    style: Option<String>,
    #[prop(optional)]
    on_change: Option<Callback<String>>,
    children: Children,
) -> impl IntoView

#[component]
pub fn RadioGroupItem(
    value: String,
    #[prop(optional, default = false)]
    disabled: bool,
    #[prop(optional, default = false)]
    required: bool,
    #[prop(optional)]
    class: Option<String>,
    #[prop(optional)]
    style: Option<String>,
    #[prop(optional)]
    on_change: Option<Callback<String>>,
    children: Children,
) -> impl IntoView

#[component]
pub fn RadioGroupItemWithLabel(
    value: String,
    #[prop(optional, default = false)]
    disabled: bool,
    #[prop(optional, default = false)]
    required: bool,
    #[prop(optional)]
    class: Option<String>,
    #[prop(optional)]
    style: Option<String>,
    children: Children,
) -> impl IntoView
```

### Usage Examples

```rust
// Basic radio group
<RadioGroup 
    value=selected_value.get() 
    on_change=handle_radio_change
>
    <RadioGroupItemWithLabel value="option1".to_string()>
        "Option 1"
    </RadioGroupItemWithLabel>
    <RadioGroupItemWithLabel value="option2".to_string()>
        "Option 2"
    </RadioGroupItemWithLabel>
    <RadioGroupItemWithLabel value="option3".to_string()>
        "Option 3"
    </RadioGroupItemWithLabel>
</RadioGroup>

// Radio group with disabled option
<RadioGroup value=selected_value.get() on_change=handle_radio_change>
    <RadioGroupItemWithLabel value="enabled".to_string()>
        "Enabled Option"
    </RadioGroupItemWithLabel>
    <RadioGroupItemWithLabel value="disabled".to_string() disabled=true>
        "Disabled Option"
    </RadioGroupItemWithLabel>
</RadioGroup>
```

### Accessibility Features
- **ARIA Attributes**: `role="radiogroup"`, `aria-required`, `aria-disabled`
- **Individual Items**: Each item has `role="radio"`, `aria-checked`, `aria-required`, `aria-disabled`
- **Keyboard Navigation**: Arrow keys navigate between options, Space/Enter selects
- **Form Integration**: Hidden radio inputs for form submission

---

## TextInput Component

### API Reference

```rust
#[component]
pub fn TextInput(
    #[prop(optional, default = TextInputType::Text)]
    input_type: TextInputType,
    #[prop(optional)]
    value: Option<String>,
    #[prop(optional)]
    placeholder: Option<String>,
    #[prop(optional, default = false)]
    disabled: bool,
    #[prop(optional, default = false)]
    required: bool,
    #[prop(optional, default = false)]
    readonly: bool,
    #[prop(optional)]
    min_length: Option<usize>,
    #[prop(optional)]
    max_length: Option<usize>,
    #[prop(optional)]
    pattern: Option<String>,
    #[prop(optional)]
    name: Option<String>,
    #[prop(optional)]
    id: Option<String>,
    #[prop(optional)]
    class: Option<String>,
    #[prop(optional)]
    style: Option<String>,
) -> impl IntoView

#[component]
pub fn TextInputWithLabel(
    #[prop(optional, default = TextInputType::Text)]
    input_type: TextInputType,
    #[prop(optional)]
    value: Option<String>,
    #[prop(optional)]
    placeholder: Option<String>,
    #[prop(optional, default = false)]
    disabled: bool,
    #[prop(optional, default = false)]
    required: bool,
    #[prop(optional, default = false)]
    readonly: bool,
    #[prop(optional)]
    name: Option<String>,
    #[prop(optional)]
    class: Option<String>,
    #[prop(optional)]
    style: Option<String>,
    children: Children,
) -> impl IntoView
```

### Input Types
- `TextInputType::Text` - Standard text input
- `TextInputType::Email` - Email input with validation
- `TextInputType::Password` - Password input (masked)
- `TextInputType::Number` - Numeric input
- `TextInputType::Tel` - Telephone number input
- `TextInputType::Url` - URL input
- `TextInputType::Search` - Search input
- `TextInputType::Date` - Date picker
- `TextInputType::Time` - Time picker
- `TextInputType::DateTimeLocal` - Date and time picker
- `TextInputType::Month` - Month picker
- `TextInputType::Week` - Week picker

### Usage Examples

```rust
// Basic text input
<TextInputWithLabel 
    input_type=TextInputType::Text
    placeholder="Enter your name...".to_string()
>
    "Full Name"
</TextInputWithLabel>

// Email input
<TextInputWithLabel 
    input_type=TextInputType::Email
    placeholder="Enter email address...".to_string()
    required=true
>
    "Email Address"
</TextInputWithLabel>

// Password input
<TextInputWithLabel 
    input_type=TextInputType::Password
    placeholder="Enter password...".to_string()
    min_length=8
>
    "Password"
</TextInputWithLabel>

// Number input with validation
<TextInputWithLabel 
    input_type=TextInputType::Number
    placeholder="Enter age...".to_string()
    min_length=1
    max_length=3
>
    "Age"
</TextInputWithLabel>

// Search input
<TextInputWithLabel 
    input_type=TextInputType::Search
    placeholder="Search...".to_string()
>
    "Search"
</TextInputWithLabel>

// Disabled input
<TextInputWithLabel 
    input_type=TextInputType::Text
    disabled=true
    placeholder="Disabled input...".to_string()
>
    "Read-only Field"
</TextInputWithLabel>
```

### Validation Features
- **HTML5 Validation**: Built-in browser validation for email, URL, number, etc.
- **Length Constraints**: `min_length` and `max_length` attributes
- **Pattern Matching**: Custom regex patterns with `pattern` attribute
- **Required Fields**: `required` attribute for form validation
- **Read-only Mode**: `readonly` attribute for non-editable fields

---

## General Styling

### CSS Classes
All components use consistent CSS class naming:

- **Button**: `.radix-button`
- **Label**: `.radix-label`
- **Separator**: `.radix-separator`
- **Dialog**: `.radix-dialog-*` (root, trigger, content, title, description, close)
- **Checkbox**: `.radix-checkbox`, `.radix-checkbox-indicator`
- **Switch**: `.radix-switch`, `.radix-switch-track`, `.radix-switch-thumb`
- **RadioGroup**: `.radix-radio-group`, `.radix-radio-group-item`, `.radix-radio-group-indicator`
- **TextInput**: `.radix-text-input`

### Data Attributes
Components use data attributes for state management:

- **Checkbox**: `data-state="checked|unchecked|indeterminate"`
- **Switch**: `data-state="on|off"`
- **RadioGroup**: `data-value` (on group), `data-value` (on items)
- **TextInput**: Standard HTML input attributes

### Custom Styling
You can override default styles by providing custom CSS classes:

```css
.radix-button {
    background-color: #007bff;
    color: white;
    border: none;
    padding: 8px 16px;
    border-radius: 4px;
    cursor: pointer;
}

.radix-button:hover {
    background-color: #0056b3;
}

.radix-button:disabled {
    background-color: #6c757d;
    cursor: not-allowed;
}
```

---

## Testing

### Component Test Suite
A comprehensive test suite is available at `examples/src/test_components.rs` that demonstrates:

- All component variants and states
- Interactive functionality
- Real-time state updates
- Accessibility features
- Form integration

### Running Tests
```bash
# Build the test suite
cargo build --package radix-leptos-examples --target wasm32-unknown-unknown

# Generate JavaScript bindings
wasm-bindgen target/wasm32-unknown-unknown/debug/radix_leptos_examples.wasm --out-dir pkg --target web

# Start local server
python3 -m http.server 8000

# Open http://localhost:8000 in your browser
```

### Test Coverage
The test suite covers:
- ✅ Button variants and click handling
- ✅ Label form association
- ✅ Separator orientations and variants
- ✅ Dialog modal behavior and focus management
- ✅ Checkbox state management and accessibility
- ✅ Switch toggle functionality
- ✅ RadioGroup selection and keyboard navigation
- ✅ TextInput types and validation

---

## Additional Resources

### Dependencies
- **Leptos**: `leptos = "0.6"`
- **Web-sys**: `web-sys = "0.3"`
- **Wasm-bindgen**: `wasm-bindgen = "0.2"`

### Browser Support
- Modern browsers with WebAssembly support
- ES6+ JavaScript features
- CSS Grid and Flexbox

### Performance Considerations
- Components are lightweight and optimized for WASM
- Minimal JavaScript overhead
- Efficient reactive updates
- Tree-shaking friendly

### Accessibility Compliance
- WCAG 2.1 AA compliance
- Proper ARIA attributes
- Keyboard navigation support
- Screen reader compatibility
- Focus management

### Future Enhancements
- Additional input types (file, range, color)
- Advanced validation patterns
- Custom styling themes
- Animation support
- Internationalization (i18n)
