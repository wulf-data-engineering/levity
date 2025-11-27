---
description: Run the application locally
---

## Concepts

The application can be run locally and accessed via the browser.
AWS is emulated using LocalStack and cognito-local in docker compose.
`cdklocal` is used to deploy the infrastructure to LocalStack.
The backend lambdas and frontend are run with hot reloading.

**IMPORTANT:** In local development the sign-in screen is pre-filled with Email address and password of the test user.
Check if the fields are already filled. If they are, **DO NOT** enter the credentials again. Just click the "Sign In" button.
Only clear the values if you need to sign-in as a different user.

## Workflow

Run localstack and cognito-local:

```bash
docker-compose down # cognito-local tends to go into a bad state
docker-compose up
```

Start the backend lambdas in another terminal:

```bash
cd backend
cargo lambda watch
```

If you encounter port problems suggest `kill -9 $(lsof -ti:9000)` to the user.

Deploy the infrastructure in another terminal:

```bash
cd infrastructure
cdklocal deploy
```

Start the frontend in another terminal:

```bash
cd frontend
npm run dev
```

The application is accessible at `http://localhost:5173`.
