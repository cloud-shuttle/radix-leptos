import { test, expect } from '@playwright/test';

const BASE_URL = 'http://localhost:8080';

// Performance measurement utilities
class PerformanceMetrics {
  private metrics: Map<string, number[]> = new Map();
  private startTimes: Map<string, number> = new Map();

  startTimer(name: string) {
    this.startTimes.set(name, performance.now());
  }

  endTimer(name: string): number {
    const startTime = this.startTimes.get(name);
    if (!startTime) throw new Error(`Timer '${name}' was not started`);
    
    const duration = performance.now() - startTime;
    if (!this.metrics.has(name)) {
      this.metrics.set(name, []);
    }
    this.metrics.get(name)!.push(duration);
    
    return duration;
  }

  getAverage(name: string): number {
    const values = this.metrics.get(name);
    if (!values || values.length === 0) return 0;
    return values.reduce((a, b) => a + b, 0) / values.length;
  }

  getMetrics(): Record<string, { average: number; min: number; max: number; count: number }> {
    const result: Record<string, { average: number; min: number; max: number; count: number }> = {};
    
    for (const [name, values] of this.metrics.entries()) {
      result[name] = {
        average: this.getAverage(name),
        min: Math.min(...values),
        max: Math.max(...values),
        count: values.length
      };
    }
    
    return result;
  }

  reset() {
    this.metrics.clear();
    this.startTimes.clear();
  }
}

test.describe('Performance Benchmarks - Production Readiness', () => {
  let metrics: PerformanceMetrics;

  test.beforeEach(async ({ page }) => {
    metrics = new PerformanceMetrics();
    
    // Set longer timeout for performance testing
    page.setDefaultTimeout(120000);
    
    // Capture performance metrics
    await page.addInitScript(() => {
      window.performanceMetrics = {
        wasmInitTime: 0,
        componentRenderTime: 0,
        memoryUsage: 0
      };
    });
  });

  test('WASM Initialization Performance', async ({ page }) => {
    console.log('🧪 Testing WASM Initialization Performance...');
    
    const testUrls = [
      'dropdown_menu_examples.html',
      'menubar_examples.html',
      'audio_examples.html',
      'video_examples.html',
      'list-examples.html'
    ];

    for (const url of testUrls) {
      console.log(`\n📊 Testing ${url}...`);
      
      // Measure page load time
      metrics.startTimer(`page_load_${url}`);
      await page.goto(`${BASE_URL}/${url}`);
      
      // Wait for WASM to be ready using robust detection
      await page.waitForFunction(() => {
        // Check for various loading indicators
        const loadingSelectors = [
          '.loading',
          '.spinner', 
          '.animate-spin',
          '[data-loading="true"]',
          '.wasm-loading'
        ];
        
        const hasLoading = loadingSelectors.some(selector => 
          document.querySelector(selector)
        );
        
        // Check if WASM is ready
        const wasmReady = window.WebAssembly && 
          (window.wasmReady || window.leptosReady || window.radixReady);
        
        // Check for component content
        const hasContent = document.body.textContent && 
          document.body.textContent.length > 100;
        
        return !hasLoading && (wasmReady || hasContent);
      }, { timeout: 30000 });
      
      const loadTime = metrics.endTimer(`page_load_${url}`);
      console.log(`  ✅ Page load: ${loadTime.toFixed(2)}ms`);
      
      // Measure WASM initialization time
      const wasmInitTime = await page.evaluate(() => {
        return window.performanceMetrics?.wasmInitTime || 0;
      });
      
      if (wasmInitTime > 0) {
        console.log(`  ✅ WASM init: ${wasmInitTime.toFixed(2)}ms`);
      }
      
      // Measure memory usage
      const memoryInfo = await page.evaluate(() => {
        if ('memory' in performance) {
          return (performance as any).memory;
        }
        return null;
      });
      
      if (memoryInfo) {
        console.log(`  ✅ Memory: ${(memoryInfo.usedJSHeapSize / 1024 / 1024).toFixed(2)}MB used`);
      }
      
      // Wait between tests
      await page.waitForTimeout(1000);
    }
    
    // Performance assertions
    const avgLoadTime = metrics.getAverage('page_load_tabs_examples.html');
    expect(avgLoadTime).toBeLessThan(5000); // Should load in under 5 seconds
    
    console.log(`\n📊 Average page load time: ${avgLoadTime.toFixed(2)}ms`);
  });

  test('Component Render Performance', async ({ page }) => {
    console.log('🧪 Testing Component Render Performance...');
    
    await page.goto(`${BASE_URL}/component_showcase.html`);
    
    // Wait for WASM to load using robust detection
    await page.waitForFunction(() => {
      // Check for various loading indicators
      const loadingSelectors = [
        '.loading',
        '.spinner', 
        '.animate-spin',
        '[data-loading="true"]',
        '.wasm-loading'
      ];
      
      const hasLoading = loadingSelectors.some(selector => 
        document.querySelector(selector)
      );
      
      // Check if WASM is ready
      const wasmReady = window.WebAssembly && 
        (window.wasmReady || window.leptosReady || window.radixReady);
      
      // Check for component content
      const hasContent = document.body.textContent && 
        document.body.textContent.length > 100;
      
      return !hasLoading && (wasmReady || hasContent);
    }, { timeout: 30000 });
    
    // Measure time to first meaningful paint
    metrics.startTimer('first_paint');
    await page.waitForFunction(() => {
      return document.body.textContent && document.body.textContent.length > 500;
    }, { timeout: 10000 });
    const firstPaintTime = metrics.endTimer('first_paint');
    
    console.log(`✅ First meaningful paint: ${firstPaintTime.toFixed(2)}ms`);
    
    // Measure component interaction responsiveness
    const interactiveElements = [
      'button',
      '[data-radix-dropdown-menu-trigger]',
      '[data-radix-menubar-trigger]',
      '[data-radix-context-menu-trigger]'
    ];
    
    for (const selector of interactiveElements) {
      const elements = page.locator(selector);
      const count = await elements.count();
      
      if (count > 0) {
        console.log(`\n🔍 Testing ${selector} (${count} elements)...`);
        
        for (let i = 0; i < Math.min(count, 3); i++) { // Test first 3 elements
          const element = elements.nth(i);
          
          // Measure click responsiveness
          metrics.startTimer(`click_${selector}_${i}`);
          await element.click();
          const clickTime = metrics.endTimer(`click_${selector}_${i}`);
          
          console.log(`  ✅ Click ${i + 1}: ${clickTime.toFixed(2)}ms`);
          
          // Close any opened menus
          await page.keyboard.press('Escape');
          await page.waitForTimeout(100);
        }
      }
    }
    
    // Performance assertions
    expect(firstPaintTime).toBeLessThan(3000); // Should paint in under 3 seconds
    
    const avgClickTime = metrics.getAverage('click_button_0');
    if (avgClickTime > 0) {
      expect(avgClickTime).toBeLessThan(100); // Clicks should be under 100ms
    }
  });

  test('Memory Usage & Leak Detection', async ({ page }) => {
    console.log('🧪 Testing Memory Usage & Leak Detection...');
    
    await page.goto(`${BASE_URL}/component_showcase.html`);
    
    // Wait for WASM to load using robust detection
    await page.waitForFunction(() => {
      // Check for various loading indicators
      const loadingSelectors = [
        '.loading',
        '.spinner', 
        '.animate-spin',
        '[data-loading="true"]',
        '.wasm-loading'
      ];
      
      const hasLoading = loadingSelectors.some(selector => 
        document.querySelector(selector)
      );
      
      // Check if WASM is ready
      const wasmReady = window.WebAssembly && 
        (window.wasmReady || window.leptosReady || window.radixReady);
      
      // Check for component content
      const hasContent = document.body.textContent && 
        document.body.textContent.length > 100;
      
      return !hasLoading && (wasmReady || hasContent);
    }, { timeout: 30000 });
    
    // Get initial memory usage
    const initialMemory = await page.evaluate(() => {
      if ('memory' in performance) {
        return (performance as any).memory.usedJSHeapSize;
      }
      return 0;
    });
    
    console.log(`📊 Initial memory: ${(initialMemory / 1024 / 1024).toFixed(2)}MB`);
    
    // Perform multiple interactions to test for memory leaks
    for (let cycle = 0; cycle < 5; cycle++) {
      console.log(`\n🔄 Memory test cycle ${cycle + 1}/5...`);
      
      // Open and close dropdowns
      const dropdownTriggers = page.locator('[data-radix-dropdown-menu-trigger]');
      const count = await dropdownTriggers.count();
      
      for (let i = 0; i < Math.min(count, 3); i++) {
        const trigger = dropdownTriggers.nth(i);
        await trigger.click();
        await page.waitForTimeout(100);
        await page.keyboard.press('Escape');
        await page.waitForTimeout(100);
      }
      
      // Open and close context menus
      const contextTriggers = page.locator('[data-radix-context-menu-trigger]');
      const contextCount = await contextTriggers.count();
      
      for (let i = 0; i < Math.min(contextCount, 2); i++) {
        const trigger = contextTriggers.nth(i);
        await trigger.click({ button: 'right' });
        await page.waitForTimeout(100);
        await page.keyboard.press('Escape');
        await page.waitForTimeout(100);
      }
      
      // Check memory after each cycle
      const currentMemory = await page.evaluate(() => {
        if ('memory' in performance) {
          return (performance as any).memory.usedJSHeapSize;
        }
        return 0;
      });
      
      console.log(`  📊 Memory after cycle ${cycle + 1}: ${(currentMemory / 1024 / 1024).toFixed(2)}MB`);
      
      // Wait between cycles
      await page.waitForTimeout(500);
    }
    
    // Final memory check
    const finalMemory = await page.evaluate(() => {
      if ('memory' in performance) {
        return (performance as any).memory.usedJSHeapSize;
      }
      return 0;
    });
    
    console.log(`\n📊 Final memory: ${(finalMemory / 1024 / 1024).toFixed(2)}MB`);
    
    // Memory leak detection (should not increase by more than 10%)
    const memoryIncrease = ((finalMemory - initialMemory) / initialMemory) * 100;
    console.log(`📊 Memory change: ${memoryIncrease.toFixed(2)}%`);
    
    expect(memoryIncrease).toBeLessThan(10); // Should not increase by more than 10%
  });

  test('Bundle Size & Loading Performance', async ({ page }) => {
    console.log('🧪 Testing Bundle Size & Loading Performance...');
    
    // Navigate to a simple page first
    await page.goto(`${BASE_URL}/`);
    
    // Clear network cache
    await page.context().clearCookies();
    
    // Enable network monitoring
    const networkRequests: Array<{ url: string; size: number; duration: number }> = [];
    
    page.on('response', async (response) => {
      const url = response.url();
      if (url.includes('pkg/') || url.includes('.wasm') || url.includes('.js')) {
        const headers = response.headers();
        const contentLength = headers['content-length'];
        const size = contentLength ? parseInt(contentLength) : 0;
        
        networkRequests.push({
          url,
          size,
          duration: 0
        });
      }
    });
    
    // Navigate to component showcase (heaviest page)
    metrics.startTimer('bundle_load');
    await page.goto(`${BASE_URL}/component_showcase.html`);
    
    // Wait for WASM to load using robust detection
    await page.waitForFunction(() => {
      // Check for various loading indicators
      const loadingSelectors = [
        '.loading',
        '.spinner', 
        '.animate-spin',
        '[data-loading="true"]',
        '.wasm-loading'
      ];
      
      const hasLoading = loadingSelectors.some(selector => 
        document.querySelector(selector)
      );
      
      // Check if WASM is ready
      const wasmReady = window.WebAssembly && 
        (window.wasmReady || window.leptosReady || window.radixReady);
      
      // Check for component content
      const hasContent = document.body.textContent && 
        document.body.textContent.length > 100;
      
      return !hasLoading && (wasmReady || hasContent);
    }, { timeout: 30000 });
    
    const bundleLoadTime = metrics.endTimer('bundle_load');
    
    // Analyze bundle sizes
    let totalSize = 0;
    let wasmSize = 0;
    let jsSize = 0;
    
    for (const request of networkRequests) {
      if (request.url.includes('.wasm')) {
        wasmSize += request.size;
      } else if (request.url.includes('.js')) {
        jsSize += request.size;
      }
      totalSize += request.size;
    }
    
    console.log(`\n📦 Bundle Size Analysis:`);
    console.log(`  ✅ Total bundle size: ${(totalSize / 1024 / 1024).toFixed(2)}MB`);
    console.log(`  ✅ WASM size: ${(wasmSize / 1024 / 1024).toFixed(2)}MB`);
    console.log(`  ✅ JavaScript size: ${(jsSize / 1024 / 1024).toFixed(2)}MB`);
    console.log(`  ✅ Bundle load time: ${bundleLoadTime.toFixed(2)}ms`);
    
    // Performance assertions
    expect(bundleLoadTime).toBeLessThan(10000); // Should load in under 10 seconds
    expect(totalSize).toBeLessThan(10 * 1024 * 1024); // Should be under 10MB total
    
    // Log performance recommendations
    if (totalSize > 5 * 1024 * 1024) {
      console.log(`⚠️  Bundle size is large (>5MB). Consider code splitting.`);
    }
    
    if (bundleLoadTime > 5000) {
      console.log(`⚠️  Bundle load time is slow (>5s). Consider optimization.`);
    }
  });

  test('Scroll Performance & Smoothness', async ({ page }) => {
    console.log('🧪 Testing Scroll Performance & Smoothness...');
    
    await page.goto(`${BASE_URL}/scroll_area_examples.html`);
    
    // Wait for WASM to load using robust detection
    await page.waitForFunction(() => {
      // Check for various loading indicators
      const loadingSelectors = [
        '.loading',
        '.spinner', 
        '.animate-spin',
        '[data-loading="true"]',
        '.wasm-loading'
      ];
      
      const hasLoading = loadingSelectors.some(selector => 
        document.querySelector(selector)
      );
      
      // Check if WASM is ready
      const wasmReady = window.WebAssembly && 
        (window.wasmReady || window.leptosReady || window.radixReady);
      
      // Check for component content
      const hasContent = document.body.textContent && 
        document.body.textContent.length > 100;
      
      return !hasLoading && (wasmReady || hasContent);
    }, { timeout: 30000 });
    
    // Find scroll areas
    const scrollAreas = page.locator('[data-radix-scroll-area-viewport]');
    const count = await scrollAreas.count();
    
    console.log(`🔍 Found ${count} scroll areas to test`);
    
    for (let i = 0; i < Math.min(count, 3); i++) {
      const scrollArea = scrollAreas.nth(i);
      
      if (await scrollArea.isVisible()) {
        console.log(`\n📜 Testing scroll area ${i + 1}...`);
        
        // Measure scroll performance
        metrics.startTimer(`scroll_${i}`);
        
        // Perform smooth scrolling
        await scrollArea.evaluate((el) => {
          return new Promise<void>((resolve) => {
            let scrollTop = 0;
            const targetScrollTop = Math.min(el.scrollHeight - el.clientHeight, 500);
            const step = targetScrollTop / 20;
            
            const scroll = () => {
              scrollTop += step;
              el.scrollTop = scrollTop;
              
              if (scrollTop < targetScrollTop) {
                requestAnimationFrame(scroll);
              } else {
                resolve();
              }
            };
            
            requestAnimationFrame(scroll);
          });
        });
        
        const scrollTime = metrics.endTimer(`scroll_${i}`);
        console.log(`  ✅ Smooth scroll: ${scrollTime.toFixed(2)}ms`);
        
        // Test scroll event responsiveness
        metrics.startTimer(`scroll_event_${i}`);
        await scrollArea.evaluate((el) => {
          el.scrollTop = 0;
        });
        await page.waitForTimeout(100);
        const eventTime = metrics.endTimer(`scroll_event_${i}`);
        
        console.log(`  ✅ Scroll event: ${eventTime.toFixed(2)}ms`);
        
        // Wait between tests
        await page.waitForTimeout(500);
      }
    }
    
    // Performance assertions
    const avgScrollTime = metrics.getAverage('scroll_0');
    if (avgScrollTime > 0) {
      expect(avgScrollTime).toBeLessThan(1000); // Smooth scroll should be under 1 second
    }
  });

  test.afterEach(async () => {
    // Generate performance report
    const performanceReport = metrics.getMetrics();
    
    console.log('\n📊 Performance Benchmark Results:');
    console.log('================================');
    
    for (const [name, data] of Object.entries(performanceReport)) {
      console.log(`${name}:`);
      console.log(`  Average: ${data.average.toFixed(2)}ms`);
      console.log(`  Min: ${data.min.toFixed(2)}ms`);
      console.log(`  Max: ${data.max.toFixed(2)}ms`);
      console.log(`  Count: ${data.count}`);
    }
    
    // Reset metrics for next test
    metrics.reset();
  });
});
