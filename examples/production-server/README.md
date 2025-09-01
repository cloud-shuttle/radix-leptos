# Radix-Leptos Examples - Production Build

This is the production build of the Radix-Leptos component library.

## Files

- `radix_leptos_examples.js` - JavaScript bindings
- `radix_leptos_examples_bg.wasm` - Optimized WebAssembly binary
- `radix_leptos_examples.d.ts` - TypeScript definitions

## Usage

```html
<script type="module">
  import init, { start_component_name } from './radix_leptos_examples.js';
  
  async function run() {
    await init();
    start_component_name();
  }
  
  run();
</script>
```

## Performance

- Optimized for production use
- LTO enabled for smaller bundle size
- Native CPU optimizations
- SIMD and bulk memory enabled
