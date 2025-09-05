#!/bin/bash

# 🧪 Radix-Leptos Comprehensive Test Runner
# This script runs all test suites and generates comprehensive reports

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
BASE_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
EXAMPLES_DIR="$BASE_DIR/examples"
TESTS_DIR="$BASE_DIR/tests"
REPORTS_DIR="$BASE_DIR/test-reports"
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")

# Test suites
TEST_SUITES=(
    "all-components-fixed:Basic Component Tests"
    "interactive-components:Interactive Component Tests"
    "performance-accessibility:Performance & Accessibility Tests"
)

# Functions
print_header() {
    echo -e "\n${BLUE}================================${NC}"
    echo -e "${BLUE}  🧪 Radix-Leptos Test Runner${NC}"
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

check_prerequisites() {
    print_section "Checking Prerequisites"
    
    # Check if we're in the right directory
    if [[ ! -f "flake.nix" ]]; then
        print_error "flake.nix not found. Please run this script from the project root."
        exit 1
    fi
    
    # Check if examples directory exists
    if [[ ! -d "$EXAMPLES_DIR" ]]; then
        print_error "Examples directory not found: $EXAMPLES_DIR"
        exit 1
    fi
    
    # Check if tests directory exists
    if [[ ! -d "$TESTS_DIR" ]]; then
        print_error "Tests directory not found: $TESTS_DIR"
        exit 1
    fi
    
    # Check if pnpm is available
    if ! command -v pnpm &> /dev/null; then
        print_error "pnpm is not installed or not in PATH"
        exit 1
    fi
    
    # Check if Playwright is installed
    if [[ ! -d "$EXAMPLES_DIR/node_modules/.pnpm/playwright" ]]; then
        print_warning "Playwright not found. Installing..."
        cd "$EXAMPLES_DIR"
        pnpm install
        pnpm exec playwright install
        cd "$BASE_DIR"
    fi
    
    print_success "All prerequisites met"
}

check_server() {
    print_section "Checking Development Server"
    
    # Check if server is running
    if curl -s "http://localhost:8080" > /dev/null 2>&1; then
        print_success "Development server is running on port 8080"
    else
        print_warning "Development server not running. Starting..."
        cd "$EXAMPLES_DIR"
        python3 -m http.server 8080 &
        SERVER_PID=$!
        cd "$BASE_DIR"
        
        # Wait for server to start
        sleep 3
        
        if curl -s "http://localhost:8080" > /dev/null 2>&1; then
            print_success "Development server started successfully"
        else
            print_error "Failed to start development server"
            exit 1
        fi
    fi
}

run_test_suite() {
    local suite_file=$1
    local suite_name=$2
    local output_file="$REPORTS_DIR/${suite_name// /_}_${TIMESTAMP}.txt"
    
    print_section "Running $suite_name"
    
    # Run the test suite from the root directory
    if pnpm exec playwright test "$suite_file" --reporter=list > "$output_file" 2>&1; then
        print_success "$suite_name completed successfully"
        return 0
    else
        print_error "$suite_name failed. Check logs: $output_file"
        return 1
    fi
}

generate_summary_report() {
    print_section "Generating Summary Report"
    
    local summary_file="$REPORTS_DIR/test_summary_${TIMESTAMP}.md"
    
    cat > "$summary_file" << EOF
# 🧪 Radix-Leptos Test Summary Report

**Generated:** $(date)
**Timestamp:** $TIMESTAMP

## Test Results Overview

EOF
    
    # Count test results
    total_tests=0
    passed_tests=0
    failed_tests=0
    
    # Check if any report files exist
    local report_files_found=false
    
    for report_file in "$REPORTS_DIR"/*_${TIMESTAMP}.txt; do
        if [[ -f "$report_file" ]]; then
            report_files_found=true
            local suite_name=$(basename "$report_file" | sed 's/_'${TIMESTAMP}'.txt//' | sed 's/_/ /g')
            # Count tests by looking for the test result pattern: ✓ [number] [browser]
            local test_count=$(grep -c "✓  [0-9]" "$report_file" 2>/dev/null || echo "0")
            local passed=$(grep -c "✓" "$report_file" 2>/dev/null || echo "0")
            local failed=$(grep -c "✗" "$report_file" 2>/dev/null || echo "0")
            
            # Ensure variables are numbers
            test_count=${test_count:-0}
            passed=${passed:-0}
            failed=${failed:-0}
            
            echo "### $suite_name" >> "$summary_file"
            echo "- **Total Tests:** $test_count" >> "$summary_file"
            echo "- **Passed:** $passed" >> "$summary_file"
            echo "- **Failed:** $failed" >> "$summary_file"
            echo "" >> "$summary_file"
            
            # Use expr for safer arithmetic
            total_tests=$(expr $total_tests + $test_count 2>/dev/null || echo "0")
            passed_tests=$(expr $passed_tests + $passed 2>/dev/null || echo "0")
            failed_tests=$(expr $failed_tests + $failed 2>/dev/null || echo "0")
        fi
    done
    
    # If no report files found, add a note
    if [[ "$report_files_found" == false ]]; then
        echo "### No Test Reports Found" >> "$summary_file"
        echo "- **Total Tests:** 0" >> "$summary_file"
        echo "- **Passed:** 0" >> "$summary_file"
        echo "- **Failed:** 0" >> "$summary_file"
        echo "" >> "$summary_file"
    fi
    
    # Add summary
    cat >> "$summary_file" << EOF
## Overall Summary

- **Total Test Suites:** ${#TEST_SUITES[@]}
- **Total Tests:** $total_tests
- **Passed:** $passed_tests
- **Failed:** $failed_tests
- **Success Rate:** $([ $total_tests -gt 0 ] && echo $((passed_tests * 100 / total_tests)) || echo "N/A")%

## Next Steps

1. Review detailed logs in the test-reports directory
2. Fix any failing tests
3. Run specific test suites as needed:
   - \`make test-basic\` - Basic component tests
   - \`make test-interactive\` - Interactive component tests
   - \`make test-performance\` - Performance & accessibility tests
   - \`make test-comprehensive\` - All tests

## Files Generated

- Summary Report: \`$summary_file\`
- Detailed Logs: \`$REPORTS_DIR/*_${TIMESTAMP}.txt\`
EOF
    
    print_success "Summary report generated: $summary_file"
    print_info "Total tests: $total_tests, Passed: $passed_tests, Failed: $failed_tests"
}

cleanup() {
    print_section "Cleanup"
    
    # Kill background server if we started it
    if [[ -n "$SERVER_PID" ]]; then
        print_info "Stopping development server..."
        kill "$SERVER_PID" 2>/dev/null || true
    fi
    
    print_success "Cleanup completed"
}

main() {
    print_header
    
    # Create reports directory
    mkdir -p "$REPORTS_DIR"
    
    # Check prerequisites
    check_prerequisites
    
    # Check server
    check_server
    
    # Run test suites
    local overall_success=true
    
    for suite in "${TEST_SUITES[@]}"; do
        IFS=':' read -r suite_file suite_name <<< "$suite"
        
        if ! run_test_suite "tests/$suite_file.spec.ts" "$suite_name"; then
            overall_success=false
        fi
    done
    
    # Generate summary report
    generate_summary_report
    
    # Final status
    if [[ "$overall_success" == true ]]; then
        print_success "All test suites completed successfully!"
        echo -e "\n${GREEN}🎉 Testing completed successfully!${NC}"
        echo -e "${GREEN}📊 Check the summary report: $REPORTS_DIR/test_summary_${TIMESTAMP}.md${NC}"
    else
        print_warning "Some test suites failed. Check the logs for details."
        echo -e "\n${YELLOW}⚠️  Testing completed with some failures.${NC}"
        echo -e "${YELLOW}📊 Check the summary report: $REPORTS_DIR/test_summary_${TIMESTAMP}.md${NC}"
    fi
    
    # Cleanup
    cleanup
    
    echo -e "\n${BLUE}================================${NC}"
    echo -e "${BLUE}  🧪 Test Runner Complete${NC}"
    echo -e "${BLUE}================================${NC}"
}

# Trap cleanup on exit
trap cleanup EXIT

# Run main function
main "$@"
