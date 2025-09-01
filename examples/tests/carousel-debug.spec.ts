import { test, expect } from "@playwright/test";

test("Carousel Debug", async ({ page }) => {
  await page.goto("http://localhost:8080/carousel_examples.html");
  console.log("Testing carousel...");
});
