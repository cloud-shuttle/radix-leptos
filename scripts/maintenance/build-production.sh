#!/bin/bash

# Production Build Script for Radix-Leptos
echo "🚀 Building Radix-Leptos for Production..."

# Set production environment
export RUSTFLAGS="-C target-cpu=native -C target-feature=+crt-static"
export CARGO_PROFILE_RELEASE_OPT_LEVEL=3
export CARGO_PROFILE_RELEASE_LTO=true
export CARGO_PROFILE_RELEASE_PANIC="abort"

# Clean previous builds
echo "🧹 Cleaning previous builds..."
cargo clean --package radix-leptos-examples
rm -rf pkg/

# Build with production optimizations
echo "🔨 Building with production optimizations..."
cargo build --release --package radix-leptos-examples --target wasm32-unknown-unknown

# Check if wasm-bindgen-cli is installed
if ! command -v wasm-bindgen &> /dev/null; then
    echo "⚠️  wasm-bindgen-cli not found. Installing..."
    cargo install wasm-bindgen-cli
fi

# Create pkg directory
mkdir -p pkg

# Generate optimized JavaScript bindings
echo "🔧 Generating optimized JavaScript bindings..."
wasm-bindgen ../target/wasm32-unknown-unknown/release/radix_leptos_examples.wasm \
    --out-dir pkg \
    --target web \
    --no-typescript \
    --no-demangle

# Optimize WASM with wasm-opt if available
if command -v wasm-opt &> /dev/null; then
    echo "⚡ Optimizing WASM with wasm-opt..."
    wasm-opt pkg/radix_leptos_examples_bg.wasm \
        -O4 \
        --enable-bulk-memory \
        --enable-reference-types \
        --enable-simd \
        --enable-threads \
        --enable-exception-handling \
        -o pkg/radix_leptos_examples_bg.wasm
else
    echo "⚠️  wasm-opt not found. Install with: cargo install wasm-opt"
fi

# Generate production package.json
echo "📦 Generating production package.json..."
cat > pkg/package.json << EOF
{
  "name": "radix-leptos-examples",
  "version": "0.1.0",
  "description": "Production build of Radix-Leptos components",
  "main": "radix_leptos_examples.js",
  "files": [
    "radix_leptos_examples.js",
    "radix_leptos_examples_bg.wasm",
    "radix_leptos_examples.d.ts"
  ],
  "keywords": ["radix", "leptos", "rust", "wasm", "ui-components"],
  "author": "Your Name",
  "license": "MIT"
}
EOF

# Generate production README
echo "📚 Generating production README..."
cat > pkg/README.md << EOF
# Radix-Leptos Examples - Production Build

This is the production build of the Radix-Leptos component library.

## Files

- \`radix_leptos_examples.js\` - JavaScript bindings
- \`radix_leptos_examples_bg.wasm\` - Optimized WebAssembly binary
- \`radix_leptos_examples.d.ts\` - TypeScript definitions

## Usage

\`\`\`html
<script type="module">
  import init, { start_component_name } from './radix_leptos_examples.js';
  
  async function run() {
    await init();
    start_component_name();
  }
  
  run();
</script>
\`\`\`

## Performance

- Optimized for production use
- LTO enabled for smaller bundle size
- Native CPU optimizations
- SIMD and bulk memory enabled
EOF

# Calculate bundle sizes
echo "📊 Bundle Size Analysis:"
JS_SIZE=$(stat -f%z pkg/radix_leptos_examples.js)
WASM_SIZE=$(stat -f%z pkg/radix_leptos_examples_bg.wasm)
TOTAL_SIZE=$((JS_SIZE + WASM_SIZE))

echo "  JavaScript: $((JS_SIZE / 1024))KB"
echo "  WASM: $((WASM_SIZE / 1024 / 1024))MB"
echo "  Total: $((TOTAL_SIZE / 1024 / 1024))MB"

# Generate production deployment guide
echo "📋 Generating production deployment guide..."
cat > PRODUCTION_DEPLOYMENT.md << EOF
# Production Deployment Guide

## 🚀 Quick Deploy

1. **Copy the \`pkg/\` directory** to your production server
2. **Serve with any static file server** (nginx, Apache, CDN)
3. **Ensure CORS is configured** for cross-origin requests
4. **Set up caching headers** for optimal performance

## 🌐 CDN Deployment

Upload the \`pkg/\` contents to your CDN:
- Cloudflare
- AWS CloudFront
- Google Cloud CDN
- Azure CDN

## 📱 Performance Optimization

- **Enable gzip compression** for JavaScript files
- **Set aggressive caching** for WASM files
- **Use HTTP/2** for parallel loading
- **Implement service worker** for offline support

## 🔧 Caching Headers

\`\`\`nginx
# JavaScript files - cache for 1 year
location ~* \\.js$ {
    expires 1y;
    add_header Cache-Control "public, immutable";
}

# WASM files - cache for 1 year
location ~* \\.wasm$ {
    expires 1y;
    add_header Cache-Control "public, immutable";
    add_header Content-Type "application/wasm";
}
\`\`\`

## 📊 Monitoring

Monitor these metrics in production:
- WASM initialization time
- Component render time
- Memory usage
- Bundle load time
EOF

echo ""
echo "🎉 Production build complete!"
echo "📁 Production files are in: pkg/"
echo "📋 Deployment guide: PRODUCTION_DEPLOYMENT.md"
echo ""
echo "🚀 Next steps:"
echo "  1. Test the production build locally"
echo "  2. Upload pkg/ to your production server/CDN"
echo "  3. Update your HTML files to use production paths"
echo "  4. Monitor performance in production"
