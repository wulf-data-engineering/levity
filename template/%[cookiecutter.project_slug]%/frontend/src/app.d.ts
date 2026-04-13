// See https://svelte.dev/docs/kit/types#app.d.ts
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		// interface Locals {}
		// interface PageData {}
		// interface PageState {}
		// interface Platform {}
	}
}

declare module '$lib/paraglide/runtime' {
    export function languageTag(): string;
    export const sourceLanguageTag: string;
    export const availableLanguageTags: readonly string[];
}

export {};
