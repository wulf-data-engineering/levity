import tailwindcss from '@tailwindcss/vite';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vitest/config';
import { loadEnv } from 'vite';
import protoPlugin from './vite-plugin-protobuf';
import { svelteTesting } from '@testing-library/svelte/vite';
import { paraglideVitePlugin } from '@inlang/paraglide-js';

export default defineConfig(({ mode }) => {
	const env = loadEnv(mode, '../', '');
	process.env.VITE_COGNITO_LOCAL_PORT = env.COGNITO_LOCAL_PORT;

	return {
		plugins: [
		tailwindcss(), 
		paraglideVitePlugin({
			project: './project.inlang',
			outdir: './src/lib/paraglide'
		}),
		sveltekit(), 
		svelteTesting(), 
		protoPlugin()
	],
	test: {
		expect: { requireAssertions: true },
		projects: [
			{
				extends: './vite.config.ts',
				test: {
					name: 'server',
					environment: 'jsdom',
					include: ['src/**/*.{test,spec}.{js,ts}'],
					exclude: ['src/**/*.svelte.{test,spec}.{js,ts}'],
					setupFiles: ['./vitest-setup.js']
				}
			}
		]
	},
	server: {
		port: parseInt(env.FRONTEND_PORT || '5173'),
		proxy: {
			// On dev redirect "/api/..." to cargo lambda watch "/lambda-url/.../"
			'/api': {
				target: `http://localhost:${env.BACKEND_PORT || '9000'}`,
				changeOrigin: true,
				secure: false,
				rewrite: (path) => {
					const withoutApi = path.replace(/^\/api/, '');
					return `/lambda-url${withoutApi}/`;
				}
			}
		}
	}
	};
});
