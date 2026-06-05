import { expect, test } from '@playwright/test';
import { signUpAndSignIn } from './utils/auth';

test('User Profile', async ({ page }) => {
	await signUpAndSignIn(page);

	await page.goto('/account');
	await page.waitForSelector('[data-hydrated="true"]');

	await expect(page.getByText('Hello, Test User!')).toBeVisible();
});
