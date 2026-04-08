<script lang="ts">
    import { onMount } from 'svelte';
    // @ts-ignore
    import init, { parse_markdown } from '../../../../frontend-wasm/markdown-parser/pkg/markdown_parser.js';

    let markdownInput = $state('# Hello WASM\n\nWelcome to the **fast** Markdown parser powered by Rust and Web Assembly!');
    let ready = $state(false);
    let htmlOutput = $derived(ready ? parse_markdown(markdownInput) : '');

    onMount(async () => {
        try {
            await init();
            ready = true;
        } catch (e) {
            console.error("Failed to load wasm", e);
        }
    });
</script>

<svelte:head>
    <title>WASM Markdown Parser | %[ cookiecutter.project_name ]%</title>
</svelte:head>

<div class="container mx-auto p-8 max-w-4xl h-full flex flex-col pt-24">
    <div class="mb-8">
        <h1 class="text-3xl font-bold mb-2">WASM Markdown Parser</h1>
        <p class="text-gray-600">
            This module leverages a high-performance Rust Markdown parser compiled to Web Assembly
            to render HTML instantly and securely on the client side.
        </p>
    </div>
    
    {#if !ready}
        <div class="flex items-center justify-center p-12 bg-gray-50 rounded-lg border">
            <p class="text-lg text-gray-500 animate-pulse">Loading Web Assembly module...</p>
        </div>
    {:else}
        <div class="grid grid-cols-1 md:grid-cols-2 gap-8 flex-grow">
            <div class="flex flex-col h-full">
                <label for="markdown" class="block font-medium text-gray-700 mb-2">Markdown Input</label>
                <textarea 
                    id="markdown" 
                    bind:value={markdownInput} 
                    class="flex-grow w-full min-h-[400px] p-4 border rounded-md shadow-sm focus:ring-primary focus:border-primary font-mono text-sm"
                ></textarea>
            </div>
            <div class="flex flex-col h-full">
                <h2 class="font-medium text-gray-700 mb-2">Live HTML Preview</h2>
                <div class="flex-grow w-full min-h-[400px] p-6 border rounded-md shadow-sm overflow-y-auto bg-gray-50 prose prose-primary max-w-none">
                    {@html htmlOutput}
                </div>
            </div>
        </div>
    {/if}
</div>
