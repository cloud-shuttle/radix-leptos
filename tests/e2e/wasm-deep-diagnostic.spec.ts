import { test, expect } from '@playwright/test';

test.describe('WASM Deep Diagnostic Tests', () => {
  test('should debug WASM mounting step by step', async ({ page }) => {
    // Navigate to the demo
    await page.goto('http://localhost:8080/examples/real-wasm-demo.html');
    
    // Wait for page to load
    await expect(page.locator('h1')).toContainText('Radix-Leptos Real WASM Demo');
    
    // Check initial state
    console.log('=== INITIAL STATE ===');
    console.log('leptos-app content:', await page.locator('#leptos-app').innerHTML());
    console.log('leptos-app children count:', await page.locator('#leptos-app').locator('*').count());
    
    // Wait for WASM to load
    await page.waitForFunction(() => window.leptosReady === true, { timeout: 10000 });
    
    console.log('=== AFTER WASM LOAD ===');
    console.log('leptos-app content:', await page.locator('#leptos-app').innerHTML());
    console.log('leptos-app children count:', await page.locator('#leptos-app').locator('*').count());
    
    // Check if there are any elements with specific classes
    const hasSpaceY6 = await page.locator('#leptos-app .space-y-6').count() > 0;
    const hasGrid = await page.locator('#leptos-app .grid').count() > 0;
    const hasButton = await page.locator('#leptos-app button').count() > 0;
    
    console.log('Has space-y-6 class:', hasSpaceY6);
    console.log('Has grid class:', hasGrid);
    console.log('Has button element:', hasButton);
    
    // Check for any div elements
    const divCount = await page.locator('#leptos-app div').count();
    console.log('Div count in leptos-app:', divCount);
    
    // Check for any text content
    const textContent = await page.locator('#leptos-app').textContent();
    console.log('Text content:', textContent);
    
    // Check if the container is completely empty
    const isEmpty = await page.locator('#leptos-app').innerHTML() === '';
    console.log('Container is empty:', isEmpty);
    
    // Take a screenshot
    await page.screenshot({ path: 'test-results/wasm-deep-diagnostic.png' });
    
    // Check console logs for any errors
    const logs = await page.evaluate(() => {
      return (window as any).consoleErrors || [];
    });
    console.log('Console errors:', logs);
    
    // Check if WASM functions are available
    const wasmFunctions = await page.evaluate(() => {
      return {
        mount_real_demo: typeof (window as any).mount_real_demo,
        leptosReady: (window as any).leptosReady,
        wasmLoaded: (window as any).wasmLoaded
      };
    });
    console.log('WASM functions available:', wasmFunctions);
  });

  test('should test manual WASM mounting', async ({ page }) => {
    await page.goto('http://localhost:8080/examples/real-wasm-demo.html');
    
    // Wait for WASM to load
    await page.waitForFunction(() => window.leptosReady === true, { timeout: 10000 });
    
    // Try to manually call the mount function
    const result = await page.evaluate(() => {
      try {
        if (typeof (window as any).mount_real_demo === 'function') {
          (window as any).mount_real_demo();
          return { success: true, error: null };
        } else {
          return { success: false, error: 'mount_real_demo function not found' };
        }
      } catch (e) {
        return { success: false, error: e.message };
      }
    });
    
    console.log('Manual mount result:', result);
    
    // Check if components are now rendered
    const hasComponents = await page.locator('#leptos-app .space-y-6').count() > 0;
    console.log('Components rendered after manual mount:', hasComponents);
  });
});
