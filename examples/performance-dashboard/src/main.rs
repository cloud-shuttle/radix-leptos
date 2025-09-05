use leptos::*;
use radix_leptos::*;
use std::time::Instant;

#[component]
pub fn PerformanceDashboard() -> impl IntoView {
    let (benchmarks, set_benchmarks) = create_signal(Vec::new());
    let (is_running, set_is_running) = create_signal(false);
    let (results, set_results) = create_signal(PerformanceResults::default());

    let run_benchmark = move |_| {
        set_is_running(true);
        set_benchmarks(Vec::new());
        
        // Run component benchmarks
        let start = Instant::now();
        
        // Button rendering benchmark
        let button_start = Instant::now();
        for _ in 0..1000 {
            let _ = view! { <Button>"Test"</Button> };
        }
        let button_time = button_start.elapsed().as_millis();
        
        // Form validation benchmark
        let form_start = Instant::now();
        for _ in 0..100 {
            let _ = view! {
                <Form>
                    <FormField>
                        <FormLabel>"Test"</FormLabel>
                        <FormControl>
                            <Input placeholder="Test" />
                        </FormControl>
                    </FormField>
                </Form>
            };
        }
        let form_time = form_start.elapsed().as_millis();
        
        // Data table benchmark
        let table_start = Instant::now();
        for _ in 0..10 {
            let data = (0..100).map(|i| format!("Row {}", i)).collect::<Vec<_>>();
            let _ = view! {
                <DataTable>
                    {data.into_iter().map(|row| {
                        view! {
                            <tr><td>{row}</td></tr>
                        }
                    }).collect::<Vec<_>>()}
                </DataTable>
            };
        }
        let table_time = table_start.elapsed().as_millis();
        
        let total_time = start.elapsed().as_millis();
        
        let benchmark_results = vec![
            BenchmarkResult {
                name: "Button Rendering".to_string(),
                duration: button_time,
                score: calculate_score(button_time, 1000, 16), // 16ms target per 1000 renders
                status: if button_time < 100 { "passed" } else { "failed" }.to_string(),
            },
            BenchmarkResult {
                name: "Form Validation".to_string(),
                duration: form_time,
                score: calculate_score(form_time, 100, 50), // 50ms target per 100 forms
                status: if form_time < 200 { "passed" } else { "failed" }.to_string(),
            },
            BenchmarkResult {
                name: "Data Table".to_string(),
                duration: table_time,
                score: calculate_score(table_time, 10, 100), // 100ms target per 10 tables
                status: if table_time < 500 { "passed" } else { "failed" }.to_string(),
            },
        ];
        
        set_benchmarks(benchmark_results.clone());
        
        let passed = benchmark_results.iter().filter(|r| r.status == "passed").count();
        let total = benchmark_results.len();
        let average_score = benchmark_results.iter().map(|r| r.score).sum::<u32>() / total as u32;
        
        set_results(PerformanceResults {
            total_tests: total,
            passed,
            failed: total - passed,
            average_score,
            total_duration: total_time,
            timestamp: chrono::Utc::now().to_rfc3339(),
        });
        
        set_is_running(false);
    };

    view! {
        <div class="performance-dashboard">
            <Card>
                <CardHeader>
                    <CardTitle>"Performance Dashboard"</CardTitle>
                    <CardDescription>
                        "Monitor and benchmark component performance in real-time"
                    </CardDescription>
                </CardHeader>
                <CardContent>
                    <div class="dashboard-controls">
                        <Button 
                            on:click=run_benchmark 
                            disabled=is_running
                            class="run-benchmark-btn"
                        >
                            {move || if is_running() { "Running Benchmarks..." } else { "Run Benchmarks" }}
                        </Button>
                    </div>
                    
                    {move || if !benchmarks().is_empty() {
                        view! {
                            <div class="performance-summary">
                                <h3>"Performance Summary"</h3>
                                <div class="summary-stats">
                                    <div class="stat">
                                        <span class="stat-label">"Total Tests:"</span>
                                        <span class="stat-value">{results().total_tests}</span>
                                    </div>
                                    <div class="stat">
                                        <span class="stat-label">"Passed:"</span>
                                        <span class="stat-value passed">{results().passed}</span>
                                    </div>
                                    <div class="stat">
                                        <span class="stat-label">"Failed:"</span>
                                        <span class="stat-value failed">{results().failed}</span>
                                    </div>
                                    <div class="stat">
                                        <span class="stat-label">"Average Score:"</span>
                                        <span class="stat-value">{results().average_score}/100</span>
                                    </div>
                                    <div class="stat">
                                        <span class="stat-label">"Total Duration:"</span>
                                        <span class="stat-value">{results().total_duration}ms</span>
                                    </div>
                                </div>
                            </div>
                            
                            <div class="benchmark-results">
                                <h3>"Benchmark Results"</h3>
                                <div class="results-grid">
                                    {benchmarks().into_iter().map(|benchmark| {
                                        view! {
                                            <Card class="benchmark-card">
                                                <CardHeader>
                                                    <CardTitle class="benchmark-title">
                                                        {benchmark.name}
                                                    </CardTitle>
                                                </CardHeader>
                                                <CardContent>
                                                    <div class="benchmark-metrics">
                                                        <div class="metric">
                                                            <span class="metric-label">"Duration:"</span>
                                                            <span class="metric-value">{benchmark.duration}ms</span>
                                                        </div>
                                                        <div class="metric">
                                                            <span class="metric-label">"Score:"</span>
                                                            <span class="metric-value">{benchmark.score}/100</span>
                                                        </div>
                                                        <div class="metric">
                                                            <span class="metric-label">"Status:"</span>
                                                            <span class=format!("metric-value {}", benchmark.status)>
                                                                {benchmark.status}
                                                            </span>
                                                        </div>
                                                    </div>
                                                </CardContent>
                                            </Card>
                                        }
                                    }).collect::<Vec<_>>()}
                                </div>
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <div class="no-results">
                                <p>"Click 'Run Benchmarks' to start performance testing"</p>
                            </div>
                        }.into_any()
                    }}
                </CardContent>
            </Card>
        </div>
    }
}

#[derive(Clone, Default)]
struct BenchmarkResult {
    name: String,
    duration: u128,
    score: u32,
    status: String,
}

#[derive(Clone, Default)]
struct PerformanceResults {
    total_tests: usize,
    passed: usize,
    failed: usize,
    average_score: u32,
    total_duration: u128,
    timestamp: String,
}

fn calculate_score(duration: u128, iterations: u32, target_ms: u32) -> u32 {
    let target_total = target_ms * iterations;
    if duration <= target_total as u128 {
        100
    } else {
        let ratio = target_total as f64 / duration as f64;
        (ratio * 100.0).min(100.0).max(0.0) as u32
    }
}

fn main() {
    mount_to_body(|| {
        view! {
            <div class="app">
                <PerformanceDashboard />
            </div>
        }
    })
}

