<script lang="ts">
	import { dev } from '$app/environment';
	import * as auth from '$lib/auth';
	import { currentUser } from '$lib/auth';
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import { toastError, toastSuccess } from './toasts';
	import { onMount } from 'svelte';
	import { validateEmail } from '$lib/validation';
	import { ValidatedInput } from '$lib/components/validatedInput';
	import { ValidatedForm } from '$lib/components/validatedForm';
	// @ts-ignore - Paraglide generates JS with JSDoc
	import * as m from '$lib/paraglide/messages.js';

	async function signOut() {
		try {
			loading = true;
			await auth.signOut();
			toastSuccess(m.auth_login_toast_sign_out_success(), m.auth_login_toast_sign_out_success());
		} catch (err) {
			console.error('Error signing out:', err);
			toastError(m.auth_login_toast_sign_out_failed_title(), m.auth_login_toast_sign_out_failed_desc());
		} finally {
			loading = false;
		}
	}

	let email = $state('');
	let password = $state('');

	let autofocusPassword = $state(false);

	onMount(async () => {
		let maybeEmail = page.url.searchParams.get('email');
		if (maybeEmail) {
			email = maybeEmail;
			autofocusPassword = true;
		} else if (dev) {
			// erased at build time
			email = '%[cookiecutter.test_user_email]%';
			password = '%[cookiecutter.test_user_password]%';
		}
	});

	let loading = $state(false);

	async function handleSubmit() {
		loading = true;

		const resetRequired = async () => {
			toastError(m.auth_login_toast_sign_in_failed_title(), m.auth_login_toast_sign_in_failed_desc_reset());
			await goto(`/resetPassword?email=${encodeURIComponent(email)}`);
		};

		try {
			const result = await auth.signIn(email, password);
			console.log('Sign in:', result);
			if (result.isSignedIn) {
				// Redirect to the originally requested page
				const redirectTo = page.url.searchParams.get('redirectTo');
				if (redirectTo) await goto(redirectTo);
				else toastSuccess(m.auth_login_toast_sign_in_success_title(), m.auth_login_toast_sign_in_success_desc());
			} else if (result.nextStep.signInStep === 'CONFIRM_SIGN_UP') {
				toastSuccess(m.auth_login_toast_next_step_title(), m.auth_login_toast_next_step_desc_confirm());
				await goto(`/confirmSignUp?email=${encodeURIComponent(email)}`);
			} else if (result.nextStep.signInStep === 'RESET_PASSWORD') await resetRequired();
			else toastError(m.auth_login_toast_next_step_title(), m.auth_login_toast_next_step_desc_unimpl({ step: result.nextStep.signInStep }));
		} catch (err) {
			console.error('Error signing in:', err);
			if (
				err instanceof Error &&
				(err.name === 'NotAuthorizedException' ||
					err.name === 'UserNotFoundException' ||
					err.name === 'InvalidPasswordException')
			)
				toastError(m.auth_login_toast_sign_in_failed_title(), m.auth_login_toast_sign_in_failed_desc_creds());
			else if (err instanceof Error && err.name === 'PasswordResetRequiredException')
				await resetRequired();
			else toastError(m.auth_login_toast_sign_in_failed_title(), m.auth_login_toast_sign_in_failed_desc_generic());
		} finally {
			loading = false;
		}
	}
</script>

<Card.Root class="m-auto mt-5 w-full max-w-sm">
	{#if $currentUser}
		<Card.Header>
			<Card.Title>{m.auth_login_signed_in_title()}</Card.Title>
			<Card.Description>{m.auth_login_signed_in_desc()}</Card.Description>
			<Card.Action>
				<Button variant="link" href="/account">{m.auth_login_your_account_link()}</Button>
			</Card.Action>
		</Card.Header>

		<Card.Content>
			<p>
				{m.auth_login_signed_in_as({ loginId: $currentUser.signInDetails?.loginId ?? '' })}
				<br />
				<small class="text-muted-foreground">{$currentUser.userId}</small>
			</p>
		</Card.Content>

		<Card.Footer class="flex-col gap-2">
			<Button
				id="sign-out-btn"
				onclick={signOut}
				disabled={loading}
				variant="outline"
				class="w-full"
			>
				{m.auth_login_sign_out_btn()}
			</Button>
		</Card.Footer>
	{:else if $currentUser === null}
		<Card.Header>
			<Card.Title>{m.auth_login_title()}</Card.Title>
			<Card.Description>{m.auth_login_desc()}</Card.Description>
		</Card.Header>

		<Card.Content>
			<ValidatedForm id="form" onsubmit={handleSubmit}>
				<div class="flex flex-col gap-6">
					<ValidatedInput
						id="email"
						label={m.auth_login_email_label()}
						type="email"
						bind:value={email}
						autofocus={!autofocusPassword}
						validations={[validateEmail]}
						required
					/>

					<ValidatedInput
						id="password"
						label={m.auth_login_password_label()}
						type="password"
						bind:value={password}
						autofocus={autofocusPassword}
						required
					/>
				</div>
			</ValidatedForm>
		</Card.Content>

		<Card.Footer class="flex-col gap-2">
			<Button id="sign-in-btn" disabled={loading} class="w-full" type="submit" form="form">
				{m.auth_login_submit_btn()}
			</Button>
			<Button
				id="password-forgotten-btn"
				disabled={loading}
				class="w-full"
				href={`/resetPassword?email=${encodeURIComponent(email)}`}
				variant="outline"
			>
				{m.auth_login_forgot_password_btn()}
			</Button>
			<p>
				{m.auth_login_no_account_text()}
				<a id="sign-up-link" href="/signUp">{m.auth_login_sign_up_link()}</a>
			</p>
		</Card.Footer>
	{:else}
		<Card.Content>
			<p>{m.auth_login_loading_text()}</p>
		</Card.Content>
	{/if}
</Card.Root>
