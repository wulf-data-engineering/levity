<script lang="ts">

    let {children} = $props();

</script>

<svelte:head>
    <title>%[cookiecutter.project_name]%</title>
</svelte:head>


<div class="m-auto max-w-4xl px-8 p-5">
    <div>
        <h1 class="scroll-m-20 text-4xl font-extrabold tracking-tight lg:text-5xl">
            %[cookiecutter.project_name]%
        </h1>
        <p class="text-muted-foreground text-xl leading-7 [&:not(:first-child)]:mt-6">
            This is a sample landing page.
        </p>
    </div>

    {@render children?.()}
</div>
