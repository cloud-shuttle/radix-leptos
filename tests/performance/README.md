# Performance Tests

This directory contains performance and benchmark tests for Radix-Leptos components.

## Structure

```
performance/
├── benchmarks/          # Performance benchmarks
├── stress-tests/        # Stress testing scenarios
├── memory-tests/        # Memory usage tests
├── render-tests/        # Rendering performance
└── bundle-tests/        # Bundle size tests
```

## Running Performance Tests

```bash
# Run performance tests
pnpm test --grep "performance"

# Run specific performance test
pnpm test tests/performance/quick-performance.spec.ts

# Run with performance profiling
pnpm test --grep "performance" --reporter=json
```

## Test Categories

### Benchmarks
- **Component Creation**: Time to create components
- **Rendering Speed**: DOM update performance
- **Event Handling**: Event processing speed
- **State Updates**: State change performance

### Stress Tests
- **Large Datasets**: Performance with 1000+ items
- **Rapid Updates**: High-frequency state changes
- **Memory Pressure**: Memory usage under load
- **Concurrent Operations**: Multiple simultaneous operations

### Bundle Analysis
- **Bundle Size**: Total bundle size measurement
- **Tree Shaking**: Dead code elimination
- **Compression**: Gzip/Brotli compression ratios
- **Dependency Analysis**: Dependency size breakdown

## Performance Targets

- **Bundle Size**: < 600KB (gzipped)
- **First Paint**: < 100ms
- **Component Creation**: < 1ms per component
- **Memory Usage**: < 50MB for 1000 components
- **Event Handling**: < 5ms per event

## Tools Used

- **Playwright**: Browser automation and performance testing
- **Lighthouse**: Performance auditing
- **Bundle Analyzer**: Bundle size analysis
- **Memory Profiler**: Memory usage tracking
- **Performance Observer**: Real-time performance monitoring
