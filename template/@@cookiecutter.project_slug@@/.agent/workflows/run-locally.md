---
description: Run the application locally
---

## Concepts

The application can be run locally and accessed via the browser.
AWS is emulated using LocalStack and cognito-local in docker compose.
`npm run cdklocal:deploy` is used to deploy the infrastructure to LocalStack.
The backend lambdas and frontend are run with hot reloading.

**IMPORTANT:** In local development the sign-in screen is pre-filled with Email address and password of the test user.
Check if the fields are already filled. If they are, **DO NOT** enter the credentials again. Just click the "Sign In" button.
Only clear the values if you need to sign-in as a different user.

## Workflow

Run localstack and cognito-local:

```bash
// turbo
docker compose down # cognito-local tends to go into a bad state
docker compose up
```

If `docker compose down` fails to clean up or you see container conflicts, use:

```bash
docker rm -f @@ cookiecutter.project_slug @@-localstack @@ cookiecutter.project_slug @@-cognito-local-init @@ cookiecutter.project_slug @@-cognito-local
```

If you had to run `docker rm` you definitely need to bootstrap cdk.

Start the backend lambdas in another terminal:

```bash
// turbo
cd backend
./cargo-lambda-watch.sh
```

If you encounter port problems suggest `kill -9 $(lsof -ti:${BACKEND_PORT:-9000})` to the user. Always use the port from the `.env` file instead of by name.

Deploy the infrastructure in another terminal:

````bash
// turbo
cd infrastructure
npm run cdklocal:bootstrap # if you had to run docker rm before or deploy fails
npm run cdklocal:deploy
````

Start the frontend in another terminal:

```bash
// turbo
cd frontend
npm run dev
```

The application is accessible at `http://localhost:${FRONTEND_PORT:-5173}` (check your `.env` for the configured port).