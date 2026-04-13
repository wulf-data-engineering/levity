<script lang="ts">
    import { authApi, signIn } from '$lib/auth';
    import { goto } from '$app/navigation';
    import { Button } from '$lib/components/ui/button';
    import * as Card from '$lib/components/ui/card';
    import { toastError, toastSuccess } from '../toasts';
    import { dev } from '$app/environment';
    import * as InputOTP from '$lib/components/ui/input-otp';
    import { get } from 'svelte/store';
    import { requestPasswordReset } from '../resetPassword/request';
    import { page } from '$app/state';
    import { validateEmail, validateNewPassword, validatePasswordRepetition } from '$lib/validation';
    import { ValidatedInput } from '$lib/components/validatedInput';
    import { ValidatedForm } from '$lib/components/validatedForm';
    import { onMount } from 'svelte';
    import { protocolLoad } from '$lib/protocols';
    import { PasswordPolicy } from '$lib/proto/password_policy/password_policy';
    // @ts-ignore - Paraglide generates JS with JSDoc
    import * as m from '$lib/paraglide/messages.js';

    let email = $state('');
    let password = $state('');
    let confirm = $state('');

    let otp = $state(dev ? '123456' : ''); // erased at build time

    let submitting = $state(false);

    let passwordPolicy: PasswordPolicy | null = $state(null);

    onMount(() => {
        const urlEmail = page.url.searchParams.get('email');
        if (urlEmail) {
            email = urlEmail;
        }

        const urlOtp = page.url.searchParams.get('otp');
        if (urlOtp) {
            otp = urlOtp;
        }

        protocolLoad('/api/password-policy', PasswordPolicy)
            .then((data) => {
                passwordPolicy = data;
            })
            .catch((err) => {
                console.error('Error loading password policy:', err);
            });
    });

    async function handleSubmit(e: Event) {
        e.preventDefault();
        submitting = true;
        try {
            await get(authApi).confirmResetPassword({
                username: email,
                newPassword: password,
                confirmationCode: otp
            });
            toastSuccess(m.auth_confirm_reset_toast_success_title(), m.auth_confirm_reset_toast_success_desc());
            await signIn(email, password);
            await goto('/');
        } catch (err) {
            console.error('Error confirming reset password:', err);
            if (err instanceof Error && err.name === 'CodeMismatchException') {
                toastError(m.auth_confirm_reset_toast_failed_title(), m.auth_confirm_reset_toast_failed_desc_wrong_code());
            } else if (err instanceof Error && err.name === 'ExpiredCodeException') {
                try {
                    await get(authApi).resetPassword({ username: email });
                    toastError(
                        m.auth_confirm_reset_toast_sent_title(),
                        m.auth_confirm_reset_toast_sent_desc_expired()
                    );
                } catch {
                    toastError(m.auth_confirm_reset_toast_failed_title(), m.auth_confirm_reset_toast_failed_desc_expired());
                }
            } else toastError(m.auth_confirm_reset_toast_failed_title(), m.auth_confirm_reset_toast_failed_desc_generic());
        } finally {
            submitting = false;
        }
    }

    async function resendCode() {
        try {
            submitting = true;
            await requestPasswordReset(email);
        } finally {
            submitting = false;
        }
    }
</script>

<Card.Root class="m-auto mt-5 w-full max-w-sm">
    <Card.Header>
        <Card.Title>{m.auth_confirm_reset_title()}</Card.Title>
        <Card.Description>{m.auth_confirm_reset_desc()}</Card.Description>
    </Card.Header>

    <Card.Content>
        <ValidatedForm id="form" onsubmit={handleSubmit}>
            <div class="flex flex-col gap-6">
                <InputOTP.Root id="otp" maxlength={6} bind:value={otp} class="justify-center" required>
                    {#snippet children({ cells })}
                        <InputOTP.Group>
                            {#each cells as cell (cell)}
                                <InputOTP.Slot {cell} />
                            {/each}
                        </InputOTP.Group>
                    {/snippet}
                </InputOTP.Root>

                <ValidatedInput
                        id="email"
                        label={m.auth_confirm_reset_email_label()}
                        type="email"
                        bind:value={email}
                        validations={[validateEmail]}
                />

                <ValidatedInput
                        id="password"
                        label={m.auth_confirm_reset_password_label()}
                        type="password"
                        bind:value={password}
                        data-policy={passwordPolicy ? 'true' : 'false'}
                        validations={[(v) => validateNewPassword(v, passwordPolicy)]}
                />

                <ValidatedInput
                        id="confirm"
                        label={m.auth_confirm_reset_confirm_label()}
                        type="password"
                        bind:value={confirm}
                        validations={[(v) => validatePasswordRepetition(password, v)]}
                />
            </div>
        </ValidatedForm>
    </Card.Content>

    <Card.Footer class="flex-col gap-2">
        <Button
                id="reset-password-btn"
                variant="default"
                disabled={submitting}
                class="w-full"
                type="submit"
                form="form"
        >
            {m.auth_confirm_reset_submit_btn()}
        </Button>
        <Button
                id="resend-code-btn"
                variant="outline"
                disabled={submitting}
                class="w-full"
                onclick={resendCode}
        >
            {m.auth_confirm_reset_resend_btn()}
        </Button>
    </Card.Footer>
</Card.Root>
