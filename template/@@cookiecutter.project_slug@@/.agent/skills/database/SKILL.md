---
name: Database Development
description: Patterns for DynamoDB access and repositories
---

# Database Rules

## Repositories

Repositories abstract the DynamoDB access. They should be implemented in `backend/src/shared/<entity>.rs`.

### UserRepo

The `UserRepo` handles access to the `users` table.

```rust
pub struct UserRepo {
    client: Client,
    table_name: crate::shared::aws_config::SsmParameter,
}

impl UserRepo {
    pub fn new(client: Client, table_name: crate::shared::aws_config::SsmParameter) -> Self;
    pub async fn insert(&self, user_data: UserData) -> Result<(), anyhow::Error>;
    pub async fn read(&self, username: &str) -> Result<Option<User>, anyhow::Error>;
    // Example for a table specific GSI
    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, anyhow::Error>;
    pub async fn update(&self, user_data: UserData, version: u16) -> Result<(), anyhow::Error>;
    pub async fn delete(&self, username: String, version: u16) -> Result<(), anyhow::Error>;
}
```

## Dependency Injection

To use repositories in Lambda functions, follow this pattern:

1.  **Infrastructure**: Create the `VersionedTable` and an SSM `StringParameter` holding the table name, then pass both to the Lambda construct.

    ```typescript
    // infrastructure/lib/constructs/backend.ts
    const usersTable = new VersionedTable(this, 'UsersTable', { ... });
    const usersTableParam = new ssm.StringParameter(this, 'UsersTableParam', {
      parameterName: '/@@ cookiecutter.project_slug @@/users-table',
      stringValue: usersTable.tableName,
    });
    const identity = new Identity(this, 'Identity', {
      deploymentConfig,
      usersTable,
      usersTableParam,
    });
    ```

2.  **Lambda Construct**: Grant both DynamoDB table data permissions and SSM parameter read permissions to the Lambda.

    ```typescript
    // infrastructure/lib/constructs/backend/identity.ts
    this.cognitoHandler = backendLambda(this, "CognitoHandlerFunction", {
      deploymentConfig: props.deploymentConfig,
      binaryName: "cognito-handler",
    });
    props.usersTable.grantReadWriteData(this.cognitoHandler);
    props.usersTableParam.grantRead(this.cognitoHandler);
    ```

3.  **Lambda Main**: Initialize `SsmParameter` (which caches in release mode and fetches fresh in debug/test mode so LocalStack redeployments work seamlessly) and create the repository.

    ```rust
    // backend/src/cognito-handler.rs
    #[tokio::main]
    async fn main() -> Result<(), Error> {
        backend::shared::lambda::init_logger();

        let config = load_aws_config().await;
        let table_name = backend::shared::aws_config::SsmParameter::new(
            &config,
            "/@@ cookiecutter.project_slug @@/users-table",
        );

        let client = Client::new(&config);
        let repo = backend::shared::users::UserRepo::new(client, table_name);

        run(service_fn(move |event| {
            let repo = repo.clone();
            let config = config.clone();
            async move { function_handler(event, &repo, &config).await }
        })).await
    }
    ```

4.  **Lambda Handler**: Accept the repository as an argument.
    ```rust
    async fn function_handler(
        event: LambdaEvent<CognitoUserPoolEvent>,
        repo: &UserRepo,
    ) -> Result<CognitoUserPoolEvent, Error> { ... }
    ```
