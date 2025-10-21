import { expect, test } from '@playwright/test';

test('sign up', async ({ page }) => {
	// Given: the home page with sign in box

	const newEmail = 'test@wulf.technology'.replace('@', `+e2e-${new Date().getTime()}-confirm@`); // TODO: %[ test_user_email ]% ...

	await page.goto('/');
	await expect(page.locator('h1')).toBeVisible();

	const email = page.locator('#email');
	const password = page.locator('#password');
	const confirm = page.locator('#confirm');
	const otp = page.locator('#otp');
	const otpInput = page.locator('input[data-slot=input-otp]');
	const submit = page.locator('button[type=submit]');
	const signUpLink = page.locator('#sign-up-link');
	const signOut = page.locator('#sign-out-btn');
	const signUp = page.locator('#sign-up-btn');
	const signInLink = page.locator('#sign-in-link');
	const signIn = page.locator('#sign-in-btn');

	const emailMsg = page.locator('#email-msg');
	const passwordMsg = page.locator('#password-msg');
	const confirmMsg = page.locator('#confirm-msg');

	const toastError = page.locator('li[data-type=error]');

	await expect(email).toBeVisible();
	await expect(password).toBeVisible();
	await expect(signUpLink).toBeVisible();

	// When: clicking sign-up link

	await signUpLink.click();

	// Then: switch to sign-up form

	await expect(email).toBeVisible();
	await expect(password).toBeVisible();
	await expect(confirm).toBeVisible();
	await expect(signUp).toBeVisible();
	await expect(signInLink).toBeVisible();

	// When: clicking the sign-up button without data

	await email.clear();
	await password.clear();
	await confirm.clear();
	await signUp.click();

	// Then: fields are marked as invalid and show a validation text

	await expect(email).toHaveAttribute('aria-invalid');
	expect(await email.getAttribute('aria-invalid')).toBe('true');
	await expect(emailMsg).toBeVisible();

	await expect(password).toHaveAttribute('aria-invalid');
	expect(await password.getAttribute('aria-invalid')).toBe('true');
	await expect(passwordMsg).toBeVisible();

	// When: submitting with valid Email invalid password

	await email.fill(newEmail);
	await password.fill('P');
	await confirm.fill('P');
	await submit.click();

	// Then: password is marked as invalid and shows a validation text

	await expect(password).toHaveAttribute('aria-invalid');
	expect(await password.getAttribute('aria-invalid')).toBe('true');
	await expect(passwordMsg).toBeVisible();

	// When: submitting with valid password but wrong repetition

	await password.fill('%[ test_user_password ]%');
	await submit.click();

	// Then: password is marked as invalid and shows a validation text

	await expect(confirm).toHaveAttribute('aria-invalid');
	expect(await confirm.getAttribute('aria-invalid')).toBe('true');
	await expect(confirmMsg).toBeVisible();

	// When: submitting with valid data

	await confirm.fill('%[ test_user_password ]%');
	await submit.click();

	// Then: switches to confirm page

	await expect(email).not.toBeVisible();
	await expect(password).not.toBeVisible();
	await expect(confirm).not.toBeVisible();
	await expect(otp).toBeVisible();

	// When: submitting with invalid code

	await otpInput.fill('0'); // replaces last digit
	await submit.click();

	// Then: error toast shows up

	await expect(toastError).toBeVisible();

	// When: submitting with code

	await otpInput.fill('6'); // replaces last digit
	await submit.click();

	// Then: switches back to home (either showing sign in or signed in state)

	async function signedInFlow() {
		await expect(signOut).toBeVisible();
		await signOut.click();
		await expect(signIn).toBeVisible();
	}

	async function signedOutFlow() {
		await expect(signIn).toBeVisible();
		await password.fill(newEmail);
		await password.fill('%[ test_user_password ]%');
		await signIn.click();
		await signedInFlow();
	}

	try {
		await signedInFlow();
	} catch {
		await signedOutFlow();
	}
});
