<script lang="ts">
    import {dev} from '$app/environment';
    import * as auth from '$lib/auth';
    import {currentUser} from '$lib/auth';
    import {page} from "$app/state";
    import {goto} from "$app/navigation";
    import {Button} from "$lib/components/ui/button";
    import {Label} from "$lib/components/ui/label";
    import {Input} from "$lib/components/ui/input";
    import * as InputOTP from "$lib/components/ui/input-otp";
    import * as Card from "$lib/components/ui/card";
    import {toast} from "svelte-sonner";


    function handleError(err: unknown) {
        complete('An error occurred', err instanceof Error ? err.message : String(err), true);
    }

    function complete(text: string, description: string, error = false) {
        if (error)
            toast.error(text, {description, richColors: true, duration: 5000});
        else
            toast.success(text, {description, duration: 3000});
        loading = false;
    }

    function indicateLoading() {
        loading = true;
    }

    async function signUp() {
        const result = await auth.signUp(email, password);
        if (result.isSignUpComplete)
            handleSignedIn('Successfully signed up and signed in.')
        else if (result.nextStep.signUpStep === 'CONFIRM_SIGN_UP') {
            complete('Next Step', 'Please complete the sign-up process.');
            confirmState = true;
        } else
            complete('Next Step', `${result.nextStep.signUpStep} is not implemented.`, true);
        console.log('Signed up:', result);
    }

    async function confirmSignUp() {
        const result = await auth.confirmSignUp(email, password, otp);
        if (result.isSignUpComplete)
            handleSignedIn('Successfully signed up and signed in.')
        else
            complete('Next Step', `${result.nextStep.signUpStep} is not implemented.`, true);
    }

    async function signIn() {
        const result = await auth.signIn(email, password);
        if (result.isSignedIn) {
            handleSignedIn('Successfully signed in.');
        } else if (result.nextStep.signInStep === 'CONFIRM_SIGN_UP') {
            complete('Next Step', 'Please complete the sign-up process.');
            confirmState = true;
            mode = 'signUp';
        } else if (result.nextStep.signInStep === 'RESET_PASSWORD') {
            complete('Next Step', 'Password reset is not implemented.', true);
        } else if (result.nextStep.signInStep.startsWith('CONFIRM_SIGN_')) {
            complete('Next Step', 'Please complete the sign-in process.');
            confirmState = true;
        } else {
            complete('Next Step', `${result.nextStep.signInStep} is not implemented.`, true);
        }
        console.log('Signed in:', result);
    }

    async function confirmSignIn() {
        const result = await auth.confirmSignIn(otp);
        if (result.isSignedIn)
            handleSignedIn('Successfully signed in.');
        else
            complete('Next Step', `${result.nextStep.signInStep} is not implemented.`, true);
    }

    async function handleSignedIn(message: string) {
        otp = '';
        password = '';
        mode = 'signIn';
        confirmState = false;
        // Redirect to the originally requested page
        const redirectTo = page.url.searchParams.get('redirectTo');
        if (redirectTo)
            await goto(redirectTo);
        else
            complete('Signed In', message);
    }

    async function signOut() {
        try {
            await auth.signOut();
            complete('Singed Out', 'Successfully signed out.');
        } catch (err) {
            console.error('Error signing out:', err);
            handleError(err);
        } finally {
            loading = false;
        }
    }

    let mode: 'signIn' | 'signUp' = $state('signIn');
    let confirmState = $state(false);

    let loading = $state(false);

    let email = $state(dev ? '{[cookiecutter.test_user_email]}' : ''); // erased at build time
    let password = $state(dev ? '{[cookiecutter.test_user_password]}' : ''); // erased at build time
    let otp = $state(dev ? '{[cookiecutter.dev_confirmation_code]}' : ''); // erased at build time

    async function handleSubmit(e: Event) {
        e.preventDefault();
        indicateLoading();
        try {
            if (mode === 'signIn') {
                if (confirmState)
                    await confirmSignIn();
                else
                    await signIn();
            } else {
                if (confirmState)
                    await confirmSignUp();
                else
                    await signUp();
            }
        } catch (err) {
            console.error('Error signing in:', err);
            handleError(err);
        } finally {
            loading = false;
        }
    }

    function toggle(newMode: 'signIn' | 'signUp') {
        return (e: Event) => {
            e.preventDefault();
            mode = newMode;
        };
    }
</script>

<svelte:head>
    <title>{[cookiecutter.project_name]}</title>
</svelte:head>

<div class="m-auto max-w-4xl px-8 p-5">
    <div>
        <h1 class="scroll-m-20 text-4xl font-extrabold tracking-tight lg:text-5xl">
            {[cookiecutter.project_name]}
        </h1>
        <p class="text-muted-foreground text-xl leading-7 [&:not(:first-child)]:mt-6">
            This is a sample landing page.
        </p>
    </div>

    <Card.Root class="w-full max-w-sm m-auto mt-5">

        {#if $currentUser}

            <Card.Header>
                <Card.Title>Signed In</Card.Title>
                <Card.Description>You are signed in</Card.Description>
                <Card.Action>
                    <Button variant="link" href="/account">Your account</Button>
                </Card.Action>
            </Card.Header>

            <Card.Content>
                <p>
                    You are signed in as {$currentUser.signInDetails?.loginId}. <br/>
                    <small class="text-muted-foreground">{$currentUser.userId}</small>
                </p>
            </Card.Content>

            <Card.Footer class="flex-col gap-2">
                <Button onclick={signOut} disabled={loading} variant="outline" class="w-full">Sign Out</Button>
            </Card.Footer>

        {:else if confirmState}

            <Card.Header>
                <Card.Title>{mode === 'signIn' ? 'Sign In' : 'Sign Up'}</Card.Title>
                <Card.Description>{mode === 'signIn' ? 'Sign in to your account' : 'Create a new account'}</Card.Description>
            </Card.Header>

            <Card.Content onsubmit={handleSubmit}>
                <form>
                    <InputOTP.Root maxlength={6} bind:value={otp} class="justify-center">
                        {#snippet children({cells})}
                            <InputOTP.Group>
                                {#each cells.slice(0, 3) as cell (cell)}
                                    <InputOTP.Slot {cell}/>
                                {/each}
                            </InputOTP.Group>
                            <InputOTP.Separator/>
                            <InputOTP.Group>
                                {#each cells.slice(3, 6) as cell (cell)}
                                    <InputOTP.Slot {cell}/>
                                {/each}
                            </InputOTP.Group>
                        {/snippet}
                    </InputOTP.Root>
                </form>
            </Card.Content>

            <Card.Footer class="flex-col gap-2">
                <Button type="submit" disabled={loading} class="w-full" onclick={handleSubmit}>
                    {mode === 'signIn' ? 'Confirm Sign In' : 'Confirm Sign Up'}
                </Button>
            </Card.Footer>

        {:else if $currentUser === null}

            <Card.Header>
                <Card.Title>{mode === 'signIn' ? 'Sign In' : 'Sign Up'}</Card.Title>
                <Card.Description>{mode === 'signIn' ? 'Sign in to your account' : 'Create a new account'}</Card.Description>
            </Card.Header>

            <Card.Content onsubmit={handleSubmit}>
                <form>
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
                    </div>
                </form>
            </Card.Content>

            <Card.Footer class="flex-col gap-2">
                <Button type="submit" disabled={loading} class="w-full" onclick={handleSubmit}>
                    {mode === 'signIn' ? 'Sign In' : 'Sign Up'}
                </Button>

                {#if !confirmState}
                    <p>
                        {#if mode === 'signIn'}
                            Don’t have an account?
                            <Button variant="link" onclick={toggle('signUp')}>Sign up</Button>
                        {:else}
                            Already have an account?
                            <Button variant="link" onclick={toggle('signIn')}>Sign in</Button>
                        {/if}
                    </p>
                {/if}

            </Card.Footer>
        {:else}
            <p>Loading…</p>
        {/if}
    </Card.Root>
</div>
