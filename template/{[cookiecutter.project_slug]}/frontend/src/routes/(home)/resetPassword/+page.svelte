<script lang="ts">
    import {page} from "$app/state";
    import {Button} from "$lib/components/ui/button";
    import {Label} from "$lib/components/ui/label";
    import {Input} from "$lib/components/ui/input";
    import * as Card from "$lib/components/ui/card";
    import {requestPasswordReset} from "./request";

    let email = $state(page.url.searchParams.get("email") || '');

    let submitting = $state(false);

    async function handleSubmit(e: Event) {
        e.preventDefault();
        submitting = true;
        try {
            await requestPasswordReset(email)
        } finally {
            submitting = false;
        }
    }
</script>

<Card.Root class="w-full max-w-sm m-auto mt-5">

    <Card.Header>
        <Card.Title>Reset Password</Card.Title>
        <Card.Description>Enter your Email address to reset your password</Card.Description>
    </Card.Header>

    <Card.Content>
        <form id="form" onsubmit={handleSubmit}>
            <div class="flex flex-col gap-6">
                <div class="grid gap-2">
                    <Label for="email">Email</Label>
                    <Input id="email" type="email" bind:value={email} required/>
                </div>
            </div>
        </form>
    </Card.Content>

    <Card.Footer class="flex-col gap-2">
        <Button disabled={submitting} class="w-full" type="submit" form="form">
            Reset Password
        </Button>
    </Card.Footer>
</Card.Root>
