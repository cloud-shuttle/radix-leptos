// WASM Loader for Radix-Leptos Examples
let wasmModule = null;

async function loadWasm() {
    try {
        console.log('🔄 Loading optimized WASM bundle...');
        
        // Load the WASM file
        const response = await fetch('./radix_leptos_examples_optimized.wasm');
        if (!response.ok) {
            throw new Error(`Failed to load WASM: ${response.status} ${response.statusText}`);
        }
        
        const wasmBuffer = await response.arrayBuffer();
        console.log(`✅ WASM loaded: ${(wasmBuffer.byteLength / (1024 * 1024)).toFixed(2)}MB`);
        
        // Initialize wasm-bindgen
        const wasmBindgen = await import('https://cdn.jsdelivr.net/npm/@wasm-bindgen/wasm-bindgen@0.2.100/+esm');
        
        // Initialize the WASM module
        wasmModule = await wasmBindgen.default('./radix_leptos_examples_optimized.wasm');
        
        console.log('✅ WASM module initialized successfully!');
        console.log('Available functions:', Object.keys(wasmModule));
        
        // Make functions globally available
        window.test_function = wasmModule.test_function;
        window.start_pagination_examples = wasmModule.start_pagination_examples;
        
        return true;
        
    } catch (error) {
        console.error('❌ Failed to load WASM:', error);
        return false;
    }
}

// Auto-load when the script is loaded
loadWasm().then(success => {
    if (success) {
        console.log('🚀 WASM loader ready!');
        // Update the page status
        if (window.updateStatus) {
            window.updateStatus('✅ WASM bundle loaded successfully!', 'success');
        }
    } else {
        console.error('❌ WASM loading failed');
        if (window.updateStatus) {
            window.updateStatus('❌ WASM loading failed', 'error');
        }
    }
});

// Export for use in other scripts
window.loadWasm = loadWasm;
window.wasmModule = wasmModule;
