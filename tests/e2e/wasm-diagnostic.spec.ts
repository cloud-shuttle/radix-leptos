import { test, expect } from '@playwright/test';

test.describe('WASM Diagnostic Tests', () => {
  test('should diagnose WASM loading issues', async ({ page }) => {
    // Navigate to the demo
    await page.goto('http://localhost:8080/examples/real-wasm-demo.html');
    
    // Wait for page to load
    await expect(page.locator('h1')).toContainText('Radix-Leptos Real WASM Demo');
    
    // Check initial state
    console.log('Initial leptos-app content:', await page.locator('#leptos-app').textContent());
    console.log('Initial status content:', await page.locator('#status').textContent());
    
    // Wait for WASM to load
    await page.waitForFunction(() => window.leptosReady === true, { timeout: 10000 });
    
    // Check what's actually in the leptos-app container
    const appContent = await page.locator('#leptos-app').textContent();
    console.log('Final leptos-app content:', appContent);
    
    // Check if the container still has loading text
    const hasLoadingText = appContent?.includes('Loading WASM components...');
    console.log('Still has loading text:', hasLoadingText);
    
    // Check if there are any actual components rendered
    const hasComponents = await page.locator('#leptos-app h3').count() > 0;
    console.log('Has components rendered:', hasComponents);
    
    // Take a screenshot for debugging
    await page.screenshot({ path: 'test-results/wasm-diagnostic.png' });
    
    // Check console logs
    const logs = await page.evaluate(() => {
      return (window as any).consoleErrors || [];
    });
    console.log('Console errors:', logs);
  });

  test('should check WASM file accessibility', async ({ page }) => {
    // Test WASM file accessibility
    const jsResponse = await page.request.get('http://localhost:8080/examples/pkg/radix_leptos_examples.js');
    expect(jsResponse.status()).toBe(200);
    
    const wasmResponse = await page.request.get('http://localhost:8080/examples/pkg/radix_leptos_examples_bg.wasm');
    expect(wasmResponse.status()).toBe(200);
    
    console.log('WASM files are accessible');
  });
});
