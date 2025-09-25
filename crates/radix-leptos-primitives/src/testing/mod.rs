//! Advanced testing framework for Radix-Leptos components
//! 
//! This module provides comprehensive testing utilities including:
//! - Property-based testing
//! - Mutation testing
//! - Performance benchmarking
//! - Accessibility testing
//! - Integration testing

pub mod property_based;
pub mod mutation_testing;
pub mod performance_benchmarking;

// Re-export main testing utilities
pub use property_based::*;
pub use mutation_testing::*;
pub use performance_benchmarking::*;

/// Comprehensive testing suite
pub struct ComprehensiveTestSuite {
    pub property_tests: bool,
    pub mutation_tests: bool,
    pub performance_benchmarks: bool,
    pub accessibility_tests: bool,
    pub integration_tests: bool,
}

impl Default for ComprehensiveTestSuite {
    fn default() -> Self {
        Self {
            property_tests: true,
            mutation_tests: true,
            performance_benchmarks: true,
            accessibility_tests: true,
            integration_tests: true,
        }
    }
}

impl ComprehensiveTestSuite {
    /// Run all tests
    pub fn run_all_tests(&self) -> TestSuiteResult {
        let mut results = TestSuiteResult::new();

        if self.property_tests {
            results.add_property_test_result("Button", ComponentPropertyTester::test_button_properties());
            results.add_property_test_result("Input", ComponentPropertyTester::test_input_properties());
            results.add_property_test_result("FormValidation", ComponentPropertyTester::test_form_validation_properties());
        }

        if self.mutation_tests {
            results.add_mutation_test_result("Button", ComponentMutationTester::test_button_mutations());
            results.add_mutation_test_result("Input", ComponentMutationTester::test_input_mutations());
            results.add_mutation_test_result("FormValidation", ComponentMutationTester::test_form_validation_mutations());
        }

        if self.performance_benchmarks {
            results.add_benchmark_result("Button", ComponentPerformanceBenchmarks::benchmark_button());
            results.add_benchmark_result("Input", ComponentPerformanceBenchmarks::benchmark_input());
            results.add_benchmark_result("Form", ComponentPerformanceBenchmarks::benchmark_form());
            results.add_benchmark_result("Table", ComponentPerformanceBenchmarks::benchmark_table());
        }

        if self.accessibility_tests {
            results.add_mutation_test_result("Accessibility", AccessibilityMutationTester::test_accessibility_mutations());
            results.add_mutation_test_result("KeyboardNavigation", AccessibilityMutationTester::test_keyboard_navigation_mutations());
        }

        if self.integration_tests {
            results.add_integration_test_result("ComponentIntegration", run_component_integration_tests());
            results.add_integration_test_result("StateIntegration", run_state_integration_tests());
        }

        results
    }
}

/// Test suite result
pub struct TestSuiteResult {
    pub property_test_results: Vec<PropertyTestResult>,
    pub mutation_test_results: Vec<MutationTestResult>,
    pub benchmark_results: Vec<BenchmarkResult>,
    pub integration_test_results: Vec<IntegrationTestResult>,
    pub overall_score: f64,
    pub execution_time: std::time::Duration,
}

impl TestSuiteResult {
    pub fn new() -> Self {
        Self {
            property_test_results: Vec::new(),
            mutation_test_results: Vec::new(),
            benchmark_results: Vec::new(),
            integration_test_results: Vec::new(),
            overall_score: 0.0,
            execution_time: std::time::Duration::ZERO,
        }
    }

    pub fn add_property_test_result(&mut self, name: &str, result: Result<(), String>) {
        self.property_test_results.push(PropertyTestResult {
            name: name.to_string(),
            passed: result.is_ok(),
            error: result.err(),
        });
    }

    pub fn add_mutation_test_result(&mut self, name: &str, result: MutationTestResult) {
        self.mutation_test_results.push(result);
    }

    pub fn add_benchmark_result(&mut self, name: &str, result: BenchmarkResult) {
        self.benchmark_results.push(result);
    }

    pub fn add_integration_test_result(&mut self, name: &str, result: IntegrationTestResult) {
        self.integration_test_results.push(result);
    }

    pub fn calculate_overall_score(&mut self) {
        let mut total_score = 0.0;
        let mut total_weight = 0.0;

        // Property tests weight: 0.3
        let property_score = self.property_test_results.iter()
            .map(|r| if r.passed { 1.0 } else { 0.0 })
            .sum::<f64>() / self.property_test_results.len().max(1) as f64;
        total_score += property_score * 0.3;
        total_weight += 0.3;

        // Mutation tests weight: 0.3
        let mutation_score = self.mutation_test_results.iter()
            .map(|r| r.mutation_score)
            .sum::<f64>() / self.mutation_test_results.len().max(1) as f64;
        total_score += mutation_score * 0.3;
        total_weight += 0.3;

        // Performance benchmarks weight: 0.2
        let performance_score = self.benchmark_results.iter()
            .map(|r| if r.average_time.as_millis() < 100 { 1.0 } else { 0.5 })
            .sum::<f64>() / self.benchmark_results.len().max(1) as f64;
        total_score += performance_score * 0.2;
        total_weight += 0.2;

        // Integration tests weight: 0.2
        let integration_score = self.integration_test_results.iter()
            .map(|r| if r.passed { 1.0 } else { 0.0 })
            .sum::<f64>() / self.integration_test_results.len().max(1) as f64;
        total_score += integration_score * 0.2;
        total_weight += 0.2;

        self.overall_score = if total_weight > 0.0 {
            total_score / total_weight
        } else {
            0.0
        };
    }

    pub fn generate_report(&self) -> String {
        let mut report = String::new();
        report.push_str("# Comprehensive Test Suite Report\n\n");

        report.push_str(&format!("## Overall Score: {:.1}%\n\n", self.overall_score * 100.0));

        // Property tests section
        if !self.property_test_results.is_empty() {
            report.push_str("## Property Tests\n\n");
            for result in &self.property_test_results {
                report.push_str(&format!("- **{}**: {}\n", result.name, if result.passed { "PASSED" } else { "FAILED" }));
                if let Some(error) = &result.error {
                    report.push_str(&format!("  - Error: {}\n", error));
                }
            }
            report.push_str("\n");
        }

        // Mutation tests section
        if !self.mutation_test_results.is_empty() {
            report.push_str("## Mutation Tests\n\n");
            for result in &self.mutation_test_results {
                report.push_str(&format!("- **{}**: {:.1}% mutation score\n", result.name, result.mutation_score * 100.0));
                report.push_str(&format!("  - Killed: {}/{}\n", result.killed_mutations, result.total_mutations));
                report.push_str(&format!("  - Execution time: {:.2}s\n", result.execution_time.as_secs_f64()));
            }
            report.push_str("\n");
        }

        // Performance benchmarks section
        if !self.benchmark_results.is_empty() {
            report.push_str("## Performance Benchmarks\n\n");
            for result in &self.benchmark_results {
                report.push_str(&format!("- **{}**: {:.2}μs average\n", result.name, result.average_time.as_secs_f64() * 1_000_000.0));
                report.push_str(&format!("  - Min: {:.2}μs, Max: {:.2}μs\n", result.min_time.as_secs_f64() * 1_000_000.0, result.max_time.as_secs_f64() * 1_000_000.0));
                report.push_str(&format!("  - P95: {:.2}μs, P99: {:.2}μs\n", result.p95_time.as_secs_f64() * 1_000_000.0, result.p99_time.as_secs_f64() * 1_000_000.0));
            }
            report.push_str("\n");
        }

        // Integration tests section
        if !self.integration_test_results.is_empty() {
            report.push_str("## Integration Tests\n\n");
            for result in &self.integration_test_results {
                report.push_str(&format!("- **{}**: {}\n", result.name, if result.passed { "PASSED" } else { "FAILED" }));
                if let Some(error) = &result.error {
                    report.push_str(&format!("  - Error: {}\n", error));
                }
            }
            report.push_str("\n");
        }

        report
    }
}

/// Property test result
pub struct PropertyTestResult {
    pub name: String,
    pub passed: bool,
    pub error: Option<String>,
}

/// Integration test result
pub struct IntegrationTestResult {
    pub name: String,
    pub passed: bool,
    pub error: Option<String>,
    pub execution_time: std::time::Duration,
}

/// Run component integration tests
pub fn run_component_integration_tests() -> IntegrationTestResult {
    let start_time = std::time::Instant::now();
    
    // Test component integration
    let runtime = create_runtime();
    let (value, set_value) = create_signal("test".to_string());
    
    let view = view! {
        <form>
            <FormField>
                <FormLabel for="test-input">"Test Label"</FormLabel>
                <Input
                    id="test-input"
                    value=value
                    on_change=set_value
                />
            </FormField>
            <Button type="submit">"Submit"</Button>
        </form>
    };
    
    let passed = view.into_any().is_some();
    runtime.dispose();
    
    IntegrationTestResult {
        name: "ComponentIntegration".to_string(),
        passed,
        error: if passed { None } else { Some("Component integration failed".to_string()) },
        execution_time: start_time.elapsed(),
    }
}

/// Run state integration tests
pub fn run_state_integration_tests() -> IntegrationTestResult {
    let start_time = std::time::Instant::now();
    
    // Test state integration
    let runtime = create_runtime();
    let (state, set_state) = create_signal(vec!["item1".to_string(), "item2".to_string()]);
    
    // Test state updates
    set_state.update(|s| s.push("item3".to_string()));
    assert_eq!(state().len(), 3);
    assert_eq!(state().last().unwrap(), "item3");
    
    // Test state filtering
    set_state.update(|s| s.retain(|item| item != "item2"));
    assert_eq!(state().len(), 2);
    assert!(!state().contains(&"item2".to_string()));
    
    runtime.dispose();
    
    IntegrationTestResult {
        name: "StateIntegration".to_string(),
        passed: true,
        error: None,
        execution_time: start_time.elapsed(),
    }
}

/// Run comprehensive test suite
pub fn run_comprehensive_test_suite() -> TestSuiteResult {
    let suite = ComprehensiveTestSuite::default();
    let mut result = suite.run_all_tests();
    result.calculate_overall_score();
    result
}

/// Generate comprehensive test report
pub fn generate_comprehensive_test_report() -> String {
    let result = run_comprehensive_test_suite();
    result.generate_report()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comprehensive_test_suite() {
        let suite = ComprehensiveTestSuite::default();
        let result = suite.run_all_tests();
        assert!(!result.property_test_results.is_empty() || !result.mutation_test_results.is_empty());
    }

    #[test]
    fn test_test_suite_result() {
        let mut result = TestSuiteResult::new();
        result.add_property_test_result("Test", Ok(()));
        result.calculate_overall_score();
        assert!(result.overall_score > 0.0);
    }

    #[test]
    fn test_integration_tests() {
        let result = run_component_integration_tests();
        assert!(result.passed);
    }

    #[test]
    fn test_state_integration_tests() {
        let result = run_state_integration_tests();
        assert!(result.passed);
    }

    #[test]
    fn test_comprehensive_test_report() {
        let report = generate_comprehensive_test_report();
        assert!(report.contains("Comprehensive Test Suite Report"));
    }
}
