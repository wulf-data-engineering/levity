use aws_config::{BehaviorVersion, ConfigLoader, Region, SdkConfig};
use aws_sdk_ssm::Client as SsmClient;

const DEFAULT_REGION: &str = "eu-central-1";

// LocalStack endpoint for local development and integration testing
#[cfg(any(debug_assertions, test))]
fn default_endpoint() -> Option<String> {
    Some(format!("http://localhost:{}", std::env::var("LOCALSTACK_PORT").unwrap_or_else(|_| "4566".to_string())))
}

#[cfg(not(any(debug_assertions, test)))]
fn default_endpoint() -> Option<String> {
    None
}

// cognito-local endpoint for local development and integration testing
#[cfg(any(debug_assertions, test))]
fn default_cognito_endpoint() -> Option<String> {
    Some(format!("http://localhost:{}", std::env::var("COGNITO_LOCAL_PORT").unwrap_or_else(|_| "9229".to_string())))
}

#[cfg(not(any(debug_assertions, test)))]
fn default_cognito_endpoint() -> Option<String> {
    None
}

#[cfg(any(debug_assertions, test))]
fn set_default_credentials_provider(config_loader: ConfigLoader) -> ConfigLoader {
    config_loader.credentials_provider(
        aws_credential_types::Credentials::builder()
            .access_key_id("test")
            .secret_access_key("test")
            .provider_name("dev")
            .build(),
    )
}

#[cfg(not(any(debug_assertions, test)))]
fn set_default_credentials_provider(config_loader: ConfigLoader) -> ConfigLoader {
    config_loader
}

/// Load AWS SDK configuration.
/// Uses the ENDPOINT_URL environment variable if set.
/// In debug builds, defaults to LocalStack endpoint if not set and local credentials.
pub async fn load_aws_config() -> SdkConfig {
    let endpoint = std::env::var("ENDPOINT_URL").ok().or_else(default_endpoint);

    load_aws_config_for_endpoint(endpoint).await
}

/// Load AWS SDK configuration.
/// Uses the COGNITO_ENDPOINT_URL environment variable if set.
/// In debug builds, defaults to cognito-local endpoint if not set and local credentials.
pub async fn load_aws_cognito_config() -> SdkConfig {
    let endpoint = std::env::var("COGNITO_ENDPOINT_URL")
        .ok()
        .or_else(default_cognito_endpoint);

    load_aws_config_for_endpoint(endpoint).await
}

async fn load_aws_config_for_endpoint(endpoint: Option<String>) -> SdkConfig {
    let mut loader = aws_config::defaults(BehaviorVersion::latest());
    if let Some(url) = &endpoint {
        loader = loader.endpoint_url(url);
    }
    loader = set_default_credentials_provider(loader);
    let region_name = std::env::var("AWS_REGION").unwrap_or(DEFAULT_REGION.to_string());

    loader = loader.region(Region::new(region_name));
    loader.load().await
}

///
/// Unit tests can use this loader to get default credentials and using given mock server.
///
/// ```
/// use wiremock::matchers::{method, path, header};
/// use wiremock::{MockServer, Mock, ResponseTemplate};
/// use aws_sdk_cognitoidentityprovider::Client;
///
///
/// #[tokio::test]
/// async fn some_test() {
///     // 1) Start mock server
///     let server = MockServer::start().await;
///     // define required routes
///
///     // 2) Create client
///     let shared_cfg = load_aws_config_for_mock(server).await;
///     let client = cognito_idp::Client::new(&shared_cfg);
///     // test your component that uses the client
/// }
/// ```
///
#[cfg(any(debug_assertions, test))]
pub async fn load_aws_config_for_mock(mock_server: &wiremock::MockServer) -> SdkConfig {
    load_aws_config_for_endpoint(Some(mock_server.uri())).await
}


/// This is a wrapper around an SSM parameter that caches the value in release mode on first access,
/// but always fetches the freshest value in dev/debug mode (to survive localstack redeployments).
#[derive(Clone)]
pub enum SsmParameter {
    Aws {
        config: SdkConfig,
        name: String,
        value: std::sync::Arc<tokio::sync::OnceCell<String>>,
    },
    #[cfg(any(debug_assertions, test))]
    FixedSsmValue(String),
}

impl SsmParameter {
    pub fn new(config: &SdkConfig, name: impl Into<String>) -> Self {
        Self::Aws {
            config: config.clone(),
            name: name.into(),
            value: std::sync::Arc::new(tokio::sync::OnceCell::new()),
        }
    }

    #[cfg(any(debug_assertions, test))]
    pub fn fixed(value: impl Into<String>) -> Self {
        Self::FixedSsmValue(value.into())
    }

    /// Gets the value of the SSM parameter.
    pub async fn get(&self) -> anyhow::Result<String> {
        match self {
            Self::Aws { config, name, value } => {
                #[cfg(not(any(debug_assertions, test)))]
                {
                    let val = value.get_or_try_init(|| async {
                        get_ssm_parameter(config, name).await
                    }).await?;
                    Ok(val.clone())
                }
                #[cfg(any(debug_assertions, test))]
                {
                    let _ = value; // Avoid unused field warning
                    // In debug/test, always fetch fresh to survive localstack restarts
                    get_ssm_parameter(config, name).await
                }
            }
            #[cfg(any(debug_assertions, test))]
            Self::FixedSsmValue(val) => Ok(val.clone()),
        }
    }
}

/// Helper to pull strings from AWS Systems Manager Parameter Store
pub async fn get_ssm_parameter(config: &SdkConfig, name: &str) -> anyhow::Result<String> {
    let ssm_client = SsmClient::new(config);
    let value = ssm_client
        .get_parameter()
        .name(name)
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to get SSM parameter '{}': {:?}", name, e))?
        .parameter()
        .ok_or_else(|| anyhow::anyhow!("SSM parameter '{}' not found in response", name))?
        .value()
        .ok_or_else(|| anyhow::anyhow!("SSM parameter '{}' has empty value", name))?
        .to_string();

    Ok(value)
}
