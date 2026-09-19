import { afterEach, describe, expect, it, vi } from 'vitest';
import type {
	GetCurrentUserOutput,
	SignInInput,
	SignInOutput,
	SignUpInput,
	SignUpOutput
} from 'aws-amplify/auth';

const mockState = vi.hoisted(() => ({
	signedIn: undefined as string | undefined
}));

const mockAmplify = vi.hoisted(() => ({
	Amplify: {
		configure: vi.fn()
	}
}));

const mockAuthApi = vi.hoisted(() => ({
	signIn: vi.fn((input: SignInInput) => {
		if (input.password === 'done') {
			mockState.signedIn = input.username;
			const signInOutput: SignInOutput = { isSignedIn: true, nextStep: { signInStep: 'DONE' } };
			return Promise.resolve(signInOutput);
		} else if (input.password === 'confirm') {
			mockState.signedIn = 'need to confirm!';
			const signInOutput: SignInOutput = {
				isSignedIn: false,
				nextStep: { signInStep: 'CONFIRM_SIGN_UP' }
			};
			return Promise.resolve(signInOutput);
		} else {
			return Promise.reject('wrong');
		}
	}),
	signOut: vi.fn(() => {
		if (mockState.signedIn) {
			mockState.signedIn = undefined;
			return Promise.resolve();
		} else {
			return Promise.reject('not signed in');
		}
	}),
	getCurrentUser: vi.fn(() => {
		if (mockState.signedIn) {
			const currentUser: GetCurrentUserOutput = {
				username: mockState.signedIn,
				userId: mockState.signedIn
			};
			return Promise.resolve(currentUser);
		} else {
			return Promise.reject('not signed in');
		}
	}),
	signUp: vi.fn((input: SignUpInput) => {
		const output: SignUpOutput = {
			isSignUpComplete: false,
			nextStep: {
				signUpStep: 'CONFIRM_SIGN_UP',
				codeDeliveryDetails: { deliveryMedium: 'EMAIL', destination: input.username }
			}
		};
		return Promise.resolve(output);
	})
}));

vi.mock('aws-amplify', () => mockAmplify);
vi.mock('aws-amplify/auth', () => mockAuthApi);

import * as auth from './auth';
import { get } from 'svelte/store';

type AuthCodeDeliveryDetails = {
	destination: string;
};

type ConfirmSignUpSignUpStep = {
	codeDeliveryDetails: AuthCodeDeliveryDetails;
};

describe('auth', () => {
	afterEach(async () => {
		vi.resetAllMocks();
		try {
			await auth.signOut();
		} catch {
			mockState.signedIn = undefined; // may not be set before signOut
		}
	});

	it('configures Amplify and loads current user', async () => {
		expect(get(auth.currentUser)).toBeUndefined();
		expect(get(auth.isLoadingUser)).toBeTruthy();
		expect(get(auth.isSignedIn)).toBeFalsy();
		expect(get(auth.isSignedOut)).toBeFalsy();

		await auth.configureAuth();

		expect(get(auth.currentUser)).toBeNull();
		expect(get(auth.isLoadingUser)).toBeFalsy();
		expect(get(auth.isSignedIn)).toBeFalsy();
		expect(get(auth.isSignedOut)).toBeTruthy();
	});

	it('rejects wrong passwords', async () => {
		await expect(auth.signIn('user', 'wrong')).rejects.toThrowError();

		expect(get(auth.currentUser)).toBeNull();
		expect(get(auth.isLoadingUser)).toBeFalsy();
		expect(get(auth.isSignedIn)).toBeFalsy();
		expect(get(auth.isSignedOut)).toBeTruthy();
	});

	it('loads user after successful sign in and drops after sign out', async () => {
		expect((await auth.signIn('user-done', 'done')).isSignedIn).toBeTruthy();

		expect(get(auth.currentUser)?.username).toBe('user-done');
		expect(get(auth.isLoadingUser)).toBeFalsy();
		expect(get(auth.isSignedIn)).toBeTruthy();
		expect(get(auth.isSignedOut)).toBeFalsy();

		await auth.signOut();

		expect(get(auth.currentUser)).toBeNull();
		expect(get(auth.isLoadingUser)).toBeFalsy();
		expect(get(auth.isSignedIn)).toBeFalsy();
		expect(get(auth.isSignedOut)).toBeTruthy();
	});

	it('does not load user that is still to confirm', async () => {
		expect((await auth.signIn('user-confirm', 'confirm')).isSignedIn).toBeFalsy();

		expect(get(auth.currentUser)).toBeNull();
		expect(get(auth.isLoadingUser)).toBeFalsy();
		expect(get(auth.isSignedIn)).toBeFalsy();
		expect(get(auth.isSignedOut)).toBeTruthy();
	});

	it('signs out current user on sign up', async () => {
		expect((await auth.signIn('user-before', 'done')).isSignedIn).toBeTruthy();
		expect(get(auth.currentUser)).not.toBeNull();

		const signUpOutput = await auth.signUp('email', 'password');
		expect(signUpOutput.isSignUpComplete).toBeFalsy();
		expect((signUpOutput.nextStep as ConfirmSignUpSignUpStep).codeDeliveryDetails.destination).toBe(
			'email'
		);
		expect(get(auth.currentUser)).toBeNull();
	});
});
