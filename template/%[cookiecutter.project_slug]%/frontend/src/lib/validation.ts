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
 * Returns error messages if invalid for `ValidatedInput` component.
 */
export function validateEmail(value: string) {
	if (value === '') return 'Email address is required.';
	else if (!checkEmail(value)) return 'Email address is not valid.';
	else return null;
}

/**
 * Checks if the given value meets the (default Cognito) criteria for a new password.
 */
export function checkNewPassword(value: string) {
	return (
		value.length >= 8 &&
		/[0-9]/.test(value) &&
		/[A-Z]/.test(value) &&
		/[a-z]/.test(value) &&
		/\d/.test(value) &&
		/[!@#$%^&*(),.?":{}|<>]/.test(value)
	);
}

/**
 * Validates presence and format of the given new password.
 * Returns error messages if invalid for `ValidatedInput` component.
 */
export function validateNewPassword(value: string) {
	if (value === '') return 'Password is required.';
	else if (!checkNewPassword(value))
		return 'Password requires 8 characters, a number, a symbol, an uppercase and a lowercase letter.';
	else return null;
}
