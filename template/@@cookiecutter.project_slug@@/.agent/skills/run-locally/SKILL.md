---
name: run-locally
description: Start the local development stack (LocalStack, cognito-local, backend cargo lambda watch, local CDK deploy, and frontend dev server).
---

# Run Locally

## Concepts

- AWS is emulated locally using **LocalStack** and **cognito-local** via `docker compose`.
- `npm run cdklocal:deploy` deploys the infrastructure to LocalStack.
- Backend Lambdas and frontend run with hot reloading.
- **Sign-in note**: In local development, the sign-in screen is pre-filled with the test user's email and password. If already filled, just click **Sign In** rather than re-entering credentials.

## Steps

1. **Start LocalStack and `cognito-local`**:
   ```bash
   docker compose down  # cognito-local tends to go into a bad state
   docker compose up
   ```
   If `docker compose down` hits container conflicts:
   ```bash
   docker rm -f @@ cookiecutter.project_slug @@-localstack @@ cookiecutter.project_slug @@-cognito-local-init @@ cookiecutter.project_slug @@-cognito-local
   ```
   *(If you ran `docker rm`, you must re-bootstrap CDK in Step 3.)*

2. **Start Backend Lambdas** (separate terminal):
   ```bash
   cd backend
   ./cargo-lambda-watch.sh
   ```
   If you hit port conflicts, check `.env` (using `read_file`) and suggest `kill -9 $(lsof -ti:${BACKEND_PORT:-9000})` to the user.

3. **Deploy Local Infrastructure** (separate terminal):
   ```bash
   cd infrastructure
   npm run cdklocal:bootstrap  # required after docker rm or if deploy fails
   npm run cdklocal:deploy
   ```

4. **Start Frontend** (separate terminal):
   ```bash
   cd frontend
   npm run dev
   ```

The app is available at `http://localhost:${FRONTEND_PORT:-5173}` (check `.env` for custom ports).
