<script lang="ts">
	// @ts-expect-error - Paraglide generates JS with JSDoc
	import * as m from '$lib/paraglide/messages.js';
	let { children } = $props();
</script>

<svelte:head>
	<title>{m.layout_title()}</title>
</svelte:head>
<div class="m-auto max-w-4xl p-5 px-8">
	<div>
		<h1 class="scroll-m-20 text-4xl font-extrabold tracking-tight lg:text-5xl">
			{m.layout_title()}
		</h1>
		<p class="text-xl leading-7 text-muted-foreground [&:not(:first-child)]:mt-6">
			{m.layout_desc()}
		</p>
	</div>
	{@render children?.()}
</div>
