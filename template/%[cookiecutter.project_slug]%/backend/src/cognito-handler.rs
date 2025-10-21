use aws_lambda_events::cognito::CognitoEventUserPoolsPreSignupRequest;
use backend::CognitoUserPoolEvent;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use tracing_subscriber;

///
/// This lambda reacts on Cognito's lifecycle events.
/// The default version just enables auto-confirm of users signing up in development
/// (except Email addresses of form [...]confirm@bar.baz which need confirmation on dev, too).
///
/// If you add more cases, make sure to add them to local/cognito-local-volume/config.json
///
async fn function_handler(
    event: LambdaEvent<CognitoUserPoolEvent>,
) -> Result<CognitoUserPoolEvent, Error> {
    let mut cognito_event = event.payload;
    match &mut cognito_event {
        CognitoUserPoolEvent::PreSignup(pre_sign_up) => {
            // auto-confirms new users in debug mode (except +confirm Email addresses)
            pre_sign_up.response.auto_confirm_user = auto_confirm(&pre_sign_up.request);
        }
        CognitoUserPoolEvent::PostConfirmation(_post_confirmation) => {
            // E.g. write an entry to a database table
        }
        CognitoUserPoolEvent::CustomMessage(_custom_message) => {
            // Set custom messages for confirm and password forgotten
            // (depending on `_custom_message.cognito_event_user_pools_header.trigger_source`)
        }
        // Check the enum `CognitoUserPoolEvent` for more lifecycle events
        _ => {}
    }

    Ok(cognito_event)
}

#[cfg(not(debug_assertions))]
fn auto_confirm(_: &CognitoEventUserPoolsPreSignupRequest) -> bool {
    false
}

// On development mode just require confirm for Email addresses of form [...]confirm@bar.baz
#[cfg(debug_assertions)]
fn auto_confirm(request: &CognitoEventUserPoolsPreSignupRequest) -> bool {
    !request
        .user_attributes
        .get("email")
        .iter()
        .any(|email| email.contains("confirm@"))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();
    run(service_fn(function_handler)).await
}
