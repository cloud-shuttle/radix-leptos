# Critical Fixes Remediation Plan

**Priority**: URGENT  
**Timeline**: Week 1 (5 business days)  
**Goal**: Make workspace build without warnings and fix compilation errors

## Day 1: Compilation Fixes

### Fix 1: Label Component Returns
**File**: `crates/radix-leptos-primitives/src/components/label.rs`
**Issue**: `LabelText` and `LabelError` components don't return views
**Solution**:
```rust
// LabelText component - add return view
view! {
    <span class={class} style={style.unwrap_or_default()}>
        {text}
        {if required { "*" } else { "" }}
        {children}
    </span>
}

// LabelError component - add return view  
view! {
    <div class={class} style={style.unwrap_or_default()} aria-live="polite">
        {if visible { error } else { "" }}
        {children}
    </div>
}
```

### Fix 2: Button Component Children
**File**: `crates/radix-leptos-primitives/src/components/button.rs`
**Issue**: Button doesn't render children
**Solution**: Add `{children()}` before closing `</button>` tag

## Day 2: Unused Variable Cleanup

### Target Files (in order):
1. `context_menu.rs` - 15 unused variables
2. `date_picker.rs` - 12 unused variables  
3. `file_upload.rs` - 10 unused variables
4. `multi_select.rs` - 18 unused variables
5. `password_toggle_field.rs` - 14 unused variables

### Pattern: Either Wire or Remove
For each unused variable, either:
- **Wire it**: Connect to actual DOM elements/handlers
- **Remove it**: Delete if not needed yet
- **Prefix with _**: If keeping for future use

## Day 3: Handler Wiring Priority

### Tabs Component
**File**: `crates/radix-leptos-primitives/src/components/tabs.rs`
**Fix**: Wire handlers to DOM elements
```rust
// In TabsTrigger component
view! {
    <button
        on:click=handle_click    // ADD THIS
        on:keydown=handle_keydown // ADD THIS
        role="tab"
        aria-selected={selected}
        // ... rest of props
    >
        {children}
    </button>
}
```

### RadioGroup Component  
**File**: `crates/radix-leptos-primitives/src/components/radio_group.rs`
**Fix**: Wire click handlers and state management

## Day 4: Utility Consolidation

### Merge Classes Utility
**Target**: Remove duplicates, use central implementation
**Files to fix**: 15+ files with local implementations
**Solution**: Update imports to use `radix_leptos_primitives::utils::merge_classes`

### Generate ID Utility
**Target**: Consolidate ID generation
**Files to fix**: 8+ files with local implementations
**Solution**: Central utility in `utils` module

## Day 5: Feature Gating

### Gate Incomplete Components
**File**: `crates/radix-leptos/src/lib.rs`
**Add feature flags**:
```rust
#[cfg(feature = "experimental")]
pub use radix_leptos_primitives::{
    calendar, date_picker, file_upload, 
    time_picker, tree_view, // ... other incomplete ones
};

// Always available (working components)
pub use radix_leptos_primitives::{
    button, checkbox, pagination,
    separator, skeleton, // ... minimal working set
};
```

### Update Cargo.toml
```toml
[features]
default = ["core"]
core = []
experimental = []
full = ["core", "experimental"]
```

## Success Criteria

- [ ] `cargo build --workspace` completes with 0 warnings
- [ ] `cargo clippy --workspace -- -D warnings` passes  
- [ ] All compilation tests pass with real component imports
- [ ] Button renders children correctly
- [ ] Label components return valid views
- [ ] Feature flags properly gate incomplete code

## Validation Commands

```bash
# Must pass with zero warnings
cargo build --workspace
cargo clippy --workspace --all-targets -- -D warnings

# Test core features only
cargo build --no-default-features --features core

# Test with experimental features  
cargo build --features experimental
```

## Risk Mitigation

- **Backup**: Create branch before changes
- **Incremental**: Test after each file fix
- **Rollback plan**: Keep commits small for easy reversion
- **Communication**: Update README about experimental features

## Expected Outcome

After this week:
- ✅ Clean builds with zero warnings
- ✅ Honest feature flagging of incomplete components  
- ✅ Core components (Button, Checkbox, Pagination) fully functional
- ✅ Foundation for proper development workflow
