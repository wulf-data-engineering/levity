<script lang="ts">
    import {dev} from '$app/environment';
    import * as auth from '$lib/auth';
    import {goto} from "$app/navigation";
    import {Button} from "$lib/components/ui/button";
    import * as InputOTP from "$lib/components/ui/input-otp";
    import * as Card from "$lib/components/ui/card";
    import {toastError, toastSuccess} from "../toasts";
    import {page} from "$app/state";
    import {onMount} from 'svelte';
    import {get} from 'svelte/store';

    let submitting = $state(false);
    let suggestResend = $state(false);

    let email: string;

    onMount(async () => {
        let maybeEmail = page.url.searchParams.get('email');
        if (!maybeEmail) {
            toastError('Error', 'Email is required to confirm sign up.');
            await goto('/signUp');
        } else {
            email = maybeEmail;
        }
    });

    let otp = $state(dev ? '123456' : ''); // erased at build time

    async function handleSubmit(e: Event) {
        e.preventDefault();
        submitting = true;
        try {
            const result = await auth.confirmSignUp(email, otp);
            console.log('Confirm Sign Up:', result);
            if (result.isSignUpComplete) {
                toastSuccess('Signed Up', 'Successfully signed up')
                if (get(auth.isSignedIn)) {
                    await goto('/')
                } else {
                    await goto(`/?email=${encodeURIComponent(email)}`)
                }
            } else
                toastError('Next Step', 'Next step is not implemented.');
        } catch (err) {
            console.error('Error confirming sign up:', err);
            if (err instanceof Error && err.name === 'CodeMismatchException') {
                toastError('Confirmation Failed', 'The provided confirmation code is incorrect.');
                suggestResend = true;
            } else if (err instanceof Error && err.name === 'ExpiredCodeException') {
                try {
                    await get(auth.authApi).resendSignUpCode({username: email!});
                    toastError('Code Sent', 'The provided confirmation code has expired. A new confirmation code has been sent to your email.');
                } catch {
                    if (dev)
                        toastError('Confirmation Failed', 'Cognito local might not support resending codes.'); // erased at build time
                    else
                        toastError('Confirmation Failed', 'The provided confirmation code has expired.');
                    suggestResend = true;
                }
            } else
                toastError('Confirmation Failed', 'Sign up could not be confirmed.');
        } finally {
            submitting = false;
        }
    }

    async function resendCode() {
        try {
            submitting = true;
            await get(auth.authApi).resendSignUpCode({username: email!});
            toastSuccess('Code Sent', 'A new confirmation code has been sent to your email.');
        } catch (err) {
            console.error('Error resending code:', err);
            toastError('Error', 'Could not resend confirmation code.');
        } finally {
            submitting = false;
            suggestResend = false;
        }
    }

</script>

<Card.Root class="w-full max-w-sm m-auto mt-5">
    <Card.Header>
        <Card.Title>Confirm Account</Card.Title>
        <Card.Description>Enter the confirmation code you received</Card.Description>
    </Card.Header>

    <Card.Content>
        <form onsubmit={handleSubmit}>
            <InputOTP.Root maxlength={6} bind:value={otp} class="justify-center">
                {#snippet children({cells})}
                    <InputOTP.Group>
                        {#each cells as cell (cell)}
                            <InputOTP.Slot {cell}/>
                        {/each}
                    </InputOTP.Group>
                {/snippet}
            </InputOTP.Root>
        </form>
    </Card.Content>

    <Card.Footer class="flex-col gap-2">
        <Button variant={suggestResend ? "outline" : "default"} disabled={submitting} class="w-full"
                onclick={handleSubmit}>
            Confirm Sign Up
        </Button>
        <Button variant={suggestResend ? "default" : "outline"} disabled={submitting} class="w-full"
                onclick={resendCode}>
            Resend Code
        </Button>
    </Card.Footer>
</Card.Root>
