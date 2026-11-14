---
name: test-e2e
description: Write, update, and run Playwright end-to-end tests in frontend/tests/e2e/.
---

# End-to-End Testing

The stack uses **Playwright** for end-to-end testing.

## Steps

1. Ensure the application is running locally as described in `@../run-locally/SKILL.md` (`/run-locally`).
2. Add or update Playwright tests in `frontend/tests/e2e/`.
3. Run the end-to-end tests:
   ```bash
   cd frontend
   npx playwright install
   npm run test:e2e
   ```
