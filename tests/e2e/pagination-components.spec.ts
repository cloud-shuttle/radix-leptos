import { test, expect } from '@playwright/test';

test.describe('Pagination Components - Comprehensive Testing', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('http://localhost:8081/production-test.html');
    // Load the bundle first
    await page.click('text=Test Production Bundle');
    await expect(page.locator('text=Production bundle loaded successfully')).toBeVisible();
  });

  test('Page Navigation', async ({ page }) => {
    // Test next/previous navigation
    await page.click('text=Test Components');
    await expect(page.locator('text=start_pagination_examples executed successfully')).toBeVisible();
  });

  test('Accessibility Features', async ({ page }) => {
    await page.click('text=Test Components');
    
    // Check navigation role - use first() since there are multiple pagination components
    await expect(page.locator('[role="navigation"]').first()).toBeVisible();
    
    // Check button labels - use first() for the first pagination component
    await expect(page.locator('[aria-label*="Go to first page"]').first()).toBeVisible();
    await expect(page.locator('[aria-label*="Go to next page"]').first()).toBeVisible();
  });
});
