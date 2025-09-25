import { test, expect } from '@playwright/test';

test.describe('Simple Demo Test', () => {
  test('should load the demo page', async ({ page }) => {
    // Navigate to the real WASM demo
    await page.goto('http://localhost:8080/examples/real-wasm-demo.html');
    
    // Wait for the page to load
    await expect(page.locator('h1')).toContainText('Radix-Leptos Real WASM Demo');
    
    // Check that the page structure is correct
    await expect(page.locator('#leptos-app')).toBeVisible();
    await expect(page.locator('#status')).toBeVisible();
    
    // Check that WASM files are accessible
    const wasmResponse = await page.request.get('http://localhost:8080/examples/pkg/radix_leptos_examples.js');
    expect(wasmResponse.status()).toBe(200);
    
    const wasmFileResponse = await page.request.get('http://localhost:8080/examples/pkg/radix_leptos_examples_bg.wasm');
    expect(wasmFileResponse.status()).toBe(200);
  });

  test('should show loading state initially', async ({ page }) => {
    await page.goto('http://localhost:8080/examples/real-wasm-demo.html');
    
    // Check initial loading state
    await expect(page.locator('#leptos-app')).toContainText('Loading WASM components...');
    await expect(page.locator('#status')).toContainText('Initializing...');
  });
});
