#!/bin/bash

# 🧪 TDD Workflow Script for Radix-Leptos
# This script helps developers follow the Test-Driven Development process

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Configuration
BASE_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TEMPLATE_FILE="$BASE_DIR/docs/TDD_TEMPLATE.md"

# Functions
print_header() {
    echo -e "\n${BLUE}================================${NC}"
    echo -e "${BLUE}  🧪 TDD Workflow Helper${NC}"
    echo -e "${BLUE}================================${NC}"
}

print_section() {
    echo -e "\n${CYAN}📋 $1${NC}"
    echo -e "${CYAN}${2//?/=}${NC}"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

print_info() {
    echo -e "${PURPLE}ℹ️  $1${NC}"
}

show_help() {
    print_header
    echo -e "${CYAN}Usage: $0 [COMMAND] [OPTIONS]${NC}"
    echo ""
    echo -e "${YELLOW}Commands:${NC}"
    echo "  new-component <name>    Start TDD for new component"
    echo "  red <test-name>         Write failing test (RED phase)"
    echo "  green                   Make tests pass (GREEN phase)"
    echo "  refactor                Refactor code (REFACTOR phase)"
    echo "  check                   Check TDD compliance"
    echo "  watch                   Run tests in watch mode"
    echo "  coverage                Generate coverage report"
    echo "  mutants                 Run mutation testing"
    echo "  help                    Show this help message"
    echo ""
    echo -e "${YELLOW}Examples:${NC}"
    echo "  $0 new-component checkbox"
    echo "  $0 red test_checkbox_renders"
    echo "  $0 green"
    echo "  $0 refactor"
    echo "  $0 check"
    echo ""
    echo -e "${YELLOW}TDD Process:${NC}"
    echo "  1. RED: Write failing test"
    echo "  2. GREEN: Make test pass with minimal code"
    echo "  3. REFACTOR: Improve code while keeping tests green"
    echo "  4. REPEAT: Continue cycle for each feature"
}

create_component_structure() {
    local component_name=$1
    local component_file="$BASE_DIR/crates/radix-leptos-primitives/src/components/${component_name}.rs"
    local mod_file="$BASE_DIR/crates/radix-leptos-primitives/src/components/mod.rs"
    
    print_section "Creating Component Structure" "Creating $component_name component"
    
    # Check if component already exists
    if [[ -f "$component_file" ]]; then
        print_error "Component $component_name already exists!"
        exit 1
    fi
    
    # Create component file with basic structure
    cat > "$component_file" << EOF
use leptos::*;
use leptos::prelude::*;

/// ${component_name^} component with proper accessibility and styling variants
///
/// The ${component_name^} component provides accessible functionality with
/// proper ARIA attributes, keyboard navigation, and flexible styling.
///
/// # Features
/// - Proper semantics and accessibility
/// - Multiple variants and sizes
/// - State management
/// - Event handling
///
/// # Example
///
/// \`\`\`rust
/// use leptos::*;
/// use radix_leptos_primitives::*;
///
/// #[component]
/// fn MyComponent() -> impl IntoView {
///     view! {
///         <${component_name^}>
///             "Content"
///         </${component_name^}>
///     }
/// }
/// \`\`\`

// TODO: Add component implementation following TDD process

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    use proptest::prelude::*;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    // TODO: Add tests following TDD template
    // 1. Basic Rendering Tests
    // 2. Props Validation Tests  
    // 3. State Management Tests
    // 4. Event Handling Tests
    // 5. Accessibility Tests
    // 6. Edge Case Tests
    // 7. Property-Based Tests
    
    // Helper function for running tests
    fn run_test<F>(f: F) where F: FnOnce(Scope) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let _ = create_runtime();
            run_scope(create_runtime(), f);
        });
    }
}
EOF
    
    # Add to mod.rs
    echo "pub mod ${component_name};" >> "$mod_file"
    echo "pub use ${component_name}::*;" >> "$mod_file"
    
    print_success "Component structure created: $component_file"
    print_info "Next steps:"
    echo "  1. Write your first failing test"
    echo "  2. Run: $0 red test_${component_name}_renders"
    echo "  3. Follow the TDD cycle"
    echo "  4. Reference: $TEMPLATE_FILE"
}

run_red_phase() {
    local test_name=$1
    
    print_section "RED Phase" "Writing Failing Test"
    
    if [[ -z "$test_name" ]]; then
        print_error "Test name is required for RED phase"
        echo "Usage: $0 red <test-name>"
        exit 1
    fi
    
    print_info "Writing failing test: $test_name"
    print_warning "This test should FAIL initially - that's expected!"
    
    # Run the specific test to show it fails
    if cargo test --workspace --lib "$test_name" 2>/dev/null; then
        print_warning "Test passed! Make sure it's actually failing first."
    else
        print_success "Test failed as expected (RED phase)"
    fi
    
    print_info "Next: Implement minimal code to make test pass"
    print_info "Run: $0 green"
}

run_green_phase() {
    print_section "GREEN Phase" "Making Tests Pass"
    
    print_info "Implementing minimal code to make tests pass..."
    
    # Run tests to see current status
    if cargo test --workspace --lib; then
        print_success "All tests are passing! (GREEN phase complete)"
        print_info "Next: Refactor code while keeping tests green"
        print_info "Run: $0 refactor"
    else
        print_warning "Some tests are still failing"
        print_info "Continue implementing minimal code to make tests pass"
    fi
}

run_refactor_phase() {
    print_section "REFACTOR Phase" "Improving Code"
    
    print_info "Refactoring code while keeping tests green..."
    
    # Run tests to ensure they still pass
    if cargo test --workspace --lib; then
        print_success "All tests still passing after refactoring!"
        print_info "Refactoring complete. Ready for next TDD cycle."
    else
        print_error "Tests failed after refactoring!"
        print_info "Fix the issues and ensure tests pass before continuing"
    fi
}

check_tdd_compliance() {
    print_section "TDD Compliance Check" "Verifying TDD Practices"
    
    # Check for placeholder assertions
    print_info "Checking for placeholder assertions..."
    if grep -r "assert.*placeholder\|TODO.*assert\|FIXME.*assert" "$BASE_DIR/crates/" 2>/dev/null; then
        print_warning "Found placeholder assertions - these should be replaced with real tests"
    else
        print_success "No placeholder assertions found"
    fi
    
    # Check for TODO items in test code
    print_info "Checking for TODO items in test code..."
    if grep -r "TODO\|FIXME" "$BASE_DIR/crates/" --include="*.rs" | grep -i test; then
        print_warning "Found TODO/FIXME items in test code"
    else
        print_success "No TODO/FIXME items in test code"
    fi
    
    # Run tests
    print_info "Running tests..."
    if cargo test --workspace --lib; then
        print_success "All tests passing"
    else
        print_error "Some tests failing"
    fi
    
    # Check test coverage (if tarpaulin is available)
    if command -v cargo-tarpaulin &> /dev/null; then
        print_info "Generating test coverage..."
        cargo tarpaulin --all-features --workspace --out Html --timeout 120 > /dev/null 2>&1
        print_success "Coverage report generated in tarpaulin-report.html"
    else
        print_warning "cargo-tarpaulin not installed - skipping coverage check"
    fi
}

run_watch_mode() {
    print_section "Watch Mode" "Running Tests in Watch Mode"
    
    print_info "Starting test watch mode..."
    print_info "Tests will re-run automatically when files change"
    print_info "Press Ctrl+C to stop"
    
    cargo watch -x "test --workspace --lib"
}

generate_coverage() {
    print_section "Coverage Report" "Generating Test Coverage"
    
    if command -v cargo-tarpaulin &> /dev/null; then
        cargo tarpaulin --all-features --workspace --out Html --timeout 120
        print_success "Coverage report generated in tarpaulin-report.html"
    else
        print_error "cargo-tarpaulin not installed"
        print_info "Install with: cargo install cargo-tarpaulin"
    fi
}

run_mutation_testing() {
    print_section "Mutation Testing" "Running Mutation Tests"
    
    if command -v cargo-mutants &> /dev/null; then
        cargo mutants
        print_success "Mutation testing complete"
    else
        print_error "cargo-mutants not installed"
        print_info "Install with: cargo install cargo-mutants"
    fi
}

# Main function
main() {
    case "${1:-help}" in
        "new-component")
            if [[ -z "$2" ]]; then
                print_error "Component name is required"
                echo "Usage: $0 new-component <name>"
                exit 1
            fi
            create_component_structure "$2"
            ;;
        "red")
            run_red_phase "$2"
            ;;
        "green")
            run_green_phase
            ;;
        "refactor")
            run_refactor_phase
            ;;
        "check")
            check_tdd_compliance
            ;;
        "watch")
            run_watch_mode
            ;;
        "coverage")
            generate_coverage
            ;;
        "mutants")
            run_mutation_testing
            ;;
        "help"|*)
            show_help
            ;;
    esac
}

# Run main function
main "$@"
