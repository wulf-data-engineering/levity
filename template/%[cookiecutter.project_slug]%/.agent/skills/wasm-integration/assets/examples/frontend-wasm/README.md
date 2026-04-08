# Frontend WASM Components

This directory contains Rust projects that are compiled to Web Assembly (WASM) and used natively by the SvelteKit frontend.

## Prerequisites

To build and hot-reload these WASM components locally, your system must have `wasm-pack` and `cargo-watch` installed:

```bash
cargo install wasm-pack cargo-watch
```

## How It Works

1. During local development, `npm run dev` kicks off `vite dev` and `cargo watch`. The watcher compiles any Rust changes automatically using `wasm-pack build --target web`.
2. The compiled outputs reside in the `pkg/` directory of each sub-project.
3. The SvelteKit frontend imports these compiled ES modules (like `import init, { ... } from '../../frontend-wasm/markdown-parser/pkg/markdown_parser.js'`) and invokes `init()` before using the WASM functions.
4. In Continuous Integration and AWS infrastructure local builds, `wasm-pack` is invoked explicitly prior to the Vite build to ensure the assets are up-to-date.
