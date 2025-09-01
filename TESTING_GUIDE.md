# 🧪 Radix-Leptos Automated Testing Guide

This guide covers the comprehensive automated testing setup for all Radix-Leptos components using Playwright and browser automation.

## 🚀 Quick Start

### 1. **Run All Tests**
```bash
# Using the test runner script
./run-tests.sh

# Using Make commands
make test-comprehensive

# Using Playwright directly
cd examples && pnpm exec playwright test --headed
```

### 2. **Run Specific Test Suites**
```bash
# Basic component tests
make test-basic

# Interactive component tests
make test-interactive

# Performance and accessibility tests
make test-performance

# Individual component tests
make test-carousel
make test-tabs
```

## 📋 Test Suites Overview

### 🧩 **Basic Component Tests** (`tests/all-components.spec.ts`)
Tests that all components load correctly and render their basic elements.

**Components Tested:**
- ✅ Tabs
- ✅ Carousel
- ✅ DropdownMenu
- ✅ ContextMenu
- ✅ Menubar
- ✅ ScrollArea
- ✅ Toast
- ✅ Alert
- ✅ Badge
- ✅ Avatar
- ✅ Image
- ✅ Video
- ✅ Audio
- ✅ Timeline
- ✅ List
- ✅ Pagination
- ✅ Component Showcase

**What It Tests:**
- Component rendering
- Basic element presence
- WASM loading
- No console errors
- Screenshot capture for verification

### 🎮 **Interactive Component Tests** (`tests/interactive-components.spec.ts`)
Deep testing of component interactions, state management, and user interactions.

**Features Tested:**
- **Carousel**: Navigation, indicators, autoplay, slide transitions
- **Tabs**: Tab switching, content updates, keyboard navigation
- **DropdownMenu**: Menu opening/closing, item selection, submenus
- **ContextMenu**: Right-click functionality, positioning, keyboard nav
- **Menubar**: Multi-level navigation, submenus, keyboard shortcuts
- **ScrollArea**: Scrolling behavior, viewport management, scrollbars
- **Toast**: Multiple toasts, stacking, dismissal, actions
- **Form Components**: Input handling, validation, state changes

### 📊 **Performance & Accessibility Tests** (`tests/performance-accessibility.spec.ts`)
Comprehensive testing of performance, accessibility, and cross-browser compatibility.

**Performance Testing:**
- Load times for each component
- WASM initialization time
- Memory usage monitoring
- DOM size metrics
- Performance benchmarks

**Accessibility Testing:**
- ARIA labels and roles
- Keyboard navigation support
- Focus management
- Color contrast analysis
- Screen reader compatibility

**Compatibility Testing:**
- CSS feature support
- JavaScript feature support
- Web API availability
- Cross-browser compatibility

**Error Handling:**
- Network failure resilience
- WASM loading failures
- Component recovery
- Graceful degradation

## 🛠️ Test Configuration

### Playwright Configuration (`playwright.config.ts`)
```typescript
export default defineConfig({
  testDir: './tests',
  fullyParallel: true,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : undefined,
  reporter: 'html',
  use: {
    baseURL: 'http://localhost:8080',
    trace: 'on-first-retry',
    screenshot: 'only-on-failure',
    video: 'retain-on-failure',
  },
  projects: [
    { name: 'chromium', use: { ...devices['Desktop Chrome'] } },
    { name: 'firefox', use: { ...devices['Desktop Firefox'] } },
    { name: 'webkit', use: { ...devices['Desktop Safari'] } },
  ],
  webServer: {
    command: 'python3 -m http.server 8080',
    url: 'http://localhost:8080',
    reuseExistingServer: !process.env.CI,
    timeout: 120 * 1000,
  },
});
```

### Test Timeouts and Settings
- **Default timeout**: 30 seconds per test
- **WASM loading wait**: Automatic detection of loading states
- **Screenshot capture**: On failure and for verification
- **Video recording**: On failure for debugging
- **Parallel execution**: Multiple browsers simultaneously

## 🔧 Running Tests

### **Command Line Options**

#### Basic Test Execution
```bash
# Run all tests
pnpm exec playwright test

# Run specific test file
pnpm exec playwright test tests/all-components.spec.ts

# Run tests matching pattern
pnpm exec playwright test -g "Carousel"

# Run tests in headed mode (visible browser)
pnpm exec playwright test --headed

# Run tests in debug mode
pnpm exec playwright test --debug
```

#### Test Filtering
```bash
# Run only failed tests
pnpm exec playwright test --grep-invert "passed"

# Run tests with specific tag
pnpm exec playwright test --grep "@smoke"

# Run tests in specific project
pnpm exec playwright test --project=chromium
```

#### Output and Reporting
```bash
# Generate HTML report
pnpm exec playwright test --reporter=html

# Generate JUnit report
pnpm exec playwright test --reporter=junit

# Show test report
pnpm exec playwright show-report
```

### **Make Commands**

```bash
# Show all available commands
make help

# Run specific test suites
make test-basic          # Basic component tests
make test-interactive    # Interactive component tests
make test-performance    # Performance & accessibility tests
make test-comprehensive  # All test suites

# Component-specific tests
make test-carousel       # Test carousel component
make test-tabs          # Test tabs component

# Generate reports
make test-report        # Generate HTML test report
make report             # Show test report
```

## 📊 Test Reports and Output

### **Generated Files**
- **Screenshots**: Component screenshots for verification
- **Videos**: Test execution recordings (on failure)
- **Traces**: Detailed execution traces (on retry)
- **HTML Reports**: Comprehensive test results
- **Console Logs**: Detailed test execution logs

### **Report Structure**
```
test-reports/
├── test_summary_20240901_143022.md    # Summary report
├── Basic_Component_Tests_20240901_143022.txt
├── Interactive_Component_Tests_20240901_143022.txt
└── Performance_&_Accessibility_Tests_20240901_143022.txt
```

### **Sample Test Output**
```
🧪 Testing Carousel Deep Interactions...
✅ Carousel container found and visible
Found 6 next buttons and 6 previous buttons
✅ Next navigation working
✅ Previous navigation working
Found 18 indicators
✅ Indicator navigation working
Found 2 autoplay carousels
✅ Autoplay functionality confirmed
```

## 🐛 Debugging Tests

### **Common Issues and Solutions**

#### 1. **WASM Loading Timeouts**
```typescript
// Increase timeout for slow loading
page.setDefaultTimeout(60000);

// Wait for specific loading states
await page.waitForFunction(() => {
  return !document.querySelector('.loading') && !document.querySelector('.spinner');
});
```

#### 2. **Component Not Found**
```typescript
// Check if component exists before testing
const component = page.locator('.my-component');
if (await component.count() > 0) {
  await expect(component).toBeVisible();
} else {
  console.log('Component not found, skipping test');
}
```

#### 3. **Timing Issues**
```typescript
// Wait for animations to complete
await page.waitForTimeout(1000);

// Wait for specific state changes
await page.waitForFunction(() => {
  return document.querySelector('[data-state="active"]');
});
```

### **Debug Mode**
```bash
# Run tests in debug mode
pnpm exec playwright test --debug

# This opens Playwright Inspector for step-by-step debugging
```

### **Trace Viewer**
```bash
# Generate traces
pnpm exec playwright test --trace=on

# View traces
pnpm exec playwright show-trace trace.zip
```

## 🚀 CI/CD Integration

### **GitHub Actions Example**
```yaml
name: Component Tests
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
        with:
          node-version: '20'
      - uses: pnpm/action-setup@v2
        with:
          version: 8
      - run: pnpm install
      - run: pnpm exec playwright install --with-deps
      - run: make build
      - run: make test-comprehensive
      - uses: actions/upload-artifact@v3
        if: failure()
        with:
          name: playwright-report
          path: examples/playwright-report/
```

### **Environment Variables**
```bash
# Set for CI environments
export CI=true
export PLAYWRIGHT_BROWSERS_PATH=0
export PLAYWRIGHT_SKIP_BROWSER_DOWNLOAD=1
```

## 📈 Performance Benchmarks

### **Expected Performance Metrics**
- **Component Load Time**: < 5 seconds
- **WASM Initialization**: < 2 seconds
- **DOM Elements**: > 100 per component
- **Memory Usage**: < 100MB per component
- **Accessibility Score**: > 80%

### **Performance Monitoring**
```typescript
// Measure load times
const startTime = Date.now();
await page.goto(url);
const loadTime = Date.now() - startTime;

// Monitor memory usage
const memoryInfo = await page.evaluate(() => {
  return (performance as any).memory;
});
```

## 🔍 Accessibility Testing

### **ARIA Compliance**
- Proper `role` attributes
- `aria-label` and `aria-labelledby`
- `aria-expanded` for collapsible content
- `aria-selected` for selectable items

### **Keyboard Navigation**
- Tab order management
- Arrow key navigation
- Enter/Space activation
- Escape key dismissal

### **Screen Reader Support**
- Semantic HTML structure
- Proper heading hierarchy
- Alt text for images
- Descriptive link text

## 🌐 Cross-Browser Testing

### **Supported Browsers**
- **Chromium**: Latest stable
- **Firefox**: Latest stable
- **WebKit**: Latest stable (Safari)

### **Feature Detection**
```typescript
// Test CSS feature support
const cssFeatures = {
  flexbox: CSS.supports('display', 'flex'),
  grid: CSS.supports('display', 'grid'),
  customProperties: CSS.supports('--custom-property', 'value')
};

// Test JavaScript features
const jsFeatures = {
  asyncAwait: typeof (async () => {}) === 'function',
  arrowFunctions: typeof (() => {}) === 'function'
};
```

## 📝 Writing New Tests

### **Test Structure**
```typescript
test('Component Name - Feature Description', async ({ page }) => {
  // Setup
  await page.goto(`${BASE_URL}/component_examples.html`);
  
  // Wait for WASM
  await page.waitForFunction(() => {
    return !document.querySelector('.loading');
  });
  
  // Test functionality
  const element = page.locator('.component-selector');
  await expect(element).toBeVisible();
  
  // Test interactions
  await element.click();
  
  // Verify results
  await expect(page.locator('.result')).toBeVisible();
  
  // Capture screenshot
  await page.screenshot({ path: 'component-test.png' });
});
```

### **Best Practices**
1. **Descriptive test names** that explain what's being tested
2. **Proper waiting** for WASM loading and state changes
3. **Screenshot capture** for visual verification
4. **Error handling** for graceful test failures
5. **Performance monitoring** for regression detection

## 🎯 Next Steps

### **Immediate Actions**
1. **Run the test suite**: `./run-tests.sh`
2. **Review results**: Check generated reports
3. **Fix failures**: Address any component issues
4. **Add new tests**: Cover additional functionality

### **Long-term Improvements**
1. **Visual regression testing** with screenshot comparison
2. **Load testing** for performance under stress
3. **Accessibility audits** with automated tools
4. **Cross-device testing** for mobile compatibility

## 📚 Additional Resources

- [Playwright Documentation](https://playwright.dev/)
- [Testing Best Practices](https://playwright.dev/docs/best-practices)
- [Accessibility Testing](https://playwright.dev/docs/accessibility-testing)
- [Performance Testing](https://playwright.dev/docs/performance-testing)

---

**Happy Testing! 🧪✨**

Your Radix-Leptos components are now comprehensively tested with professional-grade automation!
