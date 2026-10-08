<script lang="ts">
	import '../app.css';
	import favicon from '#lib/assets/favicon.svg';
	import { onMount } from 'svelte';
	import * as auth from '#lib/auth.js';
	import { Toaster } from '#lib/components/ui/sonner/index.js';
	import { browser } from '$app/env';
	import { setLocale, locales } from '#lib/paraglide/runtime.js';

	if (browser) {
		let lang = navigator.language.split('-')[0];
		// eslint-disable-next-line @typescript-eslint/no-explicit-any
		if (!locales.includes(lang as any)) {
			lang = 'en';
		}
		// eslint-disable-next-line @typescript-eslint/no-explicit-any
		setLocale(lang as any);
		document.documentElement.lang = lang;
	}
	let hydrated = $state(false);

	onMount(async () => {
		// If the static splash was injected (only on the fallback file), remove it.
		document.getElementById('cf-splash')?.remove();
		document.getElementById('cf-splash-style')?.remove();
		await auth.configureAuth();
		hydrated = true;
	});

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<Toaster />

<div data-hydrated={hydrated}>
	{@render children?.()}
</div>

<style>
	:global(.link-button) {
		display: inline;
		padding: 0;
		border: 0;
		font: inherit;
		text-decoration: underline;
		cursor: pointer;
		background: transparent;
		color: currentColor;

		appearance: none;
	}
</style>
