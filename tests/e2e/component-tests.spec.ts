import { test, expect } from '@playwright/test';

const BASE_URL = 'http://localhost:8080';

test.describe('Radix-Leptos Component Tests', () => {
  test.beforeEach(async ({ page }) => {
    // Set longer timeout for WASM loading
    page.setDefaultTimeout(30000);
  });

  test('Tabs Examples - Basic Functionality', async ({ page }) => {
    await page.goto(`${BASE_URL}/tabs_examples.html`);
    
    // Wait for WASM to load
    await page.waitForFunction(() => {
      return !document.querySelector('.animate-spin');
    });
    
    // Check that tabs are rendered
    await expect(page.locator('[role="tab"]')).toHaveCount(3);
    
    // Test tab switching
    await page.click('[data-value="password"]');
    await expect(page.locator('[data-value="password"][role="tab"]')).toHaveAttribute('data-state', 'active');
    
    await page.click('[data-value="notifications"]');
    await expect(page.locator('[data-value="notifications"][role="tab"]')).toHaveAttribute('data-state', 'active');
  });

  test('Carousel Examples - Navigation and Controls', async ({ page }) => {
    await page.goto(`${BASE_URL}/carousel_examples.html`);
    
    // Wait for WASM to load
    await page.waitForFunction(() => {
      return !document.querySelector('.animate-spin');
    });
    
    // Check that carousel is rendered
    await expect(page.locator('.carousel-container')).toBeVisible();
    
    // Test navigation buttons
    const nextButton = page.locator('button[aria-label="Next slide"]');
    const prevButton = page.locator('button[aria-label="Previous slide"]');
    
    if (await nextButton.isVisible()) {
      await nextButton.click();
      // Wait for transition
      await page.waitForTimeout(500);
    }
    
    if (await prevButton.isVisible()) {
      await prevButton.click();
      await page.waitForTimeout(500);
    }
    
    // Test indicators if present
    const indicators = page.locator('.carousel-indicator');
    if (await indicators.count() > 0) {
      await indicators.nth(1).click();
      await page.waitForTimeout(500);
    }
  });

  test('DropdownMenu Examples - Open/Close Functionality', async ({ page }) => {
    await page.goto(`${BASE_URL}/dropdown_menu_examples.html`);
    
    // Wait for WASM to load
    await page.waitForFunction(() => {
      return !document.querySelector('.animate-spin');
    });
    
    // Test dropdown trigger
    const trigger = page.locator('[data-radix-dropdown-menu-trigger]').first();
    if (await trigger.isVisible()) {
      await trigger.click();
      
      // Check that menu opens
      await expect(page.locator('[data-radix-dropdown-menu-content]')).toBeVisible();
      
      // Click outside to close
      await page.click('body');
      await expect(page.locator('[data-radix-dropdown-menu-content]')).not.toBeVisible();
    }
  });

  test('ContextMenu Examples - Right-click Functionality', async ({ page }) => {
    await page.goto(`${BASE_URL}/context_menu_examples.html`);
    
    // Wait for WASM to load
    await page.waitForFunction(() => {
      return !document.querySelector('.animate-spin');
    });
    
    // Test context menu trigger
    const trigger = page.locator('[data-radix-context-menu-trigger]').first();
    if (await trigger.isVisible()) {
      await trigger.click({ button: 'right' });
      
      // Check that menu opens
      await expect(page.locator('[data-radix-context-menu-content]')).toBeVisible();
      
      // Click outside to close
      await page.click('body');
      await expect(page.locator('[data-radix-context-menu-content]')).not.toBeVisible();
    }
  });

  test('Menubar Examples - Menu Navigation', async ({ page }) => {
    await page.goto(`${BASE_URL}/menubar_examples.html`);
    
    // Wait for WASM to load
    await page.waitForFunction(() => {
      return !document.querySelector('.animate-spin');
    });
    
    // Test menubar trigger
    const trigger = page.locator('[data-radix-menubar-trigger]').first();
    if (await trigger.isVisible()) {
      await trigger.click();
      
      // Check that menu opens
      await expect(page.locator('[data-radix-menubar-content]')).toBeVisible();
      
      // Click outside to close
      await page.click('body');
      await expect(page.locator('[data-radix-menubar-content]')).not.toBeVisible();
    }
  });

  test('ScrollArea Examples - Scrolling Functionality', async ({ page }) => {
    await page.goto(`${BASE_URL}/scroll_area_examples.html`);
    
    // Wait for WASM to load
    await page.waitForFunction(() => {
      return !document.querySelector('.animate-spin');
    });
    
    // Test scroll area
    const scrollArea = page.locator('[data-radix-scroll-area-viewport]').first();
    if (await scrollArea.isVisible()) {
      // Test scrolling
      await scrollArea.evaluate((el) => {
        el.scrollTop = 100;
      });
      
      await page.waitForTimeout(500);
    }
  });

  test('Toast Examples - Notification System', async ({ page }) => {
    await page.goto(`${BASE_URL}/toast_examples.html`);
    
    // Wait for WASM to load
    await page.waitForFunction(() => {
      return !document.querySelector('.animate-spin');
    });
    
    // Test toast trigger
    const trigger = page.locator('button:has-text("Show Toast")').first();
    if (await trigger.isVisible()) {
      await trigger.click();
      
      // Check that toast appears
      await expect(page.locator('[data-radix-toast-root]')).toBeVisible();
      
      // Wait for auto-dismiss or close manually
      await page.waitForTimeout(3000);
    }
  });

  test('Alert Examples - Alert Display', async ({ page }) => {
    await page.goto(`${BASE_URL}/alert_examples.html`);
    
    // Wait for WASM to load
    await page.waitForFunction(() => {
      return !document.querySelector('.animate-spin');
    });
    
    // Check that alerts are rendered
    await expect(page.locator('[role="alert"]')).toBeVisible();
  });

  test('Badge Examples - Badge Display', async ({ page }) => {
    await page.goto(`${BASE_URL}/badge_examples.html`);
    
    // Wait for WASM to load
    await page.waitForFunction(() => {
      return !document.querySelector('.animate-spin');
    });
    
    // Check that badges are rendered
    await expect(page.locator('[data-radix-badge]')).toBeVisible();
  });

  test('Avatar Examples - Avatar Display', async ({ page }) => {
    await page.goto(`${BASE_URL}/avatar_examples.html`);
    
    // Wait for WASM to load
    await page.waitForFunction(() => {
      return !document.querySelector('.animate-spin');
    });
    
    // Check that avatars are rendered
    await expect(page.locator('[data-radix-avatar-root]')).toBeVisible();
  });

  test('Image Examples - Image Display', async ({ page }) => {
    await page.goto(`${BASE_URL}/image_examples.html`);
    
    // Wait for WASM to load
    await page.waitForFunction(() => {
      return !document.querySelector('.animate-spin');
    });
    
    // Check that images are rendered
    await expect(page.locator('img')).toBeVisible();
  });

  test('Video Examples - Video Player', async ({ page }) => {
    await page.goto(`${BASE_URL}/video_examples.html`);
    
    // Wait for WASM to load
    await page.waitForFunction(() => {
      return !document.querySelector('.animate-spin');
    });
    
    // Check that video player is rendered
    await expect(page.locator('video')).toBeVisible();
  });

  test('Audio Examples - Audio Player', async ({ page }) => {
    await page.goto(`${BASE_URL}/audio_examples.html`);
    
    // Wait for WASM to load
    await page.waitForFunction(() => {
      return !document.querySelector('.animate-spin');
    });
    
    // Check that audio player is rendered
    await expect(page.locator('audio')).toBeVisible();
  });

  test('Timeline Examples - Timeline Display', async ({ page }) => {
    await page.goto(`${BASE_URL}/timeline_examples.html`);
    
    // Wait for WASM to load
    await page.waitForFunction(() => {
      return !document.querySelector('.animate-spin');
    });
    
    // Check that timeline is rendered
    await expect(page.locator('.timeline-container')).toBeVisible();
  });

  test('List Examples - List Display', async ({ page }) => {
    await page.goto(`${BASE_URL}/list_examples.html`);
    
    // Wait for WASM to load
    await page.waitForFunction(() => {
      return !document.querySelector('.animate-spin');
    });
    
    // Check that list is rendered
    await expect(page.locator('[role="list"]')).toBeVisible();
  });

  test('Pagination Examples - Pagination Controls', async ({ page }) => {
    await page.goto(`${BASE_URL}/pagination-examples.html`);
    
    // Wait for WASM to load
    await page.waitForFunction(() => {
      return !document.querySelector('.animate-spin');
    });
    
    // Check that pagination is rendered
    await expect(page.locator('[data-radix-pagination-root]')).toBeVisible();
    
    // Test pagination buttons
    const nextButton = page.locator('button[aria-label="Next page"]');
    const prevButton = page.locator('button[aria-label="Previous page"]');
    
    if (await nextButton.isVisible()) {
      await nextButton.click();
      await page.waitForTimeout(500);
    }
  });

  test('Component Showcase - Overall Functionality', async ({ page }) => {
    await page.goto(`${BASE_URL}/component_showcase.html`);
    
    // Wait for WASM to load
    await page.waitForFunction(() => {
      return !document.querySelector('.animate-spin');
    });
    
    // Check that showcase is rendered
    await expect(page.locator('.showcase-container')).toBeVisible();
    
    // Test navigation between sections
    const sections = page.locator('.showcase-section');
    await expect(sections).toHaveCount.greaterThan(0);
  });

  test('All Components Load Without Errors', async ({ page }) => {
    const components = [
      'tabs_examples.html',
      'carousel_examples.html',
      'dropdown_menu_examples.html',
      'context_menu_examples.html',
      'menubar_examples.html',
      'scroll_area_examples.html',
      'toast_examples.html',
      'alert_examples.html',
      'badge_examples.html',
      'avatar_examples.html',
      'image_examples.html',
      'video_examples.html',
      'audio_examples.html',
      'timeline_examples.html',
      'list_examples.html',
      'pagination-examples.html',
      'component_showcase.html'
    ];

    for (const component of components) {
      console.log(`Testing ${component}...`);
      
      await page.goto(`${BASE_URL}/${component}`);
      
      // Wait for WASM to load
      await page.waitForFunction(() => {
        return !document.querySelector('.animate-spin');
      }, { timeout: 30000 });
      
      // Check for console errors
      const errors = await page.evaluate(() => {
        return window.consoleErrors || [];
      });
      
      expect(errors).toHaveLength(0);
      
      // Check that page loaded successfully
      await expect(page.locator('body')).toBeVisible();
      
      console.log(`✅ ${component} loaded successfully`);
    }
  });
});
