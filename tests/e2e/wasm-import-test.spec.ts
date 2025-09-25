import { test, expect } from '@playwright/test';

test.describe('WASM Import Tests', () => {
  test('should test WASM module import directly', async ({ page }) => {
    await page.goto('http://localhost:8080/examples/real-wasm-demo.html');
    
    // Test the import directly
    const importResult = await page.evaluate(async () => {
      try {
        // Import the WASM module
        const module = await import('./pkg/radix_leptos_examples.js');
        console.log('Module imported:', module);
        console.log('Available exports:', Object.keys(module));
        console.log('mount_real_demo type:', typeof module.mount_real_demo);
        
        return {
          success: true,
          exports: Object.keys(module),
          mountRealDemoType: typeof module.mount_real_demo,
          mountRealDemoFunction: module.mount_real_demo
        };
      } catch (e) {
        return {
          success: false,
          error: e.message
        };
      }
    });
    
    console.log('Import result:', importResult);
    
    // Check if the function is available
    expect(importResult.success).toBe(true);
    expect(importResult.mountRealDemoType).toBe('function');
  });
  
  test('should test WASM module after init', async ({ page }) => {
    await page.goto('http://localhost:8080/examples/real-wasm-demo.html');
    
    // Wait for the page to load and check the module
    const moduleInfo = await page.evaluate(async () => {
      try {
        // Import and initialize the module
        const module = await import('./pkg/radix_leptos_examples.js');
        await module.default();
        
        return {
          success: true,
          exports: Object.keys(module),
          mountRealDemoType: typeof module.mount_real_demo,
          initType: typeof module.default
        };
      } catch (e) {
        return {
          success: false,
          error: e.message
        };
      }
    });
    
    console.log('Module info after init:', moduleInfo);
  });
});
