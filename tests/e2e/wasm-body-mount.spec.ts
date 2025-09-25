import { test, expect } from '@playwright/test';

test.describe('WASM Body Mount Tests', () => {
  test('should check if components are mounted to document body', async ({ page }) => {
    await page.goto('http://localhost:8080/examples/real-wasm-demo.html');
    
    // Call the function directly after import
    const result = await page.evaluate(async () => {
      try {
        // Import the module
        const module = await import('./pkg/radix_leptos_examples.js');
        await module.default(); // Initialize
        
        console.log('Calling mount_real_demo...');
        module.mount_real_demo();
        
        // Check if components are rendered in the document body
        const body = document.body;
        const hasSpaceY6 = body.querySelector('.space-y-6') !== null;
        const hasGrid = body.querySelector('.grid') !== null;
        const hasButton = body.querySelector('button') !== null;
        const hasComponents = hasSpaceY6 || hasGrid || hasButton;
        
        // Also check the leptos-app container
        const appContainer = document.getElementById('leptos-app');
        const appHasContent = appContainer && appContainer.innerHTML.trim() !== '';
        
        return {
          success: true,
          bodyHasComponents: hasComponents,
          bodyHasSpaceY6: hasSpaceY6,
          bodyHasGrid: hasGrid,
          bodyHasButton: hasButton,
          appHasContent,
          bodyHTML: body.innerHTML.substring(0, 500) // First 500 chars
        };
      } catch (e) {
        return {
          success: false,
          error: e.message
        };
      }
    });
    
    console.log('Body mount result:', result);
    
    expect(result.success).toBe(true);
    expect(result.bodyHasComponents).toBe(true);
    expect(result.bodyHasSpaceY6).toBe(true);
  });
});
