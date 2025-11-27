---
description: Run the application locally
---

## Concepts

The application can be run locally and accessed via the browser.
AWS is emulated using LocalStack and cognito-local in docker compose.
`cdklocal` is used to deploy the infrastructure to LocalStack.
The backend lambdas and frontend are run with hot reloading.

## Workflow

Run localstack and cognito-local:

```bash
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
