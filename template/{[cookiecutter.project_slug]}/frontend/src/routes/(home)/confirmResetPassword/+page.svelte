<script lang="ts">
    import {authApi} from '$lib/auth';
    import {goto} from "$app/navigation";
    import {Button} from "$lib/components/ui/button";
    import {Label} from "$lib/components/ui/label";
    import {Input} from "$lib/components/ui/input";
    import * as Card from "$lib/components/ui/card";
    import {toastError, toastSuccess} from "../toasts";
    import {dev} from "$app/environment";
    import * as InputOTP from "$lib/components/ui/input-otp";
    import {get} from "svelte/store";
    import {requestPasswordReset} from "../resetPassword/request";
    import {page} from "$app/state";
    import * as auth from "$lib/auth";

    let email = $state(page.url.searchParams.get("email") || '');
    let password = $state('');
    let confirm = $state('');

    let otp = $state(page.url.searchParams.get("otp") || (dev ? '123456' : '')); // erased at build time

    let submitting = $state(false);

    async function handleSubmit(e: Event) {
        e.preventDefault();
        submitting = true;
        try {
            const result = await get(authApi).confirmResetPassword({
                username: email,
                newPassword: password,
                confirmationCode: otp,
            });
            console.log('Confirm Reset Password:', result);
            toastSuccess('Password Reset', 'Your password has been reset successfully.');
            await goto('/')
        } catch (err) {
            console.error('Error confirming reset password:', err);
            if (err instanceof Error && err.name === 'CodeMismatchException') {
                toastError('Password Reset', 'The provided confirmation code is incorrect.');
            } else if (err instanceof Error && err.name === 'ExpiredCodeException') {
                try {
                    await get(authApi).resetPassword({username: email});
                    toastError('Code Sent', 'The provided confirmation code has expired. A new confirmation code has been sent to your email.');
                } catch {
                    toastError('Password Reset', 'The provided confirmation code has expired.');
                }
            } else
                toastError('Password Reset', 'Password reset failed.');
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

<Card.Root class="w-full max-w-sm m-auto mt-5">

    <Card.Header>
        <Card.Title>Reset Password</Card.Title>
        <Card.Description>Enter the code you received and the new password</Card.Description>
    </Card.Header>

    <Card.Content>
        <form id="form" onsubmit={handleSubmit}>
            <div class="flex flex-col gap-6">
                <InputOTP.Root maxlength={6} bind:value={otp} class="justify-center" required>
                    {#snippet children({cells})}
                        <InputOTP.Group>
                            {#each cells as cell (cell)}
                                <InputOTP.Slot {cell}/>
                            {/each}
                        </InputOTP.Group>
                    {/snippet}
                </InputOTP.Root>
                <div class="grid gap-2">
                    <Label for="email">Email</Label>
                    <Input id="email" type="email" bind:value={email} required/>
                </div>
                <div class="grid gap-2">
                    <div class="flex items-center">
                        <Label for="password">Password</Label>
                    </div>
                    <Input id="password" type="password" bind:value={password} required/>
                </div>
                <div class="grid gap-2">
                    <div class="flex items-center">
                        <Label for="confirm">Confirm</Label>
                    </div>
                    <Input id="confirm" type="password" bind:value={confirm} required/>
                </div>
            </div>
        </form>
    </Card.Content>

    <Card.Footer class="flex-col gap-2">
        <Button variant="default" disabled={submitting} class="w-full"
                type="submit" form="form">
            Reset Password
        </Button>
        <Button variant="outline" disabled={submitting} class="w-full"
                onclick={resendCode}>
            Resend Code
        </Button>
    </Card.Footer>
</Card.Root>
