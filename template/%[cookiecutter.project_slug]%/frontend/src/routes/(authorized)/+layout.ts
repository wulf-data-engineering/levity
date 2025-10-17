import { redirect } from '@sveltejs/kit';
import { configureAuth, currentUser } from '$lib/auth';
import { get } from 'svelte/store';

import type { CurrentUser } from '$lib/auth';

export const prerender = false;
export const ssr = false;

export async function load({ url }): Promise<{ currentUser: CurrentUser }> {
	await configureAuth();
	if (get(currentUser)) {
		return { currentUser: get(currentUser) };
	} else {
		redirect(303, `/?redirectTo=${url.pathname}`);
	}
}
