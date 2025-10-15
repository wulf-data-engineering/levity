<script lang="ts">
    import * as auth from '$lib/auth';
    import {goto} from "$app/navigation";
    import {Button} from "$lib/components/ui/button";
    import {Label} from "$lib/components/ui/label";
    import {Input} from "$lib/components/ui/input";
    import * as Card from "$lib/components/ui/card";
    import {toastError, toastSuccess} from "../toasts";

    let email = $state('');
    let password = $state('');
    let confirm = $state('');

    let submitting = $state(false);

    async function handleSubmit(e: Event) {
        e.preventDefault();
        submitting = true;
        try {
            const result = await auth.signUp(email, password);
            console.log('Sign up:', result);
            if (result.isSignUpComplete) {
                toastSuccess('Signed Up', 'Successfully signed up and signed in.')
                await goto('/')
            } else if (result.nextStep.signUpStep === 'CONFIRM_SIGN_UP') {
                toastSuccess('Next Step', 'Please complete the sign-up process.');
                await goto(`/confirmSignUp?email=${encodeURIComponent(email)}`);
            } else
                toastError('Next Step', `${result.nextStep.signUpStep} is not implemented.`);
        } catch (err) {
            console.error('Error signing up:', err);
            if (err instanceof Error && err.name === 'UsernameExistsException')
                toastError('Sign Up Failed', 'The Email address is already registered.');
            else if (err instanceof Error && err.name === 'InvalidPasswordException')
                toastError('Sign Up Failed', 'Password does not meet the required policy.');
            else
                toastError('Sign Up Failed', 'An error occurred during sign up.');
        } finally {
            submitting = false;
        }
    }
</script>

<Card.Root class="w-full max-w-sm m-auto mt-5">

    <Card.Header>
        <Card.Title>Sign Up</Card.Title>
        <Card.Description>Create a new account</Card.Description>
    </Card.Header>

    <Card.Content>
        <form id="form" onsubmit={handleSubmit}>
            <div class="flex flex-col gap-6">
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
        <Button disabled={submitting} class="w-full" type="submit" form="form">
            Sign Up
        </Button>
        <p>
            Already have an account?
            <a href="/">Sign in</a>
        </p>
    </Card.Footer>
</Card.Root>
