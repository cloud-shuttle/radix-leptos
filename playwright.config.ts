import { defineConfig, devices } from '@playwright/test';

export default defineConfig({
  testDir: './tests',
  fullyParallel: false,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 1,
  workers: 1,
  reporter: 'html',
  timeout: 90000,
  expect: {
    timeout: 15000,
  },
  use: {
    baseURL: 'http://localhost:8080',
    trace: 'on-first-retry',
    screenshot: 'only-on-failure',
    video: 'retain-on-failure',
    actionTimeout: 20000,
    navigationTimeout: 60000,
  },
  projects: [
    {
      name: 'chromium',
      use: { ...devices['Desktop Chrome'] },
    },
    // Temporarily disabled other browsers for stability
  ],
  // webServer disabled - using manual development server
  // webServer: {
  //   command: 'python3 -m http.server 8080',
  //   url: 'http://localhost:8080',
  //   reuseExistingServer: !process.env.CI,
  //   timeout: 180 * 1000,
  // },
  preserveOutput: 'failures-only',
});
