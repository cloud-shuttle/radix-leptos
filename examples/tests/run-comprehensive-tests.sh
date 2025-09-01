#!/bin/bash
echo "🧪 Radix-Leptos Test Runner"
echo "Running comprehensive tests..."
npx playwright test --config=../playwright.config.ts --reporter=html
