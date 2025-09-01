import { test, expect } from '@playwright/test';

test.describe('Performance and Stress Testing', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('http://localhost:8081/production-test.html');
  });

  test('Bundle Loading Performance', async ({ page }) => {
    const startTime = performance.now();
    
    // Click to load the bundle
    await page.click('text=Test Production Bundle');
    
    // Wait for bundle to be fully loaded
    await expect(page.locator('text=Production bundle loaded successfully')).toBeVisible();
    
    const endTime = performance.now();
    const loadTime = endTime - startTime;
    
    // Bundle should load in under 5 seconds
    expect(loadTime).toBeLessThan(5000);
    
    console.log(`Bundle load time: ${loadTime.toFixed(2)}ms`);
  });

  test('Memory Usage During Bundle Load', async ({ page }) => {
    // Load the bundle
    await page.click('text=Test Production Bundle');
    await expect(page.locator('text=Production bundle loaded successfully')).toBeVisible();
    
    // Test performance measurement
    await page.click('text=Measure Performance');
    await expect(page.locator('text=Performance measurement completed')).toBeVisible();
  });
});
