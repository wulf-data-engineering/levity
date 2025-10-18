use anyhow::{anyhow, Context, Result};
use aws_sdk_cognitoidentityprovider as cognito_idp;
use backend::{json_response, load_aws_cognito_config};
use lambda_http::{run, service_fn, tracing, Body, Error, Request, Response};
use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PasswordPolicy {
    minimum_length: i32,
    require_uppercase: bool,
    require_lowercase: bool,
    require_numbers: bool,
    require_symbols: bool,
}

#[derive(Clone)]
struct AppState {
    client: cognito_idp::Client,
    user_pool_id: String,
}

///
/// This Lambda function retrieves the password policy for a specified AWS Cognito User Pool.
/// The User Pool ID is provided via the USER_POOL_ID environment variable.
/// In local development, it uses the default local user pool id if not set.
///
#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let shared_cfg = load_aws_cognito_config().await;

    let client = cognito_idp::Client::new(&shared_cfg);

    let user_pool_id = std::env::var("USER_POOL_ID")
        .ok()
        .or_else(default_user_pool_id)
        .ok_or_else(|| anyhow::anyhow!("USER_POOL_ID env var is required"))?;

    let state = AppState {
        client,
        user_pool_id,
    };

    // Pass state into the handler via a cloning closure
    run(service_fn(move |req| {
        let state = state.clone();
        async move { password_policy_handler(req, state).await }
    }))
    .await
}

async fn password_policy_handler(_req: Request, state: AppState) -> Result<Response<Body>, Error> {
    let policy = get_password_policy(&state).await?;
    json_response(policy)
}

async fn get_password_policy(state: &AppState) -> Result<PasswordPolicy> {
    let resp = state
        .client
        .describe_user_pool()
        .user_pool_id(&state.user_pool_id)
        .send()
        .await
        .context("failed to call DescribeUserPool")?;

    let up = resp
        .user_pool()
        .ok_or_else(|| anyhow!("DescribeUserPool: missing user_pool"))?;

    let policies = up
        .policies()
        .ok_or_else(|| anyhow!("DescribeUserPool: missing policies"))?;

    let p = policies
        .password_policy()
        .ok_or_else(|| anyhow!("DescribeUserPool: missing password_policy"))?;

    Ok(PasswordPolicy {
        minimum_length: p.minimum_length().unwrap_or(8),
        require_uppercase: p.require_uppercase,
        require_lowercase: p.require_lowercase,
        require_numbers: p.require_numbers,
        require_symbols: p.require_symbols,
    })
}

#[cfg(debug_assertions)]
fn default_user_pool_id() -> Option<String> {
    Some("local_userPool".into())
}

#[cfg(not(debug_assertions))]
fn default_user_pool_id() -> Option<String> {
    None
}
