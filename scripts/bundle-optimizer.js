#!/usr/bin/env node

/**
 * Bundle Size Optimizer for Radix-Leptos
 * 
 * This script helps optimize bundle size to meet the <400KB target
 * by analyzing and suggesting optimizations.
 */

const fs = require('fs');
const path = require('path');

// Configuration
const TARGET_BUNDLE_SIZE = 400 * 1024; // 400KB in bytes
const CURRENT_BUNDLE_SIZE = 550974; // Current size from bundle-size-report.json

console.log('🚀 Radix-Leptos Bundle Size Optimizer');
console.log('=====================================\n');

// Read current bundle size report
function readBundleReport() {
    try {
        const reportPath = path.join(__dirname, '..', 'bundle-size-report.json');
        const report = JSON.parse(fs.readFileSync(reportPath, 'utf8'));
        return report;
    } catch (error) {
        console.error('❌ Could not read bundle-size-report.json');
        return null;
    }
}

// Analyze bundle composition
function analyzeBundle(report) {
    if (!report) return;

    console.log('📊 Current Bundle Analysis:');
    console.log('---------------------------');
    
    const totalSize = report.totalSize;
    const wasmSize = report.analysis.find(item => item.type === 'wasm')?.size || 0;
    const jsSize = report.analysis.find(item => item.type === 'js')?.size || 0;
    
    console.log(`Total Bundle Size: ${(totalSize / 1024).toFixed(2)} KB`);
    console.log(`WASM Size: ${(wasmSize / 1024).toFixed(2)} KB (${((wasmSize / totalSize) * 100).toFixed(1)}%)`);
    console.log(`JS Size: ${(jsSize / 1024).toFixed(2)} KB (${((jsSize / totalSize) * 100).toFixed(1)}%)`);
    
    const sizeDifference = totalSize - TARGET_BUNDLE_SIZE;
    if (sizeDifference > 0) {
        console.log(`\n⚠️  Bundle size exceeds target by ${(sizeDifference / 1024).toFixed(2)} KB`);
    } else {
        console.log(`\n✅ Bundle size is within target (${Math.abs(sizeDifference / 1024).toFixed(2)} KB under)`);
    }
    
    return { totalSize, wasmSize, jsSize, sizeDifference };
}

// Generate optimization suggestions
function generateOptimizations(analysis) {
    console.log('\n🔧 Optimization Suggestions:');
    console.log('-----------------------------');
    
    const suggestions = [];
    
    if (analysis.sizeDifference > 0) {
        // WASM optimizations
        if (analysis.wasmSize > TARGET_BUNDLE_SIZE * 0.8) {
            suggestions.push({
                category: 'WASM Optimization',
                priority: 'High',
                suggestions: [
                    'Enable WASM optimization flags in Cargo.toml',
                    'Use wasm-opt for additional size reduction',
                    'Remove unused dependencies from Cargo.toml',
                    'Enable dead code elimination',
                    'Use feature flags to exclude unused components'
                ]
            });
        }
        
        // JS optimizations
        if (analysis.jsSize > 50000) { // 50KB
            suggestions.push({
                category: 'JavaScript Optimization',
                priority: 'Medium',
                suggestions: [
                    'Minify JavaScript output',
                    'Remove console.log statements in production',
                    'Use tree shaking to eliminate unused code',
                    'Split code into smaller chunks',
                    'Use dynamic imports for heavy components'
                ]
            });
        }
        
        // General optimizations
        suggestions.push({
            category: 'General Optimization',
            priority: 'High',
            suggestions: [
                'Enable release optimizations in wasm-pack',
                'Use --release flag for production builds',
                'Remove debug symbols from WASM',
                'Compress assets with gzip/brotli',
                'Implement code splitting for large components'
            ]
        });
    }
    
    // Display suggestions
    suggestions.forEach((suggestion, index) => {
        console.log(`\n${index + 1}. ${suggestion.category} (${suggestion.priority} Priority):`);
        suggestion.suggestions.forEach((item, itemIndex) => {
            console.log(`   ${itemIndex + 1}. ${item}`);
        });
    });
    
    return suggestions;
}

// Generate Cargo.toml optimization recommendations
function generateCargoOptimizations() {
    console.log('\n📝 Cargo.toml Optimization Recommendations:');
    console.log('-------------------------------------------');
    
    const optimizations = [
        {
            file: 'Cargo.toml',
            section: '[profile.release]',
            additions: [
                'opt-level = "z"  # Optimize for size',
                'lto = true       # Link-time optimization',
                'codegen-units = 1  # Single codegen unit for better optimization',
                'panic = "abort"  # Smaller panic handling',
                'strip = true     # Remove debug symbols'
            ]
        },
        {
            file: 'examples/Cargo.toml',
            section: '[profile.release]',
            additions: [
                'opt-level = "z"',
                'lto = true',
                'codegen-units = 1',
                'panic = "abort"',
                'strip = true'
            ]
        }
    ];
    
    optimizations.forEach(opt => {
        console.log(`\n${opt.file}:`);
        console.log(`${opt.section}`);
        opt.additions.forEach(addition => {
            console.log(`${addition}`);
        });
    });
}

// Generate wasm-pack optimization commands
function generateWasmPackOptimizations() {
    console.log('\n🛠️  wasm-pack Optimization Commands:');
    console.log('------------------------------------');
    
    const commands = [
        {
            description: 'Production build with size optimization',
            command: 'wasm-pack build --target web --release -- --features wee_alloc'
        },
        {
            description: 'Build with additional size optimizations',
            command: 'wasm-pack build --target web --release -- --features wee_alloc,console_error_panic_hook'
        },
        {
            description: 'Post-build optimization with wasm-opt',
            command: 'wasm-opt -Oz pkg/radix_leptos_examples_bg.wasm -o pkg/radix_leptos_examples_bg.wasm'
        }
    ];
    
    commands.forEach((cmd, index) => {
        console.log(`\n${index + 1}. ${cmd.description}:`);
        console.log(`   ${cmd.command}`);
    });
}

// Generate component-specific optimizations
function generateComponentOptimizations() {
    console.log('\n🧩 Component-Specific Optimizations:');
    console.log('-------------------------------------');
    
    const optimizations = [
        {
            component: 'Heavy Components',
            suggestions: [
                'Implement lazy loading for Dialog, Sheet, and Popover',
                'Use dynamic imports for chart components',
                'Split large components into smaller, focused components',
                'Remove unused variants and props'
            ]
        },
        {
            component: 'Form Components',
            suggestions: [
                'Combine similar form components',
                'Use shared validation logic',
                'Minimize prop interfaces',
                'Remove unused form variants'
            ]
        },
        {
            component: 'Layout Components',
            suggestions: [
                'Use CSS-in-Rust for smaller bundle size',
                'Remove unused layout variants',
                'Combine similar layout components',
                'Use CSS custom properties instead of inline styles'
            ]
        }
    ];
    
    optimizations.forEach(opt => {
        console.log(`\n${opt.component}:`);
        opt.suggestions.forEach((suggestion, index) => {
            console.log(`   ${index + 1}. ${suggestion}`);
        });
    });
}

// Generate monitoring script
function generateMonitoringScript() {
    console.log('\n📊 Bundle Size Monitoring:');
    console.log('---------------------------');
    
    const monitoringScript = `#!/bin/bash

# Bundle Size Monitoring Script
echo "📦 Monitoring bundle size..."

# Build with optimizations
cd examples
wasm-pack build --target web --release -- --features wee_alloc

# Check bundle size
node ../scripts/bundle-size-monitor.js

# Alert if size exceeds target
BUNDLE_SIZE=$(cat ../bundle-size-report.json | jq '.totalSize')
TARGET_SIZE=409600  # 400KB in bytes

if [ $BUNDLE_SIZE -gt $TARGET_SIZE ]; then
    echo "⚠️  Bundle size exceeds target!"
    echo "Current: $((BUNDLE_SIZE / 1024))KB"
    echo "Target: $((TARGET_SIZE / 1024))KB"
    echo "Excess: $(((BUNDLE_SIZE - TARGET_SIZE) / 1024))KB"
    exit 1
else
    echo "✅ Bundle size within target!"
    echo "Current: $((BUNDLE_SIZE / 1024))KB"
    echo "Target: $((TARGET_SIZE / 1024))KB"
    echo "Under by: $(((TARGET_SIZE - BUNDLE_SIZE) / 1024))KB"
fi`;

    const scriptPath = path.join(__dirname, 'monitor-bundle-size.sh');
    fs.writeFileSync(scriptPath, monitoringScript);
    fs.chmodSync(scriptPath, '755');
    
    console.log('Created monitoring script: scripts/monitor-bundle-size.sh');
    console.log('Run with: ./scripts/monitor-bundle-size.sh');
}

// Main execution
function main() {
    const report = readBundleReport();
    const analysis = analyzeBundle(report);
    
    if (analysis) {
        const suggestions = generateOptimizations(analysis);
        generateCargoOptimizations();
        generateWasmPackOptimizations();
        generateComponentOptimizations();
        generateMonitoringScript();
        
        console.log('\n🎯 Next Steps:');
        console.log('---------------');
        console.log('1. Apply Cargo.toml optimizations');
        console.log('2. Update wasm-pack build commands');
        console.log('3. Implement component-specific optimizations');
        console.log('4. Set up bundle size monitoring');
        console.log('5. Run: make bundle-size-check');
        
        console.log('\n📈 Expected Results:');
        console.log('--------------------');
        console.log(`• Target bundle size: ${(TARGET_BUNDLE_SIZE / 1024).toFixed(0)}KB`);
        console.log(`• Current bundle size: ${(analysis.totalSize / 1024).toFixed(0)}KB`);
        console.log(`• Potential reduction: ${(analysis.sizeDifference / 1024).toFixed(0)}KB`);
        
        if (analysis.sizeDifference <= 0) {
            console.log('\n🎉 Bundle size is already within target!');
        } else {
            console.log('\n⚡ Optimization needed to meet target');
        }
    }
}

// Run the optimizer
main();
