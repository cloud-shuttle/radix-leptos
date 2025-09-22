# Architecture Refactor Plan

**Priority**: HIGH  
**Timeline**: Weeks 2-4  
**Goal**: Break down large files and establish proper module structure

## File Size Violations Analysis

### Files >800 Lines (Critical)
1. **layout_system.rs** (1,150 lines) → Split into 4 modules
2. **pagination.rs** (932 lines) → Split into 3 modules  
3. **prebuilt_themes.rs** (895 lines) → Split into 5 modules
4. **component_variants.rs** (894 lines) → Split into 6 modules
5. **form_validation.rs** (884 lines) → Split into 4 modules
6. **theme_customization.rs** (866 lines) → Split into 3 modules

## Week 2: Layout System Refactor

### Current: layout_system.rs (1,150 lines)
### Target: 4 modules (<300 lines each)

```
theming/layout_system/
├── mod.rs              # Re-exports and main types (150 lines)
├── container.rs        # Container and grid components (280 lines)  
├── spacing.rs          # Spacing utilities and components (250 lines)
└── responsive.rs       # Responsive helpers and breakpoints (270 lines)
```

### Refactor Steps:
1. **Extract container logic** → `container.rs`
2. **Extract spacing utilities** → `spacing.rs` 
3. **Extract responsive helpers** → `responsive.rs`
4. **Keep main types** in `mod.rs`

## Week 2: Pagination Refactor

### Current: pagination.rs (932 lines)
### Target: 3 modules

```
components/pagination/
├── mod.rs              # Main Pagination component (200 lines)
├── context.rs          # PaginationProvider and context (180 lines)
├── items.rs            # PaginationItem, Button, etc. (250 lines)
└── helpers.rs          # Utility functions (150 lines)
```

### Migration Priority:
1. **Move helpers first** (safest, no dependencies)
2. **Extract context** (used by main component) 
3. **Extract items** (depends on context)
4. **Update main component** to use new modules

## Week 3: Theme System Refactor

### Prebuilt Themes Split (895 lines → 5 modules)

```
theming/prebuilt/
├── mod.rs              # Theme registry and main API (120 lines)
├── light_themes.rs     # Light theme variants (180 lines)
├── dark_themes.rs      # Dark theme variants (180 lines) 
├── color_schemes.rs    # Color palette definitions (200 lines)
└── theme_builder.rs    # Theme construction utilities (215 lines)
```

### Component Variants Split (894 lines → 6 modules)

```
theming/variants/
├── mod.rs              # Variant system core (100 lines)
├── button_variants.rs  # Button styling variants (150 lines)
├── input_variants.rs   # Input/form variants (150 lines)
├── layout_variants.rs  # Layout component variants (150 lines)
├── feedback_variants.rs # Alert/toast variants (150 lines)
└── data_variants.rs    # Table/list variants (194 lines)
```

## Week 4: Form System Refactor

### Form Validation Split (884 lines → 4 modules)

```
components/form/
├── mod.rs              # Main form component (200 lines)
├── validation.rs       # Validation rules and logic (220 lines)
├── fields.rs           # Form field components (240 lines)  
└── controls.rs         # Form control components (224 lines)
```

### Theme Customization Split (866 lines → 3 modules)

```
theming/customization/
├── mod.rs              # Main customization API (200 lines)
├── css_editor.rs       # CSS variable editing (330 lines)
└── theme_export.rs     # Theme export/import (336 lines)
```

## Migration Strategy

### Phase 1: Extract Pure Functions
- Move helper functions and constants first
- These have no dependencies, safest to move
- Create comprehensive tests for moved functions

### Phase 2: Extract Types and Contexts  
- Move type definitions and context providers
- Update imports in dependent files
- Verify no breaking changes to public API

### Phase 3: Extract Components
- Move component definitions to new modules
- Update parent modules to re-export
- Maintain backward compatibility

### Phase 4: Update Tests
- Move tests to matching module files
- Create integration tests for module boundaries
- Verify all functionality still works

## File Structure Template

Each split module should follow this pattern:

```rust
// mod.rs - Main re-exports
pub use container::*;
pub use spacing::*;
pub use responsive::*;

pub mod container;
pub mod spacing;  
pub mod responsive;

// Individual modules should be <300 lines with:
// - Clear module documentation
// - Focused responsibility  
// - Comprehensive unit tests
// - Clear public API
```

## Testing Strategy

### Before Refactor
- [ ] Full test suite passes
- [ ] Component behavior documented
- [ ] API contracts defined

### During Refactor
- [ ] Move tests with code
- [ ] Verify imports still work
- [ ] Check public API unchanged

### After Refactor  
- [ ] All tests still pass
- [ ] No performance regression
- [ ] Documentation updated
- [ ] Examples still work

## Success Criteria

- [ ] No files >300 lines in main codebase
- [ ] Logical module organization with clear boundaries
- [ ] No breaking changes to public API
- [ ] All tests pass with new structure
- [ ] Build times improved due to better caching
- [ ] Easier navigation and understanding for new developers

## Rollback Plan

- Each refactor commits separately
- Keep backup branches for major modules
- Ability to revert individual module splits
- Maintain compatibility layers during transition

## Long-term Benefits

- **Maintainability**: Easier to understand and modify small, focused files
- **Testing**: Easier to achieve comprehensive test coverage
- **LLM-friendly**: Better context windows for AI assistance
- **Collaboration**: Reduced merge conflicts with smaller files
- **Performance**: Better incremental compilation caching
