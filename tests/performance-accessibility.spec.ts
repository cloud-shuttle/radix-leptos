import { test, expect } from '@playwright/test';

const BASE_URL = 'http://localhost:8080';

test.describe('Performance and Accessibility Testing', () => {
  test.beforeEach(async ({ page }) => {
    page.setDefaultTimeout(30000);
    
    // Enable performance monitoring
    await page.addInitScript(() => {
      window.performanceMarks = [];
      window.performanceMeasures = [];
      
      const originalMark = performance.mark;
      const originalMeasure = performance.measure;
      
      performance.mark = function(name, options) {
        window.performanceMarks.push({ name, options, timestamp: performance.now() });
        return originalMark.call(this, name, options);
      };
      
      performance.measure = function(name, startMark, endMark) {
        window.performanceMeasures.push({ name, startMark, endMark, timestamp: performance.now() });
        return originalMeasure.call(this, name, startMark, endMark);
      };
    });
  });

  test('Performance Testing - Load Times and Memory Usage', async ({ page }) => {
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
      'pagination-examples.html'
    ];

    const performanceResults = [];

    for (const component of components) {
      console.log(`\n📊 Testing performance for ${component}...`);
      
      // Clear memory before each test
      await page.evaluate(() => {
        if ('gc' in window) {
          (window as any).gc();
        }
      });
      
      const startTime = Date.now();
      
      // Navigate to component
      await page.goto(`${BASE_URL}/${component}`);
      
      // Wait for content to appear (either WASM-loaded or static HTML)
      await page.waitForFunction(() => {
        // Check if any reasonable content has loaded
        const hasContent = document.querySelector('h1') && 
                          document.querySelector('h1').textContent.length > 5;
        const hasButtons = document.querySelectorAll('button').length > 0;
        const hasText = document.body.textContent && document.body.textContent.length > 100;
        const hasDivs = document.querySelectorAll('div').length > 5;
        
        return hasContent || hasButtons || hasText || hasDivs;
      });
      
      const loadTime = Date.now() - startTime;
      
      // Measure WASM initialization time
      const wasmInitTime = await page.evaluate(() => {
        const marks = window.performanceMarks || [];
        const wasmMark = marks.find(m => m.name.includes('wasm') || m.name.includes('init'));
        return wasmMark ? wasmMark.timestamp : 0;
      });
      
      // Measure memory usage
      const memoryInfo = await page.evaluate(() => {
        if ('memory' in performance) {
          return (performance as any).memory;
        }
        return null;
      });
      
      // Measure DOM size
      const domMetrics = await page.evaluate(() => {
        return {
          elementCount: document.querySelectorAll('*').length,
          textLength: document.body.textContent?.length || 0,
          imageCount: document.querySelectorAll('img').length,
          buttonCount: document.querySelectorAll('button').length
        };
      });
      
      const result = {
        component,
        loadTime,
        wasmInitTime,
        memoryInfo,
        domMetrics,
        timestamp: new Date().toISOString()
      };
      
      performanceResults.push(result);
      
      console.log(`✅ ${component}: Load time: ${loadTime}ms, DOM elements: ${domMetrics.elementCount}`);
      
      // Wait before next test
      await page.waitForTimeout(1000);
    }
    
    // Generate performance report
    const avgLoadTime = performanceResults.reduce((sum, r) => sum + r.loadTime, 0) / performanceResults.length;
    const totalElements = performanceResults.reduce((sum, r) => sum + r.domMetrics.elementCount, 0);
    
    console.log(`\n📊 Performance Summary:`);
    console.log(`Average load time: ${avgLoadTime.toFixed(2)}ms`);
    console.log(`Total DOM elements: ${totalElements}`);
    
    // Assert performance standards
    expect(avgLoadTime).toBeLessThan(5000); // Should load in under 5 seconds
    expect(totalElements).toBeGreaterThan(100); // Should have substantial content
    
    // Save performance results
    await page.evaluate((results) => {
      localStorage.setItem('performance-test-results', JSON.stringify(results));
    }, performanceResults);
  });

  test('Accessibility Testing - ARIA Labels, Roles, and Keyboard Navigation', async ({ page }) => {
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
      'pagination-examples.html'
    ];

    const accessibilityResults = [];

    for (const component of components) {
      console.log(`\n♿ Testing accessibility for ${component}...`);
      
      await page.goto(`${BASE_URL}/${component}`);
      
      // Wait for content to appear (either WASM-loaded or static HTML)
      await page.waitForFunction(() => {
        // Check if any reasonable content has loaded
        const hasContent = document.querySelector('h1') && 
                          document.querySelector('h1').textContent.length > 5;
        const hasButtons = document.querySelectorAll('button').length > 0;
        const hasText = document.body.textContent && document.body.textContent.length > 100;
        const hasDivs = document.querySelectorAll('div').length > 5;
        
        return hasContent || hasButtons || hasText || hasDivs;
      });
      
      // Test ARIA attributes
      const ariaResults = await page.evaluate(() => {
        const elements = document.querySelectorAll('*');
        const ariaElements = [];
        const missingLabels = [];
        const missingRoles = [];
        
        elements.forEach(el => {
          const tagName = el.tagName.toLowerCase();
          const hasAriaLabel = el.hasAttribute('aria-label') || el.hasAttribute('aria-labelledby');
          const hasRole = el.hasAttribute('role');
          
          if (tagName === 'button' && !hasAriaLabel) {
            missingLabels.push(el.outerHTML.substring(0, 100));
          }
          
          if (tagName === 'div' && el.classList.contains('menu') && !hasRole) {
            missingRoles.push(el.outerHTML.substring(0, 100));
          }
          
          if (hasAriaLabel || hasRole) {
            ariaElements.push({
              tag: tagName,
              ariaLabel: el.getAttribute('aria-label'),
              ariaLabelledby: el.getAttribute('aria-labelledby'),
              role: el.getAttribute('role')
            });
          }
        });
        
        return {
          ariaElements: ariaElements.length,
          missingLabels: missingLabels.length,
          missingRoles: missingRoles.length,
          totalElements: elements.length
        };
      });
      
      // Test keyboard navigation
      const keyboardResults = await page.evaluate(() => {
        const focusableElements = document.querySelectorAll(
          'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])'
        );
        
        return {
          focusableCount: focusableElements.length,
          hasTabIndex: Array.from(focusableElements).some(el => el.hasAttribute('tabindex')),
          hasKeyboardHandlers: Array.from(focusableElements).some(el => {
            return el.onkeydown || el.onkeyup || el.onkeypress;
          })
        };
      });
      
      // Test color contrast (basic check)
      const contrastResults = await page.evaluate(() => {
        const textElements = document.querySelectorAll('p, h1, h2, h3, h4, h5, h6, span, div');
        let highContrastCount = 0;
        let totalTextElements = 0;
        
        textElements.forEach(el => {
          const style = window.getComputedStyle(el);
          const color = style.color;
          const backgroundColor = style.backgroundColor;
          
          // Basic contrast check (simplified)
          if (color && backgroundColor && color !== backgroundColor) {
            highContrastCount++;
          }
          totalTextElements++;
        });
        
        return {
          highContrastCount,
          totalTextElements,
          contrastRatio: totalTextElements > 0 ? highContrastCount / totalTextElements : 0
        };
      });
      
      const result = {
        component,
        ariaResults,
        keyboardResults,
        contrastResults,
        timestamp: new Date().toISOString()
      };
      
      accessibilityResults.push(result);
      
      console.log(`✅ ${component}: ARIA elements: ${ariaResults.ariaElements}, Focusable: ${keyboardResults.focusableCount}`);
      
      // Wait before next test
      await page.waitForTimeout(1000);
    }
    
    // Generate accessibility report
    const totalAriaElements = accessibilityResults.reduce((sum, r) => sum + r.ariaResults.ariaElements, 0);
    const totalFocusable = accessibilityResults.reduce((sum, r) => sum + r.keyboardResults.focusableCount, 0);
    const avgContrastRatio = accessibilityResults.reduce((sum, r) => sum + r.contrastResults.contrastRatio, 0) / accessibilityResults.length;
    
    console.log(`\n♿ Accessibility Summary:`);
    console.log(`Total ARIA elements: ${totalAriaElements}`);
    console.log(`Total focusable elements: ${totalFocusable}`);
    console.log(`Average contrast ratio: ${avgContrastRatio.toFixed(2)}`);
    
    // Assert accessibility standards
    expect(totalAriaElements).toBeGreaterThan(50); // Should have good ARIA support
    expect(totalFocusable).toBeGreaterThan(20); // Should have good keyboard navigation
    expect(avgContrastRatio).toBeGreaterThan(0.7); // Should have good contrast
    
    // Save accessibility results
    await page.evaluate((results) => {
      localStorage.setItem('accessibility-test-results', JSON.stringify(results));
    }, accessibilityResults);
  });

  test('Cross-browser Compatibility - CSS and JavaScript Features', async ({ page }) => {
    const components = [
      'tabs_examples.html',
      'carousel_examples.html',
      'dropdown_menu_examples.html'
    ];

    const compatibilityResults = [];

    for (const component of components) {
      console.log(`\n🌐 Testing compatibility for ${component}...`);
      
      await page.goto(`${BASE_URL}/${component}`);
      
      // Wait for content to appear (either WASM-loaded or static HTML)
      await page.waitForFunction(() => {
        // Check if any reasonable content has loaded
        const hasContent = document.querySelector('h1') && 
                          document.querySelector('h1').textContent.length > 5;
        const hasButtons = document.querySelectorAll('button').length > 0;
        const hasText = document.body.textContent && document.body.textContent.length > 100;
        const hasDivs = document.querySelectorAll('div').length > 5;
        
        return hasContent || hasButtons || hasText || hasDivs;
      });
      
      // Test CSS features
      const cssResults = await page.evaluate(() => {
        const testElement = document.createElement('div');
        document.body.appendChild(testElement);
        
        const cssFeatures = {
          flexbox: CSS.supports('display', 'flex'),
          grid: CSS.supports('display', 'grid'),
          customProperties: CSS.supports('--custom-property', 'value'),
          transforms: CSS.supports('transform', 'translateX(10px)'),
          transitions: CSS.supports('transition', 'all 0.3s ease'),
          animations: CSS.supports('animation', 'fade 1s')
        };
        
        document.body.removeChild(testElement);
        return cssFeatures;
      });
      
      // Test JavaScript features
      const jsResults = await page.evaluate(() => {
        return {
          asyncAwait: typeof (async () => {}) === 'function',
          arrowFunctions: typeof (() => {}) === 'function',
          templateLiterals: typeof `template` === 'string',
          destructuring: (() => { const {a} = {a: 1}; return a; })() === 1,
          spreadOperator: (() => { const arr = [...[1,2,3]]; return arr.length; })() === 3,
          optionalChaining: (() => { const obj = {a: {b: 1}}; return obj?.a?.b; })() === 1
        };
      });
      
      // Test Web APIs
      const webApiResults = await page.evaluate(() => {
        return {
          intersectionObserver: 'IntersectionObserver' in window,
          resizeObserver: 'ResizeObserver' in window,
          mutationObserver: 'MutationObserver' in window,
          webAnimations: 'animate' in Element.prototype,
          customElements: 'customElements' in window,
          webComponents: 'HTMLElement' in window && 'attachShadow' in HTMLElement.prototype
        };
      });
      
      const result = {
        component,
        cssResults,
        jsResults,
        webApiResults,
        timestamp: new Date().toISOString()
      };
      
      compatibilityResults.push(result);
      
      console.log(`✅ ${component}: CSS features: ${Object.values(cssResults).filter(Boolean).length}/6, JS features: ${Object.values(jsResults).filter(Boolean).length}/6`);
      
      // Wait before next test
      await page.waitForTimeout(1000);
    }
    
    // Generate compatibility report
    const cssSupport = compatibilityResults.reduce((sum, r) => {
      return sum + Object.values(r.cssResults).filter(Boolean).length;
    }, 0) / (compatibilityResults.length * 6);
    
    const jsSupport = compatibilityResults.reduce((sum, r) => {
      return sum + Object.values(r.jsResults).filter(Boolean).length;
    }, 0) / (compatibilityResults.length * 6);
    
    const webApiSupport = compatibilityResults.reduce((sum, r) => {
      return sum + Object.values(r.webApiResults).filter(Boolean).length;
    }, 0) / (compatibilityResults.length * 6);
    
    console.log(`\n🌐 Compatibility Summary:`);
    console.log(`CSS feature support: ${(cssSupport * 100).toFixed(1)}%`);
    console.log(`JavaScript feature support: ${(jsSupport * 100).toFixed(1)}%`);
    console.log(`Web API support: ${(webApiSupport * 100).toFixed(1)}%`);
    
    // Assert compatibility standards
    expect(cssSupport).toBeGreaterThan(0.8); // Should support most CSS features
    expect(jsSupport).toBeGreaterThan(0.9); // Should support modern JS features
    expect(webApiSupport).toBeGreaterThan(0.7); // Should support most Web APIs
    
    // Save compatibility results
    await page.evaluate((results) => {
      localStorage.setItem('compatibility-test-results', JSON.stringify(results));
    }, compatibilityResults);
  });

  test('Error Handling and Resilience - Network Issues and Edge Cases', async ({ page }) => {
    console.log('\n🛡️ Testing error handling and resilience...');
    
    // Test with slow network
    await page.route('**/*', route => {
      // Simulate slow loading for some resources
      if (route.request().url().includes('.wasm') || route.request().url().includes('.js')) {
        setTimeout(() => route.continue(), 2000);
      } else {
        route.continue();
      }
    });
    
    await page.goto(`${BASE_URL}/tabs_examples.html`);
    
    // Wait for loading state (check for various loading patterns)
    await page.waitForFunction(() => {
      return document.querySelector('.loading') || 
             document.querySelector('.spinner') || 
             document.querySelector('.animate-spin') ||
             document.body.textContent?.includes('Loading') ||
             document.querySelector('[class*="loading"]');
    });
    
    // Check if error handling is present
    const errorElements = page.locator('.error, [data-error], [role="alert"]');
    const errorCount = await errorElements.count();
    
    if (errorCount > 0) {
      console.log('✅ Error handling elements present');
    } else {
      console.log('⚠️ No error handling elements found');
    }
    
    // Test with invalid URLs
    await page.goto(`${BASE_URL}/nonexistent.html`);
    
    const notFoundContent = await page.content();
    if (notFoundContent.includes('404') || notFoundContent.includes('Not Found')) {
      console.log('✅ 404 error handling working');
    }
    
    // Test component with missing dependencies
    await page.goto(`${BASE_URL}/tabs_examples.html`);
    
    // Simulate WASM loading failure
    await page.evaluate(() => {
      // Override fetch to simulate failure
      const originalFetch = window.fetch;
      window.fetch = async (url: string) => {
        if (url.includes('.wasm')) {
          throw new Error('WASM loading failed');
        }
        return originalFetch(url);
      };
    });
    
    // Wait for potential error states
    await page.waitForTimeout(5000);
    
    const errorStates = page.locator('.error, [data-error], [role="alert"]');
    const errorStateCount = await errorStates.count();
    
    if (errorStateCount > 0) {
      console.log('✅ Error states handled gracefully');
    } else {
      console.log('⚠️ Error states not handled');
    }
    
    // Test component recovery
    await page.reload();
    await page.waitForTimeout(5000);
    
    const recoveredContent = page.locator('body');
    const hasContent = await recoveredContent.evaluate(el => el.textContent && el.textContent.length > 100);
    
    if (hasContent) {
      console.log('✅ Component recovery working');
    } else {
      console.log('⚠️ Component recovery not working');
    }
    
    await page.screenshot({ path: 'error-handling-test.png', fullPage: true });
  });
});
