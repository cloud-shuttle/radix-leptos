import { test, expect } from '@playwright/test';

test.describe('WASM Comprehensive Tests', () => {
  test('should verify complete WASM demo functionality', async ({ page }) => {
    await page.goto('http://localhost:8080/real-wasm-demo.html');
    
    // Wait for page to load
    await expect(page.locator('h1')).toContainText('Radix-Leptos Real WASM Demo');
    
    // Wait for WASM to load
    await page.waitForFunction(() => window.leptosReady === true, { timeout: 10000 });
    
    // Check that the status shows success
    await expect(page.locator('#status')).toContainText('✅ WASM components loaded and mounted successfully!');
    
    // Verify WASM components are rendered in the document body
    const hasComponents = await page.evaluate(() => {
      const body = document.body;
      const hasSpaceY6 = body.querySelector('.space-y-6') !== null;
      const hasGrid = body.querySelector('.grid') !== null;
      const hasButton = body.querySelector('button') !== null;
      const hasRealWasmDemo = body.querySelector('h3')?.textContent?.includes('Real WASM Components') || false;
      
      return {
        hasSpaceY6,
        hasGrid,
        hasButton,
        hasRealWasmDemo,
        componentCount: body.querySelectorAll('div').length
      };
    });
    
    console.log('Component verification:', hasComponents);
    
    // Verify components are present
    expect(hasComponents.hasSpaceY6).toBe(true);
    expect(hasComponents.hasGrid).toBe(true);
    expect(hasComponents.hasButton).toBe(true);
    expect(hasComponents.hasRealWasmDemo).toBe(true);
    
    // Test button interaction
    const button = page.locator('button').first();
    await expect(button).toBeVisible();
    await expect(button).toContainText('Click me (Rust/WASM)');
    
    // Test button click (should log to console)
    const consoleLogs: string[] = [];
    page.on('console', msg => {
      if (msg.type() === 'log') {
        consoleLogs.push(msg.text());
      }
    });
    
    await button.click();
    
    // Wait a moment for the click to register
    await page.waitForTimeout(100);
    
    // Check if the button click was logged
    const hasButtonClickLog = consoleLogs.some(log => log.includes('Button clicked from Rust!'));
    expect(hasButtonClickLog).toBe(true);
  });

  test('should verify WASM components are interactive', async ({ page }) => {
    await page.goto('http://localhost:8080/real-wasm-demo.html');
    
    // Wait for WASM to load
    await page.waitForFunction(() => window.leptosReady === true, { timeout: 10000 });
    
    // Check that we have the expected component structure
    const componentStructure = await page.evaluate(() => {
      const body = document.body;
      const spaceY6 = body.querySelector('.space-y-6');
      const grid = body.querySelector('.grid');
      const buttons = body.querySelectorAll('button');
      const badges = body.querySelectorAll('.inline-block');
      
      return {
        hasMainContainer: spaceY6 !== null,
        hasGrid: grid !== null,
        buttonCount: buttons.length,
        badgeCount: badges.length,
        hasButtonText: Array.from(buttons).some(btn => btn.textContent?.includes('Click me (Rust/WASM)')),
        hasBadgeText: Array.from(badges).some(badge => badge.textContent?.includes('Rust Badge'))
      };
    });
    
    console.log('Component structure:', componentStructure);
    
    expect(componentStructure.hasMainContainer).toBe(true);
    expect(componentStructure.hasGrid).toBe(true);
    expect(componentStructure.buttonCount).toBeGreaterThan(0);
    expect(componentStructure.badgeCount).toBeGreaterThan(0);
    expect(componentStructure.hasButtonText).toBe(true);
    expect(componentStructure.hasBadgeText).toBe(true);
  });

  test('should verify WASM loading and mounting process', async ({ page }) => {
    await page.goto('http://localhost:8080/real-wasm-demo.html');
    
    // Check initial state
    await expect(page.locator('#status')).toContainText('Initializing...');
    
    // Wait for WASM to load
    await page.waitForFunction(() => window.leptosReady === true, { timeout: 10000 });
    
    // Check final state
    await expect(page.locator('#status')).toContainText('✅ WASM components loaded and mounted successfully!');
    
    // Verify WASM functions are available
    const wasmFunctions = await page.evaluate(() => {
      return {
        leptosReady: (window as any).leptosReady,
        wasmLoaded: (window as any).wasmLoaded,
        hasMountFunction: typeof (window as any).mount_real_demo === 'function'
      };
    });
    
    expect(wasmFunctions.leptosReady).toBe(true);
    expect(wasmFunctions.wasmLoaded).toBe(true);
    expect(wasmFunctions.hasMountFunction).toBe(true);
  });
});
