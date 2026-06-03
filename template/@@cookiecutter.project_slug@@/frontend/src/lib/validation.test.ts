import { describe, expect, it } from 'vitest';
import {
	checkEmail,
	checkNewPassword,
	validateNewPassword,
	validateEmail,
	validatePasswordRepetition,
	validateName
} from '$lib/validation';
// @ts-expect-error - Paraglide generates JS with JSDoc
import { setLocale } from '$lib/paraglide/runtime';

describe('validation', () => {
	it('check Emails', async () => {
		expect(checkEmail('.')).toBeFalsy();
		expect(checkEmail('@')).toBeFalsy();
		expect(checkEmail('a@bde')).toBeFalsy();
		expect(checkEmail('+@b.de')).toBeFalsy();

		expect(checkEmail('a@b.de')).toBeTruthy();
		expect(checkEmail('a+b@c.de')).toBeTruthy();
		expect(checkEmail('a.b+c.d@e.f.technology')).toBeTruthy();
	});

	it('validate passwords against policy', async () => {
		const defaultPolicy = {
			minimumLength: 8,
			requireUppercase: true,
			requireLowercase: true,
			requireNumbers: true,
			requireSymbols: true
		};

		expect(checkNewPassword('Test123!', defaultPolicy)).toBeTruthy();
		expect(validateNewPassword('Test123!', defaultPolicy)).toBeNull();

		expect(checkNewPassword('Test1234', defaultPolicy)).toBeFalsy();
		expect(validateNewPassword('Test1234', defaultPolicy)).not.toBeNull();

		expect(checkNewPassword('Test1!', defaultPolicy)).toBeFalsy();
		expect(validateNewPassword('Test1!', defaultPolicy)).not.toBeNull();

		expect(checkNewPassword('test123!', defaultPolicy)).toBeFalsy();
		expect(validateNewPassword('test123!', defaultPolicy)).not.toBeNull();

		expect(checkNewPassword('TEST123!', defaultPolicy)).toBeFalsy();
		expect(validateNewPassword('TEST123!', defaultPolicy)).not.toBeNull();

		const simplePolicy = {
			minimumLength: 6,
			requireUppercase: false,
			requireLowercase: true,
			requireNumbers: false,
			requireSymbols: false
		};

		expect(checkNewPassword('test12', simplePolicy)).toBeTruthy();
		expect(validateNewPassword('test12!', simplePolicy)).toBeNull();

		expect(checkNewPassword('test1', simplePolicy)).toBeFalsy();
		expect(validateNewPassword('test1', simplePolicy)).not.toBeNull();

		expect(checkNewPassword('TEST12', simplePolicy)).toBeFalsy();
		expect(validateNewPassword('TEST12', simplePolicy)).not.toBeNull();
	});

	/**
	 * Verify that localized validation error messages are correctly returned.
	 */
	it('returns localized error messages', async () => {
		// Test English locale
		await setLocale('en');

		expect(validateEmail('')).toBe('Email address is required.');
		expect(validateEmail('invalid')).toBe('Email address is not valid.');

		expect(validateName('')).toBe('Name is required.');
		expect(validateName('   ')).toBe('Name is required.');
		expect(validateName('Alice')).toBeNull();

		expect(validatePasswordRepetition('pass1', 'pass2')).toBe(
			'Password does not match its repetition.'
		);
		expect(validatePasswordRepetition('pass1', 'pass1')).toBeNull();

		const defaultPolicy = {
			minimumLength: 8,
			requireUppercase: true,
			requireLowercase: true,
			requireNumbers: true,
			requireSymbols: true
		};
		expect(validateNewPassword('', defaultPolicy)).toBe('Password is required.');
		expect(validateNewPassword('Short1!', defaultPolicy)).toBe(
			'Password is missing some requirements: 8 characters'
		);
	});
});
