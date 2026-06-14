import { defineConfig, devices } from '@playwright/test';
import dotenv from 'dotenv';

dotenv.config({ path: new URL('../.env', import.meta.url).pathname, quiet: true });

const chrome = {
	name: 'chromium',
	use: { ...devices['Desktop Chrome'] }
};

const allProjects = [
	chrome,
	{
		name: 'firefox',
		use: { ...devices['Desktop Firefox'] }
	},
	{
		name: 'webkit',
		use: { ...devices['Desktop Safari'] }
	}
];

export default defineConfig({
	testDir: './e2e',

	/* Run tests in files in parallel */
	fullyParallel: true,
	/* Fail the build on CI if you accidentally left test.only in the source code. */
	forbidOnly: !!process.env.CI,
	/* Retry on CI only */
	retries: process.env.CI ? 2 : 0,
	/* Opt out of parallel tests on CI. */
	workers: process.env.CI ? 1 : undefined,
	/* Reporter to use. See https://playwright.dev/docs/test-reporters */
	reporter: 'html',
	/* Shared settings for all the projects below. See https://playwright.dev/docs/api/class-testoptions. */
	use: {
		/* Collect trace when retrying the failed test. See https://playwright.dev/docs/trace-viewer */
		trace: 'on-first-retry',
		locale: 'en-US',
		baseURL: `http://127.0.0.1:${process.env.FRONTEND_PORT || '5173'}`
	},
	/* Configure projects for major browsers */
	projects: process.env.CI ? allProjects : [chrome],
	/* On CI, the server will already be running. */
	webServer: process.env.CI
		? undefined
		: {
				command: 'npm run dev',
				port: parseInt(process.env.FRONTEND_PORT || '5173'),
				reuseExistingServer: true
			}
});
