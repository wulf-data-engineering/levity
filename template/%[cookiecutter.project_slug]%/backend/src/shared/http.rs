use lambda_http::http::{header::CONTENT_TYPE, StatusCode};
use lambda_http::{Body, Error, Request, Response};
use serde::Serialize;
use anyhow::anyhow;

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

/// Gets the sub from the JWT in the request context.
#[cfg(not(any(debug_assertions, test)))]
pub fn get_sub(req: &Request) -> Result<String, Error> {
    use lambda_http::RequestExt;
    let request_context = req.request_context();
    request_context
        .authorizer()
        .and_then(|auth| {
            auth.jwt
                .as_ref()
                .and_then(|jwt| jwt.claims.get("sub").cloned())
        })
        .ok_or_else(|| anyhow!("Missing sub in claims").into())
}

/// Gets the sub from the JWT in Authorization header (in debug with localstack).
#[cfg(any(debug_assertions, test))]
pub fn get_sub(req: &Request) -> Result<String, Error> {
    req.headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .and_then(|token| {
            let parts: Vec<&str> = token.split('.').collect();
            if parts.len() == 3 {
                use base64::{engine::general_purpose, Engine as _};
                let payload = parts[1];
                let len = payload.len();
                let padded = if len % 4 != 0 {
                    let pad_len = 4 - (len % 4);
                    format!("{}{}", payload, "=".repeat(pad_len))
                } else {
                    payload.to_string()
                };

                if let Ok(decoded) = general_purpose::URL_SAFE.decode(padded) {
                    if let Ok(json) = serde_json::from_slice::<serde_json::Value>(&decoded) {
                        return json
                            .get("sub")
                            .and_then(|s| s.as_str())
                            .map(|s| s.to_string());
                    }
                }
            }
            None
        })
        .ok_or_else(|| anyhow!("Missing sub in claims").into())
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
}
