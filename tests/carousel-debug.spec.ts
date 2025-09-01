import { test, expect } from '@playwright/test';

test.describe('Carousel Debug Tests', () => {
  test('Carousel Detailed Debug', async ({ page }) => {
    await page.goto('http://localhost:8080/carousel_examples.html');
    
    // Set longer timeout for WASM loading
    page.setDefaultTimeout(30000);
    
    console.log('Page loaded, waiting for WASM...');
    
    // Wait for WASM to load
    await page.waitForFunction(() => {
      return !document.querySelector('.animate-spin');
    });
    
    console.log('WASM loaded, checking carousel elements...');
    
    // Take a screenshot to see what's rendered
    await page.screenshot({ path: 'carousel-debug.png', fullPage: true });
    
    // Check if carousel container exists
    const carouselContainer = page.locator('.carousel-container');
    const containerExists = await carouselContainer.count();
    console.log(`Carousel container count: ${containerExists}`);
    
    if (containerExists > 0) {
      console.log('Carousel container found, checking visibility...');
      await expect(carouselContainer).toBeVisible();
      
      // Check for slides
      const slides = page.locator('.carousel-slide');
      const slideCount = await slides.count();
      console.log(`Slide count: ${slideCount}`);
      
      // Check for navigation buttons
      const nextButton = page.locator('button[aria-label="Next slide"]');
      const prevButton = page.locator('button[aria-label="Previous slide"]');
      
      const nextExists = await nextButton.count();
      const prevExists = await prevButton.count();
      console.log(`Next button count: ${nextExists}`);
      console.log(`Prev button count: ${prevExists}`);
      
      if (nextExists > 0) {
        console.log('Testing next button...');
        await nextButton.click();
        await page.waitForTimeout(1000);
        await page.screenshot({ path: 'carousel-after-next.png', fullPage: true });
      }
      
      if (prevExists > 0) {
        console.log('Testing prev button...');
        await prevButton.click();
        await page.waitForTimeout(1000);
        await page.screenshot({ path: 'carousel-after-prev.png', fullPage: true });
      }
      
      // Check for indicators
      const indicators = page.locator('.carousel-indicator');
      const indicatorCount = await indicators.count();
      console.log(`Indicator count: ${indicatorCount}`);
      
      if (indicatorCount > 0) {
        console.log('Testing indicator click...');
        await indicators.nth(1).click();
        await page.waitForTimeout(1000);
        await page.screenshot({ path: 'carousel-after-indicator.png', fullPage: true });
      }
    } else {
      console.log('Carousel container not found, checking page content...');
      
      // Get page content to see what's actually rendered
      const pageContent = await page.content();
      console.log('Page content preview:', pageContent.substring(0, 1000));
      
      // Check for any carousel-related elements
      const carouselElements = page.locator('[class*="carousel"]');
      const carouselElementCount = await carouselElements.count();
      console.log(`Elements with "carousel" in class: ${carouselElementCount}`);
      
      // Check for any buttons
      const buttons = page.locator('button');
      const buttonCount = await buttons.count();
      console.log(`Total button count: ${buttonCount}`);
      
      // List all button texts
      for (let i = 0; i < buttonCount; i++) {
        const buttonText = await buttons.nth(i).textContent();
        console.log(`Button ${i}: "${buttonText}"`);
      }
    }
    
    // Check console for errors
    const errors = await page.evaluate(() => {
      return window.consoleErrors || [];
    });
    
    if (errors.length > 0) {
      console.log('Console errors found:', errors);
    } else {
      console.log('No console errors found');
    }
  });
});
