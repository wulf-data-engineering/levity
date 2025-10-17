import { writable, derived, get } from 'svelte/store';

import { dev } from '$app/environment';
import type { Amplify } from 'aws-amplify';
import type { GetCurrentUserOutput } from 'aws-amplify/auth';
import type * as Auth from 'aws-amplify/auth';

export type Amplify = typeof Amplify;
export type AuthApi = typeof Auth;

// Holds the Auth namespace once the client loads Amplify
export const amplify = writable<Amplify>(undefined);
export const authApi = writable<AuthApi>(undefined);

// Tri-state user: value (signed in), null (signed out), undefined (loading)
export type CurrentUser = GetCurrentUserOutput | null | undefined;
export const currentUser = writable<CurrentUser>(undefined);

// Handy derived helpers
export const isSignedIn = derived(currentUser, (u) => !!u);
export const isSignedOut = derived(currentUser, (u) => u === null);
export const isLoadingUser = derived(currentUser, (u) => u === undefined);

let configured = false;

/**
 * Configure Amplify Auth from environment variables.
 * Uses local Cognito mock values if none are provided.
 * Provides the configured Amplify and Auth namespaces via Svelte stores.
 * Called in onMount of the base layout.
 */
export async function configureAuth() {
	if (configured) return;

	const userPoolId: string = import.meta.env.VITE_USER_POOL_ID || (dev && 'local_userPool');
	const userPoolClientId: string =
		import.meta.env.VITE_USER_POOL_CLIENT_ID || (dev && 'local_userPoolClient');
	const endpoint: string | undefined =
		import.meta.env.VITE_COGNITO_ENDPOINT || (dev && 'http://localhost:9229');

	const importedAmplify: Amplify = (await import('aws-amplify')).Amplify;
	const importedAuthApi: AuthApi = await import('aws-amplify/auth');

	importedAmplify.configure(
		{
			Auth: {
				Cognito: {
					userPoolId,
					userPoolClientId,
					userPoolEndpoint: endpoint
				}
			}
		},
		{ ssr: false }
	);

	amplify.set(importedAmplify);
	authApi.set(importedAuthApi);

	await loadCurrentUser();

	configured = true;
	console.log('[auth] Amplify configured', { userPoolId, userPoolClientId, endpoint });
}

export async function loadCurrentUser() {
	try {
		currentUser.set(await get(authApi).getCurrentUser());
	} catch {
		currentUser.set(null);
	}
}

async function performAutoSignIn() {
	await get(authApi).autoSignIn();
	await loadCurrentUser();
}

/**
 * Signs up the user with given credentials.
 * If there is a current user, they are signed out first.
 * If confirmation completes immediately with auto sign in, the user is also signed in.
 */
export async function signUp(email: string, password: string, autoSignIn: boolean = true) {
	if (get(isSignedIn)) await signOut();

	const result = await get(authApi).signUp({
		username: email,
		password: password,
		options: {
			userAttributes: {
				email: email
			},
			autoSignIn
		}
	});
	if (!result.isSignUpComplete) {
		console.log('User is not completely signed up in after signUp:', result.nextStep);
	} else if (result.nextStep.signUpStep === 'COMPLETE_AUTO_SIGN_IN') {
		if (dev) {
			// auto sign in not supported in Cognito local
			await signIn(email, password);
		} else {
			await performAutoSignIn();
		}
	}
	return result;
}

/**
 * Confirms user sign up with given OTP code.
 * If confirmation completes with auto signs in in production, the user is also signed in.
 */
export async function confirmSignUp(email: string, otp: string) {
	const result = await get(authApi).confirmSignUp({
		username: email,
		confirmationCode: otp
	});
	console.log(result.nextStep.signUpStep);
	if (!result.isSignUpComplete) {
		console.warn('User is not completely signed up in after confirm:', result.nextStep);
	} else if (result.nextStep.signUpStep === 'COMPLETE_AUTO_SIGN_IN') {
		if (!dev /* not supported in Cognito local */) {
			await performAutoSignIn();
		}
	}
	return result;
}

/**
 * Signs in the user with given credentials.
 */
export async function signIn(username: string, password: string) {
	const user = await get(authApi).signIn({
		username: username,
		password: password,
		options: {
			authFlowType: 'USER_PASSWORD_AUTH'
		}
	});
	if (!user.isSignedIn) {
		console.warn('User is not signed in after signIn:', user.nextStep);
	} else {
		await loadCurrentUser();
	}
	return user;
}

/**
 * Signs out the current user.
 */
export async function signOut() {
	await get(authApi).signOut({ global: false });
	currentUser.set(null);
}
