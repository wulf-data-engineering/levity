import { expect, Page } from '@playwright/test';

export async function signUpAndSignIn(page: Page) {
  const testUserEmail = '@@cookiecutter.test_user_email@@';
  const now = new Date();
  const yy = String(now.getFullYear()).slice(-2);
  const mm = String(now.getMonth() + 1).padStart(2, '0');
  const dd = String(now.getDate()).padStart(2, '0');
  const hh = String(now.getHours()).padStart(2, '0');
  const min = String(now.getMinutes()).padStart(2, '0');
  const ss = String(now.getSeconds()).padStart(2, '0');
  const timestamp = `${yy}${mm}${dd}${hh}${min}${ss}`;
  const randomSuffix = Math.random().toString(36).substring(2, 5);
  const email = testUserEmail.replace('@', `+e2e-${timestamp}-${randomSuffix}@`);
  const password = '@@cookiecutter.test_user_password@@';

	await page.goto('/');
	await page.waitForSelector('[data-hydrated="true"]');
	await page.locator('#sign-up-link').click();

	// Sign Up
	await expect(page.locator('#sign-up-btn')).toBeVisible();
	await page.locator('#email').clear();
	await page.locator('#email').fill(email);
	await page.locator('#password').clear();
	await page.locator('#password').fill(password);
	await page.locator('#confirm').clear();
	await page.locator('#confirm').fill(password);
	await page.locator('#sign-up-btn').click();

	// Confirm Sign Up
	await expect(page.locator('#firstName')).toBeVisible();
	// OTP is prefilled in development
	await page.locator('#firstName').fill('Test');
	await page.locator('#lastName').fill('User');
	await page.locator('button[type=submit]').click();

	// Sign In (if not auto-signed in)
	try {
		await expect(page.locator('#sign-out-btn')).toBeVisible({ timeout: 5000 });
	} catch {
		await page.locator('#email').clear();
		await page.locator('#email').fill(email);
		await page.locator('#password').clear();
		await page.locator('#password').fill(password);
		await page.locator('#sign-in-btn').click();
		await expect(page.locator('#sign-out-btn')).toBeVisible();
	}

	return { email, password };
}
