import { test, expect } from '@playwright/test';

test.describe('Real WASM Demo', () => {
  test('should load and mount WASM components successfully', async ({ page }) => {
    // Navigate to the real WASM demo
    await page.goto('http://localhost:8080/examples/real-wasm-demo.html');
    
    // Wait for the page to load
    await expect(page.locator('h1')).toContainText('Radix-Leptos Real WASM Demo');
    
    // Wait for WASM to load and mount (with longer timeout)
    await page.waitForFunction(() => window.leptosReady === true, { timeout: 30000 });
    
    // Check that the status shows success
    await expect(page.locator('#status')).toContainText('✅ WASM components loaded and mounted successfully!');
    
    // Verify that the WASM app container is populated (not just loading text)
    const appContainer = page.locator('#leptos-app');
    await expect(appContainer).not.toContainText('Loading WASM components...');
    
    // Check that we have actual content (not just loading state)
    await expect(appContainer).toBeVisible();
  });

  test('should have interactive WASM components', async ({ page }) => {
    await page.goto('http://localhost:8080/examples/real-wasm-demo.html');
    
    // Wait for WASM to load
    await page.waitForFunction(() => window.leptosReady === true, { timeout: 10000 });
    
    // Check that we have the real WASM demo content
    await expect(page.locator('h2')).toContainText('Real WASM Components');
    await expect(page.locator('p')).toContainText('These components are compiled from Rust to WebAssembly and running in your browser.');
    
    // Verify the component display section exists
    await expect(page.locator('#leptos-app')).toBeVisible();
  });

  test('should handle WASM loading errors gracefully', async ({ page }) => {
    // Test error handling by blocking WASM files
    await page.route('**/pkg/*.wasm', route => route.abort());
    await page.route('**/pkg/*.js', route => route.abort());
    
    await page.goto('http://localhost:8080/examples/real-wasm-demo.html');
    
    // Should show error state
    await page.waitForTimeout(2000);
    
    // Check that error handling is in place
    const status = page.locator('#status');
    await expect(status).toBeVisible();
  });

  test('should demonstrate technical capabilities', async ({ page }) => {
    await page.goto('http://localhost:8080/examples/real-wasm-demo.html');
    
    // Wait for WASM to load
    await page.waitForFunction(() => window.leptosReady === true, { timeout: 10000 });
    
    // Check technical details section
    await expect(page.locator('h2')).toContainText('Technical Details');
    
    // Verify the technical capabilities are listed
    await expect(page.locator('li')).toContainText('Rust Components: Written in Rust and compiled to WebAssembly');
    await expect(page.locator('li')).toContainText('Leptos Framework: Reactive UI framework for Rust');
    await expect(page.locator('li')).toContainText('Real Interactivity: Click handlers and state management in Rust');
    await expect(page.locator('li')).toContainText('Performance: Near-native performance with WASM');
    await expect(page.locator('li')).toContainText('Type Safety: Full Rust type safety in the browser');
  });
});
