---
name: translate-app
description: Translate all end-user texts (frontend ParaglideJS and backend rust-i18n) to a newly requested language and verify UI layout.
---

# Setup a New Language

Use this skill when asked to translate the app or add support for a new language.

> [!NOTE]
> **I18N Architecture**:
> - **Frontend**: Uses `paraglide-js` (not `paraglide-sveltekit`). Do **not** use `/<lang>/path` URL subpaths; language discovery happens client-side via `navigator.language` in `+layout.svelte`.
> - **Backend**: Uses `rust-i18n`, resolved via the `Accept-Language` header or authenticated user profile.

## Steps

1. **Frontend (`frontend/messages/`)**:
   - Locate `en.json` and create `<lang>.json` (e.g., `de.json`).
   - Copy all keys and translate all values contextually.
   - Run `npx paraglide-js compile --project ./project.inlang` (or `npm run dev`) to regenerate typings.

2. **Backend (`backend/locales/`)**:
   - Locate `en.yml` and create `<lang>.yml` (e.g., `de.yml`).
   - Translate all string values into the target language.

3. **Verify Layout (CRITICAL)**:
   - Languages like German often have longer words than English.
   - Start the frontend and inspect primary pages visually in the browser to ensure no UI components break, overlap, or overflow. Adjust CSS as needed.
