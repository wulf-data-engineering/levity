<script lang="ts">
	import { Label } from '#lib/components/ui/label/index.js';
	import { Input } from '#lib/components/ui/input/index.js';
	import type { WithElementRef } from '#lib/utils.js';
	import type { HTMLInputAttributes, HTMLInputTypeAttribute } from 'svelte/elements';
	import type { ValidatedFormContext } from '#lib/components/validatedForm/validatedForm.svelte';
	import { getContext, onDestroy, onMount } from 'svelte';

	// A string is an error message, null/undefined means valid.
	// An empty string will raise invalidation but show no message.
	export type Validation = (value: string) => string | null | undefined;

	type InputType = Exclude<HTMLInputTypeAttribute, 'file'>;

	type ValidationProps = {
		label?: string; // label text above the input
		info?: string; // informational text below the input (if no error)
		validations?: Validation[]; // list of validations to run
	};

	type Props = WithElementRef<
		ValidationProps &
			Omit<HTMLInputAttributes, 'type'> &
			({ type: 'file'; files?: FileList } | { type?: InputType; files?: undefined })
	>;

	let {
		ref = $bindable<HTMLInputElement | null>(null),
		value = $bindable(),
		files = $bindable(),
		type,
		id,
		label,
		info,
		validations = [],
		...restProps
	}: Props = $props();

	// Internal
	let touched = $state(false);

	function firstError(v: string): string | null {
		for (const validate of validations) {
			try {
				const msg = validate?.(v || '');
				if (typeof msg === 'string') return msg;
			} catch {
				// ignore validator errors
			}
		}
		if (restProps.required && (v || '').length === 0) {
			return '';
		}

		return null;
	}

	if (!id) {
		throw new Error("ValidatedInput requires an 'id' prop for label and validation.");
	}

	const error = $derived(firstError(value));
	const shouldValidate = $derived(touched);
	const isInvalid = $derived(error !== undefined && error != null && shouldValidate);

	const message = $derived((isInvalid && error?.length) || 0 > 0 ? error : info);
	const messageId = `${id}-msg`;

	// Interface for ValidatedForm

	function check(): boolean {
		touched = true;
		return error == null;
	}

	function focus() {
		ref?.focus();
	}

	const ctx: ValidatedFormContext | null = getContext<ValidatedFormContext>('ValidatedFormContext');

	let unregister: (() => void) | null = null;

	// Register with the nearest ValidatedForm if present
	onMount(() => {
		if (ctx?.register) {
			unregister = ctx.register({ id, check, focus });
		}
	});

	onDestroy(() => {
		unregister?.();
	});
</script>

<div class="grid gap-2">
	{#if label}
		<Label for={id} class="mb-1">{label}</Label>
	{/if}

	{#if type === 'file'}
		<Input
			{id}
			type="file"
			{...restProps}
			bind:ref
			bind:value
			bind:files
			aria-invalid={isInvalid ? 'true' : undefined}
			aria-describedby={message ? messageId : undefined}
			onblur={() => (touched = true)}
			oninvalid={() => (touched = true)}
		/>
	{:else}
		<Input
			{id}
			{type}
			{...restProps}
			bind:ref
			bind:value
			aria-invalid={isInvalid ? 'true' : undefined}
			aria-describedby={message ? messageId : undefined}
			onblur={() => (touched = true)}
			oninvalid={() => (touched = true)}
		/>
	{/if}

	{#if message}
		<small id={messageId} class={isInvalid ? 'text-destructive' : 'text-muted-foreground'}>
			{message}
		</small>
	{/if}
</div>
