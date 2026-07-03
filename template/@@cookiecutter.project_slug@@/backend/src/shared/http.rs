use anyhow::anyhow;
use lambda_http::http::{header::CONTENT_TYPE, StatusCode};
use lambda_http::{Body, Error, Request, Response};
use serde::Serialize;

/// Creates a JSON HTTP response with status code 200 OK and matching Content-Type.
pub fn json_response<T>(value: T) -> Result<Response<Body>, Error>
where
    T: Serialize,
{
    json_with_status(value, StatusCode::OK)
}

/// Creates a JSON HTTP response with chosen status code and matching Content-Type.
pub fn json_with_status<T>(value: T, status: StatusCode) -> Result<Response<Body>, Error>
where
    T: Serialize,
{
    let body = serde_json::to_vec(&value)?;
    Response::builder()
        .status(status)
        .header(CONTENT_TYPE, "application/json")
        .body(Body::from(body))
        .map_err(Into::into)
}

/// Helper to extract a claim from the authorizer in the request context.
/// Supports both API Gateway V2 (JWT authorizer) and V1 (Cognito User Pools / custom context).
pub fn extract_claim_from_authorizer(
    auth: &aws_lambda_events::event::apigw::ApiGatewayRequestAuthorizer,
    claim_name: &str,
) -> Option<String> {
    // 1. Try V2 JWT claims
    if let Some(jwt) = &auth.jwt {
        if let Some(val) = jwt.claims.get(claim_name) {
            return Some(val.clone());
        }
    }

    // 2. Try V1 Cognito claims under fields["claims"]
    if let Some(claims_val) = auth.fields.get("claims") {
        if let Some(claim_val) = claims_val.get(claim_name) {
            if let Some(s) = claim_val.as_str() {
                return Some(s.to_string());
            }
        }
    }

    // 3. Try flat fields (fallback)
    if let Some(val) = auth.fields.get(claim_name) {
        if let Some(s) = val.as_str() {
            return Some(s.to_string());
        }
    }

    // 4. Try V1 Cognito claims under other["claims"] (catch-all fields nested)
    if let Some(claims_val) = auth.other.get("claims") {
        if let Some(claim_val) = claims_val.get(claim_name) {
            if let Some(s) = claim_val.as_str() {
                return Some(s.to_string());
            }
        }
    }

    // 5. Try flat other (catch-all fields fallback)
    if let Some(val) = auth.other.get(claim_name) {
        if let Some(s) = val.as_str() {
            return Some(s.to_string());
        }
    }

    None
}

/// Gets the sub from the authorizer in the request context.
#[cfg(not(any(debug_assertions, test)))]
pub fn get_sub(req: &Request) -> Result<String, Error> {
    use lambda_http::RequestExt;
    let request_context = req.request_context();
    request_context
        .authorizer()
        .and_then(|auth| extract_claim_from_authorizer(auth, "sub"))
        .ok_or_else(|| anyhow!("Missing sub in claims").into())
}

/// Gets the sub from the JWT in request's Authorization header (in debug with localstack).
#[cfg(any(debug_assertions, test))]
pub fn get_sub(req: &Request) -> Result<String, Error> {
    let auth_header = req
        .headers()
        .get("authorization")
        .or_else(|| req.headers().get("Authorization"))
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| anyhow!("Missing Authorization header"))?;

    // Robustness: Handle multiple values (some proxies or combined headers may result in commas)
    for value in auth_header.split(',') {
        let value = value.trim();
        if let Ok(sub) = get_sub_from_authorization(value) {
            return Ok(sub);
        }
    }

    Err(anyhow!("Missing sub in claims").into())
}

/// Extracts the sub from the JWT in given Authorization header (in debug with localstack).
#[cfg(any(debug_assertions, test))]
pub fn get_sub_from_authorization(authorization_header: &str) -> Result<String, Error> {
    authorization_header.strip_prefix("Bearer ")
        .and_then(|token| get_claims_from_token(token).ok())
        .and_then(|claims| claims.get("sub").and_then(|s| s.as_str()).map(|s| s.to_string()))
        .ok_or_else(|| anyhow!("Missing sub in claims").into())
}

/// Decodes the JWT payload without validation.
pub fn get_claims_from_token(token: &str) -> Result<serde_json::Value, Error> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return Err(anyhow!("Invalid token format").into());
    }

    use base64::{engine::general_purpose, Engine as _};
    let payload = parts[1];
    let len = payload.len();
    let padded = if !len.is_multiple_of(4) {
        let pad_len = 4 - (len % 4);
        format!("{}{}", payload, "=".repeat(pad_len))
    } else {
        payload.to_string()
    };

    let decoded = general_purpose::URL_SAFE.decode(padded)
        .map_err(|e| anyhow!("Failed to decode base64: {}", e))?;
    
    let json = serde_json::from_slice::<serde_json::Value>(&decoded)
        .map_err(|e| anyhow!("Failed to parse JSON: {}", e))?;

    Ok(json)
}

#[cfg(test)]
mod tests {
    use super::json_response;
    use lambda_http::http::{header::CONTENT_TYPE, StatusCode};
    use lambda_http::{Body, Response};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Foo {
        bar: usize,
    }

    fn sample() -> Foo {
        Foo { bar: 8 }
    }

    fn body_bytes(resp: &Response<Body>) -> Vec<u8> {
        match resp.body() {
            Body::Empty => Vec::new(),
            Body::Text(s) => s.as_bytes().to_vec(),
            Body::Binary(b) => b.clone(),
            _ => panic!("Unexpected body variant"),
        }
    }

    #[test]
    fn json_response_plain_value_ok() {
        let resp = json_response(sample()).expect("response should be Ok");
        assert_eq!(resp.status(), StatusCode::OK);

        let ct = resp.headers().get(CONTENT_TYPE).unwrap();
        assert_eq!(ct, "application/json");

        let got: Foo = serde_json::from_slice(&body_bytes(&resp)).unwrap();
        assert_eq!(got, sample());
    }

    #[test]
    fn test_extract_claim_v2() {
        use aws_lambda_events::event::apigw::ApiGatewayV2httpRequestContext;
        use serde_json::json;

        let context: ApiGatewayV2httpRequestContext = serde_json::from_value(json!({
            "authorizer": {
                "jwt": {
                    "claims": {
                        "sub": "test-sub-v2"
                    }
                }
            },
            "http": {
                "method": "GET"
            }
        }))
        .unwrap();
        let auth = context.authorizer.as_ref().expect("authorizer should be present");

        assert_eq!(
            super::extract_claim_from_authorizer(auth, "sub"),
            Some("test-sub-v2".to_string())
        );
    }

    #[test]
    fn test_extract_claim_v1_nested() {
        use aws_lambda_events::event::apigw::ApiGatewayProxyRequestContext;
        use serde_json::json;

        let context: ApiGatewayProxyRequestContext = serde_json::from_value(json!({
            "authorizer": {
                "claims": {
                    "sub": "test-sub-v1"
                }
            },
            "httpMethod": "GET"
        }))
        .unwrap();
        let auth = &context.authorizer;

        assert_eq!(
            super::extract_claim_from_authorizer(auth, "sub"),
            Some("test-sub-v1".to_string())
        );
    }

    #[test]
    fn test_extract_claim_v1_flat() {
        use aws_lambda_events::event::apigw::ApiGatewayProxyRequestContext;
        use serde_json::json;

        let context: ApiGatewayProxyRequestContext = serde_json::from_value(json!({
            "authorizer": {
                "sub": "test-sub-flat"
            },
            "httpMethod": "GET"
        }))
        .unwrap();
        let auth = &context.authorizer;

        assert_eq!(
            super::extract_claim_from_authorizer(auth, "sub"),
            Some("test-sub-flat".to_string())
        );
    }

    #[test]
    fn test_extract_claim_v2_other_nested() {
        use aws_lambda_events::event::apigw::ApiGatewayV2httpRequestContext;
        use serde_json::json;

        let context: ApiGatewayV2httpRequestContext = serde_json::from_value(json!({
            "authorizer": {
                "claims": {
                    "sub": "test-sub-v2-other-nested"
                }
            },
            "http": {
                "method": "GET"
            }
        }))
        .unwrap();
        let auth = context.authorizer.as_ref().expect("authorizer should be present");

        assert_eq!(
            super::extract_claim_from_authorizer(auth, "sub"),
            Some("test-sub-v2-other-nested".to_string())
        );
    }

    #[test]
    fn test_extract_claim_v2_other_flat() {
        use aws_lambda_events::event::apigw::ApiGatewayV2httpRequestContext;
        use serde_json::json;

        let context: ApiGatewayV2httpRequestContext = serde_json::from_value(json!({
            "authorizer": {
                "sub": "test-sub-v2-other-flat"
            },
            "http": {
                "method": "GET"
            }
        }))
        .unwrap();
        let auth = context.authorizer.as_ref().expect("authorizer should be present");

        assert_eq!(
            super::extract_claim_from_authorizer(auth, "sub"),
            Some("test-sub-v2-other-flat".to_string())
        );
    }

    #[test]
    fn test_get_sub_with_multiple_headers() {
        use lambda_http::http::Request;
        let payload = serde_json::json!({
            "sub": "test-sub-multi"
        })
        .to_string();
        use base64::{engine::general_purpose, Engine as _};
        let encoded_payload = general_purpose::URL_SAFE.encode(payload);
        let token = format!("header.{}.signature", encoded_payload);

        // Combined header value (simulating multiple headers)
        let combined_value = format!("Bearer {}, Bearer {}", token, token);

        let request = Request::builder()
            .header("Authorization", combined_value)
            .body(lambda_http::Body::Empty)
            .unwrap();

        assert_eq!(super::get_sub(&request).unwrap(), "test-sub-multi");
    }
}
