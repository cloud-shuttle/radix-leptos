---
name: Bug report
about: Create a report to help us improve
title: '[BUG] '
labels: ['bug']
assignees: ''

---

**Describe the bug**
A clear and concise description of what the bug is.

**To Reproduce**
Steps to reproduce the behavior:
1. Go to '...'
2. Click on '....'
3. Scroll down to '....'
4. See error

**Expected behavior**
A clear and concise description of what you expected to happen.

**Screenshots**
If applicable, add screenshots to help explain your problem.

**Environment (please complete the following information):**
 - OS: [e.g. macOS, Windows, Linux]
 - Browser: [e.g. Chrome, Firefox, Safari]
 - Rust version: [e.g. 1.70.0]
 - Leptos version: [e.g. 0.8.0]
 - Radix-Leptos version: [e.g. 0.1.0]

**Additional context**
Add any other context about the problem here.

**Code Example**
```rust
use leptos::*;
use radix_leptos_primitives::*;

#[component]
pub fn MyComponent() -> impl IntoView {
    view! {
        <Button on_click=Callback::new(|_| println!("Clicked!"))>
            "Click me!"
        </Button>
    }
}
```
