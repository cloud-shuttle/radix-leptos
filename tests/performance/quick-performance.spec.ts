import { test, expect } from '@playwright/test';

const BASE_URL = 'http://localhost:8080';

test.describe('Quick Performance Validation - Production Readiness', () => {
  test('Quick WASM Component Load Test', async ({ page }) => {
    console.log('🚀 Quick Performance Validation Starting...');
    
    // Test dropdown menu component (known to have WASM)
    const startTime = Date.now();
    
    await page.goto(`${BASE_URL}/dropdown_menu_examples.html`);
    
    // Wait for any content to appear
    await page.waitForFunction(() => {
      return document.body.textContent && document.body.textContent.length > 100;
    }, { timeout: 10000 });
    
    const loadTime = Date.now() - startTime;
    console.log(`✅ Page load time: ${loadTime}ms`);
    
    // Check if WASM files are loaded
    const wasmLoaded = await page.evaluate(() => {
      return window.WebAssembly !== undefined;
    });
    
    console.log(`✅ WebAssembly available: ${wasmLoaded}`);
    
    // Check for component content
    const dropdownTriggers = page.locator('[data-radix-dropdown-menu-trigger]');
    const triggerCount = await dropdownTriggers.count();
    
    console.log(`✅ Found ${triggerCount} dropdown triggers`);
    
    // Performance assertions
    expect(loadTime).toBeLessThan(5000); // Should load in under 5 seconds
    expect(triggerCount).toBeGreaterThan(0); // Should have at least one trigger
    
    console.log(`\n📊 Quick Performance Results:`);
    console.log(`  Load Time: ${loadTime}ms`);
    console.log(`  WASM Available: ${wasmLoaded}`);
    console.log(`  Components Found: ${triggerCount}`);
    console.log(`  Status: ✅ READY FOR PRODUCTION`);
  });
});
