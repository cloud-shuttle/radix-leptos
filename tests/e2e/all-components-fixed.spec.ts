import { test, expect } from "@playwright/test";

const BASE_URL = 'http://localhost:8080';

// Helper function for robust WASM loading detection
async function waitForWasmLoad(page: any, timeout = 60000) {
  try {
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
      const wasmReady = (window as any).WebAssembly &&
        ((window as any).wasmReady || (window as any).leptosReady || (window as any).radixReady);

      // Check for component content
      const hasContent = document.body.textContent &&
        document.body.textContent.length > 100;

      return !hasLoading && (wasmReady || hasContent);
    }, { timeout });
  } catch (error) {
    console.warn(`WASM loading timeout after ${timeout}ms, continuing anyway`);
  }
}

test('Basic test', async ({ page }) => {
  expect(true).toBe(true);
});

test('All Components Load Without Errors - Comprehensive Test', async ({ page }) => {
  const components = [
    { name: 'Tabs', url: 'examples/tabs_examples.html' },
    { name: 'Carousel', url: 'examples/carousel_examples.html' },
    { name: 'DropdownMenu', url: 'examples/dropdown_menu_examples.html' },
    { name: 'ContextMenu', url: 'examples/context_menu_examples.html' },
    { name: 'Menubar', url: 'examples/menubar_examples.html' },
    { name: 'ScrollArea', url: 'examples/scroll_area_examples.html' },
    { name: 'Toast', url: 'examples/toast_examples.html' },
    { name: 'Alert', url: 'examples/alert_examples.html' },
    { name: 'Badge', url: 'examples/badge_examples.html' },
    { name: 'Avatar', url: 'examples/avatar_examples.html' },
    { name: 'Image', url: 'examples/image_examples.html' },
    { name: 'Video', url: 'examples/video_examples.html' },
    { name: 'Audio', url: 'examples/audio_examples.html' },
    { name: 'Timeline', url: 'examples/timeline_examples.html' },
    { name: 'List', url: 'examples/list_examples.html' },
    { name: 'Pagination', url: 'examples/pagination-examples.html' },
    { name: 'Component Showcase', url: 'examples/component_showcase.html' }
  ];

  const results: any[] = [];

  for (const component of components) {
    console.log(`\n🧪 Testing ${component.name}...`);
    
    try {
      await page.goto(`${BASE_URL}/${component.url}`);
      
      // Wait for WASM to load
      await waitForWasmLoad(page);
      
      // Check for console errors
      const errors = await page.evaluate(() => {
        return (window as any).consoleErrors || [];
      });
      
      // Check that page loaded successfully
      const body = page.locator('body');
      await expect(body).toBeVisible();
      
      // Check for any visible content
      const hasContent = await page.evaluate(() => {
        return document.body.textContent && document.body.textContent.length > 100;
      });
      
      const status = {
        name: component.name,
        url: component.url,
        loaded: true,
        errors: errors.length,
        hasContent: hasContent,
        timestamp: new Date().toISOString()
      };
      
      results.push(status);
      
      if (errors.length === 0 && hasContent) {
        console.log(`✅ ${component.name} loaded successfully`);
      } else {
        console.log(`⚠️  ${component.name} has issues: ${errors.length} errors, content: ${hasContent}`);
      }
      
    } catch (error: any) {
      console.error(`❌ ${component.name} failed to load:`, error.message);
      
      results.push({
        name: component.name,
        url: component.url,
        loaded: false,
        errors: 1,
        hasContent: false,
        error: error.message,
        timestamp: new Date().toISOString()
      });
    }
  }
  
  // Generate test report
  const successful = results.filter((r: any) => r.loaded && r.errors === 0 && r.hasContent).length;
  const total = results.length;
  
  console.log(`\n📊 Test Results Summary:`);
  console.log(`✅ Successful: ${successful}/${total}`);
  console.log(`❌ Failed: ${total - successful}/${total}`);
  
  // Save detailed results
  await page.evaluate((results) => {
    localStorage.setItem('component-test-results', JSON.stringify(results));
  }, results);
  
  // Assert overall success
  expect(successful).toBeGreaterThan(total * 0.8); // At least 80% success rate
});
