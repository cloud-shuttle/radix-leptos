import { test, expect } from '@playwright/test';

test.describe('Cross-Browser Compatibility Testing', () => {
  test('Chrome Desktop', async ({ page }) => {
    await page.goto('http://localhost:8081/production-test.html');
    
    // Test bundle loading
    await page.click('text=Test Production Bundle');
    await expect(page.locator('text=Production bundle loaded successfully')).toBeVisible();
    
    // Test component functionality
    await page.click('text=Test Components');
    await expect(page.locator('text=Production components working perfectly')).toBeVisible();
  });

  test('Mobile Viewport', async ({ page }) => {
    // Set mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });
    
    await page.goto('http://localhost:8081/production-test.html');
    
    // Test bundle loading
    await page.click('text=Test Production Bundle');
    await expect(page.locator('text=Production bundle loaded successfully')).toBeVisible();
    
    // Test component functionality
    await page.click('text=Test Components');
    await expect(page.locator('text=Production components working perfectly')).toBeVisible();
  });
});
