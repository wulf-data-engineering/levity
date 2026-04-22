---
description: Instructions and assets for adding Web Assembly (WASM) integrations.
---

# Adding Web Assembly (WASM)

## Dependencies

### Frontend Dependencies

Add `concurrently` to `frontend/package.json` under `devDependencies`. Figure out the latest version using `npm info concurrently version`:
```json
    "concurrently": "^x.x.x",
```

Add the following command to `scripts` in `frontend/package.json` to enable continuous compilation of the WASM file locally:

```json
    "dev:wasm": "cargo watch -w ../frontend-wasm/<your-wasm-project>/src -s 'cd ../frontend-wasm/<your-wasm-project> && wasm-pack build --target web'",
```
And replace the standard `dev` script:
```json
    "dev": "concurrently \"npm run dev:wasm\" \"vite dev\"",
```
*(Adjust the script appropriately if you have multiple wasm projects, or rename the `<your-wasm-project>` placeholder)*.

## Frontend Configuration

Update `frontend/vite.config.ts` to allow Vite to serve files from outside the immediate frontend root (to correctly import from `../frontend-wasm`):

```typescript
	server: {
		fs: {
			allow: ['..']
		},
```

## Infrastructure Settings

Modify the frontend local build step so that it triggers WASM compilation before building the website.
In `infrastructure/lib/constructs/frontend/deployment.ts`, update `tryBundle` to execute `wasm-pack`:
```typescript
    tryBundle(outputDir: string, options: cdk.aws_s3_deployment.BundlingOptions): boolean {
      try {
        childProcess.execSync('wasm-pack build --target web', {
            cwd: path.join(__dirname, '../../../../../../frontend-wasm/<your-wasm-project>'),
            stdio: 'inherit',
        });
        
        childProcess.execSync('npm run build', {
          cwd: path.join(__dirname, '../../../../../../frontend'),
          stdio: 'inherit',
        });
```

*(Note: You must adapt the path parameter `<your-wasm-project>` depending on what your wasm component is called).*

## CI/CD Workflows

### Pull Requests & Continuous Deployment

Modify both `.github/workflows/pull-request.yml` and `.github/workflows/continuous-deployment.yml` to hash the `frontend-wasm/` directory alongside `frontend/` so that WASM changes accurately trigger a build or bust the cache.

Look for the `hashFiles` command for the frontend and update it:
```yaml
          hashFiles('frontend/**', 'frontend-wasm/**')
```

Before the `Test Frontend` or `Build Frontend` steps (specifically *before* you run `npm ci`), insert steps to setup `wasm-pack`, cache the Rust artifacts, and build the WASM source.

**Important**: In `continuous-deployment.yml`, the frontend build step checks `if: steps.cache.outputs.cache-hit != 'true'`. You MUST place the new actions *after* the `actions/cache@v5` step so they can also use this `if` condition correctly to skip building when cached. In `pull-request.yml`, you can omit the `if` condition since checks always run.

Look up the latest tags for the GitHub actions using `gh api repos/jetli/wasm-pack-action/tags --jq '.[0].name'` and `gh api repos/Swatinem/rust-cache/tags --jq '.[0].name'`.

```yaml
      - uses: jetli/wasm-pack-action@<latest-version>
        if: steps.cache.outputs.cache-hit != 'true' # <-- Use the actual id of the cache step
        with:
          version: 'latest'
          
      - name: Rust Cache
        if: steps.cache.outputs.cache-hit != 'true'
        uses: Swatinem/rust-cache@<latest-version>
        with:
          workspaces: frontend-wasm/<your-wasm-project>
          
      - name: Build WASM
        if: steps.cache.outputs.cache-hit != 'true'
        working-directory: frontend-wasm/<your-wasm-project>
        run: wasm-pack build --target web
```

## Implementation Examples

You need a WASM Rust project within `frontend-wasm/` and a corresponding frontend. 

Follow the examples located in `assets/examples/` to see how the connection is made:
- `assets/examples/frontend-wasm/`: An example high-performance Rust WASM project setup (`markdown-parser`), along with the root README.
- `assets/examples/frontend-markdown-route`: An example Svelte page showcasing how to natively load, bind to, and trigger the compiled WASM logic `onMount`.
