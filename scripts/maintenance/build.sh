#!/bin/bash

# Build script for Radix-Leptos examples
echo "🔨 Building Radix-Leptos examples..."

# Build the examples package
echo "📦 Building examples package..."
cargo build --package radix-leptos-examples --target wasm32-unknown-unknown

# Check if wasm-bindgen-cli is installed
if ! command -v wasm-bindgen &> /dev/null; then
    echo "⚠️  wasm-bindgen-cli not found. Installing..."
    cargo install wasm-bindgen-cli
fi

# Create pkg directory
mkdir -p pkg

# Generate JavaScript bindings
echo "🔧 Generating JavaScript bindings..."
wasm-bindgen ../target/wasm32-unknown-unknown/debug/radix_leptos_examples.wasm \
    --out-dir pkg \
    --target web \
    --no-typescript

echo "✅ Build complete! You can now open examples/index.html in a browser."
echo "🌐 To serve the files, run: python3 -m http.server 8000"
echo "📱 Then visit: http://localhost:8000/examples/"
