<script lang="ts">
	import * as auth from '$lib/auth';
	import { goto } from '$app/navigation';
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import { toastError, toastSuccess } from '../toasts';
	import { ValidatedInput } from '$lib/components/validatedInput';
	import { ValidatedForm } from '$lib/components/validatedForm';
	import {
		validateEmail,
		validateNewPassword,
		validatePasswordRepetition
	} from '$lib/validation';
	import { onMount } from 'svelte';
	import { protocolLoad } from '$lib/protocols';
	import { PasswordPolicy } from '$lib/proto/password_policy/password_policy';
	// @ts-expect-error - Paraglide generates JS with JSDoc
	import * as m from '$lib/paraglide/messages.js';

	let email = $state('');
	let password = $state('');
	let confirm = $state('');

	let submitting = $state(false);

	let passwordPolicy: PasswordPolicy | null = $state(null);

	onMount(() => {
		protocolLoad('/api/password-policy', PasswordPolicy)
			.then((data) => {
				passwordPolicy = data;
			})
			.catch((err) => {
				console.error('Error loading password policy:', err);
			});
	});

	async function handleSubmit() {
		submitting = true;
		try {
			const result = await auth.signUp(email, password);
			console.log('Sign up:', result);
			if (result.isSignUpComplete) {
				toastSuccess(m.auth_signup_toast_success_title(), m.auth_signup_toast_success_desc());
				await goto('/');
			} else if (result.nextStep.signUpStep === 'CONFIRM_SIGN_UP') {
				toastSuccess(m.auth_login_toast_next_step_title(), m.auth_login_toast_next_step_desc_confirm());
				const params = new URLSearchParams({
					email
				});
				await goto(`/confirmSignUp?${params.toString()}`);
			} else toastError(m.auth_login_toast_next_step_title(), m.auth_login_toast_next_step_desc_unimpl({ step: result.nextStep.signUpStep }));
		} catch (err) {
			console.error('Error signing up:', err);
			if (err instanceof Error && err.name === 'UsernameExistsException')
				toastError(m.auth_signup_toast_failed_title(), m.auth_signup_toast_failed_desc_exists());
			else if (err instanceof Error && err.name === 'InvalidPasswordException')
				toastError(m.auth_signup_toast_failed_title(), m.auth_signup_toast_failed_desc_policy());
			else toastError(m.auth_signup_toast_failed_title(), m.auth_signup_toast_failed_desc_generic());
		} finally {
			submitting = false;
		}
	}
</script>

<Card.Root class="m-auto mt-5 w-full max-w-sm">
	<Card.Header>
		<Card.Title>{m.auth_signup_title()}</Card.Title>
		<Card.Description>{m.auth_signup_desc()}</Card.Description>
	</Card.Header>

	<Card.Content>
		<ValidatedForm id="form" onsubmit={handleSubmit}>
			<div class="flex flex-col gap-6">
				<ValidatedInput
					id="email"
					label={m.auth_signup_email_label()}
					type="email"
					bind:value={email}
					validations={[validateEmail]}
				/>

				<ValidatedInput
					id="password"
					label={m.auth_signup_password_label()}
					type="password"
					bind:value={password}
					data-policy={passwordPolicy ? 'true' : 'false'}
					validations={[(v) => validateNewPassword(v, passwordPolicy)]}
				/>

				<ValidatedInput
					id="confirm"
					label={m.auth_signup_confirm_label()}
					type="password"
					bind:value={confirm}
					validations={[(v) => validatePasswordRepetition(password, v)]}
				/>
			</div>
		</ValidatedForm>
	</Card.Content>

	<Card.Footer class="flex-col gap-2">
		<Button id="sign-up-btn" disabled={submitting} class="w-full" type="submit" form="form">
			{m.auth_signup_submit_btn()}
		</Button>
		<p>
			{m.auth_signup_already_have_account_text()}
			<a id="sign-in-link" href="/">{m.auth_signup_sign_in_link()}</a>
		</p>
	</Card.Footer>
</Card.Root>
