use aws_lambda_events::event::cognito::{
    CognitoEventUserPoolsCustomMessageTriggerSource,
    CognitoEventUserPoolsPostConfirmationTriggerSource,
};
use aws_sdk_dynamodb::Client;
use backend::{load_aws_config, CognitoUserPoolEvent};
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use std::sync::atomic::{AtomicBool, Ordering};

rust_i18n::i18n!("locales", fallback = "en");

pub mod protocols {
    include!(concat!(env!("OUT_DIR"), "/sign_up_data.rs"));
}
pub use protocols::*;

const DEFAULT_LANG: &str = "en";

static IS_SANDBOX: AtomicBool = AtomicBool::new(false);
static IS_INITIALIZED: AtomicBool = AtomicBool::new(false);

async fn check_ses_sandbox(config: &aws_config::SdkConfig) -> bool {
    if IS_INITIALIZED.load(Ordering::Relaxed) {
        return IS_SANDBOX.load(Ordering::Relaxed);
    }

    let client = aws_sdk_ses::Client::new(config);
    let is_sandbox = match client.get_send_quota().send().await {
        Ok(quota) => {
            quota.max24_hour_send() <= 200.0
        }
        Err(e) => {
            tracing::warn!("Failed to query SES send quota: {:?}", e);
            false
        }
    };

    IS_SANDBOX.store(is_sandbox, Ordering::Relaxed);
    IS_INITIALIZED.store(true, Ordering::Relaxed);
    is_sandbox
}

///
/// This lambda reacts on Cognito's lifecycle events.
///
/// The default version stores sign up data in the users table at post confirmation of sign up.
/// It also renders signup and forgot password emails using rust-i18n.
/// If SES is still in sandbox mode and the domain is not verified, it will return an error.
///
/// If you add more cases, make sure to add them to local/cognito-local-volume/config.json
/// and to infrastructure/lib/constructs/backend/identity.ts
///
async fn function_handler(
    event: LambdaEvent<CognitoUserPoolEvent>,
    repo: &backend::shared::users::UserRepo,
    aws_config: &aws_config::SdkConfig,
) -> Result<CognitoUserPoolEvent, Error> {
    let mut cognito_event = event.payload;
    match &mut cognito_event {
        CognitoUserPoolEvent::PreSignup(pre_sign_up) => {
            // if SES is in sandbox mode and the email domain is not verified, return an error.
            let ses_domain = std::env::var("DOMAIN_NAME").unwrap_or_default();
            if !ses_domain.is_empty() && check_ses_sandbox(aws_config).await {
                if let Some(email) = pre_sign_up.request.user_attributes.get("email") {
                    if let Some(email_domain) = email.split('@').nth(1) {
                        if !email_domain.eq_ignore_ascii_case(&ses_domain) {
                            return Err(Error::from(backend::shared::i18n::translate_sandbox_error(
                                &ses_domain
                            )));
                        }
                    }
                }
            }
        }
        CognitoUserPoolEvent::PostConfirmation(post_confirmation) => {
            if post_confirmation
                .cognito_event_user_pools_header
                .trigger_source
                == Some(CognitoEventUserPoolsPostConfirmationTriggerSource::ConfirmSignUp)
            {
                // Write entry to the users table
                let sign_up_data =
                    extract_sign_up_data(&post_confirmation.request.client_metadata)?;
                verify_not_empty(&sign_up_data)?;

                let user_data = backend::shared::users::UserData {
                    username: post_confirmation
                        .request
                        .user_attributes
                        .get("sub")
                        .cloned()
                        .unwrap_or_default(),
                    email: post_confirmation
                        .request
                        .user_attributes
                        .get("email")
                        .cloned()
                        .unwrap_or_default(),
                    first_name: sign_up_data.first_name,
                    last_name: sign_up_data.last_name,
                    language: if sign_up_data.language.is_empty() { DEFAULT_LANG.to_string() } else { sign_up_data.language },
                };

                tracing::info!("Storing user: {:?}", user_data.username);

                if let Err(e) = repo.insert(user_data).await {
                    return Err(Error::from(format!("Failed to insert user: {:?}", e)));
                }
            }
        }
        CognitoUserPoolEvent::CustomMessage(custom_message) => {
            // Render confirm signup or forgot password emails using rust-i18n
            let language = custom_message
                .request
                .client_metadata
                .get("language")
                .map(|s| s.as_str())
                .unwrap_or(DEFAULT_LANG);
            rust_i18n::set_locale(language);

            let code = custom_message
                .request
                .code_parameter
                .as_deref()
                .unwrap_or_default();
            let trigger_source = custom_message
                .cognito_event_user_pools_header
                .trigger_source
                .as_ref();

            let name = extract_name(
                &custom_message.request.client_metadata,
                &custom_message.request.user_attributes,
                repo,
            )
            .await;

            let (signup_name, forgot_name) = if name.is_empty() {
                ("".to_string(), "".to_string())
            } else {
                (format!(", {}", name), format!(" {}", name))
            };

            match trigger_source {
                Some(CognitoEventUserPoolsCustomMessageTriggerSource::SignUp)
                | Some(CognitoEventUserPoolsCustomMessageTriggerSource::ResendCode) => {
                    custom_message.response.email_subject =
                        Some(rust_i18n::t!("signup_email.subject").to_string());
                    custom_message.response.email_message = Some(
                        rust_i18n::t!("signup_email.body", name = signup_name, code = code).to_string(),
                    );
                }
                Some(CognitoEventUserPoolsCustomMessageTriggerSource::ForgotPassword) => {
                    custom_message.response.email_subject =
                        Some(rust_i18n::t!("forgot_password_email.subject").to_string());
                    custom_message.response.email_message = Some(
                        rust_i18n::t!("forgot_password_email.body", name = forgot_name, code = code)
                            .to_string(),
                    );
                }
                _ => {}
            }
        }
        // Check the enum `CognitoUserPoolEvent` for more lifecycle events
        _ => {}
    }

    Ok(cognito_event)
}

fn extract_sign_up_data(
    client_metadata: &std::collections::HashMap<String, String>,
) -> Result<SignUpData, Error> {
    client_metadata
        .get("sign_up_data")
        .ok_or_else(|| Error::from("Missing sign_up_data in client metadata"))
        .and_then(|json| {
            serde_json::from_str(json)
                .map_err(|e| Error::from(format!("Failed to parse sign_up_data: {}", e)))
        })
}

fn verify_not_empty(data: &SignUpData) -> Result<(), Error> {
    if data.first_name.is_empty() || data.last_name.is_empty() {
        Err(Error::from("Missing firstName or lastName in sign_up_data"))
    } else {
        Ok(())
    }
}

/// Extracts the user's name from either the `sign_up_data` inside `client_metadata`
/// or standard Cognito `user_attributes` (checking `name` then `given_name`).
/// If not found in metadata/attributes, queries the user database.
async fn extract_name(
    client_metadata: &std::collections::HashMap<String, String>,
    user_attributes: &std::collections::HashMap<String, serde_json::Value>,
    repo: &backend::shared::users::UserRepo,
) -> String {
    let mut name = "".to_string();
    if let Some(sign_up_data_str) = client_metadata.get("sign_up_data") {
        if let Ok(data) = serde_json::from_str::<SignUpData>(sign_up_data_str) {
            name = data.first_name;
        }
    }
    if name.is_empty() {
        if let Some(attr_name) = user_attributes.get("name") {
            if let Some(s) = attr_name.as_str() {
                name = s.to_string();
            }
        } else if let Some(given_name) = user_attributes.get("given_name") {
            if let Some(s) = given_name.as_str() {
                name = s.to_string();
            }
        }
    }
    if name.is_empty() {
        if let Some(sub) = user_attributes
            .get("sub")
            .and_then(|v| v.as_str())
        {
            if let Ok(Some(user)) = repo.read(sub).await {
                name = user.data.first_name;
            }
        }
    }
    if name.is_empty() {
        if let Some(email) = user_attributes
            .get("email")
            .and_then(|v| v.as_str())
        {
            if let Ok(Some(user)) = repo.find_by_email(email).await {
                name = user.data.first_name;
            }
        }
    }
    name
}

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
    let aws_config = config.clone();

    run(service_fn(move |event| {
        let repo = repo.clone();
        let aws_config = aws_config.clone();
        async move { function_handler(event, &repo, &aws_config).await }
    }))
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use aws_lambda_events::cognito::{
        CognitoEventUserPoolsPostConfirmation, CognitoEventUserPoolsPostConfirmationRequest,
    };
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_post_confirmation_writes_to_dynamodb() {
        let server = MockServer::start().await;

        // Mock DynamoDB PutItem
        Mock::given(method("POST"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({})))
            .mount(&server)
            .await;

        let shared_config = backend::shared::aws_config::load_aws_config_for_mock(&server).await;
        let client = aws_sdk_dynamodb::Client::new(&shared_config);
        let table_name = backend::shared::aws_config::SsmParameter::fixed("users");
        let repo = backend::shared::users::UserRepo::new(client, table_name);

        let sign_up_data = serde_json::json!({
            "firstName": "Test",
            "lastName": "User",
            "language": "en"
        })
        .to_string();

        let mut client_metadata = std::collections::HashMap::new();
        client_metadata.insert("sign_up_data".to_string(), sign_up_data);

        let mut user_attributes = std::collections::HashMap::new();
        user_attributes.insert("sub".to_string(), "test-sub".to_string());
        user_attributes.insert("email".to_string(), "test@example.com".to_string());

        let mut request = CognitoEventUserPoolsPostConfirmationRequest::default();
        request.user_attributes = user_attributes;
        request.client_metadata = client_metadata;

        let mut post_confirmation = CognitoEventUserPoolsPostConfirmation::default();
        post_confirmation.request = request;
        post_confirmation
            .cognito_event_user_pools_header
            .trigger_source =
            Some(CognitoEventUserPoolsPostConfirmationTriggerSource::ConfirmSignUp);

        let event = LambdaEvent::new(
            CognitoUserPoolEvent::PostConfirmation(post_confirmation),
            Default::default(),
        );

        let result = function_handler(event, &repo, &shared_config).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_post_confirmation_forgot_password_does_not_write_to_dynamodb() {
        let server = MockServer::start().await;
        // No mocks mounted - if DB is called, it will crash/fail the test

        let shared_config = backend::shared::aws_config::load_aws_config_for_mock(&server).await;
        let client = aws_sdk_dynamodb::Client::new(&shared_config);
        let table_name = backend::shared::aws_config::SsmParameter::fixed("users");
        let repo = backend::shared::users::UserRepo::new(client, table_name);

        let mut post_confirmation = CognitoEventUserPoolsPostConfirmation::default();
        post_confirmation
            .cognito_event_user_pools_header
            .trigger_source =
            Some(CognitoEventUserPoolsPostConfirmationTriggerSource::ConfirmForgotPassword);

        let event = LambdaEvent::new(
            CognitoUserPoolEvent::PostConfirmation(post_confirmation),
            Default::default(),
        );

        let result = function_handler(event, &repo, &shared_config).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_custom_message_localization() {
        let server = MockServer::start().await;
        let shared_config = backend::shared::aws_config::load_aws_config_for_mock(&server).await;
        let client = aws_sdk_dynamodb::Client::new(&shared_config);
        let table_name = backend::shared::aws_config::SsmParameter::fixed("users");
        let repo = backend::shared::users::UserRepo::new(client, table_name);

        // 1. Test SignUp (en)
        {
            let mut client_metadata = std::collections::HashMap::new();
            client_metadata.insert("language".to_string(), "en".to_string());

            let mut user_attributes = std::collections::HashMap::<String, serde_json::Value>::new();
            user_attributes.insert("given_name".to_string(), serde_json::json!("Testname"));

            let mut request = aws_lambda_events::event::cognito::CognitoEventUserPoolsCustomMessageRequest::default();
            request.client_metadata = client_metadata;
            request.user_attributes = user_attributes;
            request.code_parameter = Some("123456".to_string());

            let mut custom_message =
                aws_lambda_events::event::cognito::CognitoEventUserPoolsCustomMessage::default();
            custom_message.request = request;
            custom_message
                .cognito_event_user_pools_header
                .trigger_source = Some(CognitoEventUserPoolsCustomMessageTriggerSource::SignUp);

            let event = LambdaEvent::new(
                CognitoUserPoolEvent::CustomMessage(custom_message),
                Default::default(),
            );

            let res = function_handler(event, &repo, &shared_config).await.unwrap();
            if let CognitoUserPoolEvent::CustomMessage(res_custom_message) = res {
                assert_eq!(
                    res_custom_message.response.email_subject,
                    Some("Confirm your @@ cookiecutter.project_name @@ account".to_string())
                );
                assert_eq!(
                    res_custom_message.response.email_message,
                    Some(
                        "Welcome, Testname! Your verification code is 123456."
                            .to_string()
                    )
                );
            } else {
                panic!("Unexpected event type");
            }
        }

        // 2. Test ForgotPassword (en)
        {
            let mut client_metadata = std::collections::HashMap::new();
            client_metadata.insert("language".to_string(), "en".to_string());

            let mut user_attributes = std::collections::HashMap::<String, serde_json::Value>::new();
            user_attributes.insert("name".to_string(), serde_json::json!("Testname"));

            let mut request = aws_lambda_events::event::cognito::CognitoEventUserPoolsCustomMessageRequest::default();
            request.client_metadata = client_metadata;
            request.user_attributes = user_attributes;
            request.code_parameter = Some("654321".to_string());

            let mut custom_message =
                aws_lambda_events::event::cognito::CognitoEventUserPoolsCustomMessage::default();
            custom_message.request = request;
            custom_message
                .cognito_event_user_pools_header
                .trigger_source =
                Some(CognitoEventUserPoolsCustomMessageTriggerSource::ForgotPassword);

            let event = LambdaEvent::new(
                CognitoUserPoolEvent::CustomMessage(custom_message),
                Default::default(),
            );

            let res = function_handler(event, &repo, &shared_config).await.unwrap();
            if let CognitoUserPoolEvent::CustomMessage(res_custom_message) = res {
                assert_eq!(
                    res_custom_message.response.email_subject,
                    Some("Reset your @@ cookiecutter.project_name @@ password".to_string())
                );
                assert_eq!(
                    res_custom_message.response.email_message,
                    Some(
                        "Hello Testname, your verification code to reset your password is 654321."
                            .to_string()
                    )
                );
            } else {
                panic!("Unexpected event type");
            }
        }

        // 3. Test ForgotPassword (name lookup from DB via sub)
        {
            let server = MockServer::start().await;

            Mock::given(method("POST"))
                .and(path("/"))
                .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                    "Item": {
                        "pk": {"S": "test-sub"},
                        "username": {"S": "test-sub"},
                        "email": {"S": "test@example.com"},
                        "first_name": {"S": "DbName"},
                        "last_name": {"S": "LastName"},
                        "language": {"S": "en"},
                        "data_version": {"N": "1"},
                        "last_write": {"N": "1234567890"}
                    }
                })))
                .mount(&server)
                .await;

            let shared_config = backend::shared::aws_config::load_aws_config_for_mock(&server).await;
            let client = aws_sdk_dynamodb::Client::new(&shared_config);
            let repo = backend::shared::users::UserRepo::new(
                client,
                backend::shared::aws_config::SsmParameter::fixed("users"),
            );

            let mut client_metadata = std::collections::HashMap::new();
            client_metadata.insert("language".to_string(), "en".to_string());

            let mut user_attributes = std::collections::HashMap::<String, serde_json::Value>::new();
            user_attributes.insert("sub".to_string(), serde_json::json!("test-sub"));

            let mut request = aws_lambda_events::event::cognito::CognitoEventUserPoolsCustomMessageRequest::default();
            request.client_metadata = client_metadata;
            request.user_attributes = user_attributes;
            request.code_parameter = Some("654321".to_string());

            let mut custom_message =
                aws_lambda_events::event::cognito::CognitoEventUserPoolsCustomMessage::default();
            custom_message.request = request;
            custom_message
                .cognito_event_user_pools_header
                .trigger_source =
                Some(CognitoEventUserPoolsCustomMessageTriggerSource::ForgotPassword);

            let event = LambdaEvent::new(
                CognitoUserPoolEvent::CustomMessage(custom_message),
                Default::default(),
            );

            let res = function_handler(event, &repo, &shared_config).await.unwrap();
            if let CognitoUserPoolEvent::CustomMessage(res_custom_message) = res {
                assert_eq!(
                    res_custom_message.response.email_message,
                    Some(
                        "Hello DbName, your verification code to reset your password is 654321."
                            .to_string()
                    )
                );
            } else {
                panic!("Unexpected event type");
            }
        }

        // 4. Test ForgotPassword (empty name)
        {
            let server = MockServer::start().await;
            // DB returns None
            Mock::given(method("POST"))
                .and(path("/"))
                .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({})))
                .mount(&server)
                .await;

            let shared_config = backend::shared::aws_config::load_aws_config_for_mock(&server).await;
            let client = aws_sdk_dynamodb::Client::new(&shared_config);
            let repo = backend::shared::users::UserRepo::new(
                client,
                backend::shared::aws_config::SsmParameter::fixed("users"),
            );

            let mut client_metadata = std::collections::HashMap::new();
            client_metadata.insert("language".to_string(), "en".to_string());

            let mut user_attributes = std::collections::HashMap::<String, serde_json::Value>::new();
            user_attributes.insert("sub".to_string(), serde_json::json!("test-sub"));

            let mut request = aws_lambda_events::event::cognito::CognitoEventUserPoolsCustomMessageRequest::default();
            request.client_metadata = client_metadata;
            request.user_attributes = user_attributes;
            request.code_parameter = Some("654321".to_string());

            let mut custom_message =
                aws_lambda_events::event::cognito::CognitoEventUserPoolsCustomMessage::default();
            custom_message.request = request;
            custom_message
                .cognito_event_user_pools_header
                .trigger_source =
                Some(CognitoEventUserPoolsCustomMessageTriggerSource::ForgotPassword);

            let event = LambdaEvent::new(
                CognitoUserPoolEvent::CustomMessage(custom_message),
                Default::default(),
            );

            let res = function_handler(event, &repo, &shared_config).await.unwrap();
            if let CognitoUserPoolEvent::CustomMessage(res_custom_message) = res {
                assert_eq!(
                    res_custom_message.response.email_message,
                    Some(
                        "Hello, your verification code to reset your password is 654321."
                            .to_string()
                    )
                );
            } else {
                panic!("Unexpected event type");
            }
        }
    }

    #[tokio::test]
    async fn test_extract_name() {
        let server = MockServer::start().await;
        let shared_config = backend::shared::aws_config::load_aws_config_for_mock(&server).await;
        let client = aws_sdk_dynamodb::Client::new(&shared_config);
        let repo = backend::shared::users::UserRepo::new(
            client,
            backend::shared::aws_config::SsmParameter::fixed("users"),
        );

        // Case 1: extract name from sign_up_data in client_metadata
        let mut client_metadata = std::collections::HashMap::new();
        let sign_up_data = serde_json::json!({
            "firstName": "Alice",
            "lastName": "Smith",
            "language": "en"
        })
        .to_string();
        client_metadata.insert("sign_up_data".to_string(), sign_up_data);

        let user_attributes = std::collections::HashMap::new();
        assert_eq!(extract_name(&client_metadata, &user_attributes, &repo).await, "Alice");

        // Case 2: extract name from user_attributes name
        let client_metadata = std::collections::HashMap::new();
        let mut user_attributes = std::collections::HashMap::new();
        user_attributes.insert("name".to_string(), serde_json::json!("Bob"));
        assert_eq!(extract_name(&client_metadata, &user_attributes, &repo).await, "Bob");

        // Case 3: extract name from user_attributes given_name (name missing)
        let client_metadata = std::collections::HashMap::new();
        let mut user_attributes = std::collections::HashMap::new();
        user_attributes.insert("given_name".to_string(), serde_json::json!("Charlie"));
        assert_eq!(extract_name(&client_metadata, &user_attributes, &repo).await, "Charlie");

        // Case 4: extract name when all are missing
        let client_metadata = std::collections::HashMap::new();
        let user_attributes = std::collections::HashMap::new();
        assert_eq!(extract_name(&client_metadata, &user_attributes, &repo).await, "");
    }

    #[tokio::test]
    async fn test_pre_signup_allows_any_domain_when_not_sandbox() {
        let server = MockServer::start().await;

        let xml_response = r#"
            <GetSendQuotaResponse xmlns="https://email.amazonaws.com/doc/2010-12-01/">
                <GetSendQuotaResult>
                    <Max24HourSend>50000.0</Max24HourSend>
                    <SentLast24Hours>0.0</SentLast24Hours>
                    <MaxSendRate>14.0</MaxSendRate>
                </GetSendQuotaResult>
            </GetSendQuotaResponse>
        "#;

        Mock::given(method("POST"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200).set_body_string(xml_response))
            .mount(&server)
            .await;

        IS_INITIALIZED.store(false, Ordering::Relaxed);
        std::env::set_var("DOMAIN_NAME", "mycompany.com");

        let shared_config = backend::shared::aws_config::load_aws_config_for_mock(&server).await;
        let ddb_client = aws_sdk_dynamodb::Client::new(&shared_config);
        let repo = backend::shared::users::UserRepo::new(ddb_client, backend::shared::aws_config::SsmParameter::fixed("users"));

        let mut user_attributes = std::collections::HashMap::new();
        user_attributes.insert("email".to_string(), "external@gmail.com".to_string());

        let mut request = aws_lambda_events::cognito::CognitoEventUserPoolsPreSignupRequest::default();
        request.user_attributes = user_attributes;

        let mut pre_signup = aws_lambda_events::cognito::CognitoEventUserPoolsPreSignup::default();
        pre_signup.request = request;

        let event = LambdaEvent::new(
            CognitoUserPoolEvent::PreSignup(pre_signup),
            Default::default(),
        );

        let result = function_handler(event, &repo, &shared_config).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_pre_signup_rejects_other_domains_when_sandbox() {
        let server = MockServer::start().await;

        let xml_response = r#"
            <GetSendQuotaResponse xmlns="https://email.amazonaws.com/doc/2010-12-01/">
                <GetSendQuotaResult>
                    <Max24HourSend>200.0</Max24HourSend>
                    <SentLast24Hours>0.0</SentLast24Hours>
                    <MaxSendRate>1.0</MaxSendRate>
                </GetSendQuotaResult>
            </GetSendQuotaResponse>
        "#;

        Mock::given(method("POST"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200).set_body_string(xml_response))
            .mount(&server)
            .await;

        IS_INITIALIZED.store(false, Ordering::Relaxed);
        std::env::set_var("DOMAIN_NAME", "mycompany.com");

        let shared_config = backend::shared::aws_config::load_aws_config_for_mock(&server).await;
        let ddb_client = aws_sdk_dynamodb::Client::new(&shared_config);
        let repo = backend::shared::users::UserRepo::new(ddb_client, backend::shared::aws_config::SsmParameter::fixed("users"));

        let mut user_attributes = std::collections::HashMap::new();
        user_attributes.insert("email".to_string(), "external@gmail.com".to_string());

        let mut request = aws_lambda_events::cognito::CognitoEventUserPoolsPreSignupRequest::default();
        request.user_attributes = user_attributes;

        let mut pre_signup = aws_lambda_events::cognito::CognitoEventUserPoolsPreSignup::default();
        pre_signup.request = request;

        let event = LambdaEvent::new(
            CognitoUserPoolEvent::PreSignup(pre_signup),
            Default::default(),
        );

        let result = function_handler(event, &repo, &shared_config).await;
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert_eq!(err_msg, "Registration is just supported for mycompany.com addresses for now.");
    }

    #[tokio::test]
    async fn test_pre_signup_allows_same_domain_when_sandbox() {
        let server = MockServer::start().await;

        let xml_response = r#"
            <GetSendQuotaResponse xmlns="https://email.amazonaws.com/doc/2010-12-01/">
                <GetSendQuotaResult>
                    <Max24HourSend>200.0</Max24HourSend>
                    <SentLast24Hours>0.0</SentLast24Hours>
                    <MaxSendRate>1.0</MaxSendRate>
                </GetSendQuotaResult>
            </GetSendQuotaResponse>
        "#;

        Mock::given(method("POST"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200).set_body_string(xml_response))
            .mount(&server)
            .await;

        IS_INITIALIZED.store(false, Ordering::Relaxed);
        std::env::set_var("DOMAIN_NAME", "mycompany.com");

        let shared_config = backend::shared::aws_config::load_aws_config_for_mock(&server).await;
        let ddb_client = aws_sdk_dynamodb::Client::new(&shared_config);
        let repo = backend::shared::users::UserRepo::new(ddb_client, backend::shared::aws_config::SsmParameter::fixed("users"));

        let mut user_attributes = std::collections::HashMap::new();
        user_attributes.insert("email".to_string(), "user@mycompany.com".to_string());

        let mut request = aws_lambda_events::cognito::CognitoEventUserPoolsPreSignupRequest::default();
        request.user_attributes = user_attributes;

        let mut pre_signup = aws_lambda_events::cognito::CognitoEventUserPoolsPreSignup::default();
        pre_signup.request = request;

        let event = LambdaEvent::new(
            CognitoUserPoolEvent::PreSignup(pre_signup),
            Default::default(),
        );

        let result = function_handler(event, &repo, &shared_config).await;
        assert!(result.is_ok());
    }
}
