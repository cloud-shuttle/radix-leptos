import { test, expect } from '@playwright/test';

test.describe('Radix-Leptos Comprehensive Test Suite', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('http://localhost:8081/production-test.html');
  });

  test('Bundle Loading and Performance', async ({ page }) => {
    // Test bundle loading
    await page.click('text=Test Production Bundle');
    await expect(page.locator('text=Production bundle loaded successfully')).toBeVisible();
    
    // Verify bundle size information
    await expect(page.locator('#bundle-size')).toContainText('538KB');
  });

  test('Component Functionality', async ({ page }) => {
    // Load the bundle first
    await page.click('text=Test Production Bundle');
    await expect(page.locator('text=Production bundle loaded successfully')).toBeVisible();
    
    // Test component functionality
    await page.click('text=Test Components');
    await expect(page.locator('text=Production components working perfectly')).toBeVisible();
  });

  test('Performance Metrics', async ({ page }) => {
    // Load bundle and measure performance
    await page.click('text=Test Production Bundle');
    await expect(page.locator('text=Production bundle loaded successfully')).toBeVisible();
    
    // Test performance measurement
    await page.click('text=Measure Performance');
    await expect(page.locator('text=Performance measurement completed')).toBeVisible();
  });
});
