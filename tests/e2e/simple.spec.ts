import { test, expect } from '@playwright/test';

test('Simple test', async ({ page }) => {
  await page.goto('http://localhost:8080/production-test.html');
  await expect(page.locator('h1')).toContainText('Radix-Leptos Production Test');
});
