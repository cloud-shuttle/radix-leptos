---
name: Feature request
about: Suggest an idea for this project
title: '[FEATURE] '
labels: ['enhancement']
assignees: ''

---

**Is your feature request related to a problem? Please describe.**
A clear and concise description of what the problem is. Ex. I'm always frustrated when [...]

**Describe the solution you'd like**
A clear and concise description of what you want to happen.

**Describe alternatives you've considered**
A clear and concise description of any alternative solutions or features you've considered.

**Additional context**
Add any other context or screenshots about the feature request here.

**Proposed API (if applicable)**
```rust
use leptos::*;
use radix_leptos_primitives::*;

#[component]
pub fn NewComponent(
    #[prop(optional)] prop1: Option<String>,
    #[prop(optional)] on_change: Option<Callback<String>>,
) -> impl IntoView {
    view! {
        <div>
            // Component implementation
        </div>
    }
}
```

**Priority**
- [ ] Low - Nice to have
- [ ] Medium - Important for workflow
- [ ] High - Critical for project success
