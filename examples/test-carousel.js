const puppeteer = require('puppeteer');

async function testCarousel() {
    console.log('Starting carousel test...');
    
    const browser = await puppeteer.launch({ 
        headless: false,
        args: ['--no-sandbox', '--disable-setuid-sandbox']
    });
    
    try {
        const page = await browser.newPage();
        
        // Set longer timeout
        page.setDefaultTimeout(30000);
        
        console.log('Navigating to carousel examples...');
        await page.goto('http://localhost:8080/carousel_examples.html');
        
        // Wait for WASM to load
        console.log('Waiting for WASM to load...');
        await page.waitForFunction(() => {
            return !document.querySelector('.loading') || 
                   !document.querySelector('.spinner') ||
                   document.querySelector('.carousel-container');
        }, { timeout: 30000 });
        
        console.log('WASM loaded, analyzing page...');
        
        // Take a screenshot
        await page.screenshot({ path: 'carousel-test.png', fullPage: true });
        
        // Check for carousel elements
        const carouselElements = await page.$$('[class*="carousel"]');
        console.log(`Found ${carouselElements.length} carousel elements`);
        
        // Check for buttons
        const buttons = await page.$$('button');
        console.log(`Found ${buttons.length} buttons`);
        
        // List button texts
        for (let i = 0; i < buttons.length; i++) {
            const buttonText = await page.evaluate(el => {
                return el.textContent || el.getAttribute('aria-label') || 'No text';
            }, buttons[i]);
            console.log(`Button ${i}: "${buttonText}"`);
        }
        
        // Check for images
        const images = await page.$$('img');
        console.log(`Found ${images.length} images`);
        
        // Get page content
        const pageContent = await page.content();
        console.log('Page content preview:', pageContent.substring(0, 500));
        
        // Check for any errors in console
        const errors = await page.evaluate(() => {
            return window.consoleErrors || [];
        });
        
        if (errors.length > 0) {
            console.log('Console errors:', errors);
        } else {
            console.log('No console errors found');
        }
        
        // Test carousel navigation if buttons exist
        if (buttons.length > 0) {
            console.log('Testing carousel navigation...');
            
            // Look for next/previous buttons
            const nextButton = await page.$('button[aria-label*="Next"]');
            const prevButton = await page.$('button[aria-label*="Previous"]');
            
            if (nextButton) {
                console.log('Clicking next button...');
                await nextButton.click();
                await page.waitForTimeout(1000);
                await page.screenshot({ path: 'carousel-after-next.png', fullPage: true });
            }
            
            if (prevButton) {
                console.log('Clicking previous button...');
                await prevButton.click();
                await page.waitForTimeout(1000);
                await page.screenshot({ path: 'carousel-after-prev.png', fullPage: true });
            }
        }
        
        console.log('Carousel test completed successfully!');
        
    } catch (error) {
        console.error('Test failed:', error);
    } finally {
        await browser.close();
    }
}

testCarousel();
