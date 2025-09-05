#!/usr/bin/env node

/**
 * Bundle Size Monitor for Radix-Leptos
 * Monitors and reports on bundle sizes for performance optimization
 */

const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

class BundleSizeMonitor {
    constructor() {
        this.results = [];
        this.thresholds = {
            wasm: 1024 * 1024, // 1MB
            js: 500 * 1024,    // 500KB
            css: 100 * 1024,   // 100KB
            total: 2 * 1024 * 1024 // 2MB
        };
    }

    /**
     * Get file size in bytes
     */
    getFileSize(filePath) {
        try {
            const stats = fs.statSync(filePath);
            return stats.size;
        } catch (error) {
            console.warn(`Warning: Could not get size for ${filePath}: ${error.message}`);
            return 0;
        }
    }

    /**
     * Format bytes to human readable format
     */
    formatBytes(bytes) {
        if (bytes === 0) return '0 Bytes';
        const k = 1024;
        const sizes = ['Bytes', 'KB', 'MB', 'GB'];
        const i = Math.floor(Math.log(bytes) / Math.log(k));
        return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
    }

    /**
     * Analyze bundle files
     */
    analyzeBundles() {
        const bundlePaths = [
            'examples/pkg/radix_leptos_examples_bg.wasm',
            'examples/pkg/radix_leptos_examples.js',
            'examples/pkg/radix_leptos_examples_optimized_bg.wasm',
            'examples/pkg/radix_leptos_examples_optimized.js',
            'examples/pkg/radix_leptos_examples_optimized.wasm',
            'production-server/radix_leptos_examples_bg.wasm',
            'production-server/radix_leptos_examples.js'
        ];

        console.log('🔍 Analyzing bundle sizes...\n');

        let totalSize = 0;
        const analysis = [];

        bundlePaths.forEach(filePath => {
            if (fs.existsSync(filePath)) {
                const size = this.getFileSize(filePath);
                const ext = path.extname(filePath);
                const type = this.getFileType(ext);
                const threshold = this.thresholds[type] || this.thresholds.total;
                
                totalSize += size;
                
                const result = {
                    file: filePath,
                    size: size,
                    formattedSize: this.formatBytes(size),
                    type: type,
                    threshold: threshold,
                    exceedsThreshold: size > threshold,
                    status: size > threshold ? '❌ EXCEEDS' : '✅ OK'
                };

                analysis.push(result);
                
                console.log(`${result.status} ${result.file}`);
                console.log(`   Size: ${result.formattedSize} (${type.toUpperCase()})`);
                if (result.exceedsThreshold) {
                    console.log(`   ⚠️  Exceeds threshold: ${this.formatBytes(threshold)}`);
                }
                console.log('');
            }
        });

        // Summary
        console.log('📊 Bundle Size Summary:');
        console.log(`Total Size: ${this.formatBytes(totalSize)}`);
        console.log(`Total Threshold: ${this.formatBytes(this.thresholds.total)}`);
        console.log(`Status: ${totalSize > this.thresholds.total ? '❌ EXCEEDS TOTAL THRESHOLD' : '✅ WITHIN LIMITS'}\n`);

        // Save results
        this.saveResults(analysis, totalSize);
        
        return {
            analysis,
            totalSize,
            withinLimits: totalSize <= this.thresholds.total
        };
    }

    /**
     * Get file type based on extension
     */
    getFileType(ext) {
        switch (ext) {
            case '.wasm':
                return 'wasm';
            case '.js':
                return 'js';
            case '.css':
                return 'css';
            default:
                return 'other';
        }
    }

    /**
     * Save results to file
     */
    saveResults(analysis, totalSize) {
        const timestamp = new Date().toISOString();
        const results = {
            timestamp,
            totalSize,
            withinLimits: totalSize <= this.thresholds.total,
            thresholds: this.thresholds,
            analysis
        };

        const outputPath = 'bundle-size-report.json';
        fs.writeFileSync(outputPath, JSON.stringify(results, null, 2));
        console.log(`📄 Results saved to: ${outputPath}`);
    }

    /**
     * Generate bundle size report
     */
    generateReport() {
        console.log('🚀 Radix-Leptos Bundle Size Monitor\n');
        console.log('=' .repeat(50));
        
        const results = this.analyzeBundles();
        
        if (!results.withinLimits) {
            console.log('⚠️  Bundle size exceeds recommended limits!');
            console.log('Consider optimizing:');
            console.log('- Remove unused dependencies');
            console.log('- Enable WASM optimization');
            console.log('- Use code splitting');
            console.log('- Compress assets');
            process.exit(1);
        } else {
            console.log('✅ Bundle sizes are within recommended limits!');
        }
    }

    /**
     * Monitor bundle size changes
     */
    monitorChanges() {
        const reportPath = 'bundle-size-report.json';
        
        if (fs.existsSync(reportPath)) {
            const previousReport = JSON.parse(fs.readFileSync(reportPath, 'utf8'));
            const currentResults = this.analyzeBundles();
            
            const sizeDiff = currentResults.totalSize - previousReport.totalSize;
            const percentChange = (sizeDiff / previousReport.totalSize) * 100;
            
            console.log('\n📈 Bundle Size Changes:');
            console.log(`Previous: ${this.formatBytes(previousReport.totalSize)}`);
            console.log(`Current: ${this.formatBytes(currentResults.totalSize)}`);
            console.log(`Change: ${sizeDiff >= 0 ? '+' : ''}${this.formatBytes(Math.abs(sizeDiff))} (${percentChange.toFixed(2)}%)`);
            
            if (Math.abs(percentChange) > 10) {
                console.log('⚠️  Significant bundle size change detected!');
            }
        }
    }
}

// CLI usage
if (require.main === module) {
    const monitor = new BundleSizeMonitor();
    const command = process.argv[2];
    
    switch (command) {
        case 'analyze':
            monitor.analyzeBundles();
            break;
        case 'monitor':
            monitor.monitorChanges();
            break;
        case 'report':
        default:
            monitor.generateReport();
            break;
    }
}

module.exports = BundleSizeMonitor;

