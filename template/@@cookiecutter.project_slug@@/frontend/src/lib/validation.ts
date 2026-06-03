// @ts-expect-error - Paraglide generates JS with JSDoc
import * as m from '$lib/paraglide/messages.js';

/**
 * Checks if the given value is (most likely) a valid email address.
 * It's not possible to fully validate email addresses with regex alone, but this is a good
 * approximation.
 */
export function checkEmail(value: string) {
	return /^(?!.*[.+]{2})[a-zA-Z0-9](?:[a-zA-Z0-9._%+-]*[a-zA-Z0-9])?@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$/.test(
		value
	);
}

/**
 * Validates presence and format of the given email address.
 * Returns localized error messages if invalid for `ValidatedInput` component.
 */
export function validateEmail(value: string) {
	if (value === '') return m.validation_email_required();
	else if (!checkEmail(value)) return m.validation_email_invalid();
	else return null;
}

type PasswordPolicy = {
	minimumLength: number;
	requireUppercase: boolean;
	requireLowercase: boolean;
	requireNumbers: boolean;
	requireSymbols: boolean;
};

/**
 * Checks if the given value meets the given Cognito policy for a new password.
 */
export function checkNewPassword(value: string, policy: PasswordPolicy) {
	return (
		value.length >= policy.minimumLength &&
		(!policy.requireUppercase || /[A-Z]/.test(value)) &&
		(!policy.requireLowercase || /[a-z]/.test(value)) &&
		(!policy.requireNumbers || /\d/.test(value)) &&
		(!policy.requireSymbols || /[!@#$%^&*(),.?":{}|<>]/.test(value))
	);
}

/**
 * Validates presence of new password and match of an optional Cognito policy.
 * Returns localized error messages if invalid for `ValidatedInput` component.
 */
export function validateNewPassword(value: string, policy: PasswordPolicy | null) {
	if (value === '') {
		return m.validation_password_required();
	}
	if (policy) {
		const requirements = [];
		if (value.length < policy.minimumLength)
			requirements.push(m.validation_password_req_characters({ count: policy.minimumLength }));
		if (policy.requireUppercase && !/[A-Z]/.test(value))
			requirements.push(m.validation_password_req_uppercase());
		if (policy.requireLowercase && !/[a-z]/.test(value))
			requirements.push(m.validation_password_req_lowercase());
		if (policy.requireNumbers && !/\d/.test(value))
			requirements.push(m.validation_password_req_numbers());
		if (policy.requireSymbols && !/[!@#$%^&*(),.?":{}|<>]/.test(value))
			requirements.push(m.validation_password_req_symbols());
		if (requirements.length > 0)
			return m.validation_password_missing_requirements({ requirements: requirements.join(', ') });
	}
	return null;
}

/**
 * Validates that the password repetition matches the new password.
 * Returns a localized error message if they do not match.
 */
export function validatePasswordRepetition(newPassword: string, repetition: string) {
	return newPassword === repetition ? null : m.validation_password_repetition_mismatch();
}

/**
 * Validates presence of the given name.
 * Returns a localized error message if it is empty.
 */
export function validateName(value: string) {
	return value.trim().length > 0 ? null : m.validation_name_required();
}

