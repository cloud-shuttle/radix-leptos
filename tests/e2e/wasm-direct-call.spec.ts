import { test, expect } from '@playwright/test';

test.describe('WASM Direct Call Tests', () => {
  test('should call mount_real_demo directly', async ({ page }) => {
    await page.goto('http://localhost:8080/examples/real-wasm-demo.html');
    
    // Call the function directly after import
    const result = await page.evaluate(async () => {
      try {
        // Import the module
        const module = await import('./pkg/radix_leptos_examples.js');
        await module.default(); // Initialize
        
        console.log('Calling mount_real_demo...');
        module.mount_real_demo();
        
        // Check if components are rendered
        const appContainer = document.getElementById('leptos-app');
        const hasContent = appContainer && appContainer.innerHTML.trim() !== '';
        const hasComponents = appContainer && appContainer.querySelector('.space-y-6') !== null;
        
        return {
          success: true,
          hasContent,
          hasComponents,
          innerHTML: appContainer ? appContainer.innerHTML : 'No container found'
        };
      } catch (e) {
        return {
          success: false,
          error: e.message
        };
      }
    });
    
    console.log('Direct call result:', result);
    
    expect(result.success).toBe(true);
    expect(result.hasContent).toBe(true);
    expect(result.hasComponents).toBe(true);
  });
});
