import tailwindcss from '@tailwindcss/vite';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vitest/config';

export default defineConfig({
	plugins: [tailwindcss(), sveltekit()],
	test: {
		expect: { requireAssertions: true },
		projects: [
			{
				extends: './vite.config.ts',
				test: {
					name: 'server',
					environment: 'node',
					include: ['src/**/*.{test,spec}.{js,ts}'],
					exclude: ['src/**/*.svelte.{test,spec}.{js,ts}']
				}
			}
		]
	},
	server: {
		proxy: {
			// On dev redirect "/api/..." to cargo lambda watch "/lambda-url/.../"
			'/api': {
				target: 'http://localhost:9000',
				changeOrigin: true,
				secure: false,
				rewrite: (path) => {
					const withoutApi = path.replace(/^\/api/, '');
					const target = `/lambda-url${withoutApi}/`;
					return target;
				}
			}
		}
	}
});
