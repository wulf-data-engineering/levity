---
description: Translate all end-user texts (frontend and backend) to a newly requested language.
---
# Setup a New Language

Use this workflow when the user asks to "Translate the app to [Language]" or "Add [Language] support".

> [!NOTE]
> **I18N Architecture**: This project implements a Single-Page Application (SPA) compatible localization strategy. 
> - **Frontend**: Uses `paraglide-js` (NOT `paraglide-sveltekit`). We DO NOT use language subpaths (e.g., `/de/about`). Instead, layout language discovery happens dynamically via `navigator.language` on the client in `+layout.svelte`. Never attempt to guess or enforce a `/<language>/path` strategy.
> - **Backend**: Uses rust-i18n. Messages are resolved via the `Accept-Language` HTTP header or from the authenticated user's profile database entry.

1. **Frontend (`frontend/messages/`)**:
   - Locate the English base `en.json`.
   - Create a new file for the requested language (e.g., `de.json`).
   - Copy all keys from `en.json` and use your internal LLM capabilities to translate their values to the target language contextually.
   - Run `npx paraglide-js compile --project ./project.inlang` (or `npm run dev`) to ensure typings are updated.

2. **Backend (`backend/locales/`)**:
   - Locate the English base `en.yml`.
   - Create a new file for the requested language (e.g., `de.yml`).
   - Translate all the string values into the target language.

3. **Verify Layout (CRITICAL)**:
   - Some languages (like German) have words that are much longer than English on average.
   - Start the frontend (if not running) and open the browser visually using your tools.
   - Check the primary pages to ensure no UI components break, overlap, or overflow due to text expansion. Fix CSS as needed.
