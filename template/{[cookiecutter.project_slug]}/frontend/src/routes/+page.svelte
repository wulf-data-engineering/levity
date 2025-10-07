<script lang="ts">
    import {dev} from '$app/environment';
    import * as auth from '$lib/auth';
    import {currentUser} from '$lib/auth';
    import {page} from "$app/state";
    import {goto} from "$app/navigation";

    function handleError(err: unknown) {
        setMessage(err instanceof Error ? err.message : String(err), true);
    }

    function setMessage(text: string, error = false) {
        loading = false;
        message = [text, error];
    }

    function indicateLoading() {
        loading = true;
        message = null;
    }

    async function signUp() {
        indicateLoading();
        try {
            message = null;
            const result = await auth.signUp(email, password);
            if (result.isSignUpComplete) setMessage('Successfully signed up and signed in.');
            else setMessage('Successfully signed up. Confirmation required.');
            console.log('Signed up:', result);
        } catch (err) {
            console.error('Error signing up:', err);
            handleError(err);
        }
    }

    async function signIn() {
        indicateLoading();
        try {
            message = null;
            const result = await auth.signIn(email, password);
            if (result.isSignedIn) {
                // Redirect to the originally requested page
                const redirectTo = page.url.searchParams.get('redirectTo');
                if (redirectTo)
                    // eslint-disable-next-line svelte/no-navigation-without-resolve
                    goto(redirectTo);
                else
                    setMessage('Successfully signed in.');
            } else
                setMessage('Please complete the sign-up process.', true);
            console.log('Signed in:', result);
        } catch (err) {
            console.error('Error signing in:', err);
            handleError(err);
        }
    }

    async function signOut() {
        indicateLoading();
        try {
            await auth.signOut();
            setMessage('Successfully signed out.');
        } catch (err) {
            console.error('Error signing out:', err);
            handleError(err);
        } finally {
            loading = false;
        }
    }

    let mode: 'signIn' | 'signUp' = $state('signIn');
    let message: [message: string, error: boolean] | null = $state(null);
    let loading = $state(false);
    let email = $state(dev ? 'test@wulf.technology' : '');
    let password = $state(dev ? 'Password123!' : '');

    function handleSubmit(e: SubmitEvent) {
        e.preventDefault();
        if (mode === 'signIn') signIn();
        else signUp();
    }

    function toggle(newMode: 'signIn' | 'signUp') {
        return (e: Event) => {
            e.preventDefault();
            mode = newMode;
            message = null;
        };
    }
</script>

<svelte:head>
    <title>Home</title>
    <meta name="description" content="A Tool-Set frontend application."/>
</svelte:head>

<div>
    <h1>Welcome to test-frontend</h1>
    <p>This is a sample landing page.</p>

    {#if $currentUser}
        <p>
            You are signed in as {$currentUser.signInDetails?.loginId}
            <small>{$currentUser.userId}</small>.
        </p>

        <button onclick={signOut} disabled={loading}> Sign Out</button>
        {#if loading}
            Please wait…
        {/if}
    {:else if $currentUser === null}
        <h1>{mode === 'signIn' ? 'Sign In' : 'Sign Up'}</h1>

        <form onsubmit={handleSubmit}>
            <div>
                <label for="username">Username</label><br/>
                <input id="username" bind:value={email} required/>
            </div>

            <div>
                <label for="password">Password</label><br/>
                <input id="password" type="password" bind:value={password} required/>
            </div>

            <button type="submit" disabled={loading}>
                {#if loading}
                    Please wait…
                {:else}
                    {mode === 'signIn' ? 'Sign In' : 'Sign Up'}
                {/if}
            </button>
        </form>

        <p>
            {#if mode === 'signIn'}
                Don’t have an account?
                <button class="link-button" onclick={toggle('signUp')}>Sign up</button>
            {:else}
                Already have an account?
                <button class="link-button" onclick={toggle('signIn')}>Sign in</button>
            {/if}
        </p>

        {#if message}
            <p style="margin-top: 1rem; color: {message[1] ? 'red' : 'green'};">
                {message[0]}
            </p>
        {/if}
    {:else}
        <p>Loading…</p>
    {/if}
</div>
