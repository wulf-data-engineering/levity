import { writable, derived, get } from 'svelte/store';

import type { Amplify } from 'aws-amplify';
import type { GetCurrentUserOutput } from 'aws-amplify/auth';
import type * as AuthAPI from 'aws-amplify/auth';

export type Amplify = typeof Amplify;
export type Auth = typeof AuthAPI;

// Holds the Auth namespace once the client loads Amplify
export const amplify = writable<Amplify>(undefined);
export const auth = writable<Auth>(undefined);

// Tri-state user: value (signed in), null (signed out), undefined (loading)
export type CurrentUser = GetCurrentUserOutput | null | undefined;
export const currentUser = writable<CurrentUser>(undefined);

// Handy derived helpers
export const isSignedIn = derived(currentUser, (u) => !!u);
export const isSignedOut = derived(currentUser, (u) => u === null);
export const isLoadingUser = derived(currentUser, (u) => u === undefined);

const localUserPoolId: string = 'local_userPool';
const localUserPoolClientId: string = 'local_userPoolClient';
const localEndpoint: string = 'http://localhost:9229';

let configured = false;

/**
 * Configure Amplify Auth from environment variables.
 * Uses local Cognito mock values if none are provided.
 * Provides the configured Amplify and Auth namespaces via Svelte stores.
 * Called in onMount of the base layout.
 */
export async function configureAuth() {
	if (configured) return;

	const userPoolId: string = import.meta.env.VITE_USER_POOL_ID || localUserPoolId;
	const userPoolClientId: string =
		import.meta.env.VITE_USER_POOL_CLIENT_ID || localUserPoolClientId;
	const endpoint: string | undefined =
		import.meta.env.VITE_COGNITO_ENDPOINT || userPoolId === localUserPoolId
			? localEndpoint
			: undefined;

	const amplifyApi: Amplify = (await import('aws-amplify')).Amplify;
	const authApi: Auth = await import('aws-amplify/auth');

	amplifyApi.configure(
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

	amplify.set(amplifyApi);
	auth.set(authApi);

	await loadCurrentUser();

	configured = true;
	console.log('[auth] Amplify configured', { userPoolId, userPoolClientId, endpoint });
}

export async function loadCurrentUser() {
	try {
		currentUser.set(await get(auth).getCurrentUser());
	} catch {
		currentUser.set(null);
	}
}

/**
 * Signs up the user with given credentials.
 * If there is a current user, they are signed out first.
 * If sign up is completed, the user is also signed in.
 */
export async function signUp(email: string, password: string) {
	if (get(currentUser)) await signOut();

	const user = await get(auth).signUp({
		username: email,
		password: password,
		options: {
			userAttributes: {
				email: email
			}
		}
	});
	if (!user.isSignUpComplete) {
		console.warn('User is not completely signed up in after signUp:', user.nextStep);
	} else {
		await signIn(email, password);
	}
	return user;
}

/**
 * Confirms user sign up with given OTP code.
 * If sign up is completed, the user is also signed in.
 */
export async function confirmSignUp(email: string, password: string, otp: string) {
	if (get(currentUser)) await signOut();

	const user = await get(auth).confirmSignUp({
		username: email,
		confirmationCode: otp
	});
	if (!user.isSignUpComplete) {
		console.warn('User is not completely signed up in after confirm:', user.nextStep);
	} else {
		await signIn(email, password);
	}
	return user;
}

/**
 * Signs in the user with given credentials.
 */
export async function signIn(username: string, password: string) {
	const user = await get(auth).signIn({
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
 * Confirms user sign in with given OTP code.
 */
export async function confirmSignIn(otp: string) {
	if (get(currentUser)) await signOut();

	const user = await get(auth).confirmSignIn({
		challengeResponse: otp
	});
	if (!user.isSignedIn) {
		console.warn('User is not completely signed in in after confirm:', user.nextStep);
	} else {
		await loadCurrentUser();
	}
	return user;
}

/**
 * Signs out the current user.
 */
export async function signOut() {
	await get(auth).signOut({ global: false });
	currentUser.set(null);
}
