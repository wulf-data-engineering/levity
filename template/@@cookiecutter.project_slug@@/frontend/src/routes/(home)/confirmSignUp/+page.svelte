<script lang="ts">
	import { dev } from '$app/environment';
	import * as auth from '$lib/auth';
	import { goto } from '$app/navigation';
	import { Button } from '$lib/components/ui/button';
	import * as InputOTP from '$lib/components/ui/input-otp';
	import * as Card from '$lib/components/ui/card';
	import { toastError, toastSuccess } from '../toasts';
	import { page } from '$app/state';
	import { onMount } from 'svelte';
	import { get } from 'svelte/store';
	import { ValidatedForm } from '$lib/components/validatedForm';

	import { SignUpData } from '$lib/proto/sign_up_data/sign_up_data';
	// @ts-expect-error - Paraglide generates JS with JSDoc, which svelte-check might complain about missing .d.ts
	import { getLocale } from '$lib/paraglide/runtime';
	// @ts-expect-error - Paraglide generates JS with JSDoc
	import * as m from '$lib/paraglide/messages.js';

	import { validateName } from '$lib/validation';
	import { ValidatedInput } from '$lib/components/validatedInput';

	let submitting = $state(false);

	let email: string;
	let firstName = $state('');
	let lastName = $state('');

	onMount(async () => {
		let maybeEmail = page.url.searchParams.get('email');
		if (!maybeEmail) {
			toastError(m.auth_confirm_signup_toast_error_title(), m.auth_confirm_signup_toast_error_email_req());
			await goto('/signUp');
		} else {
			email = maybeEmail;
		}
	});

	let otp = $state(dev ? '123456' : ''); // erased at build time

	async function handleSubmit() {
		submitting = true;
		try {
			const signUpData: SignUpData = { firstName, lastName, language: getLocale() };
			const result = await auth.confirmSignUp(email, otp, signUpData);
			console.log('Confirm Sign Up:', result);
			if (result.isSignUpComplete) {
				toastSuccess(m.auth_signup_toast_success_title(), m.auth_signup_toast_success_desc());
				if (get(auth.isSignedIn)) {
					await goto('/');
				} else {
					await goto(`/?email=${encodeURIComponent(email)}`);
				}
			} else toastError(m.auth_login_toast_next_step_title(), m.auth_login_toast_next_step_desc_unimpl({ step: 'Next step' }));
		} catch (err) {
			console.error('Error confirming sign up:', err);
			if (err instanceof Error && err.name === 'CodeMismatchException') {
				toastError(m.auth_confirm_signup_toast_failed_title(), m.auth_confirm_signup_toast_failed_desc_wrong_code());
			} else if (err instanceof Error && err.name === 'ExpiredCodeException') {
				try {
					await get(auth.authApi).resendSignUpCode({ username: email! });
					toastError(
						m.auth_confirm_signup_toast_sent_title(),
						m.auth_confirm_signup_toast_sent_desc_expired()
					);
				} catch {
					if (dev)
						toastError(m.auth_confirm_signup_toast_failed_title(), m.auth_confirm_signup_toast_failed_desc_local()); // erased at build time
					else toastError(m.auth_confirm_signup_toast_failed_title(), m.auth_confirm_signup_toast_failed_desc_expired());
				}
			} else toastError(m.auth_confirm_signup_toast_failed_title(), m.auth_confirm_signup_toast_failed_desc_generic());
		} finally {
			submitting = false;
		}
	}

	async function resendCode() {
		try {
			submitting = true;
			await get(auth.authApi).resendSignUpCode({ username: email! });
			toastSuccess(m.auth_confirm_signup_toast_sent_title(), m.auth_confirm_signup_toast_sent_desc());
		} catch (err) {
			console.error('Error resending code:', err);
			toastError(m.auth_confirm_signup_toast_error_title(), m.auth_confirm_signup_toast_error_resend());
		} finally {
			submitting = false;
		}
	}
</script>

<Card.Root class="m-auto mt-5 w-full max-w-sm">
	<Card.Header>
		<Card.Title>{m.auth_confirm_signup_title()}</Card.Title>
		<Card.Description>{m.auth_confirm_signup_desc()}</Card.Description>
	</Card.Header>

	<Card.Content>
		<ValidatedForm id="form" onsubmit={handleSubmit}>
			<div class="flex flex-col gap-6">
				<ValidatedInput
					id="firstName"
					label={m.auth_confirm_signup_first_name_label()}
					type="text"
					bind:value={firstName}
					validations={[validateName]}
				/>

				<ValidatedInput
					id="lastName"
					label={m.auth_confirm_signup_last_name_label()}
					type="text"
					bind:value={lastName}
					validations={[validateName]}
				/>

				<InputOTP.Root id="otp" maxlength={6} bind:value={otp} class="justify-center" required>
					{#snippet children({ cells })}
						<InputOTP.Group>
							{#each cells as cell (cell)}
								<InputOTP.Slot {cell} />
							{/each}
						</InputOTP.Group>
					{/snippet}
				</InputOTP.Root>
			</div>
		</ValidatedForm>
	</Card.Content>

	<Card.Footer class="flex-col gap-2">
		<Button variant="default" disabled={submitting} class="w-full" type="submit" form="form">
			{m.auth_confirm_signup_submit_btn()}
		</Button>
		<Button variant="outline" disabled={submitting} class="w-full" onclick={resendCode}>
			{m.auth_confirm_signup_resend_btn()}
		</Button>
	</Card.Footer>
</Card.Root>
